use anyhow::{anyhow, bail, Result};
use std::collections::BTreeMap;
use std::io::BufRead;

#[derive(Debug)]
enum Op {
    Mul,
    Add,
}

#[derive(Debug)]
enum Value {
    Old,
    Imm(u64),
}

#[derive(Debug)]
struct Operation {
    op: Op,
    value: Value,
}

#[derive(Debug)]
struct Monkey {
    inspection_count: u64,
    items: Vec<u64>,
    operation: Operation,
    test_divisible: u64,
    true_monkey: usize,
    false_monkey: usize,
}

impl Monkey {
    fn run(&mut self) -> BTreeMap<usize, Vec<u64>> {
        let mut out: BTreeMap<usize, Vec<u64>> = BTreeMap::new();
        for i in std::mem::take(&mut self.items) {
            self.inspection_count += 1;
            let v = match self.operation.value {
                Value::Old => i,
                Value::Imm(j) => j,
            };
            let worry = match self.operation.op {
                Op::Add => i + v,
                Op::Mul => i * v,
            } / 3;
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

fn main() -> Result<()> {
    let mut iter = std::io::stdin().lock().lines().map(Result::unwrap);

    let mut monkeys = vec![];
    while let Some(monkey) = iter.next() {
        println!("monkey: {monkey}");
        let items = iter.next().ok_or_else(|| anyhow!("Missing items"))?;
        let items = items
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

        let test = iter.next().ok_or_else(|| anyhow!("Missing Test line"))?;
        let test_divisible: u64 = test
            .strip_prefix("  Test: divisible by ")
            .ok_or_else(|| anyhow!("Could not get Test prefix"))?
            .parse()?;

        let true_monkey =
            iter.next().ok_or_else(|| anyhow!("Missing Test line"))?;
        let true_monkey: usize = true_monkey
            .strip_prefix("    If true: throw to monkey ")
            .ok_or_else(|| anyhow!("Could not get Test prefix"))?
            .parse()?;

        let false_monkey =
            iter.next().ok_or_else(|| anyhow!("Missing Test line"))?;
        let false_monkey: usize = false_monkey
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

    for round in 1..=20 {
        for i in 0..monkeys.len() {
            let out = monkeys[i].run();
            for (v, items) in out.into_iter() {
                monkeys[v].items.extend(items);
            }
        }
        println!("{round}");
        for (i, m) in monkeys.iter().enumerate() {
            println!("  {i}: {:?}", m.items);
        }
    }

    let mut monkey_business = monkeys
        .iter()
        .map(|m| m.inspection_count)
        .collect::<Vec<_>>();
    monkey_business.sort();
    monkey_business.reverse();
    println!("Part 1: {}", monkey_business[0] * monkey_business[1]);

    Ok(())
}
