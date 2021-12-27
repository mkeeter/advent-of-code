use std::fs;
use std::io::{Read, Write};
use std::path::Path;

fn reg_index(s: &str) -> usize {
    match s {
        "x" => 0,
        "y" => 1,
        "z" => 2,
        "w" => 3,
        c => panic!("Invalid register '{}'", c),
    }
}

fn main() -> Result<(), std::io::Error> {
    let input_dir = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let input_path = Path::new(&input_dir).join("input");
    println!(
        "cargo:rerun-if-changed=build.rs
cargo:rerun-if-changed={}",
        input_path.as_os_str().to_str().unwrap()
    );

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("gen.rs");
    let mut f = std::fs::File::create(dest_path)?;
    let mut index = 0;

    let mut input = fs::File::open(input_path).unwrap();
    let mut lines = String::new();
    input.read_to_string(&mut lines)?;

    writeln!(
        f,
        "type Registers = [i64; 4];

#[allow(clippy::all)]
mod monad {{
    use super::Registers;"
    )?;

    let return_str = "        return [x, y, z, w];\n    }";
    let mut inputs = vec![];
    for line in lines.lines() {
        let mut words = line.split(' ');
        match words.next().unwrap() {
            "inp" => {
                if index > 0 {
                    writeln!(f, "{}", return_str)?;
                }
                writeln!(
                    f,
                    "    pub fn pass{}(regs: Registers, inp: u8) -> Registers {{
        let [mut x, mut y, mut z, mut w] = regs;
        let _ = (x, y, z, w);",
                    index
                )?;
                index += 1;
                let reg = words.next().unwrap();
                writeln!(f, "        {} = inp as i64; // {}", reg, line)?;
                inputs.push(reg_index(reg));
            }
            "add" => {
                let a = words.next().unwrap();
                let b = words.next().unwrap();
                writeln!(f, "        {0} = {0} + {1}; // {2}", a, b, line)?;
            }
            "mul" => {
                let a = words.next().unwrap();
                let b = words.next().unwrap();
                writeln!(f, "        {0} = {0} * {1}; // {2}", a, b, line)?;
            }
            "div" => {
                let a = words.next().unwrap();
                let b = words.next().unwrap();
                writeln!(f, "        {0} = {0} / {1}; // {2}", a, b, line)?;
            }
            "mod" => {
                let a = words.next().unwrap();
                let b = words.next().unwrap();
                writeln!(f, "        {0} = {0} % {1}; // {2}", a, b, line)?;
            }
            "eql" => {
                let a = words.next().unwrap();
                let b = words.next().unwrap();
                writeln!(f, "        {0} = ({0} == {1}).into(); // {2}", a, b, line)?;
            }
            _ => panic!("Invalid instruction {}", line),
        }
    }
    writeln!(
        f,
        "{}
}}

const PASSES: [fn(Registers, u8) -> Registers; {}] = [",
        return_str, index
    )?;
    for i in 0..index {
        writeln!(f, "    monad::pass{},", i)?;
    }
    writeln!(
        f,
        "];

const INPUTS: [usize; {}] = [",
        index
    )?;
    for i in &inputs {
        writeln!(f, "    {},", i)?;
    }
    writeln!(f, "];")?;

    Ok(())
}
