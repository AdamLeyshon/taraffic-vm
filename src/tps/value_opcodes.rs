use crate::shared::{Instruction, OperandValueType};
use crate::tps::Rule;
use pest::Span;
use pest::error::ErrorVariant;

pub fn parse_single_value_operand_opcodes(
    span: Span,
    opcode: &str,
    operand_value_type: OperandValueType,
) -> Result<Instruction, pest::error::Error<Rule>> {
    match opcode {
        "PUSH" => Ok(Instruction::PUSH(operand_value_type)),
        "DPWW" => Ok(Instruction::DPWW(operand_value_type)),
        "JMP" => Ok(Instruction::JMP(operand_value_type)),
        "JPR" => Ok(Instruction::JPR(operand_value_type)),
        "JSR" => Ok(Instruction::JSR(operand_value_type)),
        "SLP" => Ok(Instruction::SLP(operand_value_type)),
        _ => Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Failed to parse instruction".into(),
            },
            span,
        )),
    }
}
