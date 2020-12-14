use nom::{
    IResult,
    character::complete::{space0, alpha1, alphanumeric0},
    number::complete::double,
    combinator::recognize,
    sequence::pair,
};
use super::vm::Instruction;
use nom::error::{ErrorKind};
use anyhow::{anyhow, Error};

#[derive(Debug)]
pub struct NomError(pub anyhow::Error);

impl std::fmt::Display for NomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<NomError> for anyhow::Error {
    fn from(e: NomError) -> Self {
        e.0
    }
}

impl From<anyhow::Error> for NomError {
    fn from(e: Error) -> Self {
        NomError(e)
    }
}

impl nom::error::ParseError<&str> for NomError {
    fn from_error_kind(input: &str, kind: ErrorKind) -> Self {
        NomError(anyhow!("error {} at: {}", kind.description(), input))
    }

    fn append(input: &str, kind: ErrorKind, other: Self) -> Self {
        NomError(other.0.context(format!("error {} at: {}", kind.description(), input)))
    }
}

fn parse_i32(input: &str) -> IResult<&str, i32, NomError> {
    let (input, d) = double(input)?;
    Ok((input, d as i32))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction, NomError> {
    let (input, _) = space0(input)?;
    let (input, inst_name) = recognize(pair(alpha1, alphanumeric0))(input)?;
    let (input, _) = space0(input)?;
    let (input, imm) = parse_i32(input)?;
    let (input, _) = space0(input)?;

    let inst = match inst_name {
        "nop" => Instruction::Nop(imm),
        "acc" => Instruction::Acc(imm),
        "jmp" => Instruction::Jmp(imm),
        _ => Err(nom::Err::Failure(NomError(anyhow!("no such instruction: {}", inst_name))))?,
    };

    Ok((input, inst))
}

pub fn parse_asm(input: &str) -> IResult<&str, Vec<Instruction>, NomError> {
    let mut result = Vec::new();
    for line in input.lines().map(str::trim) {
        if line.is_empty() {
            continue;
        }

        let (input, inst) = parse_instruction(line)?;
        if !input.is_empty() {
            Err(nom::Err::Error(NomError(anyhow!("unexpected suffix on instruction: {}", input))))?;
        }
        result.push(inst);
    }
    Ok(("", result))
}
