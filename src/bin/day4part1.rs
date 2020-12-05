use std::io::Read;
use std::fmt::Write;
use aoc2020::passport;
use aoc2020::passport::Passport;

fn main() -> anyhow::Result<()> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;

    let mut num_valid = 0;
    let mut num_validated = 0;
    let mut debug = String::new();

    for block in passport::split_blocks(&contents) {
        match Passport::parse(block) {
            Ok((_, p)) => {
                num_valid += 1;
                println!("valid passport: {}", serde_json::to_string(&p)?);

                match p.validate() {
                    Ok(()) => {
                        num_validated += 1;
                        println!(" validated passport");
                    },
                    Err(e) => {
                        println!(" not validated passport: {}", e);
                        write!(&mut debug, "-- {}\n{}\n\n", serde_json::to_string(&p)?, e)?;
                    },
                }
            },
            Err(e) => {
                println!("invalid passport: {}", e);
            },
        }
    }

    println!("num valid {}", num_valid);
    println!("num validated {}", num_validated);

    println!("\n\n{}", debug);
    Ok(())
}
