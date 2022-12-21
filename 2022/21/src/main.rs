use anyhow::{anyhow, bail, Context, Error, Result};
use std::{
    collections::{BTreeMap, VecDeque},
    io::{BufRead, Write},
};

enum Operation {
    Constant(i64),
    Add(Name, Name),
    Sub(Name, Name),
    Div(Name, Name),
    Mul(Name, Name),
}

impl Operation {
    fn eval(&self, values: &BTreeMap<Name, i64>) -> Option<i64> {
        let out = match self {
            Operation::Constant(i) => *i,
            Operation::Add(a, b) => values.get(a)? + values.get(b)?,
            Operation::Sub(a, b) => values.get(a)? - values.get(b)?,
            Operation::Div(a, b) => values.get(a)? / values.get(b)?,
            Operation::Mul(a, b) => values.get(a)? * values.get(b)?,
        };
        Some(out)
    }
}

impl std::fmt::Display for Operation {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        match self {
            Operation::Constant(i) => write!(f, "{}", i)?,
            Operation::Add(a, b) => write!(f, "{} + {}", a, b)?,
            Operation::Sub(a, b) => write!(f, "{} - {}", a, b)?,
            Operation::Mul(a, b) => write!(f, "{} * {}", a, b)?,
            Operation::Div(a, b) => write!(f, "{} / {}", a, b)?,
        }
        Ok(())
    }
}

impl std::str::FromStr for Operation {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if let Ok(c) = s.parse() {
            return Ok(Self::Constant(c));
        }
        let mut iter = s.split(' ');
        let a = iter
            .next()
            .ok_or_else(|| anyhow!("Missing first operand"))?;
        let op = iter.next().ok_or_else(|| anyhow!("Missing operator"))?;
        let b = iter
            .next()
            .ok_or_else(|| anyhow!("Missing second operand"))?;
        if let Some(extra) = iter.next() {
            bail!("Extra data at the end of line: '{extra}'");
        }

        let a: Name = a.parse()?;
        let b: Name = b.parse()?;

        let out = match op {
            "+" => Self::Add(a, b),
            "-" => Self::Sub(a, b),
            "*" => Self::Mul(a, b),
            "/" => Self::Div(a, b),
            _ => bail!("Unknown operator '{op}'"),
        };
        Ok(out)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Name([u8; 4]);

impl std::str::FromStr for Name {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 4 || !s.is_ascii() {
            bail!("Invalid name {s}");
        }
        let mut out = [0; 4];
        for (c, o) in s.chars().zip(&mut out) {
            *o = c as u8;
        }
        Ok(Self(out))
    }
}

impl std::fmt::Display for Name {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        for c in self.0 {
            write!(f, "{}", c as char)?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let monkeys = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut iter = line.split(": ");
            let name = iter.next().ok_or_else(|| anyhow!("Missing name"))?;
            let op = iter.next().ok_or_else(|| anyhow!("Missing op"))?;
            if let Some(extra) = iter.next() {
                bail!("Extra data: '{extra}'");
            }

            let name: Name = name.parse()?;
            let op: Operation = op.parse()?;
            Ok((name, op))
        })
        .collect::<Result<BTreeMap<Name, Operation>, _>>()?;

    let mut todo: VecDeque<Name> = monkeys.keys().cloned().collect();
    let mut values = BTreeMap::new();
    while let Some(t) = todo.pop_front() {
        if let Some(v) = monkeys[&t].eval(&values) {
            values.insert(t, v);
        } else {
            todo.push_back(t);
        }
    }
    println!("Part 1: {}", values[&"root".parse().unwrap()]);

    let mut f = std::fs::File::create("monkeys.z3")?;
    for m in monkeys.keys() {
        writeln!(&mut f, "(declare-const {m} Int)")?;
    }
    for (k, v) in monkeys.iter() {
        match &k.0 {
            b"humn" => (),
            b"root" => {
                let (a, b) = match v {
                    Operation::Constant(..) => {
                        bail!("'root' does not have two operands")
                    }
                    Operation::Add(a, b)
                    | Operation::Sub(a, b)
                    | Operation::Div(a, b)
                    | Operation::Mul(a, b) => (a, b),
                };
                writeln!(&mut f, "(assert (= {a} {b}))")?;
            }
            _ => writeln!(
                &mut f,
                "(assert (= {k} {}))",
                match v {
                    Operation::Constant(i) => format!("{}", i),
                    Operation::Add(a, b) => format!("(+ {a} {b})"),
                    Operation::Sub(a, b) => format!("(- {a} {b})"),
                    Operation::Div(a, b) => format!("(/ {a} {b})"),
                    Operation::Mul(a, b) => format!("(* {a} {b})"),
                }
            )?,
        }
    }
    writeln!(&mut f, "(check-sat)")?;
    writeln!(&mut f, "(eval humn)")?;
    let out = std::process::Command::new("z3")
        .arg("monkeys.z3")
        .output()
        .context("Failed to call 'z3'; is it installed?")?;
    let out = std::str::from_utf8(&out.stdout)?;
    let mut iter = out.split('\n');
    iter.next(); // Skip 'sat'
    let result = iter.next().ok_or_else(|| anyhow!("Missing output line"))?;
    println!("Part 2: {result}");

    Ok(())
}
