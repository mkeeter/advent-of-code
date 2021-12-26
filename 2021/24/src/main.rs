use rayon::prelude::*;
use std::io::{BufRead, Write};

type State = ([i64; 4], (usize, usize));

struct StateVec(Vec<State>);
impl StateVec {
    fn apply<'a, F, I>(&mut self, mut words: I, f: F)
    where
        F: Fn(i64, i64) -> i64 + std::marker::Send + std::marker::Sync,
        I: Iterator<Item = &'a str>,
    {
        let ra = reg_index(words.next().unwrap());
        let rb = words.next().unwrap();
        self.0.par_iter_mut().for_each(|(regs, _)| {
            let a = regs[ra];
            let b = reg_value(rb, regs);
            regs[ra] = f(a, b)
        });
    }
}

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
    let mut state = StateVec(vec![([0; 4], (0, 0))]);

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
            state.0.len()
        );
        std::io::stdout().flush().unwrap();

        match words.next().unwrap() {
            "inp" => {
                let r = reg_index(words.next().unwrap());

                // Clear the register that's about to be written
                state.0.par_iter_mut().for_each(|k| k.0[r] = 0);

                // Sort by register state, then do single-pass compaction
                state.0.par_sort_unstable_by_key(|k| k.0);
                let mut i = 0;
                let mut j = 1;
                while j < state.0.len() {
                    if state.0[i].0 == state.0[j].0 {
                        let (imin, imax) = state.0[i].1;
                        let (jmin, jmax) = state.0[j].1;
                        state.0[i].1 = (imin.min(jmin), imax.max(jmax));
                    } else {
                        i += 1;
                        state.0[i] = state.0[j];
                    }
                    j += 1;
                }
                assert!(i < state.0.len());
                state.0.resize(i + 1, ([0; 4], (0, 0)));

                state.0 = (1..=9)
                    .into_par_iter()
                    .flat_map(|i| {
                        state.0.par_iter().map(move |(mut regs, (min, max))| {
                            regs[r] = i;
                            (regs, (min * 10 + i as usize, max * 10 + i as usize))
                        })
                    })
                    .collect();
            }
            "add" => state.apply(words, |a, b| a + b),
            "mul" => state.apply(words, |a, b| a * b),
            "div" => state.apply(words, |a, b| a / b),
            "mod" => state.apply(words, |a, b| a % b),
            "eql" => state.apply(words, |a, b| (a == b) as i64),
            _ => panic!("Invalid instruction {}", line),
        }
    }
    let (min, max) = state
        .0
        .par_iter()
        .filter(|(k, _)| k[2] == 0)
        .map(|(_, v)| *v)
        .reduce(|| (usize::MAX, 0), |a, b| (a.0.min(b.0), a.1.max(b.1)));
    println!("Part 1: {}                         ", max);
    println!("Part 2: {}                         ", min);
}
