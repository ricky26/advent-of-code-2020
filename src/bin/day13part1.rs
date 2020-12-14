use std::io::Read;
use anyhow::anyhow;

fn pos_div_mod(x: i64, m: i64) -> (i64, i64) {
    let o = x / m;
    let o = if o * m < x {
        o + 1
    } else {
        o
    };
    let t = o * m;
    (o, t - x)
}

fn main() -> anyhow::Result<()> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;
    let mut lines = contents.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty());

    let departure = lines.next()
        .ok_or_else(|| anyhow!("missing departure time"))?
        .parse::<i64>()?;
    let busses = lines.next()
        .ok_or_else(|| anyhow!("missing timetable"))?
        .split(",")
        .map(|s| -> Result<_, anyhow::Error> {
            if s == "x" {
                Ok(None)
            } else {
                Ok(Some(s.parse::<i64>()?))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut best = None;
    for bus in busses.iter().copied().filter_map(|x| x) {
        let (_, w) = pos_div_mod(departure, bus);
        if let Some((wait, _)) = best {
            if wait > w {
                best = Some((w, bus));
            }
        } else {
            best = Some((w, bus));
        }
    }

    if let Some((w, bus)) = best {
        println!("best {} wait {} = {}", bus, w, bus * w);
    }

    let iter_bus_indexes = busses.iter()
        .copied()
        .enumerate()
        .filter_map(|(x, y)| y.map(|y| (x, y)));

    let (magic, _) = iter_bus_indexes.clone()
        .fold((0, 1), |(offset, m), (idx, x)| {
            let mut first = None;
            for n in 0.. {
                println!("m {} x {} idx {} n {}", m, x, idx, n);
                let base = offset + m * n;
                let test = base + (idx as i64);
                println!("test {} m {}", test, test % x);
                if test % x == 0 {
                    if let Some(first) = first {
                        println!("next {} {}", first, base - first);

                        for (j, bus) in iter_bus_indexes.clone() {
                            if j > idx {
                                break
                            }
                            assert_eq!((first + (j as i64)) % bus, 0, "bus {} idx {} bad mod", bus, j);
                        }

                        return (first, base - first)
                    }

                    first = Some(base);
                }
            }

            unreachable!()
        });
    println!("magic {}", magic);

    for (idx, bus) in iter_bus_indexes.clone() {
        println!("bus {} idx {} offset {}", bus, idx, (magic + (idx as i64)) % bus);
    }

    Ok(())
}
