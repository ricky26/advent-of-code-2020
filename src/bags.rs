use nom::{
    IResult,
    character::complete::{space0, digit1, alpha1, space1, char},
    bytes::complete::{tag},
    combinator::{opt, recognize, map},
    sequence::{pair, tuple},
    multi::separated_list1,
    branch::alt,
};
use std::collections::BTreeMap;

fn skip_whitespace(input: &str) -> &str {
    space0::<&str, nom::error::Error<&str>>(input).unwrap().0
}

fn parse_bag_name(input: &str) -> IResult<&str, &str> {
    let input = skip_whitespace(input);
    let (input, name) = recognize(tuple((alpha1, space1, alpha1)))(input)?;
    let input = skip_whitespace(input);
    let (input, _) = tag("bag")(input)?;
    let (input, _) = opt(char('s'))(input)?;
    let input = skip_whitespace(input);
    Ok((input, name))
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (input, num_str) = digit1(input)?;
    let num = num_str.parse().unwrap();
    Ok((input, num))
}

fn parse_bag_count(input: &str) -> IResult<&str, (usize, &str)> {
    let input = skip_whitespace(input);
    let (input, count) = parse_usize(input)?;
    let input = skip_whitespace(input);
    let (input, bag_name) = parse_bag_name(input)?;
    let input = skip_whitespace(input);

    Ok((input, (count, bag_name)))
}

fn parse_bag_list(input: &str) -> IResult<&str, Vec<(usize, &str)>> {
    separated_list1(pair(char(','), space1), parse_bag_count)(input)
}

fn parse_statement(input: &str) -> IResult<&str, (&str, Vec<(usize, &str)>)> {
    let (input, bag_name) = parse_bag_name(input)?;
    let input = skip_whitespace(input);
    let (input, _) = tag("contain")(input)?;
    let input = skip_whitespace(input);
    let (input, list) = alt((parse_bag_list, map(tag("no other bags"), |_| Vec::new())))(input)?;
    let (input, _) = char('.')(input)?;
    let input = skip_whitespace(input);
    Ok((input, (bag_name, list)))
}

pub fn parse_mapping(input: &str) -> IResult<&str, BTreeMap<String, BTreeMap<String, usize>>> {
    let mut mappings = BTreeMap::new();
    for line in input.lines().map(str::trim) {
        if line.is_empty() {
            continue
        }

        let (input, (bag_name, list)) = parse_statement(line)?;
        if !input.is_empty() {
            return Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::LengthValue)))
        }

        let mut mapping = BTreeMap::new();
        mapping.extend(list.iter().map(|(a, b)| (b.to_string(), *a)));
        mappings.insert(bag_name.to_string(), mapping);
    }
    Ok(("", mappings))
}

#[test]
fn test_parse() {
    use std::iter::FromIterator;

    let test_input = "\
        dotted black bags contain no other bags.
        bright white bags contain 1 shiny gold bag.
        light red bags contain 1 bright white bag, 2 muted yellow bags.
        ";

    assert_eq!(parse_mapping(test_input).unwrap().1, BTreeMap::from_iter(vec![
        ("dotted black".to_string(), BTreeMap::new()),
        ("bright white".to_string(), BTreeMap::from_iter(vec![
            ("shiny gold".to_string(), 1),
        ])),
        ("light red".to_string(), BTreeMap::from_iter(vec![
            ("bright white".to_string(), 1),
            ("muted yellow".to_string(), 2),
        ])),
    ]));
}
