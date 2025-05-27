use crate::rgal::Rule;
use crate::shared::{Instruction, OperandValueType};
use pest::Span;
use pest::error::ErrorVariant;

pub fn parse_value_register_value_operand_opcodes(
    span: Span,
    opcode: &str,
    value_a: OperandValueType,
    register: OperandValueType,
    value_b: OperandValueType,
) -> Result<Instruction, pest::error::Error<Rule>> {
    let OperandValueType::Register(register) = register else {
        return Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Expected value, register, value operands".into(),
            },
            span,
        ));
    };

    match opcode {
        "BEQ" => Ok(Instruction::BEQ(value_a, register, value_b)),
        "BNE" => Ok(Instruction::BNE(value_a, register, value_b)),
        "BGE" => Ok(Instruction::BGE(value_a, register, value_b)),
        "BLE" => Ok(Instruction::BLE(value_a, register, value_b)),
        "BGT" => Ok(Instruction::BGT(value_a, register, value_b)),
        "BLT" => Ok(Instruction::BLT(value_a, register, value_b)),
        "BREQ" => Ok(Instruction::BREQ(value_a, register, value_b)),
        "BRNE" => Ok(Instruction::BRNE(value_a, register, value_b)),
        "BRGE" => Ok(Instruction::BRGE(value_a, register, value_b)),
        "BRLE" => Ok(Instruction::BRLE(value_a, register, value_b)),
        "BRGT" => Ok(Instruction::BRGT(value_a, register, value_b)),
        "BRLT" => Ok(Instruction::BRLT(value_a, register, value_b)),

        _ => Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Failed to parse instruction".into(),
            },
            span,
        )),
    }
}
