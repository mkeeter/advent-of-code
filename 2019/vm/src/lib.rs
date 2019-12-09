#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::collections::VecDeque;
use std::str::FromStr;

const OP_ADD:    i64 = 1;
const OP_MUL:    i64 = 2;
const OP_INPUT:  i64 = 3;
const OP_OUTPUT: i64 = 4;
const OP_JIT:    i64 = 5;
const OP_JIF:    i64 = 6;
const OP_LT:     i64 = 7;
const OP_EQ:     i64 = 8;
const OP_RBO:    i64 = 9;
const OP_BREAK:  i64 = 99;

const MODE_POSITION:  i64 = 0;
const MODE_IMMEDIATE: i64 = 1;
const MODE_RELATIVE:  i64 = 2;

#[derive(Clone)]
pub struct Vm {
    mem: Vec<i64>,
    ip: usize,
    input: VecDeque<i64>,
    base: i64,
}

impl Vm {
    pub fn from_str(s: &str) -> Self {
        Self::new(&s.trim()
            .split(',')
            .map(|i| i64::from_str(i))
            .map(|r| r.expect("Could not parse int"))
            .collect::<Vec<_>>())
    }

    pub fn new(mem: &[i64]) -> Self {
        let mut mem = mem.to_vec();
        mem.resize(1024_usize.pow(2), 0);
        Self { mem: mem, ip: 0, input: VecDeque::new(), base: 0 }
    }

    pub fn running(&self) -> bool {
        self.mem[self.ip] != OP_BREAK
    }

    pub fn blocked(&self) -> bool {
        self.mem[self.ip] % 100 == OP_INPUT && self.input.len() == 0
    }

    fn param(&mut self, index: u32) -> &mut i64 {
        let m = (self.mem[self.ip] / 10_i64.pow(index + 1)) % 10;
        let arg = self.ip + index as usize;
        let pos = self.mem[arg];
        match m {
            MODE_POSITION  => &mut self.mem[pos as usize],
            MODE_IMMEDIATE => &mut self.mem[arg],
            MODE_RELATIVE  => &mut self.mem[(pos + self.base) as usize],
            _ => panic!(),
        }
    }

    pub fn input(&mut self, i: i64) {
        self.input.push_front(i);
    }

    pub fn step(&mut self) -> Option<i64> {
        let opcode = self.mem[self.ip] % 100;
        match opcode {
            OP_ADD => {
                let lhs = *self.param(1);
                let rhs = *self.param(2);
                *self.param(3) = lhs + rhs;
                self.ip += 4;
            }
            OP_MUL => {
                let lhs = *self.param(1);
                let rhs = *self.param(2);
                *self.param(3) = lhs * rhs;
                self.ip += 4;
            }
            OP_INPUT => {
                if let Some(i) = self.input.pop_back() {
                    *self.param(1) = i;
                    self.ip += 2;
                }
            }
            OP_OUTPUT => {
                let out = *self.param(1);
                self.ip += 2;
                return Some(out);
            }
            OP_JIT => {
                if *self.param(1) != 0 {
                    self.ip = *self.param(2) as usize;
                } else {
                    self.ip += 3;
                }
            }
            OP_JIF => {
                if *self.param(1) == 0 {
                    self.ip = *self.param(2) as usize;
                } else {
                    self.ip += 3;
                }
            }
            OP_LT => {
                let lhs = *self.param(1);
                let rhs = *self.param(2);
                *self.param(3) = (lhs < rhs) as i64;
                self.ip += 4;
            }
            OP_EQ => {
                let lhs = *self.param(1);
                let rhs = *self.param(2);
                *self.param(3) = (lhs == rhs) as i64;
                self.ip += 4;
            }
            OP_RBO => {
                self.base += *self.param(1);
                self.ip += 2;
            }
            OP_BREAK => (),
            _ => panic!("Invalid opcode {}", opcode),
        };
        None
    }

    pub fn run(&mut self) -> Vec<i64> {
        let mut out = Vec::new();
        while self.running() {
            assert!(!self.blocked());
            if let Some(i) = self.step() {
                out.push(i);
            }
        }
        out
    }

    pub fn peek(&self, i: usize) -> i64 {
        self.mem[i]
    }

    pub fn poke(&mut self, i: usize, v: i64) {
        self.mem[i] = v;
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        for (tape, output) in &[
            (vec![1,0,0,0,99], vec![2,0,0,0,99]),
            (vec![2,3,0,3,99], vec![2,3,0,6,99]),
            (vec![2,4,4,5,99,0], vec![2,4,4,5,99,9801]),
            (vec![1,1,1,4,99,5,6,0,99], vec![30,1,1,4,2,5,6,0,99])]
        {
            let mut vm = Vm::new(&tape);
            vm.run();
            assert_eq!(vm.mem[..tape.len()], output[..]);
        }
    }

