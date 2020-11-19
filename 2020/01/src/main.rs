use std::io::Read;
use anyhow::Result;

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    println!("Got input: {}", input);

    Ok(())
}
