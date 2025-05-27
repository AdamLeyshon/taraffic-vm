use crate::shared::{OperandValueType, Register};
use crate::tpu::{TPU, create_basic_tpu_config};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rgal;
    use crate::shared::{AnalogPin, DigitalPin, Instruction};
    use std::rc::Rc;
    use strum::IntoEnumIterator;

    #[test]
    fn test_tpu_init() {
        let tpu = create_basic_tpu_config(vec![]);
        assert!(!tpu.busy());
    }

    #[test]
    fn test_single_instruction() {
        let mut tpu = create_basic_tpu_config(vec![Rc::new(Instruction::PUSH(
            OperandValueType::Immediate(1),
        ))]);

        tpu.tick();

        assert_eq!(tpu.tpu_state.stack.is_empty(), false)
    }

    #[test]
    fn test_single_instruction_from_str() {
        let program = "PUSH 1";
        let parsed = rgal::parse_program(program).expect("parse failure");
        assert_eq!(
            parsed,
            vec![Rc::new(Instruction::PUSH(OperandValueType::Immediate(1)))]
        );
        let mut tpu = create_basic_tpu_config(parsed);
        tpu.tick();
        assert_eq!(tpu.tpu_state.stack.is_empty(), false)
    }

    #[test]
    fn test_tpu_state_display() {
        // Create a TPU with some test values
        let mut tpu = TPU::new(
            0x1234,
            [true, false, true, false],
            [false, true, false, true, false, true, false, true],
            vec![],
        );

        // Set some register values
        for (i, reg) in Register::iter().enumerate() {
            tpu.write_register(reg, (i * 0x100) as u16);
        }

        // Push some values to the stack
        for i in 0..(TPU::STACK_SIZE as u16) - 8 {
            tpu.push(0xABCD + i);
        }

        // Set some RAM values
        for i in 0..TPU::RAM_SIZE {
            tpu.write_ram(i, (0x1000 + i) as u16);
        }

        // Set some analog pin values
        for (i, pin) in AnalogPin::iter().enumerate() {
            tpu.set_analog_pin(pin, (0x2000 + i) as u16);
        }

        // Set some digital pin values
        for (i, pin) in DigitalPin::iter().enumerate() {
            tpu.set_digital_pin(pin, i % 2 == 0);
        }

        // Print the TPU state
        println!("{}", tpu.tpu_state);

        // Basic assertion to ensure the test runs
        assert!(true);
    }
}
