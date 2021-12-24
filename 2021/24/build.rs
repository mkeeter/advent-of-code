use std::io::{Read, Write};
use std::fs;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let input_dir = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let input_path = Path::new(&input_dir).join("input");
    println!("cargo:rerun-if-changed=build.rs");
    println!(
        "cargo:rerun-if-changed={}",
        input_path.as_os_str().to_str().unwrap()
    );

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("prog.rs");
    let mut f = std::fs::File::create(dest_path)?;
    writeln!(f, 
        "pub fn monad(mut model: i64) -> bool {{
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    let mut w = 0;
    let _ = (x, y, z, w);

    // Swap digit order to make input easier
    let mut input = 0;
    for _ in 0..14 {{
        if model % 10 == 0 {{
            return false;
        }}
        input = input * 10 + (model % 10);
        model /= 10;
    }}
    "
    )?;

    let mut input = fs::File::open(input_path).unwrap();
    let mut lines = String::new();
    input.read_to_string(&mut lines)?;
    for line in lines.lines() {
        let mut words = line.split(' ');
        match words.next().unwrap() {
            "inp" => {
                let reg = words.next().unwrap();
                writeln!(f, "    {} = input % 10; // {}
    input /= 10;", reg, line)?;
            },
            "add" => {
                let a = words.next().unwrap();
                let b = words.next().unwrap();
                writeln!(f, "    {0} = {0} + {1}; // {2}", a, b, line)?;
            },
            "mul" => {
                let a = words.next().unwrap();
                let b = words.next().unwrap();
                writeln!(f, "    {0} = {0} * {1}; // {2}", a, b, line)?;
            },
            "div" => {
                let a = words.next().unwrap();
                let b = words.next().unwrap();
                writeln!(f, "    {0} = {0} / {1}; // {2}", a, b, line)?;
            },
            "mod" => {
                let a = words.next().unwrap();
                let b = words.next().unwrap();
                writeln!(f, "    {0} = {0} % {1}; // {2}", a, b, line)?;
            },
            "eql" => {
                let a = words.next().unwrap();
                let b = words.next().unwrap();
                writeln!(f, "    {0} = ({0} == {1}).into(); // {2}", a, b, line)?;
            },
            _ => panic!("Invalid instruction {}", line),
        }
    }
    writeln!(f, "    let _ = input;
    z == 0
}}")?;

    Ok(())
}
