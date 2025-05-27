use crate::rgal::parse_program;
use crate::shared::Register;
use crate::tpu::flow::*;
use crate::tpu::{TPU, TpuState};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{
        AnalogPin, DigitalPin, ExecuteResult, HaltReason, Instruction, OperandValueType,
    };
    use crate::tpu::ExecutionState;
    use strum::EnumCount;

    const LOOP_PROGRAM: &'static str = r#"LDR A, 10
        DEC A
        BEZ 4, A
        JMP 1
        LDR A, 255
        HLT"#;

    // Helper function to create a TPU with specific register values and a RGAL program
    fn create_tpu_with_program(program_str: &str, a: u16, x: u16, y: u16) -> TPU {
        // Parse the program
        let program = parse_program(program_str).expect("Failed to parse program");

        let mut tpu_state = TpuState {
            stack: Vec::new(),
            analog_pins: [0; AnalogPin::COUNT],
            digital_pins: [false; DigitalPin::COUNT],
            analog_pin_config: [false; AnalogPin::COUNT],
            digital_pin_config: [true; DigitalPin::COUNT],
            ram: [0; TPU::RAM_SIZE],
            rom: program,
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

    // Helper function to create a TPU with a specific program counter and a RGAL program
    fn create_tpu_with_pc(program_str: &str, pc: usize) -> TPU {
        let mut tpu = create_tpu_with_program(program_str, 0, 0, 0);
        tpu.tpu_state.program_counter = pc;
        tpu
    }

    #[test]
    fn test_op_jmp() {
        // Test case 1: Jump to a valid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        let target = OperandValueType::Immediate(4); // Jump to line 4
        let result = op_jmp(&mut tpu, &target);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 2: Jump with register operand
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 2);
        let target = OperandValueType::Register(Register::X); // Jump to line 2
        let result = op_jmp(&mut tpu, &target);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 2); // PC is now at line 2

        // Test case 3: Error case - jump to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        let target = OperandValueType::Immediate(10); // Invalid line
        let result = op_jmp(&mut tpu, &target);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        // PC does not advance to the next line because the next jump caused a HLT
        assert_eq!(tpu.tpu_state.program_counter, 0);
    }

    #[test]
    fn test_op_bez() {
        // Test case 1: Branch when value is zero
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::A, 0);
        let target = OperandValueType::Immediate(4);
        let result = op_bez(&mut tpu, &target, &Register::A);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 2: Don't branch when value is not zero
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::A, 5);
        let target = OperandValueType::Immediate(4);
        let result = op_bez(&mut tpu, &target, &Register::A);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        // PC increments by 1 to the next line because the branch was not taken
        assert_eq!(tpu.tpu_state.program_counter, 1); // PC remains unchanged

        // Test case 3: Error case - branch to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::A, 0);
        let target = OperandValueType::Immediate(10);
        let result = op_bez(&mut tpu, &target, &Register::A);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        // PC does not advance to the next line because the next jump caused a HLT
        assert_eq!(tpu.tpu_state.program_counter, 0);
    }

    #[test]
    fn test_op_bnz() {
        // Create a simple program with 6 lines
        let program = "
            LDA 10
            SUB A, 1
            BEZ 4
            JMP 1
            LDA 255
            HLT
        ";

        // Test case 1: Branch when value is not zero
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::A, 5);
        let target = OperandValueType::Immediate(4);
        let result = op_bnz(&mut tpu, &target, &Register::A);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 2: Don't branch when value is zero
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::A, 0);
        let target = OperandValueType::Immediate(4);
        let result = op_bnz(&mut tpu, &target, &Register::A);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 1); // PC advance to next line

        // Test case 3: Error case - branch to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::A, 5);
        let target = OperandValueType::Immediate(10);
        let result = op_bnz(&mut tpu, &target, &Register::A);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        // PC does not advance to the next line because the next jump caused a HLT
        assert_eq!(tpu.tpu_state.program_counter, 0);
    }

    #[test]
    fn test_op_beq() {
        // Test case 1: Branch when values are equal
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(4);
        let value = OperandValueType::Register(Register::Y);
        let result = op_beq(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 2: Don't branch when values are not equal
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 10);
        let target = OperandValueType::Immediate(4);
        let value = OperandValueType::Register(Register::Y);
        let result = op_beq(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        // PC increments by 1 to the next line because the branch was not taken
        assert_eq!(tpu.tpu_state.program_counter, 1);

        // Test case 3: Error case - branch to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(10);
        let value = OperandValueType::Register(Register::Y);
        let result = op_beq(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        // PC does not advance to the next line because the next jump caused a HLT
        assert_eq!(tpu.tpu_state.program_counter, 0);
    }

    #[test]
    fn test_op_bne() {
        // Test case 1: Branch when values are not equal
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 10);
        let target = OperandValueType::Immediate(4);
        let value = OperandValueType::Register(Register::Y);
        let result = op_bne(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 2: Don't branch when values are equal
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(4);
        let value = OperandValueType::Register(Register::Y);
        let result = op_bne(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 1); // PC advance to next line

        // Test case 3: Error case - branch to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 10);
        let target = OperandValueType::Immediate(10);
        let value = OperandValueType::Register(Register::Y);
        let result = op_bne(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        // PC does not advance to the next line because the next jump caused a HLT
        assert_eq!(tpu.tpu_state.program_counter, 0);
    }

    #[test]
    fn test_op_bge() {
        // Test case 1: Branch when first value is greater than second
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 10);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(4);
        let value = OperandValueType::Register(Register::Y);
        let result = op_bge(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 2: Branch when values are equal
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(4);
        let value = OperandValueType::Register(Register::Y);
        let result = op_bge(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 3: Don't branch when first value is less than second
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 10);
        let target = OperandValueType::Immediate(4);
        let value = OperandValueType::Register(Register::Y);
        let result = op_bge(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 1); // PC advance to next line

        // Test case 4: Error case - branch to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 10);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(10);
        let value = OperandValueType::Register(Register::Y);
        let result = op_bge(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        // PC does not advance to the next line because the next jump caused a HLT
        assert_eq!(tpu.tpu_state.program_counter, 0);
    }

    #[test]
    fn test_op_ble() {
        // Test case 1: Branch when first value is less than second
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 10);
        let target = OperandValueType::Immediate(4);
        let value = OperandValueType::Register(Register::Y);
        let result = op_ble(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 2: Branch when values are equal
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(4);
        let value = OperandValueType::Register(Register::Y);
        let result = op_ble(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 3: Don't branch when first value is greater than second
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 10);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(4);
        let value = OperandValueType::Register(Register::Y);
        let result = op_ble(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 1); // PC advance to next line

        // Test case 4: Error case - branch to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 10);
        let target = OperandValueType::Immediate(10);
        let value = OperandValueType::Register(Register::Y);
        let result = op_ble(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        // PC does not advance to the next line because the next jump caused a HLT
        assert_eq!(tpu.tpu_state.program_counter, 0);
    }

    #[test]
    fn test_op_bgt() {
        // Test case 1: Branch when first value is greater than second
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 10);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(4);
        let value = OperandValueType::Register(Register::Y);
        let result = op_bgt(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 2: Don't branch when values are equal
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(4);
        let value = OperandValueType::Register(Register::Y);
        let result = op_bgt(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 1); // PC advance to next line

        // Test case 3: Don't branch when first value is less than second
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 10);
        let target = OperandValueType::Immediate(4);
        let value = OperandValueType::Register(Register::Y);
        let result = op_bgt(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 1); // PC advance to next line

        // Test case 4: Error case - branch to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 10);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(10);
        let value = OperandValueType::Register(Register::Y);
        let result = op_bgt(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        // PC does not advance to the next line because the next jump caused a HLT
        assert_eq!(tpu.tpu_state.program_counter, 0);
    }

    #[test]
    fn test_op_blt() {
        // Test case 1: Branch when first value is less than second
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 10);
        let target = OperandValueType::Immediate(4);
        let value = OperandValueType::Register(Register::Y);
        let result = op_blt(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 2: Don't branch when values are equal
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(4);
        let value = OperandValueType::Register(Register::Y);
        let result = op_blt(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 1); // PC advance to next line

        // Test case 3: Don't branch when first value is greater than second
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 10);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(4);
        let value = OperandValueType::Register(Register::Y);
        let result = op_blt(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 1); // PC advance to next line

        // Test case 4: Error case - branch to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 10);
        let target = OperandValueType::Immediate(10);
        let value = OperandValueType::Register(Register::Y);
        let result = op_blt(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        // PC does not advance to the next line because the next jump caused a HLT
        assert_eq!(tpu.tpu_state.program_counter, 0);
    }

    #[test]
    fn test_op_jpr() {
        // Test case 1: Jump forward
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        let target = OperandValueType::Immediate(3); // Jump 3 lines forward
        let result = op_jpr(&mut tpu, &target);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 2: Jump with register operand
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 2);
        let target = OperandValueType::Register(Register::X); // Jump 2 lines forward
        let result = op_jpr(&mut tpu, &target);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 3); // PC is now at line 3

        // Test case 3: Error case - jump to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        let target = OperandValueType::Immediate(10); // Invalid jump
        let result = op_jpr(&mut tpu, &target);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        // PC does not advance to the next line because the next jump caused a HLT
        assert_eq!(tpu.tpu_state.program_counter, 0);
    }

    #[test]
    fn test_op_brez() {
        // Test case 1: Branch when value is zero
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::A, 0);
        let target = OperandValueType::Immediate(3);
        let result = op_brez(&mut tpu, &target, &Register::A);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 2: Don't branch when value is not zero
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::A, 5);
        let target = OperandValueType::Immediate(3);
        let result = op_brez(&mut tpu, &target, &Register::A);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 2); // PC advances to next line

        // Test case 3: Error case - branch to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::A, 0);
        let target = OperandValueType::Immediate(10);
        let result = op_brez(&mut tpu, &target, &Register::A);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        assert_eq!(tpu.tpu_state.program_counter, 1); // PC remains unchanged
    }

    #[test]
    fn test_op_brnz() {
        // Test case 1: Branch when value is not zero
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::A, 5);
        let target = OperandValueType::Immediate(3);
        let result = op_brnz(&mut tpu, &target, &Register::A);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 2: Don't branch when value is zero
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::A, 0);
        let target = OperandValueType::Immediate(3);
        let result = op_brnz(&mut tpu, &target, &Register::A);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 2); // PC advances to next line

        // Test case 3: Error case - branch to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::A, 5);
        let target = OperandValueType::Immediate(10);
        let result = op_brnz(&mut tpu, &target, &Register::A);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        assert_eq!(tpu.tpu_state.program_counter, 1); // PC remains unchanged
    }

    #[test]
    fn test_op_breq() {
        // Test case 1: Branch when values are equal
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(3);
        let value = OperandValueType::Register(Register::Y);
        let result = op_breq(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 2: Don't branch when values are not equal
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 10);
        let target = OperandValueType::Immediate(3);
        let value = OperandValueType::Register(Register::Y);
        let result = op_breq(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 2); // PC remains unchanged

        // Test case 3: Error case - branch to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(10);
        let value = OperandValueType::Register(Register::Y);
        let result = op_breq(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        assert_eq!(tpu.tpu_state.program_counter, 1); // PC remains unchanged
    }

    #[test]
    fn test_op_brne() {
        // Test case 1: Branch when values are not equal
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 10);
        let target = OperandValueType::Immediate(3);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brne(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 2: Don't branch when values are equal
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(3);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brne(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 2); // PC advances to next line

        // Test case 3: Error case - branch to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 10);
        let target = OperandValueType::Immediate(10);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brne(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        assert_eq!(tpu.tpu_state.program_counter, 1); // PC remains unchanged
    }

    #[test]
    fn test_op_brge() {
        // Test case 1: Branch when first value is greater than second
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 10);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(3);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brge(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 2: Branch when values are equal
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(3);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brge(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 3: Don't branch when first value is less than second
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 10);
        let target = OperandValueType::Immediate(3);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brge(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 2); // PC advances to next line

        // Test case 4: Error case - branch to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 10);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(10);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brge(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        assert_eq!(tpu.tpu_state.program_counter, 1); // PC remains unchanged
    }

    #[test]
    fn test_op_brle() {
        // Test case 1: Branch when first value is less than second
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 10);
        let target = OperandValueType::Immediate(3);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brle(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 2: Branch when values are equal
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(3);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brle(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 3: Don't branch when first value is greater than second
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 10);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(3);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brle(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 2); // PC advances to next line

        // Test case 4: Error case - branch to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 10);
        let target = OperandValueType::Immediate(10);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brle(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        assert_eq!(tpu.tpu_state.program_counter, 1); // PC remains unchanged
    }

    #[test]
    fn test_op_brgt() {
        // Test case 1: Branch when first value is greater than second
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 10);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(3);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brgt(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 2: Don't branch when values are equal
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(3);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brgt(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 2); // PC advances to next line

        // Test case 3: Don't branch when first value is less than second
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 10);
        let target = OperandValueType::Immediate(3);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brgt(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 2); // PC advances to next line

        // Test case 4: Error case - branch to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 10);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(10);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brgt(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        assert_eq!(tpu.tpu_state.program_counter, 1); // PC remains unchanged
    }

    #[test]
    fn test_op_brlt() {
        // Test case 1: Branch when first value is less than second
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 10);
        let target = OperandValueType::Immediate(3);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brlt(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4

        // Test case 2: Don't branch when values are equal
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(3);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brlt(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 2); // PC advances to next line

        // Test case 3: Don't branch when first value is greater than second
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 10);
        tpu.write_register(Register::Y, 5);
        let target = OperandValueType::Immediate(3);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brlt(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 2); // PC advances to next line

        // Test case 4: Error case - branch to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 5);
        tpu.write_register(Register::Y, 10);
        let target = OperandValueType::Immediate(10);
        let value = OperandValueType::Register(Register::Y);
        let result = op_brlt(&mut tpu, &target, &Register::X, &value);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        assert_eq!(tpu.tpu_state.program_counter, 1); // PC remains unchanged
    }

    #[test]
    fn test_op_jsr() {
        // Test case 1: Call subroutine
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        let target = OperandValueType::Immediate(4); // Call subroutine at line 4
        let result = op_jsr(&mut tpu, &target);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4
        assert_eq!(tpu.tpu_state.stack.len(), 1); // Stack has one item
        assert_eq!(tpu.tpu_state.stack[0], 0); // Return address is 0

        // Test case 2: Call subroutine with register operand
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 1);
        tpu.write_register(Register::X, 4);
        let target = OperandValueType::Register(Register::X); // Call subroutine at line 4
        let result = op_jsr(&mut tpu, &target);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 4); // PC is now at line 4
        assert_eq!(tpu.tpu_state.stack.len(), 1); // Stack has one item
        assert_eq!(tpu.tpu_state.stack[0], 1); // Return address is 1

        // Test case 3: Error case - call to an invalid line
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 0);
        let target = OperandValueType::Immediate(10); // Invalid line
        let result = op_jsr(&mut tpu, &target);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::InvalidPC)); // Error
        // PC does not advance to the next line because the next jump caused a HLT
        assert_eq!(tpu.tpu_state.program_counter, 0);
        assert_eq!(tpu.tpu_state.stack.len(), 0); // Stack is empty
    }

    #[test]
    fn test_op_rsub() {
        // Test case 1: Return from subroutine
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 4);
        tpu.push(1); // Push return address
        let result = op_rts(&mut tpu);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 1); // PC is now at return address
        assert_eq!(tpu.tpu_state.stack.len(), 0); // Stack is empty

        // Test case 2: Return from nested subroutine
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 4);
        tpu.push(1); // Push first return address
        tpu.push(2); // Push second return address
        let result = op_rts(&mut tpu);
        assert_eq!(result, ExecuteResult::PCModified); // No error
        assert_eq!(tpu.tpu_state.program_counter, 2); // PC is now at second return address
        assert_eq!(tpu.tpu_state.stack.len(), 1); // Stack has one item left

        // Test case 3: Error case - return with empty stack
        let mut tpu = create_tpu_with_pc(LOOP_PROGRAM, 4);
        let result = op_rts(&mut tpu);
        assert_eq!(result, ExecuteResult::PCModified); // No error (pop from empty stack returns 0)
        assert_eq!(tpu.tpu_state.program_counter, 0); // PC is set to 0
        assert_eq!(tpu.tpu_state.stack.len(), 0); // Stack is empty
    }

    #[test]
    fn test_full_program_execution() {
        let mut tpu = create_tpu_with_program(LOOP_PROGRAM, 0, 0, 0);

        // Execute the program step by step
        // LDA 10 - Load 10 into A
        assert_eq!(tpu.tpu_state.program_counter, 0);
        tpu.tick();
        assert_eq!(tpu.read_register(Register::A), 10);
        assert_eq!(tpu.tpu_state.program_counter, 1);

        // Continue the loop until A becomes 0
        while tpu.read_register(Register::A) > 0 {
            // SUB A, 1
            tpu.tick();
            tpu.tick();

            // BEZ 4
            tpu.tick();
            tpu.tick();

            if tpu.read_register(Register::A) == 0 {
                // If A is 0, we should have branched to line 4
                assert_eq!(tpu.tpu_state.program_counter, 4);
                break;
            }
            // JMP 1
            tpu.tick();
            assert_eq!(tpu.tpu_state.program_counter, 1);
        }

        // Step 5: LDA 255 - Load 255 into A
        tpu.tick();
        assert_eq!(tpu.read_register(Register::A), 255);
        assert_eq!(tpu.tpu_state.program_counter, 5);

        // Step 6: HLT - Halt the CPU
        tpu.tick();
        assert_eq!(tpu.tpu_state.halted, true);
    }
}
