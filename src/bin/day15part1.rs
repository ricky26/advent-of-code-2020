use std::io::Read;
use std::collections::BTreeMap;

fn main() -> anyhow::Result<()> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;
    let inputs = contents.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .flat_map(|s| s.split(','))
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()?;

    let mut last_said_map = BTreeMap::new();
    let mut turn = 0;
    let mut next = -1;

    for i in inputs.iter().copied() {
        println!("Said: {:010} {}", turn, i);
        next = last_said_map.get(&i).copied().map_or(0, |l| turn - l);
        last_said_map.insert(i, turn);
        turn += 1;
    }

    loop {
        let i = next;
        println!("Said: {:010} {}", turn, i);
        next = last_said_map.get(&i).copied().map_or(0, |l| turn - l);
        last_said_map.insert(i, turn);
        turn += 1;
        if turn == 30000000 {
            println!("turn 2020: {}", i);
            break
        }
    }

    Ok(())
}
