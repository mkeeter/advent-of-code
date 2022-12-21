use anyhow::{anyhow, bail, Error, Result};
use std::{collections::BTreeMap, io::BufRead};
use z3::ast::Ast;

enum Operation {
    Unknown,
    Constant(i32),
    Add(Name, Name),
    Sub(Name, Name),
    Div(Name, Name),
    Mul(Name, Name),
    Equal(Name, Name),
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

fn run<'a, I: Iterator<Item = (&'a Name, &'a Operation)> + Clone>(
    monkeys: I,
    target: Name,
) -> Result<i64> {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let vars = monkeys
        .clone()
        .map(|(name, _)| {
            (*name, z3::ast::Real::new_const(&ctx, name.to_string()))
        })
        .collect::<BTreeMap<Name, _>>();

    let sol = z3::Solver::new(&ctx);
    for (k, v) in monkeys {
        match v {
            Operation::Unknown => continue,
            Operation::Constant(i) => {
                sol.assert(&vars[k]._eq(&z3::ast::Real::from_real(&ctx, *i, 1)))
            }
            Operation::Add(a, b) => {
                let v = vars[a].clone() + vars[b].clone();
                sol.assert(&vars[k]._eq(&v))
            }
            Operation::Sub(a, b) => {
                let v = vars[a].clone() - vars[b].clone();
                sol.assert(&vars[k]._eq(&v))
            }
            Operation::Div(a, b) => {
                let v = vars[a].clone() / vars[b].clone();
                sol.assert(&vars[k]._eq(&v));
            }
            Operation::Mul(a, b) => {
                let v = vars[a].clone() * vars[b].clone();
                sol.assert(&vars[k]._eq(&v));
            }
            Operation::Equal(a, b) => {
                sol.assert(&vars[a]._eq(&vars[b]));
            }
        }
    }
    if sol.check() != z3::SatResult::Sat {
        bail!("Could not find solution");
    }
    let model = sol.get_model().unwrap();
    let i = model
        .eval(&vars[&target], false)
        .unwrap()
        .as_real()
        .unwrap();
    assert_eq!(i.1, 1);
    Ok(i.0)
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

    println!("Part 1: {}", run(monkeys.iter(), Name(*b"root"))?);

    let mut monkeys = monkeys;
    monkeys.insert(Name(*b"humn"), Operation::Unknown);
    let root = monkeys.get_mut(&Name(*b"root")).unwrap();
    let (a, b) = match root {
        Operation::Unknown | Operation::Constant(..) => {
            bail!("'root' doesn't have two arguments")
        }
        Operation::Add(a, b)
        | Operation::Sub(a, b)
        | Operation::Div(a, b)
        | Operation::Equal(a, b)
        | Operation::Mul(a, b) => (a, b),
    };
    *root = Operation::Equal(*a, *b);
    println!("Part 2: {}", run(monkeys.iter(), Name(*b"humn"))?);

    Ok(())
}
