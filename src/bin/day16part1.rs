use std::io::Read;
use std::collections::{BTreeMap, BTreeSet};
use nom::{
    IResult,
    Finish,
    InputTakeAtPosition,
    combinator::{map, all_consuming},
    bytes::complete::tag,
    character::complete::{char, space0, digit1, newline},
    sequence::tuple,
    multi::{separated_list1, fold_many1},
};
use aoc2020::asm::NomError;

fn parse_i32(input: &str) -> IResult<&str, i32, NomError> {
    let (input, digits) = digit1(input)?;
    let v = digits.parse().map_err(NomError::fail)?;
    Ok((input, v))
}

fn parse_i32_list(input: &str) -> IResult<&str, Vec<i32>, NomError> {
    separated_list1(char(','), parse_i32)(input)
}

#[derive(Clone, Debug)]
struct FieldValidator {
    ranges: Vec<(i32, i32)>,
}

impl FieldValidator {
    pub fn parse(input: &str) -> IResult<&str, FieldValidator, NomError> {
        let separator = tuple((space0, tag("or"), space0));
        let value = map(
            tuple((parse_i32, char('-'), parse_i32)),
            |(a, _, b)| (a, b));
        let (input, ranges) = separated_list1(separator, value)(input)?;
        let validator = FieldValidator{ranges};
        Ok((input, validator))
    }

    pub fn contains(&self, v: i32) -> bool {
        self.ranges.iter()
            .copied()
            .any(|(start, end)| (v >= start) && (v <= end))
    }
}

#[derive(Clone, Debug)]
struct Notes {
    fields: BTreeMap<String, FieldValidator>,
    my_ticket: Vec<i32>,
    nearby_tickets: Vec<i32>,
}

impl Notes {
    fn parse_field(input: &str) -> IResult<&str, (&str, FieldValidator), NomError> {
        let (input, name) = input.split_at_position1_complete(
            |c| !c.is_alphanumeric() && c != ' ',
            nom::error::ErrorKind::Char)?;
        let (input, _) = space0(input)?;
        let (input, _) = char(':')(input)?;
        let (input, _) = space0(input)?;
        let (input, fields) = FieldValidator::parse(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = newline(input)?;
        Ok((input, (name, fields)))
    }

    pub fn parse(input: &str) -> IResult<&str, Notes, NomError> {
        let (input, fields) = fold_many1(
            Notes::parse_field,
            BTreeMap::new(),
            |mut acc, (key, value)| {
                acc.insert(key.to_string(), value);
                acc
            })(input)?;
        let (input, _) = newline(input)?;

        let mut ticket = map(
            tuple((parse_i32_list, newline)),
            |(t, _)| t);

        let (input, _) = tag("your ticket:\n")(input)?;
        let (input, my_ticket) = ticket(input)?;
        let (input, _) = newline(input)?;
        let (input, _) = tag("nearby tickets:\n")(input)?;
        let (input, nearby_tickets) = fold_many1(
            ticket,
            Vec::new(),
            |mut acc, item| {
                acc.extend(item.into_iter());
                acc
            })(input)?;

        Ok((input, Notes{
            fields,
            my_ticket,
            nearby_tickets,
        }))
    }

    pub fn is_any_field(&self, v: i32) -> bool {
        self.fields.iter().any(|(_, f)| f.contains(v))
    }
}

fn main() -> anyhow::Result<()> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;

    let (_, notes) = all_consuming(Notes::parse)(&contents).finish()?;
    let ticket_len = notes.fields.len();

    let mut bad_sum = 0;
    let mut masks = notes.fields.iter()
        .map(|_| {
            let mut set = BTreeSet::new();
            set.extend(0..ticket_len);
            set
        })
        .collect::<Vec<_>>();

    for ticket in notes.nearby_tickets.chunks(ticket_len) {
        let mut is_valid = true;
        for field in ticket.iter().copied() {
            if !notes.is_any_field(field) {
                bad_sum += field;
                is_valid = false;
            }
        }
        if !is_valid {
            continue
        }

        for (idx_a, field) in ticket.iter().copied().enumerate() {
            for (idx_b, (_, v)) in notes.fields.iter().enumerate() {
                if !v.contains(field) {
                    masks[idx_a].remove(&idx_b);
                }
            }
        }
    }

    println!("sum {}", bad_sum);
    println!("masks {:?}", masks);

    let mut finished = vec![false; ticket_len];
    let mut field_map = vec![None; ticket_len];
    loop {
        let mut changed = false;
        for (idx, mask) in masks.iter_mut().enumerate() {
            if finished[idx] {
                continue
            }

            if mask.len() == 1 {
                let target = *mask.iter().next().unwrap();
                field_map[target] = Some(idx);
                finished[idx] = true;
                changed = true;
                continue;
            }

            let completed = field_map.iter()
                .copied()
                .enumerate()
                .filter_map(|(idx, v)| if v.is_some() { Some(idx) } else { None });

            for c in completed {
                changed |= mask.remove(&c);
            }
        }

        if !changed {
            break
        }
    }

    println!("masks {:?}", masks);

    if finished.iter().any(|f| !*f) {
        println!("Not finished: {:?}", finished);
        return Ok(())
    }
    let field_map = field_map.iter()
        .copied()
        .map(Option::unwrap)
        .collect::<Vec<_>>();
    let target_fields = notes.fields.iter()
        .enumerate()
        .filter(|(_, (name, _))| name.starts_with("departure"))
        .map(|(idx, _)| field_map[idx])
        .collect::<Vec<_>>();

    let result = target_fields.iter().copied()
        .fold(1, |acc, item| acc * (notes.my_ticket[item] as i64));
    println!("result {}", result);
    Ok(())
}
