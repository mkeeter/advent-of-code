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

#[derive(Clone)]
struct Vm {
    mem: Vec<i64>,
    ip: usize,
}

impl Vm {
    fn done(&self) -> bool {
        self.mem[self.ip] == OP_BREAK
    }

    fn param(&self, index: u32) -> i64 {
        let m = (self.mem[self.ip] / 10_i64.pow(index + 1)) % 10;
        let arg = self.ip + index as usize;
        match m {
            MODE_POSITION  => self.mem[self.mem[arg] as usize],
            MODE_IMMEDIATE => self.mem[arg],
            _ => panic!(),
        }
    }

    fn step(&mut self,
            input: &mut VecDeque<i64>,
            output: &mut VecDeque<i64>)
    {
        let opcode = self.mem[self.ip] % 100;
        match opcode {
            OP_ADD => {
                let lhs = self.param(1);
                let rhs = self.param(2);
                let out = self.mem[self.ip + 3] as usize;
                self.mem[out] = lhs + rhs;
                self.ip += 4;
            }
            OP_MUL => {
                let lhs = self.param(1);
                let rhs = self.param(2);
                let out = self.mem[self.ip + 3] as usize;
                self.mem[out] = lhs * rhs;
                self.ip += 4;
            }
            OP_INPUT => {
                if let Some(input) = input.pop_back() {
                    let out = self.mem[self.ip + 1] as usize;
                    self.mem[out] = input;
                    self.ip += 2;
                }
            }
            OP_OUTPUT => {
                output.push_front(self.param(1));
                self.ip += 2;
            }
            OP_JIT => {
                let p = self.param(1);
                if p != 0 {
                    self.ip = self.param(2) as usize;
                } else {
                    self.ip += 3;
                }
            }
            OP_JIF => {
                let p = self.param(1);
                if p == 0 {
                    self.ip = self.param(2) as usize;
                } else {
                    self.ip += 3;
                }
            }
            OP_LT => {
                let lhs = self.param(1);
                let rhs = self.param(2);
                let out = self.mem[self.ip + 3] as usize;
                self.mem[out] = if lhs < rhs { 1 } else { 0 };
                self.ip += 4;
            }
            OP_EQ => {
                let lhs = self.param(1);
                let rhs = self.param(2);
                let out = self.mem[self.ip + 3] as usize;
                self.mem[out] = if lhs == rhs { 1 } else { 0 };
                self.ip += 4;
            }
            OP_BREAK => (),
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
        // Build a fresh set of VMs and queues
        let mut vms = vec![Vm { mem: mem.clone(), ip: 0 }; 5];
        let mut queues = vec![VecDeque::new(); 6];
        for (i, q) in queues.iter_mut().enumerate().take(5) {
            q.push_front(ps[i] as i64);
        }
        queues[0].push_front(0);

        while queues[5].len() == 0 {
            for i in 0..5 {
                let (qa, qb) = queues.split_at_mut(i + 1);
                vms[i].step(&mut qa[i], &mut qb[0]);
            }
        }
        let out = queues[5].pop_back().unwrap();
        if out > best {
            best = out;
        }
    });
    println!("Part 1: {}", best);

    let mut phases = [0,1,2,3,4];
    let mut best = 0;
    permutohedron::heap_recursive(&mut phases, |ps| {
        // Build a fresh set of VMs and queues
        let mut vms = vec![Vm { mem: mem.clone(), ip: 0 }; 5];
        let mut queues = vec![VecDeque::new(); 5];
        for (i, q) in queues.iter_mut().enumerate().take(5) {
            q.push_front(ps[i] + 5 as i64);
        }
        queues[0].push_front(0);

        let mut last = 0;
        while vms.iter().any(|vm| !vm.done()) {
            for i in 0..5 {
                if i == 4 {
                    let (qa, qb) = queues.split_at_mut(i);
                    vms[i].step(&mut qb[0], &mut qa[0]);
                } else {
                    let (qa, qb) = queues.split_at_mut(i + 1);
                    vms[i].step(&mut qa[i], &mut qb[0]);
                }
            }
            // Peek at the output queue of the last amplifier
            if let Some(o) = queues[0].back() {
                last = *o;
            }
        }
        if last > best {
            best = last;
        }
    });
    println!("Part 2: {}", best);
}