    #[quickcheck]
    fn io(i: i64) -> bool  {
        let mut vm = Vm::new(&[3,0,4,0,99]);
        vm.input(i);
        vm.run() == vec![i]
    }

    // Using position mode, tests if the input is 8
    fn test_eq8_pos(i: i64) -> bool {
        let mut vm = Vm::new(&[3,9,8,9,10,9,4,9,99,-1,8]);
        vm.input(i);
        vm.run() == vec![(i == 8) as i64]
    }
    #[test]
    fn eq8_pos() { assert!(test_eq8_pos(8)) }
    #[quickcheck]
    fn eq8_pos_qc(i: i64) -> bool { test_eq8_pos(i) }

    // Using position mode, tests if the input is less than 8
    #[quickcheck]
    fn lt8_pos(i: i64) -> bool {
        let mut vm = Vm::new(&[3,9,7,9,10,9,4,9,99,-1,8]);
        vm.input(i);
        vm.run() == vec![(i < 8) as i64]
    }

    // Using immediate mode, tests if the input is 8
    fn test_eq8_imm(i: i64) -> bool {
        let mut vm = Vm::new(&[3,3,1108,-1,8,3,4,3,99]);
        vm.input(i);
        vm.run() == vec![(i == 8) as i64]
    }
    #[test]
    fn eq8_imm() { assert!(test_eq8_imm(8)); }
    #[quickcheck]
    fn eq8_imm_qc(i: i64) -> bool { test_eq8_imm(i) }

    // Using immediate mode, tests if the input is less than 8
    #[quickcheck]
    fn lt8_imm(i: i64) -> bool {
        let mut vm = Vm::new(&[3,3,1107,-1,8,3,4,3,99]);
        vm.input(i);
        vm.run() == vec![(i < 8) as i64]
    }

    // Position-mode jump test, which outputs 1 if the input is non-zero
    fn test_eq0_pos(i: i64) -> bool {
        let mut vm = Vm::new(&[
            3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]);
        vm.input(i);
        vm.run() == vec![(i != 0) as i64]
    }
    #[test]
    fn eq0_pos() { assert!(test_eq0_pos(0)); }
    #[quickcheck]
    fn eq0_pos_qc(i: i64) -> bool { test_eq0_pos(i) }

    // Immediate-mode jump test, which outputs 1 if the input is non-zero
    fn test_eq0_imm(i: i64) -> bool {
        let mut vm = Vm::new(&[
            3,3,1105,-1,9, 1101,0,0,12,4,12,99,1]);
        vm.input(i);
        vm.run() == vec![(i != 0) as i64]
    }
    #[test]
    fn eq0_imm() { assert!(test_eq0_imm(0)) }
    #[quickcheck]
    fn eq0_imm_qc(i: i64) -> bool { test_eq0_imm(i) }

    // Larger program which outputs 999, 1000, or 1001 depending on how
    // the input compares to 8
    fn test_cmp8(i: i64) -> bool {
        let mut vm = Vm::new(&[
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]);
        vm.input(i);
        let out = vm.run();
        if i < 8 {
            out == vec![999]
        } else if i == 8 {
            out == vec![1000]
        } else {
            out == vec![1001]
        }
    }
    #[test]
    fn cmp8() { assert!(test_cmp8(8)); }
    #[quickcheck]
    fn cmp8_qc(i: i64) -> bool { test_cmp8(i) }

    // Outputs its own source code
    #[test]
    fn quine() {
        let prog = [109,1,204,-1,1001,100,1,100,
                    1008,100,16,101,1006,101,0,99];
        let mut v = Vm::new(&prog);
        let out = v.run();
        assert_eq!(out, prog);
    }

    // Large-number multiplication
    #[test]
    fn large_mul() {
        let mut v = Vm::new(&[1102,34915192,34915192,7,4,7,99,0]);
        let out = v.run();
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].to_string().chars().count(), 16);
    }

    // Just large numbers in general
    #[test]
    fn large_output() {
        let mut v = Vm::new(&[104,1125899906842624,99]);
        assert_eq!(v.run(), vec![1125899906842624]);
    }

    // Fibonacci sequence generator
    #[test]
    fn fib() {
        let mut v = Vm::new(&[
            3,3,
            1106,-1,24,
            104,1,
            1101,1,0,6,
            1001,8,0,9,
            1001,6,0,8,
            1001,3,-1,3,
            1105,99,2]);
        v.input(10);
        assert_eq!(v.run(), vec![1,1,2,3,5,8,13,21,34,55]);
    }
}

