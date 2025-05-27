mod no_operands;
mod reg_opcode;
mod reg_reg_opcodes;
mod reg_reg_value_opcodes;
mod reg_value_opcodes;
mod reg_value_reg_opcodes;
mod reg_value_value_opcodes;
mod value_opcodes;
mod value_reg_opcodes;
mod value_reg_value_opcodes;
mod value_value_opcodes;
mod value_value_reg;

use crate::rgal::no_operands::parse_no_operand_opcodes;
use crate::rgal::reg_opcode::parse_single_register_operand_opcodes;
use crate::rgal::reg_reg_opcodes::parse_two_register_operand_opcodes;
use crate::rgal::reg_reg_value_opcodes::parse_two_register_value_operand_opcodes;
use crate::rgal::reg_value_opcodes::parse_register_value_operand_opcodes;
use crate::rgal::reg_value_reg_opcodes::parse_register_value_register_operand_opcodes;
use crate::rgal::value_opcodes::parse_single_value_operand_opcodes;
use crate::rgal::value_reg_opcodes::parse_value_register_operand_opcodes;
use crate::rgal::value_reg_value_opcodes::parse_value_register_value_operand_opcodes;
use crate::rgal::value_value_opcodes::parse_two_value_operand_opcodes;
use crate::rgal::value_value_reg::parse_value_value_register_operand_opcodes;
use crate::shared::{Instruction, OperandValueType, Register};
use pest::error::ErrorVariant;
use pest::iterators::Pair;
use pest::{Parser, Position};
use pest_derive::Parser;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "rgal/rgal.pest"]
pub struct RgalParser;

// Parse a TPU program from a string
pub fn parse_program(input: &str) -> Result<Vec<Rc<Instruction>>, pest::error::Error<Rule>> {
    let pairs = RgalParser::parse(Rule::program, input.trim())?;
    let mut instructions = Vec::new();

    for pair in pairs {
        if pair.as_rule() == Rule::program {
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() == Rule::instruction {
                    for inner_pair in inner_pair.into_inner() {
                        instructions.push(Rc::new(parse_instruction_from_pair(inner_pair)?));
                    }
                }
            }
        }
    }

    Ok(instructions)
}

// Parse a single instruction from a string
pub fn parse_instruction(input: &str) -> Result<Instruction, pest::error::Error<Rule>> {
    let pairs = RgalParser::parse(Rule::instruction, input)?;

    for pair in pairs {
        if pair.as_rule() == Rule::instruction {
            for inner_pair in pair.into_inner() {
                return parse_instruction_from_pair(inner_pair);
            }
        }
    }

    Err(pest::error::Error::new_from_pos(
        ErrorVariant::CustomError {
            message: "Failed to parse instruction".into(),
        },
        Position::from_start(input),
    ))
}

