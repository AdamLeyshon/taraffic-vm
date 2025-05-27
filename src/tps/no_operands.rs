use crate::shared::Instruction;
use crate::tps::Rule;
use pest::Span;
use pest::error::ErrorVariant;

pub fn parse_no_operand_opcodes(
    span: Span,
    opcode: &str,
) -> Result<Instruction, pest::error::Error<Rule>> {
    match opcode {
        "SCR" => Ok(Instruction::SCR),
        "RECV" => Ok(Instruction::RECV),
        "TXBS" => Ok(Instruction::TXBS),
        "RXBS" => Ok(Instruction::RXBS),
        "NOP" => Ok(Instruction::NOP),
        "WRX" => Ok(Instruction::WRX),
        "HLT" => Ok(Instruction::HLT),
        "RTS" => Ok(Instruction::RTS),
        _ => Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: "Failed to parse instruction".into(),
            },
            span,
        )),
    }
}
