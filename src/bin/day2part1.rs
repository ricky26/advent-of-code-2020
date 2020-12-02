use std::str::FromStr;
use std::ops::Range;
use anyhow::Error;
use nom::{
    IResult,
    character::complete::{space0, digit1, anychar},
    bytes::complete::{tag},
    combinator::{map_res},
};
use std::io::Read;

fn skip_whitespace(input: &str) -> &str {
    space0::<&str, nom::error::Error<&str>>(input).unwrap().0
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(
        digit1,
        usize::from_str,
    )(input)
}

fn parse_range(input: &str) -> IResult<&str, Range<usize>> {
    let input = skip_whitespace(input);
    let (input, start) = parse_usize(input)?;
    let input = skip_whitespace(input);
    let (input, _) = tag("-")(input)?;
    let input = skip_whitespace(input);
    let (input, end) = parse_usize(input)?;
    Ok((input, start..end+1))
}

pub struct Matcher {
    range: std::ops::Range<usize>,
    accept: char,
}

impl Matcher {
    fn parse(input: &str) -> IResult<&str, Matcher> {
        let (input, range) = parse_range(input)?;
        let input = skip_whitespace(input);
        let (input, accept) = anychar(input)?;

        Ok((input, Matcher{
            range,
            accept,
        }))
    }
}

fn parse_prompt(input: &str) -> IResult<&str, (Matcher, &str)> {
    let (input, matcher) = Matcher::parse(input)?;
    let input = skip_whitespace(input);
    let (input, _) = tag(":")(input)?;
    let input = skip_whitespace(input);
    let input = input.trim();
    Ok(("", (matcher, input)))
}

fn validate_prompt(m: Matcher, input: &str) -> Result<(), Error> {
    let instances = input.chars().filter(|c| *c == m.accept).count();
    if !m.range.contains(&instances) {
        Err(Error::msg("wrong number of instances"))?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut content = String::new();
    std::io::stdin().read_to_string(&mut content)?;

    let mut num_valid = 0;
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (rest, (m, input)) = match parse_prompt(line) {
            Ok(x) => x,
            Err(err) => Err(Error::msg(err.to_string()))?,
        };
        if !rest.is_empty() {
            Err(Error::msg("expected empty"))?;
        }

        if validate_prompt(m, input).is_ok() {
            num_valid += 1;
            println!("valid: {}", line)
        } else {
            println!("invalid: {}", line);
        }
    }

    println!("num: {}", num_valid);

    Ok(())
}
