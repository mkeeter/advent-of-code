use std::io::{self, Read};
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
struct Registers([usize; 6]);

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Op {
    addr, addi,
    mulr, muli,
    banr, bani,
    borr, bori,
    setr, seti,
    gtir, gtri, gtrr,
    eqir, eqri, eqrr,
}
use crate::Op::*;

impl Op {
    fn from_str(s: &str) -> Op {
        match s {
            "addr" => addr,
            "addi" => addi,
            "mulr" => mulr,
            "muli" => muli,
            "banr" => banr,
            "bani" => bani,
            "borr" => borr,
            "bori" => bori,
            "setr" => setr,
            "seti" => seti,
            "gtir" => gtir,
            "gtri" => gtri,
            "gtrr" => gtrr,
            "eqir" => eqir,
            "eqri" => eqri,
            "eqrr" => eqrr,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    op: Op,
    a: usize,
    b: usize,
    c: usize,
}

impl Instruction {
    fn eval(&self, s: &mut Registers) {
        s.0[self.c] = match self.op {
            addr => s.0[self.a] + s.0[self.b],
            addi => s.0[self.a] + self.b,
            mulr => s.0[self.a] * s.0[self.b],
            muli => s.0[self.a] * self.b,
            banr => s.0[self.a] & s.0[self.b],
            bani => s.0[self.a] & self.b,
            borr => s.0[self.a] | s.0[self.b],
            bori => s.0[self.a] | self.b,
            setr => s.0[self.a],
            seti => self.a,
            gtir => (self.a > s.0[self.b]) as usize,
            gtri => (s.0[self.a] > self.b) as usize,
            gtrr => (s.0[self.a] > s.0[self.b]) as usize,
            eqir => (self.a == s.0[self.b]) as usize,
            eqri => (s.0[self.a] == self.b) as usize,
            eqrr => (s.0[self.a] == s.0[self.b]) as usize,
        };
    }
}

// Optimized implementation of the weird calculation,
// tracking the value of reg[3] at the termination
// condition and returning the last non-looping result.
fn part2() {
    let mut v3 = 0;
    let mut v2;

    let mut seen = HashSet::new();
    let mut prev = 0;

    loop {
        v2 = v3 | 65536;
        v3 = 1099159;
        loop {
            v3 += v2 & 255;
            v3 = ((v3 & 16777215) * 65899) & 16777215;
            if v2 < 256 {
                break;
            }
            v2 = (0..).skip_while(|i| (i + 1) * 256 <= v2)
                      .next().unwrap();
        }

        if seen.contains(&v3) {
            println!("Part 2: {}", prev);
            break;
        } else {
            prev = v3;
            seen.insert(v3);
        }
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut ip_reg = 0;
    let tape = buffer
        .lines()
        .filter_map(|line| {
            let words = line.split(' ').collect::<Vec<_>>();
            if words[0] == "#ip" {
                ip_reg = str::parse::<usize>(words[1]).unwrap();
                None
            } else {
                let op = Op::from_str(words[0]);
                let a = str::parse::<usize>(words[1]).unwrap();
                let b = str::parse::<usize>(words[2]).unwrap();
                let c = str::parse::<usize>(words[3]).unwrap();
                Some(Instruction { op: op, a: a, b: b, c: c})
            }
        })
        .collect::<Vec<Instruction>>();

    // Run until we hit line 28 for the first time, which is
    // our termination condition (if reg[3] == reg[0]).  We
    // then pull out reg[3]'s value, which is the value for reg[0]
    // that will cause the earliest possible termination.
    let mut state = Registers([0, 0, 0, 0, 0, 0]);
    let mut ip = 0;
    loop {
        if ip >= tape.len() {
            break;
        }
        state.0[ip_reg] = ip;
        tape[ip].eval(&mut state);
        ip = state.0[ip_reg] + 1;
        if ip == 28 {
            println!("Part 1: {}", state.0[3]);
            break;
        }
    }

    part2();
}
