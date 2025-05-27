mod alu;
mod decoder;
mod execution;
mod flow;
mod io_matrix;
mod mmu;
#[cfg(test)]
mod tpu_test;

use crate::shared::{
    AnalogPin, DecodeResult, DigitalPin, HaltReason, Instruction, NetPacket, Register,
};
use crate::shared::{ExecuteResult, OperandValueType};
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;
use strum::{EnumCount, IntoEnumIterator};
use tracing::{error, trace};

#[derive(Clone)]
pub struct TpuState {
    /// Stack for operations
    pub stack: Vec<u16>,
    /// Analog I/O
    pub analog_pins: [u16; AnalogPin::COUNT],
    /// Digital I/O
    pub digital_pins: [bool; DigitalPin::COUNT],
    /// Analog Pin configurations (true = input, false = output)
    pub analog_pin_config: [bool; AnalogPin::COUNT],
    /// Digital Pin configurations (true = input, false = output)
    pub digital_pin_config: [bool; DigitalPin::COUNT],
    /// Memory
    pub ram: [u16; TPU::RAM_SIZE],
    /// The program ROM
    pub rom: Vec<Rc<Instruction>>,
    /// My network address
    pub network_address: u16,
    /// Queue of incoming packets
    pub incoming_packets: VecDeque<NetPacket>,
    /// Queue of outgoing packets
    pub outgoing_packets: VecDeque<NetPacket>,
    /// Registers (A, X, Y, R1-R6)
    pub registers: [u16; Register::COUNT],
    /// Tracks the current line of program
    pub program_counter: usize,
    /// Are we in an error state?
    pub halted: bool,
    /// The state of the current execution (if any)
    pub execution_state: ExecutionState,
}

#[derive(Clone, Debug, Default)]
pub struct ExecutionState {
    /// This is the function that we execute when `wait_cycles` reaches zero.
    /// It actually executes the instruction that we previously decoded.
    pub instruction: Option<Rc<Instruction>>,
    /// Track how many cycles are left until the current instruction is finished.
    pub wait_cycles: u16,
    /// Should the current instruction be called every cycle until finished?
    pub execute_each_cycle: bool,
}

