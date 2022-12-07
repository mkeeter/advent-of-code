use std::io::{self, Read};

#[derive(Debug, Eq, PartialEq)]
struct Registers([usize; 6]);

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
    fn eval(&self, s: &Registers) -> Registers {
        let mut out = s.0.clone();
        out[self.c] = match self.op {
            addr => out[self.a] + out[self.b],
            addi => out[self.a] + self.b,
            mulr => out[self.a] * out[self.b],
            muli => out[self.a] * self.b,
            banr => out[self.a] & out[self.b],
            bani => out[self.a] & self.b,
            borr => out[self.a] | out[self.b],
            bori => out[self.a] | self.b,
            setr => out[self.a],
            seti => self.a,
            gtir => (self.a > out[self.b]) as usize,
            gtri => (out[self.a] > self.b) as usize,
            gtrr => (out[self.a] > out[self.b]) as usize,
            eqir => (self.a == out[self.b]) as usize,
            eqri => (out[self.a] == self.b) as usize,
            eqrr => (out[self.a] == out[self.b]) as usize,
        };
        return Registers(out);
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
                Some(Instruction {
                    op: op,
                    a: a,
                    b: b,
                    c: c,
                })
            }
        })
        .collect::<Vec<Instruction>>();

    let mut state = Registers([0, 0, 0, 0, 0, 0]);
    let mut ip = 0;
    loop {
        if ip >= tape.len() {
            break;
        }
        state.0[ip_reg] = ip;
        state = tape[ip].eval(&state);
        ip = state.0[ip_reg] + 1;
    }
    println!("Part 1: {}", state.0[0]);

    // Run for long enough for setup to be complete,
    // then skip to a more optimized algorithm.
    let mut state = Registers([1, 0, 0, 0, 0, 0]);
    let mut ip = 0;
    loop {
        if ip >= tape.len() || ip == 3 {
            break;
        }
        state.0[ip_reg] = ip;
        state = tape[ip].eval(&state);
        ip = state.0[ip_reg] + 1;
    }

    // Spoilers: we're counting the sum of divisors for a particular value
    let t = state.0[2];
    let out: usize = (1..=t).filter(|i| t % i == 0).sum();
    println!("Part 2: {}", out);
}
