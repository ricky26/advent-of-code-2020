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
    let mut wx = 10f32;
    let mut wy = 1f32;

    for (action, arg) in instructions {
        match action {
            'F' => {
                x += wx * arg;
                y += wy * arg;
            },
            'R' | 'L' => {
                let arg = if action == 'L' {
                    -arg
                } else {
                    arg
                };
                let d = (wx * wx + wy * wy).sqrt();
                let a = (-wy).atan2(wx) + arg.to_radians();
                wx = a.cos() * d;
                wy = -a.sin() * d;
            },
            'N' => wy += arg,
            'S' => wy -= arg,
            'E' => wx += arg,
            'W' => wx -= arg,
            _ => unreachable!(),
        }

        println!("x {} y {} wx {} wy {}", x, y, wx, wy);
    }

    println!("x {} y {} d {}", x, y, x.abs() + y.abs());
    Ok(())
}
