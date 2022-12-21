use anyhow::{anyhow, bail, Error, Result};
use std::{collections::BTreeMap, io::BufRead};
use z3::ast::Ast;

enum Opcode {
    Add,
    Sub,
    Div,
    Mul,
    Equal,
}
enum Operation {
    Unknown,
    Constant(i32),
    Op(Opcode, Name, Name),
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

        let op = match op {
            "+" => Opcode::Add,
            "-" => Opcode::Sub,
            "*" => Opcode::Mul,
            "/" => Opcode::Div,
            _ => bail!("Unknown operator '{op}'"),
        };
        Ok(Operation::Op(op, a, b))
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
            Operation::Op(op, a, b) => {
                let a = vars[a].clone();
                let b = vars[b].clone();
                match op {
                    Opcode::Add => sol.assert(&vars[k]._eq(&(a + b))),
                    Opcode::Sub => sol.assert(&vars[k]._eq(&(a - b))),
                    Opcode::Div => sol.assert(&vars[k]._eq(&(a / b))),
                    Opcode::Mul => sol.assert(&vars[k]._eq(&(a * b))),
                    Opcode::Equal => sol.assert(&a._eq(&b)),
                }
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

    // Modify the monkeys, switching humn to unknown and root to equality
    let mut monkeys = monkeys;
    *monkeys.get_mut(&Name(*b"humn")).unwrap() = Operation::Unknown;
    let root = monkeys.get_mut(&Name(*b"root")).unwrap();
    match root {
        Operation::Op(op, ..) => *op = Opcode::Equal,
        _ => bail!("Invalid operation for 'root'"),
    }
    println!("Part 2: {}", run(monkeys.iter(), Name(*b"humn"))?);

    Ok(())
}
