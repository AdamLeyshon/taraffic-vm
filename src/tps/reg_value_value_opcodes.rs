use crate::shared::{Instruction, OperandValueType};
use crate::tps::Rule;
use pest::Span;
use pest::error::ErrorVariant;

pub fn parse_register_value__value_operand_opcodes(
    span: Span,
    opcode: &str,
    register: OperandValueType,
    value_a: OperandValueType,
    value_b: OperandValueType,
) -> Result<Instruction, pest::error::Error<Rule>> {
    let OperandValueType::Register(register_a) = register else {
        return Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Expected register, value, register operands".into(),
            },
            span,
        ));
    };

    match opcode {
        _ => Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Failed to parse instruction".into(),
            },
            span,
        )),
    }
}
