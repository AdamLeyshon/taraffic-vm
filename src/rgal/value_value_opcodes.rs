use crate::shared::{Instruction, OperandValueType};
use crate::rgal::Rule;
use pest::Span;
use pest::error::ErrorVariant;

pub fn parse_two_value_operand_opcodes(
    span: Span,
    opcode: &str,
    operand_a: OperandValueType,
    operand_b: OperandValueType,
) -> Result<Instruction, pest::error::Error<Rule>> {
    match opcode {
        "STM" => Ok(Instruction::STM(operand_a, operand_b)),
        "DPW" => Ok(Instruction::DPW(operand_a, operand_b)),
        "APW" => Ok(Instruction::APW(operand_a, operand_b)),
   
        _ => Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Failed to parse instruction".into(),
            },
            span,
        )),
    }
}
