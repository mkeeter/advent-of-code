use anyhow::Result;
use std::io::BufRead;

fn main() -> Result<()> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<String>, _>>()?;

    println!("Hello, world!");
    Ok(())
}
