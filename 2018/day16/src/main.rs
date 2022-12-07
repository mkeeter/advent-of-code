use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq)]
struct Registers([usize; 4]);

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Op {
    addr,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
}

use crate::Op::*;

impl Op {
    fn eval(&self, s: &Registers, a: usize, b: usize, c: usize) -> Registers {
        let mut out = s.0.clone();
        out[c] = match self {
            addr => out[a] + out[b],
            addi => out[a] + b,
            mulr => out[a] * out[b],
            muli => out[a] * b,
            banr => out[a] & out[b],
            bani => out[a] & b,
            borr => out[a] | out[b],
            bori => out[a] | b,
            setr => out[a],
            seti => a,
            gtir => (a > out[b]) as usize,
            gtri => (out[a] > b) as usize,
            gtrr => (out[a] > out[b]) as usize,
            eqir => (a == out[b]) as usize,
            eqri => (out[a] == b) as usize,
            eqrr => (out[a] == out[b]) as usize,
        };
        return Registers(out);
    }
}

#[derive(Debug)]
struct MachineCode([usize; 4]);

impl MachineCode {
    fn eval(&self, s: &Registers, map: &HashMap<usize, Op>) -> Registers {
        map.get(&self.0[0])
            .expect("Could not find opcode")
            .eval(s, self.0[1], self.0[2], self.0[3])
    }
}

fn main() {
    let input = include_str!("../input");
    let re = Regex::new(r"(\d+)").unwrap();

    let lines = input
        .lines()
        .map(|line| {
            re.captures_iter(line)
                .filter_map(|i| str::parse::<usize>(&i[1]).ok())
                .collect::<Vec<usize>>()
        })
        .map(|v| {
            if v.len() == 4 {
                Some([v[0], v[1], v[2], v[3]])
            } else {
                None
            }
        })
        .collect::<Vec<Option<[usize; 4]>>>();

    let mut table: HashMap<usize, HashSet<Op>> = HashMap::new();
    let mut geq3 = 0;
    for chunk in lines.split(|p| p.is_none()) {
        if chunk.len() == 3 {
            let initial_state = Registers(chunk[0].unwrap());
            let machine_code = MachineCode(chunk[1].unwrap());
            let final_state = Registers(chunk[2].unwrap());

            let op_num = machine_code.0[0];
            let mut matches = HashSet::new();
            for op in &[
                addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir,
                eqri, eqrr,
            ] {
                let mut map = HashMap::new();
                map.insert(op_num, *op);
                let out = machine_code.eval(&initial_state, &map);

                if out == final_state {
                    matches.insert(*op);
                }
            }
            geq3 += (matches.len() >= 3) as usize;

            table
                .entry(op_num)
                .and_modify(|t| *t = t.intersection(&matches).cloned().collect())
                .or_insert(matches);

        // The last chunk is longer, and contains a program to execute
        } else if chunk.len() != 0 {
            println!("Part 1: {}", geq3);

            // Build the canonical table of numbers to opcodes
            let mut canonical = HashMap::new();
            while let Some(i) = table.iter().filter(|i| i.1.len() == 1).next() {
                let op = i.1.iter().next().unwrap().clone();
                canonical.insert(i.0.clone(), op);

                for (_, v) in table.iter_mut() {
                    v.remove(&op);
                }
            }

            let mut state = Registers([0, 0, 0, 0]);
            for line in chunk.into_iter() {
                state = MachineCode(line.unwrap()).eval(&state, &canonical);
            }
            println!("Part 2: {}", state.0[0]);
        }
    }
}
