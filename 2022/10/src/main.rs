use anyhow::{anyhow, Error, Result};
use std::io::BufRead;

#[derive(Copy, Clone, Debug)]
enum Op {
    Noop,
    AddX(i64),
}

impl std::str::FromStr for Op {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut iter = s.split(' ');
        let a = iter
            .next()
            .ok_or_else(|| anyhow!("No instruction in {s}"))?;
        match a {
            "noop" => Ok(Self::Noop),
            "addx" => {
                let x = iter
                    .next()
                    .ok_or_else(|| anyhow!("No argument in {s}"))?
                    .parse()?;
                Ok(Self::AddX(x))
            }
            _ => Err(anyhow!("No such opcode {a}")),
        }
    }
}

struct Vm<'a> {
    tape: &'a [Op],
    ip: usize,
    busy: bool,
    x: i64,
}

impl<'a> Vm<'a> {
    fn new(tape: &'a [Op]) -> Self {
        Self {
            tape,
            ip: 0,
            busy: false,
            x: 1,
        }
    }
    /// Executes a single cycle
    fn step(&mut self) {
        match self.tape[self.ip] {
            Op::AddX(dx) if self.busy => {
                self.busy = false;
                self.x += dx;
            }
            Op::AddX(..) => self.busy = true,
            Op::Noop => (),
        }
        if !self.busy {
            self.ip += 1;
        }
    }
}

fn main() -> Result<()> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse())
        .collect::<Result<Vec<Op>, _>>()?;

    let mut vm = Vm::new(&lines);
    let mut sum = 0;
    for i in 1..=220 {
        if (i - 20) % 40 == 0 {
            sum += i * vm.x;
        }
        vm.step();
    }
    println!("Part 1: {sum}");

    let mut vm = Vm::new(&lines);
    println!("Part 2:");
    for _row in 0..6 {
        for col in 0..40 {
            if (vm.x - col).abs() <= 1 {
                print!("█")
            } else {
                print!(" ")
            }
            vm.step()
        }
        println!()
    }
    Ok(())
}
