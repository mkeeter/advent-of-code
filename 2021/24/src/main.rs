use std::io::{BufRead, Write};
use rayon::prelude::*;

fn reg_index(s: &str) -> usize {
    match s {
        "x" => 0,
        "y" => 1,
        "z" => 2,
        "w" => 3,
        c => panic!("Invalid register '{}'", c),
    }
}

fn reg_value(s: &str, regs: &[i64]) -> i64 {
    match s {
        "x" | "y" | "z" | "w" => regs[reg_index(s)],
        i => i.parse().unwrap(),
    }
}

fn main() {
    let mut state: Vec<([i64; 4], (usize, usize))> = vec![([0; 4], (0, 0))];

    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    for (i, line) in lines.iter().enumerate() {
        let mut words = line.split(' ');
        print!(
            "  [{}/{}] {} ({})                \r",
            i + 1,
            lines.len(),
            line,
            state.len()
        );
        std::io::stdout().flush().unwrap();

        match words.next().unwrap() {
            "inp" => {
                let mut next = Vec::with_capacity(state.len() * 9);
                let r = reg_index(words.next().unwrap());
                for (regs, (min, max)) in state.iter() {
                    for i in 1..=9 {
                        let mut regs = *regs;
                        regs[r] = i;
                        next.push((regs, (min * 10 + i as usize, max * 10 + i as usize)));
                    }
                }
                state = next;
                state.par_sort_unstable_by_key(|k| k.0);
                let mut i = 0;
                let mut j = 1;
                while j < state.len() {
                    if state[i].0 == state[j].0 {
                        let (imin, imax) = state[i].1;
                        let (jmin, jmax) = state[j].1;
                        state[i].1 = (imin.min(jmin), imax.max(jmax));
                    } else {
                        i += 1;
                        state[i] = state[j];
                    }
                    j += 1;
                }
                state.resize(i + 1, ([0; 4], (0, 0)));
            }
            "add" => {
                let ra = reg_index(words.next().unwrap());
                let rb = words.next().unwrap();
                for (regs, _) in state.iter_mut() {
                    let a = regs[ra];
                    let b = reg_value(rb, regs);
                    regs[ra] = a + b;
                }
            }
            "mul" => {
                let ra = reg_index(words.next().unwrap());
                let rb = words.next().unwrap();
                for (regs, _) in state.iter_mut() {
                    let a = regs[ra];
                    let b = reg_value(rb, regs);
                    regs[ra] = a * b;
                }
            }
            "div" => {
                let ra = reg_index(words.next().unwrap());
                let rb = words.next().unwrap();
                for (regs, _) in state.iter_mut() {
                    let a = regs[ra];
                    let b = reg_value(rb, regs);
                    regs[ra] = a / b;
                }
            }
            "mod" => {
                let ra = reg_index(words.next().unwrap());
                let rb = words.next().unwrap();
                for (regs, _) in state.iter_mut() {
                    let a = regs[ra];
                    let b = reg_value(rb, regs);
                    regs[ra] = a % b;
                }
            }
            "eql" => {
                let ra = reg_index(words.next().unwrap());
                let rb = words.next().unwrap();
                for (regs, _) in state.iter_mut() {
                    let a = regs[ra];
                    let b = reg_value(rb, regs);
                    regs[ra] = (a == b) as i64;
                }
            }
            _ => panic!("Invalid instruction {}", line),
        }
    }
    println!(
        "Part 1: {}                         ",
        state
            .iter()
            .filter(|(k, _)| k[2] == 0)
            .map(|(_, v)| v.1)
            .max()
            .unwrap()
    );
    println!(
        "Part 2: {}                         ",
        state
            .iter()
            .filter(|(k, _)| k[2] == 0)
            .map(|(_, v)| v.0)
            .min()
            .unwrap()
    );
}
