use crate::shared::{Operand, Register};
use crate::tpu::alu::*;
use crate::tpu::{TPU, TpuState, create_basic_tpu_config};

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
            current_instruction: None,
            ram: [0; TPU::RAM_SIZE],
            rom: Vec::new(),
            network_address: 0x1,
            incoming_packets: std::collections::VecDeque::new(),
            outgoing_packets: std::collections::VecDeque::new(),
            registers: [0; Register::COUNT],
            wait_cycles: 0,
            program_counter: 0,
            halted: false,
        };

        // Set register values
        tpu_state.registers[Register::A as usize] = a;
        tpu_state.registers[Register::X as usize] = x;
        tpu_state.registers[Register::Y as usize] = y;

        TPU::new_from_state(tpu_state)
    }

    #[test]
    fn test_op_inca() {
        // Test case 1: Normal increment
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands: [Operand; 0] = [];
        let result = op_inca(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 11); // A is incremented

        // Test case 2: Wrapping on overflow
        let mut tpu = create_tpu_with_registers(65535, 20, 30); // A at max value
        let operands: [Operand; 0] = [];
        let result = op_inca(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 0); // A wraps around to 0
    }

    #[test]
    fn test_op_incx() {
        // Test case 1: Normal increment
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands: [Operand; 0] = [];
        let result = op_incx(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::X), 21); // X is incremented

        // Test case 2: Wrapping on overflow
        let mut tpu = create_tpu_with_registers(10, 65535, 30); // X at max value
        let operands: [Operand; 0] = [];
        let result = op_incx(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::X), 0); // X wraps around to 0
    }

    #[test]
    fn test_op_incy() {
        // Test case 1: Normal increment
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands: [Operand; 0] = [];
        let result = op_incy(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::Y), 31); // Y is incremented

        // Test case 2: Wrapping on overflow
        let mut tpu = create_tpu_with_registers(10, 20, 65535); // Y at max value
        let operands: [Operand; 0] = [];
        let result = op_incy(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::Y), 0); // Y wraps around to 0
    }

    #[test]
    fn test_op_deca() {
        // Test case 1: Normal decrement
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands: [Operand; 0] = [];
        let result = op_deca(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 9); // A is decremented

        // Test case 2: Wrapping on underflow
        let mut tpu = create_tpu_with_registers(0, 20, 30); // A at min value
        let operands: [Operand; 0] = [];
        let result = op_deca(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 65535); // A wraps around to 65535
    }

    #[test]
    fn test_op_decx() {
        // Test case 1: Normal decrement
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands: [Operand; 0] = [];
        let result = op_decx(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::X), 19); // X is decremented

        // Test case 2: Wrapping on underflow
        let mut tpu = create_tpu_with_registers(10, 0, 30); // X at min value
        let operands: [Operand; 0] = [];
        let result = op_decx(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::X), 65535); // X wraps around to 65535
    }

    #[test]
    fn test_op_decy() {
        // Test case 1: Normal decrement
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands: [Operand; 0] = [];
        let result = op_decy(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::Y), 29); // Y is decremented

        // Test case 2: Wrapping on underflow
        let mut tpu = create_tpu_with_registers(10, 20, 0); // Y at min value
        let operands: [Operand; 0] = [];
        let result = op_decy(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::Y), 65535); // Y wraps around to 65535
    }

    #[test]
    fn test_op_add() {
        // Test case 1: Basic addition
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [Operand::Constant(5), Operand::Constant(3)];
        op_add(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 8);

        // Test case 2: Addition with registers
        let mut tpu = create_tpu_with_registers(5, 3, 0);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
        ];
        op_add(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 8);

        // Test case 3: Overflow
        let mut tpu = create_tpu_with_registers(65535, 1, 0);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
        ];
        op_add(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 0); // Wrapping addition
    }

    #[test]
    fn test_op_sub() {
        // Test case 1: Basic subtraction
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [Operand::Constant(5), Operand::Constant(3)];
        op_sub(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 2);

        // Test case 2: Subtraction with registers
        let mut tpu = create_tpu_with_registers(5, 3, 0);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
        ];
        op_sub(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 2);

        // Test case 3: Underflow
        let mut tpu = create_tpu_with_registers(0, 1, 0);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
        ];
        op_sub(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 65535); // Wrapping subtraction
    }

    #[test]
    fn test_op_mul() {
        // Test case 1: Basic multiplication
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [Operand::Constant(5), Operand::Constant(3)];
        op_mul(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 15);

        // Test case 2: Multiplication with registers
        let mut tpu = create_tpu_with_registers(5, 3, 0);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
        ];
        op_mul(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 15);

        // Test case 3: Overflow
        let mut tpu = create_tpu_with_registers(25000, 3, 0);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
        ];
        op_mul(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 9464); // Wrapping multiplication
    }

    #[test]
    fn test_op_div() {
        // Test case 1: Basic division
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [Operand::Constant(15), Operand::Constant(3)];
        let result = op_div(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 5); // Quotient
        assert_eq!(tpu.read_register(Register::X), 0); // Remainder

        // Test case 2: Division with remainder
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [Operand::Constant(17), Operand::Constant(5)];
        let result = op_div(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 3); // Quotient
        assert_eq!(tpu.read_register(Register::X), 2); // Remainder

        // Test case 3: Division by zero
        let mut tpu = create_tpu_with_registers(5, 0, 0);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
        ];
        let result = op_div(&mut tpu, &operands);
        assert_eq!(result, true); // Error
    }

    #[test]
    fn test_op_mod() {
        // Test case 1: Basic modulo
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [Operand::Constant(17), Operand::Constant(5)];
        let result = op_mod(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 2); // Remainder

        // Test case 2: Modulo with registers
        let mut tpu = create_tpu_with_registers(17, 5, 0);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
        ];
        let result = op_mod(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 2); // Remainder

        // Test case 3: Modulo by zero
        let mut tpu = create_tpu_with_registers(5, 0, 0);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
        ];
        let result = op_mod(&mut tpu, &operands);
        assert_eq!(result, true); // Error
    }

    #[test]
    fn test_op_and() {
        // Test case 1: Basic AND
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [Operand::Constant(0b1010), Operand::Constant(0b1100)];
        op_and(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 0b1000);

        // Test case 2: AND with registers
        let mut tpu = create_tpu_with_registers(0b1010, 0b1100, 0);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
        ];
        op_and(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 0b1000);
    }

    #[test]
    fn test_op_or() {
        // Test case 1: Basic OR
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [Operand::Constant(0b1010), Operand::Constant(0b1100)];
        op_or(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 0b1110);

        // Test case 2: OR with registers
        let mut tpu = create_tpu_with_registers(0b1010, 0b1100, 0);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
        ];
        op_or(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 0b1110);
    }

    #[test]
    fn test_op_xor() {
        // Test case 1: Basic XOR
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [Operand::Constant(0b1010), Operand::Constant(0b1100)];
        op_xor(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 0b0110);

        // Test case 2: XOR with registers
        let mut tpu = create_tpu_with_registers(0b1010, 0b1100, 0);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
        ];
        op_xor(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 0b0110);
    }

    #[test]
    fn test_op_not() {
        // Test case 1: Basic NOT
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [Operand::Constant(0b1010101010101010)];
        op_not(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 0b0101010101010101);

        // Test case 2: NOT with register
        let mut tpu = create_tpu_with_registers(0b1010101010101010, 0, 0);
        let operands = [Operand::Register(Register::A)];
        op_not(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 0b0101010101010101);
    }

    #[test]
    fn test_op_shlr() {
        // Test case 1: Basic shift left into register
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [
            Operand::Register(Register::A),
            Operand::Constant(0b00000001),
            Operand::Constant(2),
        ];
        let result = op_sll(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 0b00000100);

        // Test case 2: Shift left with registers
        let mut tpu = create_tpu_with_registers(0, 0b00000001, 2);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
            Operand::Register(Register::Y),
        ];
        let result = op_sll(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 0b00000100);

        // Test case 3: Error case - first operand is not a register
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [
            Operand::Constant(0),
            Operand::Constant(0b00000001),
            Operand::Constant(2),
        ];
        let result = op_sll(&mut tpu, &operands);
        assert_eq!(result, true); // Error
    }

    #[test]
    fn test_op_shlc() {
        // Test case 1: Basic shift left with carry
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [
            Operand::Register(Register::X),
            Operand::Constant(0b1000000000000001),
            Operand::Constant(2),
        ];
        let result = op_slc(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::X), 0b00000100); // Shifted value
        assert_eq!(tpu.read_register(Register::A), 0b00000010); // Carry bits

        // Test case 2: Shift left with carry using registers
        let mut tpu = create_tpu_with_registers(0, 0b1000000000000001, 2);
        let operands = [
            Operand::Register(Register::R0),
            Operand::Register(Register::X),
            Operand::Register(Register::Y),
        ];
        let result = op_slc(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::R0), 0b00000100); // Shifted value
        assert_eq!(tpu.read_register(Register::A), 0b00000010); // Carry bits
    }

    #[test]
    fn test_op_shla() {
        // Test case 1: Basic shift left into accumulator
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [Operand::Constant(0b00000001), Operand::Constant(2)];
        op_shla(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 0b00000100);

        // Test case 2: Shift left into accumulator with registers
        let mut tpu = create_tpu_with_registers(0, 0b00000001, 2);
        let operands = [
            Operand::Register(Register::X),
            Operand::Register(Register::Y),
        ];
        op_shla(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 0b00000100);
    }

    #[test]
    fn test_op_shrr() {
        // Test case 1: Basic shift right into register
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [
            Operand::Register(Register::A),
            Operand::Constant(0b10000000),
            Operand::Constant(2),
        ];
        let result = op_slr(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 0b00100000);

        // Test case 2: Shift right with registers
        let mut tpu = create_tpu_with_registers(0, 0b10000000, 2);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
            Operand::Register(Register::Y),
        ];
        let result = op_slr(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 0b00100000);
    }

    #[test]
    fn test_op_shrc() {
        // Test case 1: Basic shift right with carry
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [
            Operand::Register(Register::X),
            Operand::Constant(0b10000011),
            Operand::Constant(2),
        ];
        let result = op_src(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::X), 0b00100000); // Shifted value
        assert_eq!(tpu.read_register(Register::A), 0b1100000000000000); // Carry bits

        // Test case 2: Shift right with carry using registers
        let mut tpu = create_tpu_with_registers(0, 0b10000011, 2);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
            Operand::Register(Register::Y),
        ];
        let result = op_src(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 0b1100000000000000); // Carry bits
    }

    #[test]
    fn test_op_shra() {
        // Test case 1: Basic shift right into accumulator
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [Operand::Constant(0b10000000), Operand::Constant(2)];
        op_shra(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 0b00100000);

        // Test case 2: Shift right into accumulator with registers
        let mut tpu = create_tpu_with_registers(0, 0b10000000, 2);
        let operands = [
            Operand::Register(Register::X),
            Operand::Register(Register::Y),
        ];
        op_shra(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 0b00100000);
    }

    #[test]
    fn test_op_rol() {
        // Test case 1: Basic rotate left
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [
            Operand::Register(Register::A),
            Operand::Constant(0b1000000000000001),
            Operand::Constant(1),
        ];
        let result = op_rol(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 0b11);

        // Test case 2: Rotate left with registers
        let mut tpu = create_tpu_with_registers(0, 0b1000000000000001, 1);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
            Operand::Register(Register::Y),
        ];
        let result = op_rol(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 0b11);

        // Test case 3: Rotate left by 8 places
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [
            Operand::Register(Register::A),
            Operand::Constant(0b0000_0001_1000_0000),
            Operand::Constant(9),
        ];
        let result = op_rol(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 0b11);
    }

    #[test]
    fn test_op_ror() {
        // Test case 1: Basic rotate right
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [
            Operand::Register(Register::A),
            Operand::Constant(0b1),
            Operand::Constant(1),
        ];
        let result = op_ror(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 32768);

        // Test case 2: Rotate right with registers
        let mut tpu = create_tpu_with_registers(0, 0b1, 1);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
            Operand::Register(Register::Y),
        ];
        let result = op_ror(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 32768);

        // Test case 3: Rotate right by more than 7 places (should be modulo 8)
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let operands = [
            Operand::Register(Register::A),
            Operand::Constant(0b1000000000000001),
            Operand::Constant(1),
        ];
        let result = op_ror(&mut tpu, &operands);
        assert_eq!(result, false); // No error
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
        let operands = [Operand::Constant(5), Operand::Constant(3)];
        op_add(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 8);

        // Test SUB operation
        let operands = [Operand::Constant(10), Operand::Constant(4)];
        op_sub(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 6);

        // Test MUL operation
        let operands = [Operand::Constant(3), Operand::Constant(7)];
        op_mul(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 21);

        // Test DIV operation
        let operands = [Operand::Constant(20), Operand::Constant(4)];
        op_div(&mut tpu, &operands);
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

        let empty_operands: [Operand; 0] = [];

        op_inc(&mut tpu, &empty_operands);
        assert_eq!(tpu.read_register(Register::A), 11);

        op_dec(&mut tpu, &empty_operands);
        assert_eq!(tpu.read_register(Register::A), 10);

    }
}
