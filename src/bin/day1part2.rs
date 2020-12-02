use std::io::{stdin, Read};
use anyhow::Result;
use std::str::FromStr;

fn main() -> Result<()> {
    let mut input = stdin();
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;

    let entries = contents
        .split('\n')
        .map(str::trim)
        .filter(|s| s.len() > 0)
        .map(i32::from_str)
        .collect::<std::result::Result<Vec<_>, _>>()?;

    for (i, a) in entries.iter().copied().enumerate() {
        for (j, b) in entries[i+1..].iter().copied().enumerate() {
            for c in entries[j+1..].iter().copied() {
                let total = a + b + c;
                if total == 2020 {
                    print!("a {} b {} c {} p {}", a, b, c, a * b * c);
                }
            }
        }
    }

    Ok(())
}
