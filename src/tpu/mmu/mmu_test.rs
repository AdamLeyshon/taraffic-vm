use crate::shared::{ExecuteResult, OperandValueType, Register};
use crate::tpu::mmu::*;
use crate::tpu::{ExecutionState, TPU, TpuState, create_basic_tpu_config};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{AnalogPin, DigitalPin, HaltReason};
    use strum::EnumCount;

    // Helper function to create a TPU with specific register values
    fn create_tpu_with_registers(a: u16, x: u16, y: u16) -> TPU {
        let mut tpu_state = TpuState {
            stack: Vec::new(),
            analog_pins: [0; AnalogPin::COUNT],
            digital_pins: [false; DigitalPin::COUNT],
            analog_pin_config: [false; AnalogPin::COUNT],
            digital_pin_config: [true; DigitalPin::COUNT],

            ram: [0; TPU::RAM_SIZE],
            rom: vec![],
            network_address: 0x1,
            incoming_packets: std::collections::VecDeque::new(),
            outgoing_packets: std::collections::VecDeque::new(),
            registers: [0; Register::COUNT],

            program_counter: 0,
            halted: false,
            execution_state: ExecutionState::default(),
        };

        // Set register values
        tpu_state.registers[Register::A as usize] = a;
        tpu_state.registers[Register::X as usize] = x;
        tpu_state.registers[Register::Y as usize] = y;

        TPU::new_from_state(tpu_state)
    }

    // Helper function to create a TPU with specific RAM values
    fn create_tpu_with_ram(ram_values: &[(usize, u16)]) -> TPU {
        let mut tpu = create_tpu_with_registers(0, 0, 0);

        // Set RAM values
        for (address, value) in ram_values {
            tpu.write_ram(*address, *value);
        }

        tpu
    }

    #[test]
    fn test_op_rcy() {
        // Test case 1: Copy from one register to another
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let result = op_rcy(&mut tpu, &Register::A, &Register::X);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 20); // A now has X's value
        assert_eq!(tpu.read_register(Register::X), 20); // X remains unchanged

        // Test case 2: Copy between general registers
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        tpu.write_register(Register::R0, 42);
        let result = op_rcy(&mut tpu, &Register::R1, &Register::R0);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::R1), 42); // R1 now has R0's value
        assert_eq!(tpu.read_register(Register::R0), 42); // R0 remains unchanged
    }

    #[test]
    fn test_op_rmv() {
        // Test case 1: Move from one register to another
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let result = op_rmv(&mut tpu, &Register::A, &Register::X);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 20); // A now has X's value
        assert_eq!(tpu.read_register(Register::X), 0); // X is now zero

        // Test case 2: Move between general registers
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        tpu.write_register(Register::R0, 42);
        let result = op_rmv(&mut tpu, &Register::R1, &Register::R0);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::R1), 42); // R1 now has R0's value
        assert_eq!(tpu.read_register(Register::R0), 0); // R0 is now zero
    }
    
    #[test]
    fn test_op_ldr() {
        // Test case 1: Load constant into register
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let result = op_ldr(&mut tpu, &Register::A, &OperandValueType::Immediate(42));
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 42); // A now has the constant value

        // Test case 2: Load from one register to another
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let result = op_ldr(
            &mut tpu,
            &Register::A,
            &OperandValueType::Register(Register::Y),
        );
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 30); // A now has Y's value
    }

    #[test]
    fn test_op_ldm() {
        // Test case 1: Load from memory into register using immediate address
        let ram_values = [(7, 42)];
        let mut tpu = create_tpu_with_ram(&ram_values);
        let result = op_ldm(
            &mut tpu,
            &Register::A,
            &OperandValueType::Immediate(7), // Address
        );
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 42); // A now has the value from memory

        // Test case 2: Load from memory using register address
        let ram_values = [(9, 99)];
        let mut tpu = create_tpu_with_ram(&ram_values);
        tpu.write_register(Register::X, 9); // Address in X
        let result = op_ldm(
            &mut tpu,
            &Register::Y,
            &OperandValueType::Register(Register::X), // Address from X
        );
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::Y), 99); // Y now has the value from memory
        assert_eq!(tpu.read_register(Register::X), 9); // X remains unchanged
    }
    
    #[test]
    fn test_op_stm() {
        // Test case 1: Store constant into memory
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let result = op_stm(
            &mut tpu,
            &OperandValueType::Immediate(7),  // Address
            &OperandValueType::Immediate(42), // Value
        );
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_ram(7), 42); // Memory at address 7 now has the constant value

        // Test case 2: Store with register address and register value
        let mut tpu = create_tpu_with_registers(10, 9, 30);
        let result = op_stm(
            &mut tpu,
            &OperandValueType::Register(Register::X), // Address from X
            &OperandValueType::Register(Register::A), // Value from A
        );
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_ram(9), 10); // Memory at address 9 now has A's value
    }
    
    #[test]
    fn test_op_stmo() {
        // Test case 1: Store register into memory with offset
        let mut tpu = create_tpu_with_registers(10, 5, 30);
        let result = op_stmo(
            &mut tpu,
            &OperandValueType::Immediate(17), // Base address
            &OperandValueType::Register(Register::A),
            &Register::X,
        );
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_ram(22), 10); // Memory at address 22 now has A's value
        assert_eq!(tpu.read_register(Register::X), 5); // X remains unchanged

        // Test case 2: Store with register address
        let mut tpu = create_tpu_with_registers(10, 5, 18);
        let result = op_stmo(
            &mut tpu,
            &OperandValueType::Register(Register::Y), // Base address from Y
            &OperandValueType::Register(Register::A),
            &Register::X,
        );
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_ram(23), 10); // Memory at address 23 now has A's value
        assert_eq!(tpu.read_register(Register::X), 5); // X remains unchanged
    }

    #[test]
    fn test_op_smoi() {
        // Test case 1: Store register into memory with offset and increment offset register
        let mut tpu = create_tpu_with_registers(10, 5, 30);
        let result = op_smoi(
            &mut tpu,
            &OperandValueType::Immediate(17), // Base address
            &OperandValueType::Register(Register::A),
            &Register::X,
        );
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_ram(22), 10); // Memory at address 22 now has A's value
        assert_eq!(tpu.read_register(Register::X), 6); // X is incremented

        // Test case 2: Store with register address
        let mut tpu = create_tpu_with_registers(10, 5, 18);
        let result = op_smoi(
            &mut tpu,
            &OperandValueType::Register(Register::Y), // Base address from Y
            &OperandValueType::Register(Register::A),
            &Register::X,
        );
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_ram(23), 10); // Memory at address 23 now has A's value
        assert_eq!(tpu.read_register(Register::X), 6); // X is incremented

        // Test case 3: X wrapping around on increment
        let mut tpu = create_tpu_with_registers(10, 65535, 30); // X at max value
        let result = op_smoi(
            &mut tpu,
            &OperandValueType::Immediate(19),
            &OperandValueType::Register(Register::A),
            &Register::X,
        );
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::X), 0); // X wraps around to 0
    }

    #[test]
    fn test_with_basic_tpu_config() {
        // Test using create_basic_tpu_config
        let program = vec![
            // No instructions needed for this test
        ];
        let mut tpu = create_basic_tpu_config(program);

        // Test RCY operation
        tpu.write_register(Register::X, 42);
        op_rcy(&mut tpu, &Register::A, &Register::X);
        assert_eq!(tpu.read_register(Register::A), 42);
        assert_eq!(tpu.read_register(Register::X), 42);

        // Test RMV operation
        tpu.write_register(Register::Y, 24);
        op_rmv(&mut tpu, &Register::A, &Register::Y);
        assert_eq!(tpu.read_register(Register::A), 24);
        assert_eq!(tpu.read_register(Register::Y), 0);

        // Test STM operation
        op_stm(
            &mut tpu,
            &OperandValueType::Immediate(25),
            &OperandValueType::Immediate(55),
        );
        assert_eq!(tpu.read_ram(25), 55);

        // Test LDR operation with immediate value
        op_ldr(&mut tpu, &Register::A, &OperandValueType::Immediate(99));
        assert_eq!(tpu.read_register(Register::A), 99);
    }

    #[test]
    fn test_op_push_and_pop() {
        let mut tpu = create_tpu_with_registers(10, 20, 30);

        // Test PUSH
        let result = op_push(&mut tpu, &OperandValueType::Immediate(42));
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.tpu_state.stack.len(), 1);
        assert_eq!(tpu.tpu_state.stack[0], 42);

        // Push a register value
        let result = op_push(&mut tpu, &OperandValueType::Register(Register::A));
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.tpu_state.stack.len(), 2);
        assert_eq!(tpu.tpu_state.stack[1], 10);

        // Test POP
        let result = op_pop(&mut tpu, &Register::X);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.tpu_state.stack.len(), 1);
        assert_eq!(tpu.read_register(Register::X), 10);

        // Test stack overflow
        // Clear the stack first
        let _ = op_scr(&mut tpu);

        // Fill the stack to capacity (TPU::STACK_SIZE = 16)
        for i in 0..TPU::STACK_SIZE {
            let result = op_push(&mut tpu, &OperandValueType::Immediate(i as u16));
            assert_eq!(result, ExecuteResult::PCAdvance);
        }

        // Verify stack is full
        assert_eq!(tpu.tpu_state.stack.len(), TPU::STACK_SIZE);

        // Try to push one more value - should result in stack overflow
        let result = op_push(&mut tpu, &OperandValueType::Immediate(99));
        assert_eq!(result, ExecuteResult::Halt(HaltReason::StackOverflow));

        // Verify the stack size hasn't changed
        assert_eq!(tpu.tpu_state.stack.len(), TPU::STACK_SIZE);
    }

    #[test]
    fn test_op_peek() {
        let mut tpu = create_tpu_with_registers(10, 20, 30);

        // Push some values to the stack
        op_push(&mut tpu, &OperandValueType::Immediate(42));
        op_push(&mut tpu, &OperandValueType::Immediate(43));
        op_push(&mut tpu, &OperandValueType::Immediate(44));

        // Test PEEK
        let result = op_peek(&mut tpu, &Register::Y, &OperandValueType::Immediate(1));
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::Y), 43);
        assert_eq!(tpu.tpu_state.stack.len(), 3); // Stack remains unchanged
    }

    #[test]
    fn test_op_ldo_and_ldoi() {
        // Test LDO - Load with offset
        let ram_values = [(15, 42)];
        let mut tpu = create_tpu_with_ram(&ram_values);
        tpu.write_register(Register::X, 5); // Offset

        let result = op_ldo(
            &mut tpu,
            &Register::A,
            &OperandValueType::Immediate(10), // Base address
            &Register::X,
        );
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 42); // A now has the value from memory
        assert_eq!(tpu.read_register(Register::X), 5); // X remains unchanged

        // Test LDOI - Load with offset and increment
        let ram_values = [(25, 99)];
        let mut tpu = create_tpu_with_ram(&ram_values);
        tpu.write_register(Register::X, 5); // Offset

        let result = op_ldoi(
            &mut tpu,
            &Register::A,
            &OperandValueType::Immediate(20), // Base address
            &Register::X,
        );
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 99); // A now has the value from memory
        assert_eq!(tpu.read_register(Register::X), 6); // X is incremented
    }
    #[test]
    fn test_op_scr() {
        let mut tpu = create_tpu_with_registers(10, 20, 30);

        // Push some values to the stack
        op_push(&mut tpu, &OperandValueType::Immediate(42));
        op_push(&mut tpu, &OperandValueType::Immediate(43));
        assert_eq!(tpu.tpu_state.stack.len(), 2);

        // Test stack clear
        let result = op_scr(&mut tpu);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.tpu_state.stack.len(), 0);
    }

    #[test]
    fn test_op_rsp() {
        let mut tpu = create_tpu_with_registers(10, 20, 30);

        // Push some values to the stack
        op_push(&mut tpu, &OperandValueType::Immediate(42));
        op_push(&mut tpu, &OperandValueType::Immediate(43));
        op_push(&mut tpu, &OperandValueType::Immediate(44));
        assert_eq!(tpu.tpu_state.stack.len(), 3);

        // Test reading stack pointer
        let result = op_rsp(&mut tpu, &Register::A);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 3); // Stack pointer is 3
    }
}
