use rayon::prelude::*;
use std::io::{BufRead, Write};

type State = ([i64; 4], (usize, usize));

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
    let mut state: Vec<State> = vec![([0; 4], (0, 0))];

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
                let r = reg_index(words.next().unwrap());
                state.par_sort_unstable_by_key(|k| {
                    let mut out = k.0;
                    out[r] = 0;
                    out
                });
                let mut next: Vec<State> = Vec::with_capacity(state.len() * 9);
                for i in 1..=9 {
                    for (mut regs, (min, max)) in state.iter() {
                        regs[r] = i;
                        let min = min * 10 + i as usize;
                        let max = max * 10 + i as usize;
                        if next.last().map(|p| p.0 == regs).unwrap_or(false) {
                            let (prev_min, prev_max) = &mut next.last_mut().unwrap().1;
                            *prev_min = (*prev_min).min(min);
                            *prev_max = (*prev_max).max(max);
                        } else {
                            next.push((regs, (min, max)));
                        }
                    }
                }
                state = next;
            }
            "add" => {
                let ra = reg_index(words.next().unwrap());
                let rb = words.next().unwrap();
                state.par_iter_mut().for_each(|(regs, _)| {
                    let a = regs[ra];
                    let b = reg_value(rb, regs);
                    regs[ra] = a + b;
                });
            }
            "mul" => {
                let ra = reg_index(words.next().unwrap());
                let rb = words.next().unwrap();
                state.par_iter_mut().for_each(|(regs, _)| {
                    let a = regs[ra];
                    let b = reg_value(rb, regs);
                    regs[ra] = a * b;
                });
            }
            "div" => {
                let ra = reg_index(words.next().unwrap());
                let rb = words.next().unwrap();
                state.par_iter_mut().for_each(|(regs, _)| {
                    let a = regs[ra];
                    let b = reg_value(rb, regs);
                    regs[ra] = a / b;
                });
            }
            "mod" => {
                let ra = reg_index(words.next().unwrap());
                let rb = words.next().unwrap();
                state.par_iter_mut().for_each(|(regs, _)| {
                    let a = regs[ra];
                    let b = reg_value(rb, regs);
                    regs[ra] = a % b;
                });
            }
            "eql" => {
                let ra = reg_index(words.next().unwrap());
                let rb = words.next().unwrap();
                state.par_iter_mut().for_each(|(regs, _)| {
                    let a = regs[ra];
                    let b = reg_value(rb, regs);
                    regs[ra] = (a == b) as i64;
                });
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
