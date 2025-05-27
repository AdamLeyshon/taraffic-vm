use crate::shared::{Instruction, OperandValueType};
use crate::rgal::Rule;
use pest::Span;
use pest::error::ErrorVariant;

pub fn parse_value_register_operand_opcodes(
    span: Span,
    opcode: &str,
    value: OperandValueType,
    register: OperandValueType,
) -> Result<Instruction, pest::error::Error<Rule>> {
    let OperandValueType::Register(register) = register
    else {
        return Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Expected a register and value operand".into(),
            },
            span,
        ));
    };

    match opcode {
        "BEZ" => Ok(Instruction::BEZ(value, register)),
        "BNZ" => Ok(Instruction::BNZ(value, register)),
        "BREZ"=> Ok(Instruction::BREZ(value, register)),
        "BRNZ"=> Ok(Instruction::BRNZ(value, register)),

        _ => Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Failed to parse instruction".into(),
            },
            span,
        )),
    }
}
