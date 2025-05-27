use crate::shared::{Operand, Register};
use crate::tpu::mmu::*;
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
            rom: vec![],
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
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
        ];
        let result = op_rcy(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 20); // A now has X's value
        assert_eq!(tpu.read_register(Register::X), 20); // X remains unchanged

        // Test case 2: Copy between general registers
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        tpu.write_register(Register::R0, 42);
        let operands = [
            Operand::Register(Register::R1),
            Operand::Register(Register::R0),
        ];
        let result = op_rcy(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::R1), 42); // R1 now has R0's value
        assert_eq!(tpu.read_register(Register::R0), 42); // R0 remains unchanged

        // Test case 3: Error case - first operand is not a register
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands = [Operand::Constant(5), Operand::Register(Register::X)];
        let result = op_rcy(&mut tpu, &operands);
        assert_eq!(result, true); // Error
    }

    #[test]
    fn test_op_rmv() {
        // Test case 1: Move from one register to another
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
        ];
        let result = op_rmv(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 20); // A now has X's value
        assert_eq!(tpu.read_register(Register::X), 0); // X is now zero

        // Test case 2: Move between general registers
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        tpu.write_register(Register::R0, 42);
        let operands = [
            Operand::Register(Register::R1),
            Operand::Register(Register::R0),
        ];
        let result = op_rmv(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::R1), 42); // R1 now has R0's value
        assert_eq!(tpu.read_register(Register::R0), 0); // R0 is now zero

        // Test case 3: Error case - first operand is not a register
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands = [Operand::Constant(5), Operand::Register(Register::X)];
        let result = op_rmv(&mut tpu, &operands);
        assert_eq!(result, true); // Error
    }

    #[test]
    fn test_op_str() {
        // Test case 1: Store register value into memory
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands = [
            Operand::Constant(5), // Address
            Operand::Register(Register::A),
        ];
        let result = op_str(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_ram(5), 10); // Memory at address 5 now has A's value

        // Test case 2: Store with register address
        let mut tpu = create_tpu_with_registers(10, 8, 30);
        let operands = [
            Operand::Register(Register::X), // Address from X
            Operand::Register(Register::A),
        ];
        let result = op_str(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_ram(8), 10); // Memory at address 8 now has A's value

        // Test case 3: Error case - second operand is not a register
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands = [Operand::Constant(12), Operand::Constant(5)];
        let result = op_str(&mut tpu, &operands);
        assert_eq!(result, true); // Error
    }

    #[test]
    fn test_op_ldr() {
        // Test case 1: Load constant into register
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands = [Operand::Register(Register::A), Operand::Constant(42)];
        let result = op_ldr(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 42); // A now has the constant value

        // Test case 2: Load from one register to another
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::Y),
        ];
        let result = op_ldr(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 30); // A now has Y's value

        // Test case 3: Error case - first operand is not a register
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands = [Operand::Constant(5), Operand::Constant(42)];
        let result = op_ldr(&mut tpu, &operands);
        assert_eq!(result, true); // Error
    }

    #[test]
    fn test_op_ldm() {
        // Test case 1: Load from memory into register
        let ram_values = [(10, 42)];
        let mut tpu = create_tpu_with_ram(&ram_values);
        let operands = [
            Operand::Register(Register::A),
            Operand::Constant(10), // Address
        ];
        let result = op_ldm(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 42); // A now has the value from memory

        // Test case 2: Load with register address
        let ram_values = [(15, 42)];
        let mut tpu = create_tpu_with_ram(&ram_values);
        tpu.write_register(Register::X, 15);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X), // Address from X
        ];
        let result = op_ldm(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 42); // A now has the value from memory

        // Test case 3: Error case - first operand is not a register
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands = [Operand::Constant(5), Operand::Constant(100)];
        let result = op_ldm(&mut tpu, &operands);
        assert_eq!(result, true); // Error
    }

    #[test]
    fn test_op_lda() {
        // Test case 1: Load constant into accumulator
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands = [Operand::Constant(42)];
        let result = op_lda(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 42); // A now has the constant value

        // Test case 2: Load from register into accumulator
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands = [Operand::Register(Register::Y)];
        let result = op_lda(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 30); // A now has Y's value
    }

    #[test]
    fn test_op_ldx() {
        // Test case 1: Load from memory with offset into register
        let ram_values = [(15, 42)];
        let mut tpu = create_tpu_with_ram(&ram_values);
        tpu.write_register(Register::X, 5); // Offset
        let operands = [
            Operand::Register(Register::A),
            Operand::Constant(10), // Base address
        ];
        let result = op_ldx(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 42); // A now has the value from memory
        assert_eq!(tpu.read_register(Register::X), 5); // X remains unchanged

        // Test case 2: Load with register address
        let ram_values = [(15, 42)];
        let mut tpu = create_tpu_with_ram(&ram_values);
        tpu.write_register(Register::X, 5); // Offset
        tpu.write_register(Register::Y, 10); // Base address
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::Y),
        ];
        let result = op_ldx(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 42); // A now has the value from memory
        assert_eq!(tpu.read_register(Register::X), 5); // X remains unchanged

        // Test case 3: Error case - first operand is not a register
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands = [Operand::Constant(5), Operand::Constant(20)];
        let result = op_ldx(&mut tpu, &operands);
        assert_eq!(result, true); // Error
    }

    #[test]
    fn test_op_ldxi() {
        // Test case 1: Load from memory with offset into register and increment X
        let ram_values = [(15, 42)];
        let mut tpu = create_tpu_with_ram(&ram_values);
        tpu.write_register(Register::X, 5); // Offset
        let operands = [
            Operand::Register(Register::A),
            Operand::Constant(10), // Base address
        ];
        let result = op_ldxi(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 42); // A now has the value from memory
        assert_eq!(tpu.read_register(Register::X), 6); // X is incremented

        // Test case 2: Load with register address
        let ram_values = [(15, 42)];
        let mut tpu = create_tpu_with_ram(&ram_values);
        tpu.write_register(Register::X, 5); // Offset
        tpu.write_register(Register::Y, 10); // Base address
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::Y),
        ];
        let result = op_ldxi(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::A), 42); // A now has the value from memory
        assert_eq!(tpu.read_register(Register::X), 6); // X is incremented

        // Test case 3: X wrapping around on increment
        let ram_values = [(20, 42)];
        let mut tpu = create_tpu_with_ram(&ram_values);
        tpu.write_register(Register::X, 65535); // Max value
        let operands = [Operand::Register(Register::A), Operand::Constant(20)];
        let result = op_ldxi(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::X), 0); // X wraps around to 0

        // Test case 4: Error case - first operand is not a register
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands = [Operand::Constant(5), Operand::Constant(100)];
        let result = op_ldxi(&mut tpu, &operands);
        assert_eq!(result, true); // Error
    }

    #[test]
    fn test_op_stm() {
        // Test case 1: Store constant into memory
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands = [
            Operand::Constant(7),  // Address
            Operand::Constant(42), // Value
        ];
        let result = op_stm(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_ram(7), 42); // Memory at address 7 now has the constant value

        // Test case 2: Store with register address and register value
        let mut tpu = create_tpu_with_registers(10, 9, 30);
        let operands = [
            Operand::Register(Register::X), // Address from X
            Operand::Register(Register::A), // Value from A
        ];
        let result = op_stm(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_ram(9), 10); // Memory at address 9 now has A's value
    }

    #[test]
    fn test_op_sta() {
        // Test case 1: Store accumulator into memory
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands = [Operand::Constant(11)]; // Address
        let result = op_sta(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_ram(11), 10); // Memory at address 11 now has A's value

        // Test case 2: Store with register address
        let mut tpu = create_tpu_with_registers(10, 12, 30);
        let operands = [Operand::Register(Register::X)]; // Address from X
        let result = op_sta(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_ram(12), 10); // Memory at address 12 now has A's value
    }

    #[test]
    fn test_op_stx() {
        // Test case 1: Store register into memory with offset
        let mut tpu = create_tpu_with_registers(10, 5, 30);
        let operands = [
            Operand::Constant(13), // Base address
            Operand::Register(Register::A),
        ];
        let result = op_stx(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_ram(18), 10); // Memory at address 18 now has A's value
        assert_eq!(tpu.read_register(Register::X), 5); // X remains unchanged

        // Test case 2: Store with register address
        let mut tpu = create_tpu_with_registers(10, 5, 14);
        let operands = [
            Operand::Register(Register::Y), // Base address from Y
            Operand::Register(Register::A),
        ];
        let result = op_stx(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_ram(19), 10); // Memory at address 19 now has A's value
        assert_eq!(tpu.read_register(Register::X), 5); // X remains unchanged

        // Test case 3: Error case - second operand is not a register
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands = [Operand::Constant(16), Operand::Constant(42)];
        let result = op_stx(&mut tpu, &operands);
        assert_eq!(result, true); // Error
    }

    #[test]
    fn test_op_stxi() {
        // Test case 1: Store register into memory with offset and increment X
        let mut tpu = create_tpu_with_registers(10, 5, 30);
        let operands = [
            Operand::Constant(17), // Base address
            Operand::Register(Register::A),
        ];
        let result = op_srix(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_ram(22), 10); // Memory at address 22 now has A's value
        assert_eq!(tpu.read_register(Register::X), 6); // X is incremented

        // Test case 2: Store with register address
        let mut tpu = create_tpu_with_registers(10, 5, 18);
        let operands = [
            Operand::Register(Register::Y), // Base address from Y
            Operand::Register(Register::A),
        ];
        let result = op_srix(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_ram(23), 10); // Memory at address 23 now has A's value
        assert_eq!(tpu.read_register(Register::X), 6); // X is incremented

        // Test case 3: X wrapping around on increment
        let mut tpu = create_tpu_with_registers(10, 65535, 30); // X at max value
        let operands = [Operand::Constant(19), Operand::Register(Register::A)];
        let result = op_srix(&mut tpu, &operands);
        assert_eq!(result, false); // No error
        assert_eq!(tpu.read_register(Register::X), 0); // X wraps around to 0

        // Test case 4: Error case - second operand is not a register
        let mut tpu = create_tpu_with_registers(10, 20, 30);
        let operands = [Operand::Constant(21), Operand::Constant(42)];
        let result = op_srix(&mut tpu, &operands);
        assert_eq!(result, true); // Error
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
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::X),
        ];
        op_rcy(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 42);
        assert_eq!(tpu.read_register(Register::X), 42);

        // Test RMV operation
        tpu.write_register(Register::Y, 24);
        let operands = [
            Operand::Register(Register::A),
            Operand::Register(Register::Y),
        ];
        op_rmv(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 24);
        assert_eq!(tpu.read_register(Register::Y), 0);

        // Test STM and LDM operations
        let operands = [Operand::Constant(25), Operand::Constant(55)];
        op_stm(&mut tpu, &operands);
        assert_eq!(tpu.read_ram(25), 55);

        let operands = [Operand::Register(Register::A), Operand::Constant(25)];
        op_ldm(&mut tpu, &operands);
        assert_eq!(tpu.read_register(Register::A), 55);
        
    }
}
