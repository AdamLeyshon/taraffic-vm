use crate::shared::{Instruction, OperandValueType};
use crate::tps::Rule;
use pest::Span;
use pest::error::ErrorVariant;

pub fn parse_two_register_operand_opcodes(
    span: Span,
    opcode: &str,
    register_a: OperandValueType,
    register_b: OperandValueType,
) -> Result<Instruction, pest::error::Error<Rule>> {
    let (OperandValueType::Register(register_a), OperandValueType::Register(register_b)) =
        (register_a, register_b)
    else {
        return Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Expected two register operands".into(),
            },
            span,
        ));
    };

    match opcode {
        "ADD" => Ok(Instruction::ADD(register_a, register_b)),
        "SUB" => Ok(Instruction::SUB(register_a, register_b)),
        "MUL" => Ok(Instruction::MUL(register_a, register_b)),
        "DIV" => Ok(Instruction::DIV(register_a, register_b)),
        "MOD" => Ok(Instruction::MOD(register_a, register_b)),
        "AND" => Ok(Instruction::AND(register_a, register_b)),
        "OR" => Ok(Instruction::OR(register_a, register_b)),
        "XOR" => Ok(Instruction::XOR(register_a, register_b)),
        "RCY" => Ok(Instruction::RCY(register_a, register_b)),
        "RMV" => Ok(Instruction::RMV(register_a, register_b)),
        
        _ => Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Failed to parse instruction".into(),
            },
            span,
        )),
    }
}
