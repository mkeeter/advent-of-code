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
const OP_BREAK:  i64 = 99;

const MODE_POSITION:  i64 = 0;
const MODE_IMMEDIATE: i64 = 1;

struct Vm {
    mem: Vec<i64>,
    ip: usize,
}

#[derive(Debug, PartialEq)]
enum VmResult {
    Okay,
    UsedInput,
    NeedsInput,
    Output(i64),
    Done,
}

impl Vm {
    fn param(&self, index: u32) -> i64 {
        let m = (self.mem[self.ip] / 10_i64.pow(index + 1)) % 10;
        let arg = self.ip + index as usize;
        match m {
            MODE_POSITION  => self.mem[self.mem[arg] as usize],
            MODE_IMMEDIATE => self.mem[arg],
            _ => panic!(),
        }
    }

    fn step(&mut self, input: Option<&i64>) -> VmResult {
        let opcode = self.mem[self.ip] % 100;
        match opcode {
            OP_ADD => {
                let lhs = self.param(1);
                let rhs = self.param(2);
                let out = self.mem[self.ip + 3] as usize;
                self.mem[out] = lhs + rhs;
                self.ip += 4;
                VmResult::Okay
            }
            OP_MUL => {
                let lhs = self.param(1);
                let rhs = self.param(2);
                let out = self.mem[self.ip + 3] as usize;
                self.mem[out] = lhs * rhs;
                self.ip += 4;
                VmResult::Okay
            }
            OP_INPUT => {
                if let Some(input) = input {
                    let out = self.mem[self.ip + 1] as usize;
                    self.mem[out] = *input;
                    self.ip += 2;
                    VmResult::UsedInput
                } else {
                    VmResult::NeedsInput
                }
            }
            OP_OUTPUT => {
                let out = self.param(1);
                self.ip += 2;
                VmResult::Output(out)
            }
            OP_JIT => {
                let p = self.param(1);
                if p != 0 {
                    self.ip = self.param(2) as usize;
                } else {
                    self.ip += 3;
                }
                VmResult::Okay
            }
            OP_JIF => {
                let p = self.param(1);
                if p == 0 {
                    self.ip = self.param(2) as usize;
                } else {
                    self.ip += 3;
                }
                VmResult::Okay
            }
            OP_LT => {
                let lhs = self.param(1);
                let rhs = self.param(2);
                let out = self.mem[self.ip + 3] as usize;
                self.mem[out] = if lhs < rhs { 1 } else { 0 };
                self.ip += 4;
                VmResult::Okay
            }
            OP_EQ => {
                let lhs = self.param(1);
                let rhs = self.param(2);
                let out = self.mem[self.ip + 3] as usize;
                self.mem[out] = if lhs == rhs { 1 } else { 0 };
                self.ip += 4;
                VmResult::Okay
            }
            OP_BREAK => VmResult::Done,
            _ => panic!("Invalid opcode {}", opcode),
        }
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

    let mut phases = [0,1,2,3,4];
    let mut best = 0;
    permutohedron::heap_recursive(&mut phases, |ps| {
        let mut vms = (0..5).into_iter()
            .map(|_| Vm { mem: mem.clone(), ip: 0 })
            .collect::<Vec<_>>();
        let mut queues = (0..5).into_iter()
            .map(|i| {
                let mut q = VecDeque::new();
                q.push_front(ps[i]);
                q
            })
            .collect::<Vec<_>>();
        queues[0].push_front(0);
        loop {
            let mut all_done = true;
            for i in 0..5 {
                let result = vms[i].step(queues[i].back());
                all_done &= result == VmResult::Done;
                match result {
                    VmResult::UsedInput => { queues[i].pop_back(); () }
                    VmResult::Output(o) => if i == 4 {
                        if o > best {
                            best = o;
                        }
                    } else {
                        queues[i + 1].push_front(o);
                    },
                    _ => (),
                }
            }
            if all_done {
                break;
            }
        }
    });
    println!("Part 1: {}", best);
}
