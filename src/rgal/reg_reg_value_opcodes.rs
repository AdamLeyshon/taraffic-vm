use crate::shared::{Instruction, OperandValueType};
use crate::rgal::Rule;
use pest::Span;
use pest::error::ErrorVariant;

pub fn parse_two_register_value_operand_opcodes(
    span: Span,
    opcode: &str,
    register_a: OperandValueType,
    register_b: OperandValueType,
    value: OperandValueType,
) -> Result<Instruction, pest::error::Error<Rule>> {
    let (OperandValueType::Register(register_a), OperandValueType::Register(register_b)) =
        (register_a, register_b)
    else {
        return Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Expected two register operands and a value".into(),
            },
            span,
        ));
    };

    match opcode {
        "SLL" => Ok(Instruction::SLL(register_a, register_b, value)),
        "SLC" => Ok(Instruction::SLC(register_a, register_b, value)),
        "SLR" => Ok(Instruction::SLR(register_a, register_b, value)),
        "SRC" => Ok(Instruction::SRC(register_a, register_b, value)),
        "ROL" => Ok(Instruction::ROL(register_a, register_b, value)),
        "ROR" => Ok(Instruction::ROR(register_a, register_b, value)),
        _ => Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Failed to parse instruction".into(),
            },
            span,
        )),
    }
}
