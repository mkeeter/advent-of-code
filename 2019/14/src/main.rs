use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufWriter, Write};
use std::process::Command;
use std::str::FromStr;

#[derive(Debug)]
struct Mol(i32, String);

fn parse(s: &str) -> (Vec<Mol>, Mol)
{
    let words = s.replace(|c| !char::is_alphanumeric(c), " ")
        .split(' ')
        .filter(|s| s.len() > 0)
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    let mut mols = words.chunks(2)
        .map(|c| Mol(i32::from_str(&c[0]).unwrap(), c[1].clone()))
        .collect::<Vec<Mol>>();

    let out = mols.remove(mols.len() - 1);
    (mols, out)
}

fn main() -> Result<(), Box<dyn Error>> {
    let eqns = std::io::stdin().lock().lines()
        .map(|line| parse(&line.unwrap()))
        .collect::<Vec<_>>();

    let mut sums : HashMap<&str, Vec<(usize, i32)>> = HashMap::new();
    for (i, (inputs, output)) in eqns.iter().enumerate() {
        for mol in inputs {
            sums.entry(&mol.1).or_insert(Vec::new()).push((i, -mol.0))
        }
        sums.entry(&output.1).or_insert(Vec::new()).push((i, output.0))
    }

    // Part 1
    let tmp = File::create("part1.lp")?;
    let mut out = BufWriter::new(&tmp);
    writeln!(out, "Minimize in: ore_in

Subject to")?;
    for (k, vs) in sums.iter() {
        write!(out, "  {}: ", k)?;
        for v in vs.iter() {
            if v.1 < 0 {
                write!(out, "- {} eqn{} ", v.1.abs(), v.0)?;
            } else {
                write!(out, "+ {} eqn{} ", v.1, v.0)?;
            }
        }
        if k == &"ORE" {
            write!(out, " + ore_in ")?
        }
        if k == &"FUEL" {
            writeln!(out, ">= 1")?;
        } else {
            writeln!(out, ">= 0")?;
        }
    }
    writeln!(out, "  FUEL >= 1

Integer")?;
    for (i, _) in eqns.iter().enumerate() {
        writeln!(out, "  eqn{}", i)?;
    }
    writeln!(out, "\nend")?;

    Command::new("glpsol")
            .arg("--lp")
            .arg("part1.lp")
            .arg("--output")
            .arg("sol1.out")
            .spawn()
            .expect("failed to execute process");

////////////////////////////////////////////////////////////////////////////////

    // Part 1
    let tmp = File::create("part2.lp")?;
    let mut out = BufWriter::new(&tmp);
    writeln!(out, "Maximize out: fuel_out

Subject to")?;
    for (k, vs) in sums.iter() {
        write!(out, "  {}: ", k)?;
        for v in vs.iter() {
            if v.1 < 0 {
                write!(out, "- {} eqn{} ", v.1.abs(), v.0)?;
            } else {
                write!(out, "+ {} eqn{} ", v.1, v.0)?;
            }
        }
        if k == &"ORE" {
            write!(out, "+ ore_in ")?
        }
        if k == &"FUEL" {
            writeln!(out, "- fuel_out >= 0")?;
        } else {
            writeln!(out, ">= 0")?;
        }
    }
    writeln!(out, "  ore_in = 1000000000000

Integer")?;
    for (i, _) in eqns.iter().enumerate() {
        writeln!(out, "  eqn{}", i)?;
    }
    writeln!(out, "  fuel_out
  ore_in
\nend")?;

    Command::new("glpsol")
            .arg("--lp")
            .arg("part2.lp")
            .arg("--output")
            .arg("sol2.out")
            .spawn()
            .expect("failed to execute process");
    Ok(())
}
