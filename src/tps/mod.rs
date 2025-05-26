use crate::shared::{Instruction, Opcode, Operand, Register};
use pest::error::ErrorVariant;
use pest::iterators::Pair;
use pest::{Parser, Position};
use pest_derive::Parser;
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "tps/tpl.pest"]
pub struct TplParser;

// Parse a TPU program from a string
pub fn parse_program(input: &str) -> Result<Vec<Instruction>, pest::error::Error<Rule>> {
    let pairs = TplParser::parse(Rule::program, input.trim())?;
    let mut instructions = Vec::new();

    for pair in pairs {
        if pair.as_rule() == Rule::program {
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() == Rule::instruction {
                    for inner_pair in inner_pair.into_inner() {
                        instructions.push(parse_instruction_from_pair(inner_pair)?);
                    }
                }
            }
        }
    }

    Ok(instructions)
}

// Parse a single instruction from a string
pub fn parse_instruction(input: &str) -> Result<Instruction, pest::error::Error<Rule>> {
    let pairs = TplParser::parse(Rule::instruction, input)?;

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
    let mut operands = Vec::new();
    let opcode_str;

    match rule {
        Rule::no_operand_instruction => {
            opcode_str = pair.as_str();
        }
        Rule::one_reg_operand_instruction | Rule::one_any_operand_instruction => {
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
                operands.push(parse_any_operand_from_pair(operand_pair)?)
            }
        }
        Rule::two_reg_any_operand_instruction
        | Rule::two_any_reg_operand_instruction
        | Rule::two_reg_reg_operand_instruction
        | Rule::two_any_any_operand_instruction => {
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

            if let Some(operand1_pair) = inner_pairs.next() {
                operands.push(parse_any_operand_from_pair(operand1_pair)?)
            }

            if let Some(operand2_pair) = inner_pairs.next() {
                operands.push(parse_any_operand_from_pair(operand2_pair)?)
            }
        }
        Rule::three_reg_any_any_operand_instruction
        | Rule::three_any_any_any_operand_instruction => {
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

            if let Some(operand1_pair) = inner_pairs.next() {
                operands.push(parse_any_operand_from_pair(operand1_pair)?)
            }

            if let Some(operand2_pair) = inner_pairs.next() {
                operands.push(parse_any_operand_from_pair(operand2_pair)?)
            }

            if let Some(operand3_pair) = inner_pairs.next() {
                operands.push(parse_any_operand_from_pair(operand3_pair)?)
            }
        }
        x @ _ => {
            return Err(pest::error::Error::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("Failed to parse instruction {x:?}"),
                },
                span,
            ));
        }
    }

    let opcode = Opcode::from_str(opcode_str).expect("get Opccode");
    Ok(Instruction { opcode, operands })
}

fn parse_any_operand_from_pair(pair: Pair<Rule>) -> Result<Operand, pest::error::Error<Rule>> {
    let span = pair.as_span();

    match pair.as_rule() {
        Rule::register => {
            let register_str = pair.as_str();
            match register_str {
                "A" => Ok(Operand::Register(Register::A)),
                "X" => Ok(Operand::Register(Register::X)),
                "Y" => Ok(Operand::Register(Register::Y)),
                "R0" => Ok(Operand::Register(Register::R0)),
                "R1" => Ok(Operand::Register(Register::R1)),
                "R2" => Ok(Operand::Register(Register::R2)),
                "R3" => Ok(Operand::Register(Register::R3)),
                "R4" => Ok(Operand::Register(Register::R4)),
                "R5" => Ok(Operand::Register(Register::R5)),
                "R6" => Ok(Operand::Register(Register::R6)),
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
                .map(Operand::Constant)
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
                .map(Operand::Constant)
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
            .map(Operand::Constant)
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
        assert_eq!(instruction.opcode, Opcode::PUSH);
        assert_eq!(instruction.operands, vec![Operand::Constant(42)]);

        let instruction = parse_instruction("POP A").unwrap();
        assert_eq!(instruction.opcode, Opcode::POP);
        assert_eq!(instruction.operands, vec![Operand::Register(Register::A)]);

        let instruction = parse_instruction("ADD A, 10").unwrap();
        assert_eq!(instruction.opcode, Opcode::ADD);
        assert_eq!(
            instruction.operands,
            vec![Operand::Register(Register::A), Operand::Constant(10)]
        );

        let instruction = parse_instruction("HLT").unwrap();
        assert_eq!(instruction.opcode, Opcode::HLT);
        assert_eq!(instruction.operands, vec![]);

        // Test analog pin operands
        match parse_instruction("APR A, 0") {
            Ok(instruction) => {
                assert_eq!(instruction.opcode, Opcode::APR);
                assert_eq!(
                    instruction.operands,
                    vec![Operand::Register(Register::A), Operand::Constant(0)]
                );
            }
            Err(e) => {
                panic!("Failed to parse 'APR A, Analog0': {:?}", e);
            }
        }

        // Test digital pin operands
        match parse_instruction("DPR A, 0") {
            Ok(instruction) => {
                assert_eq!(instruction.opcode, Opcode::DPR);
                assert_eq!(
                    instruction.operands,
                    vec![Operand::Register(Register::A), Operand::Constant(0)]
                );
            }
            Err(e) => {
                panic!("Failed to parse 'DPR A, Digital0': {:?}", e);
            }
        }
    }

    #[test]
    fn test_parse_program() {
        let program = parse_program("PUSH 42\nPOP A\nADD A, 10\nPUSHX\nSUB R0, R1\nHLT");

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
        assert_eq!(program[0].opcode, Opcode::PUSH);
        assert_eq!(program[0].operands, vec![Operand::Constant(42)]);
        assert_eq!(program[1].opcode, Opcode::POP);
        assert_eq!(program[1].operands, vec![Operand::Register(Register::A)]);
        assert_eq!(program[2].opcode, Opcode::ADD);
        assert_eq!(
            program[2].operands,
            vec![Operand::Register(Register::A), Operand::Constant(10)]
        );
        assert_eq!(program[3].opcode, Opcode::PUSHX);
        assert_eq!(program[3].operands, vec![]);
        assert_eq!(program[4].opcode, Opcode::SUB);
        assert_eq!(
            program[4].operands,
            vec![
                Operand::Register(Register::R0),
                Operand::Register(Register::R1)
            ]
        );
        assert_eq!(program[5].opcode, Opcode::HLT);
        assert_eq!(program[5].operands, vec![]);

        // Test a program with analog and digital pin operations
        let program_str = "APR A, 0\nDPR X, 1\nAPW 2, 42\nDPW 3, 1";
        match parse_program(program_str) {
            Ok(program) => {
                assert_eq!(program.len(), 4);
                assert_eq!(program[0].opcode, Opcode::APR);
                assert_eq!(
                    program[0].operands,
                    vec![Operand::Register(Register::A), Operand::Constant(0)]
                );
                assert_eq!(program[1].opcode, Opcode::DPR);
                assert_eq!(
                    program[1].operands,
                    vec![Operand::Register(Register::X), Operand::Constant(1)]
                );
                assert_eq!(program[2].opcode, Opcode::APW);
                assert_eq!(
                    program[2].operands,
                    vec![Operand::Constant(2), Operand::Constant(42)]
                );
                assert_eq!(program[3].opcode, Opcode::DPW);
                assert_eq!(
                    program[3].operands,
                    vec![Operand::Constant(3), Operand::Constant(1)]
                );
            }
            Err(e) => {
                panic!("Failed to parse program: {:?}", e);
            }
        }
    }
}