impl fmt::Display for TpuState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Helper function to format u16 as hex with leading zeros, split into 4-char blocks
        // fn format_u16_hex(value: u16) -> String {
        //     format!("{:04x}", value)
        // }
        //
        // // Helper function to format an array of u16 values
        // fn format_u16_array<const N: usize>(arr: &[u16; N]) -> String {
        //     let mut result = String::new();
        //     for (i, &value) in arr.iter().enumerate() {
        //         if i > 0 && i % 4 == 0 {
        //             result.push('\n');
        //         }
        //         result.push_str(&format!("{:04x} ", value));
        //     }
        //     result
        // }

        // UTF-8 box drawing characters
        let h_line = "─";
        let v_line = "│";
        let tl_corner = "┌";
        let tr_corner = "┐";
        let bl_corner = "└";
        let br_corner = "┘";
        let t_down = "┬";
        let t_up = "┴";
        let t_right = "├";
        let t_left = "┤";

        // Create a header
        writeln!(f, "{}{}{}", tl_corner, h_line.repeat(59), tr_corner)?;
        writeln!(
            f,
            "{} TPU State Display                                         {}",
            v_line, v_line
        )?;
        writeln!(
            f,
            "{}{}{}{}{}",
            t_right,
            h_line.repeat(29),
            t_down,
            h_line.repeat(29),
            t_left
        )?;
        // System Status
        writeln!(
            f,
            "{} System Status               {} Network                     {}",
            v_line, v_line, v_line
        )?;
        writeln!(
            f,
            "{} Program Counter: {:08x}   {} Network Address:  {:04x}      {}",
            v_line, self.program_counter, v_line, self.network_address, v_line
        )?;
        writeln!(
            f,
            "{} Wait Cycles:     {:04x}       {} Incoming Packets: {:04x}      {}",
            v_line,
            self.execution_state.wait_cycles,
            v_line,
            self.incoming_packets.len(),
            v_line
        )?;
        writeln!(
            f,
            "{} Halted: {:<19} {} Outgoing Packets: {:04x}      {}",
            v_line,
            self.halted,
            v_line,
            self.outgoing_packets.len(),
            v_line
        )?;
        writeln!(
            f,
            "{}{}{}{}{}",
            t_right,
            h_line.repeat(29),
            t_up,
            h_line.repeat(29),
            t_left
        )?;

        // Registers
        writeln!(
            f,
            "{} Registers                                                 {}",
            v_line, v_line
        )?;
        write!(f, "{} ", v_line)?;
        for (i, reg) in Register::iter().enumerate() {
            let value = self.registers[i];
            write!(
                f,
                "{}{:?}: {:04x} ",
                if format!("{:?}", reg).len() == 1 {
                    " "
                } else {
                    ""
                },
                reg,
                value
            )?;
            if (i + 1) % 5 == 0 && i < Register::COUNT - 1 {
                writeln!(f, "             {}", v_line)?;
                write!(f, "{} ", v_line)?;
            }
        }
        writeln!(f, "             {}", v_line)?;
        writeln!(f, "{}{}{}", t_right, h_line.repeat(59), t_left)?;

        // Stack
        writeln!(
            f,
            "{} Stack (Size: {:04x})                                        {}",
            v_line,
            self.stack.len(),
            v_line
        )?;
        if self.stack.is_empty() {
            writeln!(
                f,
                "{} <empty>                                                   {}",
                v_line, v_line
            )?;
        } else {
            let mut line_n = 0;
            for (i, &value) in self.stack.iter().enumerate() {
                if i % 8 == 0 && i > 0 {
                    writeln!(f, "                   {}", v_line)?;
                    write!(f, "{} ", v_line)?;
                    line_n = 0;
                } else if i > 0 {
                    write!(f, " ")?;
                } else {
                    write!(f, "{} ", v_line)?;
                }
                line_n += 1;
                write!(f, "{:04x}", value)?;
            }
            writeln!(f, "{:>width$}{}", "", v_line, width = 59 - (line_n * 5))?;
        }
        writeln!(f, "{}{}{}", t_right, h_line.repeat(59), t_left)?;

        // RAM
        writeln!(
            f,
            "{} RAM                                                       {}",
            v_line, v_line
        )?;
        for i in 0..TPU::RAM_SIZE {
            if i % 8 == 0 {
                if i > 0 {
                    writeln!(f, "               {}", v_line)?;
                }
                write!(f, "{} {:02x}: ", v_line, i)?;
            } else {
                write!(f, " ")?;
            }
            write!(f, "{:04x}", self.ram[i])?;
        }
        writeln!(f, "               {}", v_line)?;
        writeln!(f, "{}{}{}", t_right, h_line.repeat(59), t_left)?;

        // I/O Pins
        writeln!(
            f,
            "{} I/O Pins                                                  {}",
            v_line, v_line
        )?;

        // Analog pins
        write!(f, "{} Analog:  ", v_line)?;
        for (i, _) in AnalogPin::iter().enumerate() {
            let value = self.analog_pins[i];
            let config = if self.analog_pin_config[i] { "I" } else { "O" };
            write!(f, "{}{}:{:04x} ", config, i, value)?;
        }
        writeln!(
            f,
            "{:>width$}{}",
            "",
            v_line,
            width = 49 - AnalogPin::COUNT * 8
        )?;

        // Digital pins
        write!(f, "{} Digital: ", v_line)?;
        for (i, _) in DigitalPin::iter().enumerate() {
            let value = if self.digital_pins[i] { "1" } else { "0" };
            let config = if self.digital_pin_config[i] { "I" } else { "O" };
            write!(f, "{}{}:{} ", config, i, value)?;
        }
        writeln!(
            f,
            "{:>width$}{}",
            "",
            v_line,
            width = 49 - DigitalPin::COUNT * 5
        )?;

        // Footer
        writeln!(f, "{}{}{}", bl_corner, h_line.repeat(59), br_corner)?;

        Ok(())
    }
}

