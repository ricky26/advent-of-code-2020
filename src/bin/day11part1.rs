use std::io::Read;
use anyhow::anyhow;

const EMPTY: u8 = 'L' as u8;
const OCCUPIED: u8 = '#' as u8;

fn main() -> anyhow::Result<()> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;
    let mut lines = contents.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty());
    let first_line = lines.next().ok_or_else(|| anyhow!("at least one line required"))?;

    let width = first_line.len();
    let iwidth = width as isize;
    let mut seats = Vec::new();
    seats.extend_from_slice(first_line.as_bytes());
    seats.extend(lines.map(|line| {
        if line.len() == width {
            Ok(line.as_bytes())
        } else {
            Err(anyhow!("lines must all be the same length"))
        }
    }).collect::<Result<Vec<_>, _>>()?.iter().copied().flatten());
    let height = seats.len() / width;
    let iheight = height as isize;

    let mut seats_a = seats.clone();
    let mut seats_b = seats.clone();
    let mut swapped = false;

    let dirs = [
        (-1, -1),
        ( 0, -1),
        ( 1, -1),
        (-1,  0),
        ( 1,  0),
        (-1,  1),
        ( 0,  1),
        ( 1,  1),
    ];

    loop {
        let mut changed = false;
        let (src, dest) = if swapped {
            (&seats_b, &mut seats_a)
        } else {
            (&seats_a, &mut seats_b)
        };

        for idx in 0..dest.len() {
            let x = (idx % width) as isize;
            let y = (idx / width) as isize;
            let mut n_occupied = 0;

            for (dx, dy) in dirs.iter().copied() {
                let t_x = x + dx;
                let t_y = y + dy;
                if t_x < 0 || t_x >= iwidth || t_y < 0 || t_y >= iheight {
                    continue
                }
                let t_idx = (t_x + (t_y * iwidth)) as usize;
                match src[t_idx] {
                    OCCUPIED => n_occupied += 1,
                    _ => {},
                }
            }

            dest[idx] = match src[idx] {
                OCCUPIED => {
                    if n_occupied >= 4 {
                        changed = true;
                        EMPTY
                    } else {
                        OCCUPIED
                    }
                },
                EMPTY => {
                    if n_occupied == 0 {
                        changed = true;
                        OCCUPIED
                    } else {
                        EMPTY
                    }
                },
                x => x,
            }
        }

        println!("AXXXXX");
        for chunk in dest.chunks(width) {
            let line = std::str::from_utf8(chunk)?;
            println!("{}", line);
        }

        swapped = !swapped;
        if !changed {
            break
        }
    }

    let output = if swapped { &seats_b } else { &seats_a };
    let occupied = output.iter().copied().filter(|b| *b == OCCUPIED).count();
    println!("occupied {}", occupied);
    Ok(())
}