fn parse_instruction_from_pair(pair: Pair<Rule>) -> Result<Instruction, pest::error::Error<Rule>> {
    let rule = pair.as_rule();
    let span = pair.as_span();
    let opcode_str;

    match rule {
        Rule::no_operand_instruction => parse_no_operand_opcodes(span, pair.as_str()),
        Rule::one_reg_operand_instruction => {
            let span = pair.as_span();
            let mut inner_pairs = pair.into_inner();
            opcode_str = inner_pairs
                .next()
                .ok_or(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))?
                .as_str();

            if let Some(operand_pair) = inner_pairs.next() {
                parse_single_register_operand_opcodes(
                    span,
                    opcode_str,
                    parse_any_operand_from_pair(operand_pair)?,
                )
            } else {
                Err(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))
            }
        }
        Rule::one_any_operand_instruction => {
            let span = pair.as_span();
            let mut inner_pairs = pair.into_inner();
            opcode_str = inner_pairs
                .next()
                .ok_or(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))?
                .as_str();

            if let Some(operand_pair) = inner_pairs.next() {
                parse_single_value_operand_opcodes(
                    span,
                    opcode_str,
                    parse_any_operand_from_pair(operand_pair)?,
                )
            } else {
                Err(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))
            }
        }
        Rule::two_reg_reg_operand_instruction => {
            let mut inner_pairs = pair.into_inner();
            opcode_str = inner_pairs
                .next()
                .ok_or(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))?
                .as_str();

            if let (Some(operand1_pair), Some(operand2_pair)) =
                (inner_pairs.next(), inner_pairs.next())
            {
                parse_two_register_operand_opcodes(
                    span,
                    opcode_str,
                    parse_any_operand_from_pair(operand1_pair)?,
                    parse_any_operand_from_pair(operand2_pair)?,
                )
            } else {
                Err(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))
            }
        }
        Rule::two_reg_any_operand_instruction => {
            let mut inner_pairs = pair.into_inner();
            opcode_str = inner_pairs
                .next()
                .ok_or(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))?
                .as_str();

            if let (Some(operand1_pair), Some(operand2_pair)) =
                (inner_pairs.next(), inner_pairs.next())
            {
                parse_register_value_operand_opcodes(
                    span,
                    opcode_str,
                    parse_any_operand_from_pair(operand1_pair)?,
                    parse_any_operand_from_pair(operand2_pair)?,
                )
            } else {
                Err(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))
            }
        }
        Rule::two_any_reg_operand_instruction => {
            let mut inner_pairs = pair.into_inner();
            opcode_str = inner_pairs
                .next()
                .ok_or(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))?
                .as_str();

            if let (Some(operand1_pair), Some(operand2_pair)) =
                (inner_pairs.next(), inner_pairs.next())
            {
                parse_value_register_operand_opcodes(
                    span,
                    opcode_str,
                    parse_any_operand_from_pair(operand1_pair)?,
                    parse_any_operand_from_pair(operand2_pair)?,
                )
            } else {
                Err(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))
            }
        }

        Rule::two_any_any_operand_instruction => {
            let mut inner_pairs = pair.into_inner();
            opcode_str = inner_pairs
                .next()
                .ok_or(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))?
                .as_str();

            if let (Some(operand1_pair), Some(operand2_pair)) =
                (inner_pairs.next(), inner_pairs.next())
            {
                parse_two_value_operand_opcodes(
                    span,
                    opcode_str,
                    parse_any_operand_from_pair(operand1_pair)?,
                    parse_any_operand_from_pair(operand2_pair)?,
                )
            } else {
                Err(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))
            }
        }
        Rule::three_reg_any_any_operand_instruction => {
            let mut inner_pairs = pair.into_inner();
            opcode_str = inner_pairs
                .next()
                .ok_or(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))?
                .as_str();

            if let (Some(operand1_pair), Some(operand2_pair), Some(operand3_pair)) =
                (inner_pairs.next(), inner_pairs.next(), inner_pairs.next())
            {
                parse_two_value_operand_opcodes(
                    span,
                    opcode_str,
                    parse_any_operand_from_pair(operand1_pair)?,
                    parse_any_operand_from_pair(operand2_pair)?,
                )
            } else {
                Err(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))
            }
        }
        Rule::three_any_reg_any_operand_instruction => {
            let mut inner_pairs = pair.into_inner();
            opcode_str = inner_pairs
                .next()
                .ok_or(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))?
                .as_str();

            if let (Some(operand1_pair), Some(operand2_pair), Some(operand3_pair)) =
                (inner_pairs.next(), inner_pairs.next(), inner_pairs.next())
            {
                parse_value_register_value_operand_opcodes(
                    span,
                    opcode_str,
                    parse_any_operand_from_pair(operand1_pair)?,
                    parse_any_operand_from_pair(operand2_pair)?,
                    parse_any_operand_from_pair(operand3_pair)?,
                )
            } else {
                Err(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))
            }
        }
        Rule::three_reg_reg_any_operand_instruction => {
            let mut inner_pairs = pair.into_inner();
            opcode_str = inner_pairs
                .next()
                .ok_or(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))?
                .as_str();

            if let (Some(operand1_pair), Some(operand2_pair), Some(operand3_pair)) =
                (inner_pairs.next(), inner_pairs.next(), inner_pairs.next())
            {
                parse_two_register_value_operand_opcodes(
                    span,
                    opcode_str,
                    parse_any_operand_from_pair(operand1_pair)?,
                    parse_any_operand_from_pair(operand2_pair)?,
                    parse_any_operand_from_pair(operand3_pair)?,
                )
            } else {
                Err(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))
            }
        }
        Rule::three_any_any_reg_operand_instruction => {
            let mut inner_pairs = pair.into_inner();
            opcode_str = inner_pairs
                .next()
                .ok_or(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))?
                .as_str();

            if let (Some(operand1_pair), Some(operand2_pair), Some(operand3_pair)) =
                (inner_pairs.next(), inner_pairs.next(), inner_pairs.next())
            {
                parse_value_value_register_operand_opcodes(
                    span,
                    opcode_str,
                    parse_any_operand_from_pair(operand1_pair)?,
                    parse_any_operand_from_pair(operand2_pair)?,
                    parse_any_operand_from_pair(operand3_pair)?,
                )
            } else {
                Err(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))
            }
        }
        Rule::three_reg_any_reg_operand_instruction => {
            let mut inner_pairs = pair.into_inner();
            opcode_str = inner_pairs
                .next()
                .ok_or(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))?
                .as_str();

            if let (Some(operand1_pair), Some(operand2_pair), Some(operand3_pair)) =
                (inner_pairs.next(), inner_pairs.next(), inner_pairs.next())
            {
                parse_register_value_register_operand_opcodes(
                    span,
                    opcode_str,
                    parse_any_operand_from_pair(operand1_pair)?,
                    parse_any_operand_from_pair(operand2_pair)?,
                    parse_any_operand_from_pair(operand3_pair)?,
                )
            } else {
                Err(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Failed to parse instruction".into(),
                    },
                    span,
                ))
            }
        }
        _ => todo!(),
    }
}