/// A simple Traffic Processing Unit (TPU) Virtual Machine
#[derive(Clone)]
pub struct TPU {
    tpu_state: TpuState,
}

impl fmt::Display for TPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tpu_state)
    }
}

impl TPU {
    pub const STACK_SIZE: usize = 16;
    pub const NET_BUFFER_SIZE: usize = 8;
    pub const RAM_SIZE: usize = 128;

    // Helper function to get a value from an operand
    // Returns a tuple (delay, value) where delay is 1 for register access, 0 for constant
    pub fn get_operand_value(&self, operand: &OperandValueType) -> u16 {
        match operand {
            OperandValueType::Register(reg) => self.read_register(*reg),
            OperandValueType::Immediate(val) => *val,
        }
    }

    pub fn check_operand_cost(operands: &[&OperandValueType]) -> u16 {
        let mut cost = 0;
        for operand in operands {
            if matches!(operand, OperandValueType::Register(_)) {
                cost += 1;
            }
        }
        cost
    }

    /// Create a new TPU VM with a specified network address and pin configurations
    pub fn new(
        network_address: u16,
        analog_pin_config: [bool; AnalogPin::COUNT],
        digital_pin_config: [bool; DigitalPin::COUNT],
        program: Vec<Rc<Instruction>>,
    ) -> Self {
        let mut tpu = Self {
            tpu_state: TpuState {
                stack: Vec::new(),
                analog_pins: [0; AnalogPin::COUNT],
                digital_pins: [false; DigitalPin::COUNT],
                analog_pin_config,
                digital_pin_config,
                ram: [0; TPU::RAM_SIZE],
                rom: program,
                network_address,
                incoming_packets: VecDeque::new(),
                outgoing_packets: VecDeque::new(),
                registers: [0; Register::COUNT],
                program_counter: 0,
                halted: false,
                execution_state: ExecutionState {
                    instruction: None,
                    wait_cycles: 0,
                    execute_each_cycle: false,
                },
            },
        };

        tpu.reset();
        tpu
    }

    pub fn new_from_state(tpu_state: TpuState) -> TPU {
        TPU { tpu_state }
    }

    fn reset(&mut self) {
        trace!("RESET");

        // Clear stack
        self.tpu_state.stack.clear();

        // Clear program counter
        self.tpu_state.program_counter = 0;

        // Clear halt
        self.tpu_state.halted = false;

        // Clear execution state
        self.tpu_state.execution_state = ExecutionState::default();

        // Reset registers
        for register in Register::iter() {
            self.write_register(register, 0);
        }

        // Clear RAM
        for index in 0..TPU::RAM_SIZE {
            self.tpu_state.ram[index] = 0;
        }

        // Clear network buffers
        self.tpu_state.incoming_packets.clear();
        self.tpu_state.outgoing_packets.clear();

        // Reset I/O pins
        for pin in DigitalPin::iter() {
            self.set_digital_pin(pin, false);
        }
        for pin in AnalogPin::iter() {
            self.set_analog_pin(pin, 0);
        }
    }

    /// Allow the CPU to execute for a single clock cycle
    pub fn tick(&mut self) {
        trace!("TICK");
        self.decrement_wait_cycles();

        if self.tpu_state.halted {
            return;
        }

        // If we don't need to execute each cycle, and there's still wait cycles left, do nothing
        if !self.tpu_state.execution_state.execute_each_cycle
            && self.tpu_state.execution_state.wait_cycles > 0
        {
            return;
        }

        // If we have a decoded instruction ready, execute it now
        if let Some(instruction) = self.tpu_state.execution_state.instruction.take() {
            self.execute_instruction(instruction, self.tpu_state.execution_state.wait_cycles);
            return;
        }

        self.fetch_instruction()
    }

    fn decrement_wait_cycles(&mut self) {
        self.tpu_state.execution_state.wait_cycles =
            self.tpu_state.execution_state.wait_cycles.saturating_sub(1);
    }

