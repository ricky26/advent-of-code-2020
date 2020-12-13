use std::io::Read;
use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;
    let instructions = contents.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| -> anyhow::Result<(char, f32)> {
            let action = s[..1].chars().next().ok_or_else(|| anyhow!("bad instruction"))?;
            let num = s[1..].parse()?;
            Ok((action, num))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut x = 0f32;
    let mut y = 0f32;
    let mut a = 0f32;

    for (action, arg) in instructions {
        match action {
            'F' => {
                x += a.to_radians().cos() * arg;
                y -= a.to_radians().sin() * arg;
            },
            'R' => a += arg,
            'L' => a -= arg,
            'N' => y += arg,
            'S' => y -= arg,
            'E' => x += arg,
            'W' => x -= arg,
            _ => unreachable!(),
        }
    }

    println!("x {} y {} a {} d {}", x, y, a, x.abs() + y.abs());
    Ok(())
}
