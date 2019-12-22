use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::io::{BufRead, Write};
use std::process::{Command, Stdio};
use std::str::FromStr;

fn parse(s: &str) -> Vec<(i32, String)>
{
    let words = s.replace(|c| !char::is_alphanumeric(c), " ")
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    let mut mols = words.chunks(2)
        .map(|c| (i32::from_str(&c[0]).unwrap(), c[1].clone()))
        .collect::<Vec<(i32, String)>>();

    // The output is the only positive value
    mols.iter_mut().rev().skip(1).for_each(|m| m.0 *= -1);
    mols
}

fn main() -> Result<(), Box<dyn Error>> {
    let eqns = std::io::stdin().lock().lines()
        .map(|line| parse(&line.unwrap()))
        .collect::<Vec<_>>();

    let mut sums : HashMap<&str, Vec<(usize, i32)>> = HashMap::new();
    for (i, inputs) in eqns.iter().enumerate() {
        for (count, name) in inputs {
            sums.entry(&name).or_insert_with(Vec::new).push((i, *count))
        }
    }

    // Build up a string containing all of our reaction constraints
    let mut constraints = String::new();
    writeln!(&mut constraints, "Subject to")?;
    for (k, vs) in sums.iter() {
        write!(&mut constraints, "  {}: ", k)?;
        for v in vs.iter() {
            if v.1 < 0 {
                write!(&mut constraints, "-")?;
            } else {
                write!(&mut constraints, "+")?;
            }
            write!(&mut constraints, " {} eqn{} ", v.1.abs(), v.0)?;
        }

        if k == &"ORE" {
            write!(&mut constraints, "+ ore_in ")?
        } else if k == &"FUEL" {
            write!(&mut constraints, "- fuel_out ")?
        }
        writeln!(&mut constraints, ">= 0")?;
    }

    // Build up a string which marks that all variables are integers
    let mut integers = String::new();
    writeln!(&mut integers, "Integer
  ore_in
  fuel_out")?;
    for i in 0..eqns.len() {
        writeln!(&mut integers, "  eqn{}", i)?;
    }

    let solver = || Command::new("glpsol")
            .arg("--lp")
            .arg("/dev/stdin")
            .arg("--output")
            .arg("/dev/stdout")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn solver");

    // Part 1
    let mut subprocess = solver();
    let pipe = subprocess.stdin.as_mut().expect("Failed to open stdin");
    writeln!(pipe, "Minimize\n  objective: ore_in\n")?;
    writeln!(pipe, "{}", constraints)?;
    writeln!(pipe, "  fuel_out >= 1\n")?;
    writeln!(pipe, "{}\n", integers)?;
    writeln!(pipe, "end")?;

    let output = subprocess.wait_with_output()?;
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if line.starts_with("Objective:") {
            println!("Part 1: {}", line.replace(|c| !char::is_numeric(c), ""));
        }
    }

////////////////////////////////////////////////////////////////////////////////

    // Part 2
    let mut subprocess = solver();
    let pipe = subprocess.stdin.as_mut().expect("Failed to open stdin");
    writeln!(pipe, "Maximize\n  objective: fuel_out\n")?;
    writeln!(pipe, "{}", constraints)?;
    writeln!(pipe, "  ore_in = 1000000000000\n")?;
    writeln!(pipe, "{}\n", integers)?;
    writeln!(pipe, "end")?;

    let output = subprocess.wait_with_output()?;
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if line.starts_with("Objective:") {
            println!("Part 2: {}", line.replace(|c| !char::is_numeric(c), ""));
        }
    }

    Ok(())
}
