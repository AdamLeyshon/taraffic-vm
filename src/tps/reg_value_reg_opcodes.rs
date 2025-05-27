use crate::shared::{Instruction, OperandValueType};
use crate::tps::Rule;
use pest::Span;
use pest::error::ErrorVariant;

pub fn parse_register_value_register_operand_opcodes(
    span: Span,
    opcode: &str,
    register_a: OperandValueType,
    value: OperandValueType,
    register_b: OperandValueType,
) -> Result<Instruction, pest::error::Error<Rule>> {
    let (OperandValueType::Register(register_a), OperandValueType::Register(register_b)) =
        (register_a, register_b)
    else {
        return Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Expected register, value, register operands".into(),
            },
            span,
        ));
    };

    match opcode {
        "LDO" => Ok(Instruction::LDO(register_a, value, register_b)),
        "LDOI" => Ok(Instruction::LDOI(register_a, value, register_b)),
        _ => Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Failed to parse instruction".into(),
            },
            span,
        )),
    }
}
