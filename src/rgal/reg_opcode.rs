use crate::shared::{Instruction, OperandValueType};
use crate::rgal::Rule;
use pest::Span;
use pest::error::ErrorVariant;

pub fn parse_single_register_operand_opcodes(
    span: Span,
    opcode: &str,
    operand_value_type: OperandValueType,
) -> Result<Instruction, pest::error::Error<Rule>> {
    let OperandValueType::Register(register_operand) = operand_value_type else {
        return Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Expected single register operand".into(),
            },
            span,
        ));
    };

    match opcode {
        "POP" => Ok(Instruction::POP(register_operand)),
        "RSP" => Ok(Instruction::RSP(register_operand)),
        "NOT" => Ok(Instruction::NOT(register_operand)),
        "INC" => Ok(Instruction::INC(register_operand)),
        "DEC" => Ok(Instruction::DEC(register_operand)),
        "DPRW" => Ok(Instruction::DPRW(register_operand)),
        
        _ => Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Failed to parse instruction".into(),
            },
            span,
        )),
    }
}
