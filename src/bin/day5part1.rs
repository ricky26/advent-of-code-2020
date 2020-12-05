use std::io::Read;

fn binary_partition(s: &str) -> anyhow::Result<i16> {
    let mut next = 1 << (s.len() - 1);
    let mut v = 0;

    for c in s.chars() {
        match c {
            'F' | 'L' => {},
            'B' | 'R' => {
                v = v | next;
            },
            c => Err(anyhow::Error::msg(format!("unexpected char {}", c)))?,
        };
        next = next >> 1;
    }

    Ok(v)
}

fn boarding_pass_to_coords(p: &str) -> anyhow::Result<(i16, i16)> {
    if p.len() != 10 {
        Err(anyhow::Error::msg(format!("invalid pass len {}", p.len())))?;
    }

    let row = binary_partition(&p[..7])?;
    let col = binary_partition(&p[7..])?;
    return Ok((row, col))
}

fn boarding_pass_to_seat(p: &str) -> anyhow::Result<i16> {
    boarding_pass_to_coords(p).map(|(r, c)| r * 8 + c)
}

fn main() -> anyhow::Result<()> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;

    let max = contents.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(boarding_pass_to_seat)
        .fold(Ok::<_, anyhow::Error>(0), |acc, x| {
            let acc = acc?;
            let x = x?;
            Ok(acc.max(x))
        })?;

    println!("max {}", max);
    Ok(())
}

#[test]
fn test_examples() {
    assert_eq!(boarding_pass_to_coords("BFFFBBFRRR").unwrap(), (70, 7));
    assert_eq!(boarding_pass_to_seat("BFFFBBFRRR").unwrap(), 567);
    assert_eq!(boarding_pass_to_coords("FFFBBBFRRR").unwrap(), (14, 7));
    assert_eq!(boarding_pass_to_seat("FFFBBBFRRR").unwrap(), 119);
    assert_eq!(boarding_pass_to_coords("BBFFBBFRLL").unwrap(), (102, 4));
    assert_eq!(boarding_pass_to_seat("BBFFBBFRLL").unwrap(), 820);
}
