use std::io::Read;
use std::collections::BTreeSet;

fn main() -> anyhow::Result<()> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;

    let mut all = 0;
    let mut any = 0;

    for block in contents.split("\n\n") {
        let mut union = BTreeSet::new();
        let mut intersection: Option<BTreeSet<_>> = None;

        for line in block.lines().map(str::trim) {
            if line.is_empty() {
                continue
            }

            let mut entry = BTreeSet::new();
            entry.extend(line.chars());

            if let Some(b) = intersection {
                intersection = Some(&b & &entry);
            } else {
                intersection = Some(entry.clone());
            }
            union = &union | &entry;
        }

        all += intersection.map_or(0, |s| s.len());
        any += union.len();
    }

    println!("all {} any {}", all, any);
    Ok(())
}
