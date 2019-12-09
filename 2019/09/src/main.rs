use std::io::Read;
use std::str::FromStr;
use std::collections::VecDeque;

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
struct Vm {
    mem: Vec<i64>,
    ip: usize,
    input: VecDeque<i64>,
    base: i64,
}

impl Vm {
    fn new(mem: &[i64]) -> Vm {
        let mut mem = mem.to_vec();
        mem.resize(1024_usize.pow(2), 0);
        Vm { mem: mem, ip: 0, input: VecDeque::new(), base: 0 }
    }

    fn running(&self) -> bool {
        self.mem[self.ip] != OP_BREAK
    }

    fn param(&mut self, index: u32) -> &mut i64 {
        let m = (self.mem[self.ip] / 10_i64.pow(index + 1)) % 10;
        let arg = self.ip + index as usize;
        let pos = self.mem[arg];
        match m {
            MODE_POSITION  => &mut self.mem[pos as usize],
            MODE_IMMEDIATE => &mut self.mem[arg],
            MODE_RELATIVE => &mut self.mem[(pos + self.base) as usize],
            _ => panic!(),
        }
    }

    fn step(&mut self) -> Option<i64>
    {
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
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mem = buffer.trim()
        .split(',')
        .map(|i| i64::from_str(i))
        .map(|r| r.expect("Could not parse int"))
        .collect::<Vec<_>>();

    let mut vm = Vm::new(&mem);
    vm.input.push_front(1);
    while vm.running() {
        if let Some(out) = vm.step() {
            println!("Part 1: {} ", out);
        }
    }

    let mut vm = Vm::new(&mem);
    vm.input.push_front(2);
    while vm.running() {
        if let Some(out) = vm.step() {
            println!("Part 2: {} ", out);
        }
    }
}