    /// Executes until the next instruction is complete
    pub fn step(&mut self) {
        trace!("STEP");
        let old_pc = self.tpu_state.program_counter;
        while !self.tpu_state.halted && self.tpu_state.program_counter == old_pc {
            self.tick();
        }
    }

    fn fetch_instruction(&mut self) {
        let instruction = self.tpu_state.rom[self.tpu_state.program_counter].clone();
        let result = decoder::decode(&instruction);

        // This instruction executes in a single clock cycle, so do it now.
        if result.cycles == 1 {
            self.execute_instruction(instruction, 1);
            return;
        } else {
            // Subtract 1 from the number of cycles to wait because this counts as a cycle
            self.tpu_state.execution_state.wait_cycles = result.cycles - 1;
            self.tpu_state.execution_state.execute_each_cycle = result.call_every_cycle;
            self.tpu_state.execution_state.instruction = Some(instruction);
        }
    }

    fn execute_instruction(&mut self, instruction: Rc<Instruction>, wait_cycles: u16) {
        let result = execution::execute(self, &instruction, wait_cycles);

        match result {
            ExecuteResult::PCAdvance => {
                // Clear the execution state
                self.tpu_state.execution_state.wait_cycles = 0;
                self.tpu_state.execution_state.instruction = None;
                self.tpu_state.execution_state.execute_each_cycle = false;

                // Advance the program counter
                // Check that the program counter is not going out of bounds
                if self.tpu_state.program_counter + 1 > (self.tpu_state.rom.len() - 1) {
                    self.tpu_state.halted = true;
                }
                self.tpu_state.program_counter += 1;
            }
            ExecuteResult::PCModified => {
                self.tpu_state.execution_state.wait_cycles = 0;
                self.tpu_state.execution_state.instruction = None;
                self.tpu_state.execution_state.execute_each_cycle = false;
                return;
            }
            ExecuteResult::NoPCAdvance => {
                self.tpu_state.execution_state.instruction = Some(instruction)
            }
            ExecuteResult::Halt(reason) => {
                error!("TPU Halted: {reason:?}");
                self.tpu_state.halted = true
            }
        }
    }

    pub fn busy(&self) -> bool {
        self.tpu_state.execution_state.wait_cycles > 0
    }

    pub fn halted(&self) -> bool {
        self.tpu_state.halted
    }

    pub fn state(&self) -> &TpuState {
        &self.tpu_state
    }

    /// Read the value of a register
    pub fn read_register(&self, register: Register) -> u16 {
        self.tpu_state.registers[register as usize]
    }

    /// Write a value to a register
    fn write_register(&mut self, register: Register, value: u16) {
        self.tpu_state.registers[register as usize] = value;
    }

    /// Push a value onto the stack
    fn push(&mut self, value: u16) {
        self.tpu_state.stack.push(value);
    }

    /// Pop a value from the stack
    fn pop(&mut self) -> u16 {
        self.tpu_state.stack.pop().unwrap_or(0)
    }

    /// Set an analog pin value
    /// If the pin is configured as an input, this function does nothing
    fn set_analog_pin(&mut self, pin: AnalogPin, value: u16) {
        // Check if the pin is configured as an input (true)
        if self.tpu_state.analog_pin_config[pin as usize] {
            // Pin is an input, do nothing
            return;
        }
        // Pin is an output, set the value
        self.tpu_state.analog_pins[pin as usize] = value;
    }

    /// Get an analog input value
    pub fn get_analog_pin(&self, pin: AnalogPin) -> u16 {
        self.tpu_state.analog_pins[pin as usize]
    }

    /// Set a digital pin value
    /// If the pin is configured as an input, this function does nothing
    fn set_digital_pin(&mut self, pin: DigitalPin, value: bool) {
        // Check if the pin is configured as an input (true)
        if self.tpu_state.digital_pin_config[pin as usize] {
            // Pin is an input, do nothing
            return;
        }
        // Pin is an output, set the value
        self.tpu_state.digital_pins[pin as usize] = value;
    }

