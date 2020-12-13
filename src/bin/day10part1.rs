use std::io::Read;
use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;

    let mut nums = contents.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()?;
    nums.sort();

    let mut counters = vec![0i64; nums.len()];
    let mut ones = 0;
    let mut threes = 1;
    let mut last_num = 0;
    let mut num_iter = nums.iter().copied().enumerate();

    while let Some((idx, num)) = num_iter.next() {
        let delta = num - last_num;
        if delta < 1 || delta > 3 {
            Err(anyhow!("invalid delta {} ({} -> {})", delta, last_num, num))?;
        }
        last_num = num;

        if delta == 1 {
            ones += 1;
        } else if delta == 3 {
            threes += 1;
        }

        if num <= 3 {
            counters[idx] += 1;
        }

        let count = counters[idx];
        for (idx_b, _num_b) in num_iter.clone().take_while(|n| n.1 <= num + 3) {
            counters[idx_b] += count;
        }
    }

    println!("ones {} threes {} product {} perms {}", ones, threes, ones * threes, counters[counters.len() - 1]);
    Ok(())
}