fn parse_any_operand_from_pair(
    pair: Pair<Rule>,
) -> Result<OperandValueType, pest::error::Error<Rule>> {
    let span = pair.as_span();

    match pair.as_rule() {
        Rule::register => {
            let register_str = pair.as_str();
            match register_str {
                "A" => Ok(OperandValueType::Register(Register::A)),
                "X" => Ok(OperandValueType::Register(Register::X)),
                "Y" => Ok(OperandValueType::Register(Register::Y)),
                "R0" => Ok(OperandValueType::Register(Register::R0)),
                "R1" => Ok(OperandValueType::Register(Register::R1)),
                "R2" => Ok(OperandValueType::Register(Register::R2)),
                "R3" => Ok(OperandValueType::Register(Register::R3)),
                "R4" => Ok(OperandValueType::Register(Register::R4)),
                "R5" => Ok(OperandValueType::Register(Register::R5)),
                "R6" => Ok(OperandValueType::Register(Register::R6)),
                r @ _ => Err(pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: format!("Invalid register: {r}"),
                    },
                    span,
                )),
            }
        }
        Rule::hex_number => {
            let hex_str = pair.as_str().trim_start_matches("0x");
            u16::from_str_radix(hex_str, 16)
                .map(OperandValueType::Immediate)
                .map_err(|e| {
                    pest::error::Error::new_from_span(
                        ErrorVariant::CustomError {
                            message: format!("Invalid hex number: {e}"),
                        },
                        span,
                    )
                })
        }
        Rule::binary_number => {
            let bin_str = pair.as_str().trim_start_matches("0b");
            u16::from_str_radix(bin_str, 2)
                .map(OperandValueType::Immediate)
                .map_err(|e| {
                    pest::error::Error::new_from_span(
                        ErrorVariant::CustomError {
                            message: format!("Invalid binary number: {e}"),
                        },
                        span,
                    )
                })
        }
        Rule::decimal_number => u16::from_str(pair.as_str())
            .map(OperandValueType::Immediate)
            .map_err(|e| {
                pest::error::Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: format!("Invalid decimal number: {e}"),
                    },
                    span,
                )
            }),

        x @ _ => Err(pest::error::Error::new_from_span(
            ErrorVariant::CustomError {
                message: format!("Invalid operand {x:?}"),
            },
            span,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::error::ErrorVariant;
    use pest::error::LineColLocation::Pos;

    #[test]
    fn test_parse_instruction() {
        let instruction = parse_instruction("PUSH 42").unwrap();
        match instruction {
            Instruction::PUSH(operand) => {
                assert_eq!(operand, OperandValueType::Immediate(42));
            }
            _ => panic!("Unexpected instruction: {:?}", instruction),
        }

        let instruction = parse_instruction("POP A").unwrap();
        match instruction {
            Instruction::POP(reg) => {
                assert_eq!(reg, Register::A);
            }
            _ => panic!("Unexpected instruction: {:?}", instruction),
        }

        let instruction = parse_instruction("ADD A, X").unwrap();
        match instruction {
            Instruction::ADD(reg, operand) => {
                assert_eq!(reg, Register::A);
                assert_eq!(operand, Register::X);
            }
            _ => panic!("Unexpected instruction: {:?}", instruction),
        }

        let instruction = parse_instruction("HLT").unwrap();
        assert_eq!(instruction, Instruction::HLT);

        // Test analog pin operands
        match parse_instruction("APR A, 0") {
            Ok(instruction) => match instruction {
                Instruction::APR(reg, operand) => {
                    assert_eq!(reg, Register::A);
                    assert_eq!(operand, OperandValueType::Immediate(0));
                }
                _ => panic!("Unexpected instruction: {:?}", instruction),
            },
            Err(e) => {
                panic!("Failed to parse 'APR A, 0': {:?}", e);
            }
        }

        // Test digital pin operands
        match parse_instruction("DPR A, 0") {
            Ok(instruction) => match instruction {
                Instruction::DPR(reg, operand) => {
                    assert_eq!(reg, Register::A);
                    assert_eq!(operand, OperandValueType::Immediate(0));
                }
                _ => panic!("Unexpected instruction: {:?}", instruction),
            },
            Err(e) => {
                panic!("Failed to parse 'DPR A, 0': {:?}", e);
            }
        }
    }

    #[test]
    fn test_parse_program() {
        let program = parse_program("PUSH 42\nPOP A\nADD A, X\nNOP\nSUB R0, R1\nHLT");

        let program = match program {
            Ok(program) => program,
            Err(e) => match e.variant {
                ErrorVariant::ParsingError { ref positives, .. } => {
                    let (line, col) = match e.line_col {
                        Pos((line, col)) => (line, col),
                        _ => (1, 1),
                    };
                    panic!(
                        "Error in program at line {line}, {col}, Expected {:?}, found {} instead",
                        positives,
                        e.line()
                    );
                }
                _ => panic!("Failed to parse program: {:?}", e),
            },
        };

        assert_eq!(program.len(), 6);

        match &*program[0] {
            Instruction::PUSH(operand) => {
                assert_eq!(*operand, OperandValueType::Immediate(42));
            }
            _ => panic!("Unexpected instruction at index 0: {:?}", program[0]),
        }

        match &*program[1] {
            Instruction::POP(reg) => {
                assert_eq!(*reg, Register::A);
            }
            _ => panic!("Unexpected instruction at index 1: {:?}", program[1]),
        }

        match &*program[2] {
            Instruction::ADD(reg, operand) => {
                assert_eq!(*reg, Register::A);
                assert_eq!(*operand, Register::X);
            }
            _ => panic!("Unexpected instruction at index 2: {:?}", program[2]),
        }

        match &*program[3] {
            Instruction::NOP => {}
            _ => panic!("Unexpected instruction at index 3: {:?}", program[3]),
        }

        match &*program[4] {
            Instruction::SUB(reg1, reg2) => {
                assert_eq!(*reg1, Register::R0);
                assert_eq!(*reg2, Register::R1);
            }
            _ => panic!("Unexpected instruction at index 4: {:?}", program[4]),
        }

        match &*program[5] {
            Instruction::HLT => {}
            _ => panic!("Unexpected instruction at index 5: {:?}", program[5]),
        }

        // Test a program with analog and digital pin operations
        let program_str = "APR A, 0\nDPR X, 1\nAPW 2, 42\nDPW 3, 1";
        match parse_program(program_str) {
            Ok(program) => {
                assert_eq!(program.len(), 4);

                match &*program[0] {
                    Instruction::APR(reg, operand) => {
                        assert_eq!(*reg, Register::A);
                        assert_eq!(*operand, OperandValueType::Immediate(0));
                    }
                    _ => panic!("Unexpected instruction at index 0: {:?}", program[0]),
                }

                match &*program[1] {
                    Instruction::DPR(reg, operand) => {
                        assert_eq!(*reg, Register::X);
                        assert_eq!(*operand, OperandValueType::Immediate(1));
                    }
                    _ => panic!("Unexpected instruction at index 1: {:?}", program[1]),
                }

                match &*program[2] {
                    Instruction::APW(pin, value) => {
                        assert_eq!(*pin, OperandValueType::Immediate(2));
                        assert_eq!(*value, OperandValueType::Immediate(42));
                    }
                    _ => panic!("Unexpected instruction at index 2: {:?}", program[2]),
                }

                match &*program[3] {
                    Instruction::DPW(pin, value) => {
                        assert_eq!(*pin, OperandValueType::Immediate(3));
                        assert_eq!(*value, OperandValueType::Immediate(1));
                    }
                    _ => panic!("Unexpected instruction at index 3: {:?}", program[3]),
                }
            }
            Err(e) => {
                panic!("Failed to parse program: {:?}", e);
            }
        }
    }
}