    pub fn set_digital_pins(&mut self, word: u16) {
        // Apply the word to the digital pins
        for pin in DigitalPin::iter() {
            let bit = word & (1 << pin as u16);
            self.set_digital_pin(pin, bit != 0);
        }
    }

    pub fn get_digital_pins(&self) -> u16 {
        // Get the current digital pin values
        let mut word = 0;
        for pin in DigitalPin::iter() {
            word |= (self.get_digital_pin(pin) as u16) << pin as u16;
        }
        word
    }

    /// Get a digital input value
    fn get_digital_pin(&self, pin: DigitalPin) -> bool {
        self.tpu_state.digital_pins[pin as usize]
    }

    /// Read a byte from RAM
    pub fn read_ram(&self, address: usize) -> u16 {
        if address < self.tpu_state.ram.len() {
            self.tpu_state.ram[address]
        } else {
            0
        }
    }

    /// Get the RAM size
    pub fn ram_size(&self) -> usize {
        self.tpu_state.ram.len()
    }

    /// Write a byte to RAM
    fn write_ram(&mut self, address: usize, value: u16) {
        if address < self.tpu_state.ram.len() {
            self.tpu_state.ram[address] = value;
        }
    }

    pub fn read_rom(&self) -> &Vec<Rc<Instruction>> {
        &self.tpu_state.rom
    }

    /// Send a packet
    fn send_packet(&mut self, address: u16, data: u16) {
        self.tpu_state.outgoing_packets.push_back(NetPacket {
            sender: self.tpu_state.network_address,
            target: address,
            data,
        });
    }

    /// Receive a packet, if one is available
    /// Returns 0 if no packet is available
    fn receive_packet(&mut self) -> NetPacket {
        self.tpu_state
            .incoming_packets
            .pop_front()
            .unwrap_or_default()
    }

    /// Get the current stack pointer (size of the stack)
    pub fn stack_pointer(&self) -> u16 {
        self.tpu_state.stack.len() as u16
    }

    // Misc operations
    fn op_nop() -> ExecuteResult {
        // Sleep is handled by the wait_cycles mechanism
        // No additional action needed here
        ExecuteResult::PCAdvance
    }

    fn op_slp(&mut self, value: &OperandValueType) -> ExecuteResult {
        // Get the sleep duration
        let delay = TPU::check_operand_cost(&[value]).saturating_add(self.get_operand_value(value));
        self.tpu_state.execution_state.wait_cycles = delay;
        ExecuteResult::PCAdvance
    }

    fn decode_op_nop() -> DecodeResult {
        DecodeResult {
            cycles: 1,
            call_every_cycle: false,
        }
    }

    fn decode_op_slp() -> DecodeResult {
        DecodeResult {
            cycles: 1, // We'll modify this in the op_slp
            call_every_cycle: false,
        }
    }

    fn op_wrx(tpu: &mut TPU) -> ExecuteResult {
        // Check if there are any incoming packets
        if tpu.tpu_state.incoming_packets.is_empty() {
            // Keep resetting the wait cycles until we get a packet
            // If none ever arrives, we're basically stuck in an infinite loop
            tpu.tpu_state.execution_state.wait_cycles = 1;
            ExecuteResult::NoPCAdvance
        } else {
            tpu.tpu_state.execution_state.wait_cycles = 4;
            io_matrix::op_recv(tpu);
            ExecuteResult::PCAdvance
        }
    }

    fn decode_op_wrx() -> DecodeResult {
        DecodeResult {
            cycles: 65535,
            call_every_cycle: true,
        }
    }

    fn op_hlt() -> ExecuteResult {
        ExecuteResult::Halt(HaltReason::HLTOpcode)
    }

    fn decode_op_hlt() -> DecodeResult {
        DecodeResult {
            cycles: 1,
            call_every_cycle: false,
        }
    }
}

pub fn create_basic_tpu_config<'t>(program: Vec<Rc<Instruction>>) -> TPU {
    TPU::new(
        0x1,
        [false; AnalogPin::COUNT],
        [false; DigitalPin::COUNT],
        program,
    )
}
