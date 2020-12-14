use std::io::Read;
use anyhow::anyhow;
use nom::{
    IResult,
    character::complete::{space0, digit1, alphanumeric1, char},
    combinator::eof,
    sequence::tuple,
    InputTakeAtPosition,
};
use aoc2020::asm::NomError;

fn parse_number(input: &str) -> IResult<&str, (usize, usize), NomError> {
    let mut set = 0;
    let mut clear = 0;

    let (input, bits) = input.split_at_position_complete(|c| c != '0' && c != '1' && c != 'X')?;
    let l = bits.len();

    for (idx, i) in bits.chars().enumerate() {
        let v = 1 << (l - idx - 1);

        match i {
            '0' => clear |= v,
            '1' => set |= v,
            _ => {},
        }
    }

    assert_eq!(set & clear, 0);
    Ok((input, (set, clear)))
}

fn to_nom_error<T: Into<anyhow::Error>>(src: T) -> nom::Err<NomError> {
    nom::Err::Failure(NomError(src.into()))
}

enum Command {
    Mask(usize, usize),
    Set(usize, usize),
}

impl Command {
    pub fn parse(input: &str) -> IResult<&str, Command, NomError> {
        let (input, cmd) = alphanumeric1(input)?;
        let (input, _) = space0(input)?;

        match cmd {
            "mem" => {
                let (input, (_, _, idx, _, _)) = tuple((char('['), space0, digit1, space0, char(']')))(input)?;
                let (input, _) = space0(input)?;
                let (input, _) = char('=')(input)?;
                let (input, _) = space0(input)?;
                let (input, num) = digit1(input)?;

                let idx = idx.parse::<usize>().map_err(to_nom_error)?;
                let num = num.parse::<usize>().map_err(to_nom_error)?;

                Ok((input, Command::Set(idx, num)))
            },
            "mask" => {
                let (input, _) = char('=')(input)?;
                let (input, _) = space0(input)?;
                let (input, (set, clear)) = parse_number(input)?;
                Ok((input, Command::Mask(set, clear)))
            },
            _ => Err(nom::Err::Failure(anyhow!("unexpected command").into()))?,
        }
    }
}


fn main() -> anyhow::Result<()> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;
    let lines = contents.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty());

    let mut set = 0;
    let mut clear = 0;
    let mut mem = Vec::new();

    for line in lines {
        let (input, cmd) = Command::parse(line)?;
        eof::<_, NomError>(input)?;

        match cmd {
            Command::Mask(s, c) => {
                set = s;
                clear = c;
            },
            Command::Set(addr, value) => {
                let value = (value & !clear) | set;

                while mem.len() <= addr {
                    mem.push(0);
                }

                mem[addr] = value;
            },
        }
    }

    println!("end result, mask {:b} / {:b}", set, clear);
    for (idx, m) in mem.iter().copied().enumerate() {
        println!("mem {:08x}: {:038b}", idx, m);
    }

    let total: usize = mem.iter().copied().sum();
    println!("mem total {}", total);

    Ok(())
}
