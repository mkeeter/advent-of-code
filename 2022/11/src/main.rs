use anyhow::{anyhow, bail, Result};
use std::collections::BTreeMap;
use std::io::BufRead;

#[derive(Copy, Clone, Debug)]
enum Op {
    Mul,
    Add,
}

#[derive(Copy, Clone, Debug)]
enum Value {
    Old,
    Imm(u64),
}

#[derive(Copy, Clone, Debug)]
struct Operation {
    op: Op,
    value: Value,
}

#[derive(Copy, Clone, Debug)]
enum Worry {
    Divide(u64),
    Modulo(u64),
}

impl Operation {
    fn apply(&self, a: u64) -> u64 {
        let b = match self.value {
            Value::Old => a,
            Value::Imm(i) => i,
        };
        match self.op {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    inspection_count: u64,
    items: Vec<u64>,
    operation: Operation,
    test_divisible: u64,
    true_monkey: usize,
    false_monkey: usize,
}

impl Monkey {
    fn run(&mut self, worry_update: Worry) -> BTreeMap<usize, Vec<u64>> {
        let mut out: BTreeMap<usize, Vec<u64>> = BTreeMap::new();
        for worry in std::mem::take(&mut self.items) {
            self.inspection_count += 1;
            let mut worry = self.operation.apply(worry);
            match worry_update {
                Worry::Divide(d) => worry /= d,
                Worry::Modulo(d) => worry %= d,
            };
            let out_monkey = if worry % self.test_divisible == 0 {
                self.true_monkey
            } else {
                self.false_monkey
            };
            out.entry(out_monkey).or_default().push(worry);
        }
        out
    }
}

fn run(monkeys: &[Monkey], rounds: usize, worry_update: Worry) -> u64 {
    let mut monkeys = monkeys.to_vec();
    for _r in 0..rounds {
        for i in 0..monkeys.len() {
            let out = monkeys[i].run(worry_update);
            for (v, items) in out.into_iter() {
                monkeys[v].items.extend(items);
            }
        }
    }

    let mut monkey_business = monkeys
        .iter()
        .map(|m| m.inspection_count)
        .collect::<Vec<_>>();
    monkey_business.sort();
    monkey_business.reverse();
    monkey_business[0] * monkey_business[1]
}

fn main() -> Result<()> {
    let mut iter = std::io::stdin().lock().lines().map(Result::unwrap);

    let mut monkeys = vec![];
    while let Some(monkey) = iter.next() {
        if monkey != format!("Monkey {}:", monkeys.len()) {
            bail!("Invalid Monkey header: {monkey}");
        }
        let items = iter
            .next()
            .ok_or_else(|| anyhow!("Missing items"))?
            .strip_prefix("  Starting items: ")
            .ok_or_else(|| anyhow!("Missing item prefix"))?
            .split(", ")
            .map(|s| s.parse())
            .collect::<Result<Vec<u64>, _>>()?;

        let op = iter.next().ok_or_else(|| anyhow!("Missing operation"))?;
        let mut op_iter = op
            .strip_prefix("  Operation: new = old ")
            .ok_or_else(|| anyhow!("Could not get Operation prefix"))?
            .split(' ');
        let op = match op_iter.next().unwrap() {
            "*" => Op::Mul,
            "+" => Op::Add,
            op => bail!("Unknown operation {op}"),
        };
        let value = match op_iter.next().unwrap() {
            "old" => Value::Old,
            i => Value::Imm(i.parse()?),
        };

        let test_divisible: u64 = iter
            .next()
            .ok_or_else(|| anyhow!("Missing Test line"))?
            .strip_prefix("  Test: divisible by ")
            .ok_or_else(|| anyhow!("Could not get Test prefix"))?
            .parse()?;

        let true_monkey: usize = iter
            .next()
            .ok_or_else(|| anyhow!("Missing Test line"))?
            .strip_prefix("    If true: throw to monkey ")
            .ok_or_else(|| anyhow!("Could not get Test prefix"))?
            .parse()?;

        let false_monkey: usize = iter
            .next()
            .ok_or_else(|| anyhow!("Missing Test line"))?
            .strip_prefix("    If false: throw to monkey ")
            .ok_or_else(|| anyhow!("Could not get Test prefix"))?
            .parse()?;

        monkeys.push(Monkey {
            inspection_count: 0,
            items,
            operation: Operation { op, value },
            test_divisible,
            true_monkey,
            false_monkey,
        });

        iter.next(); // skip newline
    }

    println!("Part 1: {}", run(&monkeys, 20, Worry::Divide(3)));

    let prod = monkeys.iter().map(|m| m.test_divisible).product();
    println!("Part 2: {}", run(&monkeys, 10000, Worry::Modulo(prod)));

    Ok(())
}
