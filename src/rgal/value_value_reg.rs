use crate::rgal::Rule;
use crate::shared::{Instruction, OperandValueType};
use pest::Span;
use pest::error::ErrorVariant;

pub fn parse_value_value_register_operand_opcodes(
    span: Span,
    opcode: &str,
    value_a: OperandValueType,
    value_b: OperandValueType,
    register: OperandValueType,
) -> Result<Instruction, pest::error::Error<Rule>> {
    let OperandValueType::Register(register) = register else {
        return Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Expected a register and value operand".into(),
            },
            span,
        ));
    };

    match opcode {
        "STMO" => Ok(Instruction::STMO(value_a, value_b, register)),
        "SMOI" => Ok(Instruction::SMOI(value_a, value_b, register)),

        _ => Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Failed to parse instruction".into(),
            },
            span,
        )),
    }
}
