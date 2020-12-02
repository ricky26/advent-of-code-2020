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

    let mut done = false;
    for (i, a) in entries.iter().copied().enumerate() {
        for b in entries[i+1..].iter().copied() {
            let total = a + b;
            if total == 2020 {
                print!("a {} b {} p {}", a, b, a * b);
                if done {
                    panic!("oh no")
                }

                done = true;
            }
        }
    }

    Ok(())
}
