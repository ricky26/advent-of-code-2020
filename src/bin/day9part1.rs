use std::io::Read;
use clap::{Arg, App};
use anyhow::anyhow;

fn find_sum(src: &[i64], target: i64) -> bool {
    for (i, a) in src.iter().copied().enumerate() {
        if a >= target {
            continue
        }

        for b in src[i + 1..].iter().copied() {
            if a + b == target {
                return true
            }
        }
    }

    return false
}

fn find_contiguous_sum(src: &[i64], target: i64) -> Option<(usize, usize)> {
    for i in 0..src.len() {
        let mut sum = 0;
        let mut j = i;

        while let Some(x) = src.get(j) {
            j += 1;
            sum += x;

            if sum == target {
                return Some((i, j))
            } else if sum > target {
                break
            }
        }
    }

    None
}

fn main() -> anyhow::Result<()> {
    let args = App::new("day9part1")
        .arg(Arg::with_name("window")
            .short("w")
            .long("window")
            .takes_value(true)
            .required(true))
        .get_matches();
    let window_size = args.value_of("window").unwrap()
        .parse()?;

    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;

    let nums = contents.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| Ok(s.parse()?))
        .collect::<anyhow::Result<Vec<i64>>>()?;

    if nums.len() < window_size {
        Err(anyhow!("not enough input for window"))?;
    }

    let mut offset = window_size;
    let mut window = Vec::with_capacity(window_size);

    let mut nums_iter = nums.iter().copied();
    window.extend((&mut nums_iter).take(window_size));

    let mut to_test = None;

    for num in nums_iter {
        if !find_sum(&window, num) {
            println!("not found {} - {}", num, offset);
            to_test = Some(num);
        }

        let write_idx = offset % window.len();
        window[write_idx] = num;
        offset += 1;
    }

    if let Some(to_test) = to_test {
        if let Some((start, end)) = find_contiguous_sum(&nums, to_test) {
            let range = &nums[start..end];
            let (min, max) = range.iter().copied().fold((i64::max_value(), i64::min_value()), |(min, max), x| {
                (min.min(x), max.max(x))
            });
            println!("found {} -> {} = {} + {} = {}", start, end, min, max, min + max);
        }

    }

    Ok(())
}
