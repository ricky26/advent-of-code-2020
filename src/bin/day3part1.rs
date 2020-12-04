use std::str::FromStr;
use std::io::Read;
use aoc2020::toboggan;

fn main() -> anyhow::Result<()> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;
    let slope = toboggan::Slope::from_str(&contents)?;

    let angles = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let mut product = 1;

    for (dx, dy) in angles.iter() {
        let mut x = 0;
        let mut y = 0;
        let mut trees = 0;

        while y < slope.height() {
            if slope.get(x, y) == Some('#' as u8) {
                trees += 1;
            }

            x += dx;
            y += dy;
        }

        println!("trees ({}, {}) {}", dx, dy, trees);

        product = product * (trees as i64);
    }

    println!("product {}", product);

    Ok(())
}
