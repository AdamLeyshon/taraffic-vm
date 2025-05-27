use crate::shared::{ExecuteResult, OperandValueType, Register};
use crate::tpu::alu::*;
use crate::tpu::{ExecutionState, TPU, TpuState, create_basic_tpu_config};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{AnalogPin, DigitalPin};
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
            rom: Vec::new(),
            network_address: 0x1,
            incoming_packets: std::collections::VecDeque::new(),
            outgoing_packets: std::collections::VecDeque::new(),
            registers: [0; Register::COUNT],
            program_counter: 0,
            halted: false,
            execution_state: ExecutionState {
                instruction: None,
                wait_cycles: 0,
                execute_each_cycle: false,
            },
        };

        // Set register values
        tpu_state.registers[Register::A as usize] = a;
        tpu_state.registers[Register::X as usize] = x;
        tpu_state.registers[Register::Y as usize] = y;

        TPU::new_from_state(tpu_state)
    }

    #[test]
    fn test_op_inc() {
        // Test case 1: Normal increment
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let result = op_inc(&mut tpu, &Register::A);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 11); // A is incremented

        // Test case 2: Wrapping on overflow
        let mut tpu = create_tpu_with_registers(65535, 20, 30); // A at max value
        let result = op_inc(&mut tpu, &Register::A);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 0); // A wraps around to 0
    }

    #[test]
    fn test_op_dec() {
        // Test case 1: Normal decrement
        let mut tpu = create_tpu_with_registers(10, 20, 30);

        let result = op_dec(&mut tpu, &Register::A);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 9); // A is decremented

        // Test case 2: Wrapping on underflow
        let mut tpu = create_tpu_with_registers(0, 20, 30); // A at min value
        let result = op_dec(&mut tpu, &Register::A);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 65535); // A wraps around to 65535
    }

    #[test]
    fn test_op_add() {
        // Test case 1: Basic addition
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        tpu.write_register(Register::R0, 5);
        tpu.write_register(Register::R1, 3);
        let result = op_add(&mut tpu, &Register::R0, &Register::R1);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 8);

        // Test case 2: Addition with registers
        let mut tpu = create_tpu_with_registers(5, 3, 0);
        let result = op_add(&mut tpu, &Register::A, &Register::X);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 8);

        // Test case 3: Overflow
        let mut tpu = create_tpu_with_registers(65535, 1, 0);
        let result = op_add(&mut tpu, &Register::A, &Register::X);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 0); // Wrapping addition
    }

    #[test]
    fn test_op_sub() {
        // Test case 1: Basic subtraction
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        tpu.write_register(Register::R0, 5);
        tpu.write_register(Register::R1, 3);
        let result = op_sub(&mut tpu, &Register::R0, &Register::R1);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 2);

        // Test case 2: Subtraction with registers
        let mut tpu = create_tpu_with_registers(5, 3, 0);
        let result = op_sub(&mut tpu, &Register::A, &Register::X);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 2);

        // Test case 3: Underflow
        let mut tpu = create_tpu_with_registers(0, 1, 0);
        let result = op_sub(&mut tpu, &Register::A, &Register::X);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 65535); // Wrapping subtraction
    }

    #[test]
    fn test_op_mul() {
        // Test case 1: Basic multiplication
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        tpu.write_register(Register::R0, 5);
        tpu.write_register(Register::R1, 3);
        let result = op_mul(&mut tpu, &Register::R0, &Register::R1);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 15);

        // Test case 2: Multiplication with registers
        let mut tpu = create_tpu_with_registers(5, 3, 0);
        let result = op_mul(&mut tpu, &Register::A, &Register::X);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 15);

        // Test case 3: Overflow
        let mut tpu = create_tpu_with_registers(25000, 3, 0);
        let result = op_mul(&mut tpu, &Register::A, &Register::X);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 9464); // Wrapping multiplication
    }

    #[test]
    fn test_op_div() {
        // Test case 1: Basic division
        let mut tpu = create_tpu_with_registers(0, 15, 4);
        let result = op_div(&mut tpu, &Register::X, &Register::Y);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 3); // Quotient

        // Test case 2: Division with remainder
        let mut tpu = create_tpu_with_registers(0, 17, 5);
        let result = op_div(&mut tpu, &Register::X, &Register::Y);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 3); // Quotient

        // Test case 3: Division by zero
        let mut tpu = create_tpu_with_registers(5, 0, 0);

        let result = op_div(&mut tpu, &Register::X, &Register::Y);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::Div0));
    }

    #[test]
    fn test_op_mod() {
        // Test case 1: Basic modulo
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        tpu.write_register(Register::R0, 17);
        tpu.write_register(Register::R1, 5);
        let result = op_mod(&mut tpu, &Register::R0, &Register::R1);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 2); // Remainder

        // Test case 2: Modulo with registers
        let mut tpu = create_tpu_with_registers(17, 5, 0);
        let result = op_mod(&mut tpu, &Register::A, &Register::X);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 2); // Remainder

        // Test case 3: Modulo by zero
        let mut tpu = create_tpu_with_registers(5, 0, 0);
        let result = op_mod(&mut tpu, &Register::A, &Register::X);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::Div0)); // Error
    }

    #[test]
    fn test_op_and() {
        // Test case 1: Basic AND
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        tpu.write_register(Register::R0, 0b1010);
        tpu.write_register(Register::R1, 0b1100);
        let result = op_and(&mut tpu, &Register::R0, &Register::R1);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 0b1000);

        // Test case 2: AND with registers
        let mut tpu = create_tpu_with_registers(0b1010, 0b1100, 0);
        let result = op_and(&mut tpu, &Register::A, &Register::X);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 0b1000);
    }

    #[test]
    fn test_op_or() {
        // Test case 1: Basic OR
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        tpu.write_register(Register::R0, 0b1010);
        tpu.write_register(Register::R1, 0b1100);
        let result = op_or(&mut tpu, &Register::R0, &Register::R1);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 0b1110);

        // Test case 2: OR with registers
        let mut tpu = create_tpu_with_registers(0b1010, 0b1100, 0);
        let result = op_or(&mut tpu, &Register::A, &Register::X);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 0b1110);
    }

    #[test]
    fn test_op_xor() {
        // Test case 1: Basic XOR
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        tpu.write_register(Register::R0, 0b1010);
        tpu.write_register(Register::R1, 0b1100);
        let result = op_xor(&mut tpu, &Register::R0, &Register::R1);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 0b0110);

        // Test case 2: XOR with registers
        let mut tpu = create_tpu_with_registers(0b1010, 0b1100, 0);
        let result = op_xor(&mut tpu, &Register::A, &Register::X);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 0b0110);
    }

    #[test]
    fn test_op_not() {
        // Test case 1: Basic NOT
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        tpu.write_register(Register::R0, 0b1010101010101010);
        let result = op_not(&mut tpu, &Register::R0);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 0b0101010101010101);

        // Test case 2: NOT with register
        let mut tpu = create_tpu_with_registers(0b1010101010101010, 0, 0);
        let result = op_not(&mut tpu, &Register::A);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 0b0101010101010101);
    }

    #[test]
    fn test_op_shlr() {
        // Test case 1: Basic shift left into register
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        tpu.write_register(Register::R1, 0b00000001);
        let result = op_sll(
            &mut tpu,
            &Register::A,
            &Register::R1,
            &OperandValueType::Immediate(2),
        );
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 0b00000100);

        // Test case 2: Shift left with registers
        let mut tpu = create_tpu_with_registers(0, 0b00000001, 0);
        let result = op_sll(
            &mut tpu,
            &Register::A,
            &Register::X,
            &OperandValueType::Immediate(2),
        );
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 0b00000100);
    }

    #[test]
    fn test_op_shlc() {
        // Test case 1: Basic shift left with carry
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        tpu.write_register(Register::R1, 0b1000000000000001);
        let result = op_slc(
            &mut tpu,
            &Register::X,
            &Register::R1,
            &OperandValueType::Immediate(2),
        );
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::X), 0b00000100); // Shifted value
        assert_eq!(tpu.read_register(Register::A), 0b00000010); // Carry bits

        // Test case 2: Shift left with carry using registers
        let mut tpu = create_tpu_with_registers(0, 0b1000000000000001, 2);
        let result = op_slc(
            &mut tpu,
            &Register::R0,
            &Register::X,
            &OperandValueType::Register(Register::Y),
        );
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::R0), 0b00000100); // Shifted value
        assert_eq!(tpu.read_register(Register::A), 0b00000010); // Carry bits
    }

    #[test]
    fn test_op_shrr() {
        // Test case 1: Basic shift right into register
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        tpu.write_register(Register::R1, 0b10000000);
        let result = op_slr(&mut tpu, &Register::A, &Register::R1, &OperandValueType::Immediate(2));
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 0b00100000);

        // Test case 2: Shift right with registers
        let mut tpu = create_tpu_with_registers(0, 0b10000000, 2);
        let result = op_slr(&mut tpu, &Register::A, &Register::X, &OperandValueType::Register(Register::Y));
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 0b00100000);
    }

    #[test]
    fn test_op_shrc() {
        // Test case 1: Basic shift right with carry
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        tpu.write_register(Register::R1, 0b10000011);
        let result = op_src(
            &mut tpu,
            &Register::X,
            &Register::R1,
            &OperandValueType::Immediate(2),
        );
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::X), 0b00100000); // Shifted value
        assert_eq!(tpu.read_register(Register::A), 0b1100000000000000); // Carry bits

        // Test case 2: Shift right with carry using registers
        let mut tpu = create_tpu_with_registers(0, 0b10000011, 2);
        let result = op_src(
            &mut tpu,
            &Register::A,
            &Register::X,
            &OperandValueType::Immediate(2),
        );
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 0b1100000000000000); // Carry bits
    }

    #[test]
    fn test_op_rol() {
        // Test case 1: Basic rotate left
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        tpu.write_register(Register::R1, 0b1000000000000001);
        let result = op_rol(&mut tpu, &Register::A, &Register::R1, &OperandValueType::Immediate(1));
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 0b11);

        // Test case 2: Rotate left with registers
        let mut tpu = create_tpu_with_registers(0, 0b1000000000000001, 1);
        let result = op_rol(&mut tpu, &Register::A, &Register::X, &OperandValueType::Register(Register::Y));
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 0b11);

        // Test case 3: Rotate left by 8 places
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        tpu.write_register(Register::R1, 0b0000_0001_1000_0000);
        let result = op_rol(&mut tpu, &Register::A, &Register::R1, &OperandValueType::Immediate(9));
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 0b11);
    }

    #[test]
    fn test_op_ror() {
        // Test case 1: Basic rotate right
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        tpu.write_register(Register::R1, 0b1);
        let result = op_ror(&mut tpu, &Register::A, &Register::R1, &OperandValueType::Immediate(1));
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 32768);

        // Test case 2: Rotate right with registers
        let mut tpu = create_tpu_with_registers(0, 0b1, 1);
        let result = op_ror(&mut tpu, &Register::A, &Register::X, &&OperandValueType::Register(Register::Y));
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 32768);

        // Test case 3: Rotate right by more than 7 places (should be modulo 8)
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        tpu.write_register(Register::R1, 0b1000000000000001);
        let result = op_ror(&mut tpu, &Register::A, &Register::R1, &OperandValueType::Immediate(1));
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 49152);
    }

    #[test]
    fn test_with_basic_tpu_config() {
        // Test using create_basic_tpu_config
        let program = vec![
            // No instructions needed for this test
        ];
        let mut tpu = create_basic_tpu_config(program);

        // Test ADD operation
        tpu.write_register(Register::R0, 5);
        tpu.write_register(Register::R1, 3);
        let result = op_add(&mut tpu, &Register::R0, &Register::R1);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 8);

        // Test SUB operation
        tpu.write_register(Register::R0, 10);
        tpu.write_register(Register::R1, 4);
        let result = op_sub(&mut tpu, &Register::R0, &Register::R1);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 6);

        // Test MUL operation
        tpu.write_register(Register::R0, 3);
        tpu.write_register(Register::R1, 7);
        let result = op_mul(&mut tpu, &Register::R0, &Register::R1);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 21);

        // Test DIV operation
        tpu.write_register(Register::R0, 20);
        tpu.write_register(Register::R1, 4);
        let result = op_div(&mut tpu, &Register::R0, &Register::R1);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 5);
        assert_eq!(tpu.read_register(Register::X), 0);

        // Test clone and compare
        let tpu_clone = tpu.clone();
        assert_eq!(
            tpu_clone.read_register(Register::A),
            tpu.read_register(Register::A)
        );
        assert_eq!(
            tpu_clone.read_register(Register::X),
            tpu.read_register(Register::X)
        );

        // Test increment and decrement operations
        tpu.write_register(Register::A, 10);
        tpu.write_register(Register::X, 20);
        tpu.write_register(Register::Y, 30);

        let result = op_inc(&mut tpu, &Register::A);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 11);

        let result = op_dec(&mut tpu, &Register::A);
        assert_eq!(result, ExecuteResult::PCAdvance);
        assert_eq!(tpu.read_register(Register::A), 10);
    }
}
