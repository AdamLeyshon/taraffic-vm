use crate::shared::{Instruction, OperandValueType};
use crate::rgal::Rule;
use pest::Span;
use pest::error::ErrorVariant;

pub fn parse_register_value_operand_opcodes(
    span: Span,
    opcode: &str,
    register: OperandValueType,
    value: OperandValueType,
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
        "PEEK" => Ok(Instruction::PEEK(register, value)),
        "XMIT" => Ok(Instruction::XMIT(register, value)),
        "LDR" => Ok(Instruction::LDR(register, value)),
        "DPR" => Ok(Instruction::DPR(register, value)),
        "APR" => Ok(Instruction::APR(register, value)),
        
        _ => Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Failed to parse instruction".into(),
            },
            span,
        )),
    }
}
