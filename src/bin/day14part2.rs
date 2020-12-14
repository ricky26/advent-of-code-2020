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
use std::collections::BTreeMap;

fn parse_number(input: &str) -> IResult<&str, (u64, u64), NomError> {
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
    Mask(u64, u64),
    Set(u64, u64),
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

                let idx = idx.parse::<u64>().map_err(to_nom_error)?;
                let num = num.parse::<u64>().map_err(to_nom_error)?;

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
    let mut mem = BTreeMap::new();

    for line in lines {
        let (input, cmd) = Command::parse(line)?;
        eof::<_, NomError>(input)?;

        match cmd {
            Command::Mask(s, c) => {
                set = s;
                clear = c;
            },
            Command::Set(addr, value) => {
                let floating = !(set | clear) & ((1 << 36) - 1);
                let perms = 1 << (floating.count_ones() as u64);

                for mut bits in 0..perms {
                    let mut addr = set | (addr & clear);

                    for n in 0.. {
                        let t = floating >> n;
                        if t == 0 {
                            break
                        } else if t & 1 == 0 {
                            continue
                        }

                        addr |= (bits & 1) << n;
                        bits >>= 1;
                    }

                    mem.insert(addr, value);
                }
            },
        }
    }

    println!("end result, mask {:b} / {:b}", set, clear);
    for (idx, m) in mem.iter() {
        let idx = *idx;
        let m = *m;
        if m == 0 {
            continue
        }
        println!("mem {:08x}: {:038b}", idx, m);
    }

    let total: u64 = mem.iter().map(|s| *s.1).sum();
    println!("mem total {}", total);

    Ok(())
}
