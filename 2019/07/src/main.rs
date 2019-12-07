use std::io::Read;
use std::str::FromStr;
use std::collections::VecDeque;
use itertools::Itertools;

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
    input: VecDeque<i64>,
}

impl Vm {
    fn new(mem: &[i64]) -> Vm {
        Vm { mem: mem.to_vec(), ip: 0, input: VecDeque::new() }
    }

    fn running(&self) -> bool {
        self.mem[self.ip] != OP_BREAK
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

    fn step(&mut self) -> Option<i64>
    {
        let opcode = self.mem[self.ip] % 100;
        match opcode {
            OP_ADD => {
                let out = self.mem[self.ip + 3] as usize;
                self.mem[out] = self.param(1) + self.param(2);
                self.ip += 4;
            }
            OP_MUL => {
                let out = self.mem[self.ip + 3] as usize;
                self.mem[out] = self.param(1) * self.param(2);
                self.ip += 4;
            }
            OP_INPUT => {
                if let Some(i) = self.input.pop_back() {
                    let out = self.mem[self.ip + 1] as usize;
                    self.mem[out] = i;
                    self.ip += 2;
                }
            }
            OP_OUTPUT => {
                let out = self.param(1);
                self.ip += 2;
                return Some(out);
            }
            OP_JIT => {
                if self.param(1) != 0 {
                    self.ip = self.param(2) as usize;
                } else {
                    self.ip += 3;
                }
            }
            OP_JIF => {
                if self.param(1) == 0 {
                    self.ip = self.param(2) as usize;
                } else {
                    self.ip += 3;
                }
            }
            OP_LT => {
                let out = self.mem[self.ip + 3] as usize;
                self.mem[out] = (self.param(1) < self.param(2)) as i64;
                self.ip += 4;
            }
            OP_EQ => {
                let out = self.mem[self.ip + 3] as usize;
                self.mem[out] = (self.param(1) == self.param(2)) as i64;
                self.ip += 4;
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

    ////////////////////////////////////////////////////////////////////////////
    // Part 1
    let best = (0..5).permutations(5)
        .map(|ps| {
            // Build a fresh set of VMs and queues
            let mut vms = vec![Vm::new(&mem); 5];
            for (i, vm) in vms.iter_mut().enumerate() {
                vm.input.push_front(ps[i] as i64);
            }
            vms[0].input.push_front(0);

            loop {
                for i in 0..vms.len() {
                    if let Some(out) = vms[i].step() {
                        if let Some(vm) = vms.get_mut(i + 1) {
                            vm.input.push_front(out);
                        } else {
                            return out;
                        }
                    }
                }
            }
        }).max().unwrap();
    println!("Part 1: {}", best);

    ////////////////////////////////////////////////////////////////////////////
    let best = (5..10).permutations(5)
        .map(|ps| {
            // Build a fresh set of VMs and queues
            let mut vms = vec![Vm::new(&mem); 5];
            for (i, vm) in vms.iter_mut().enumerate() {
                vm.input.push_front(ps[i] as i64);
            }
            vms[0].input.push_front(0);

            while vms.iter().any(Vm::running) {
                for i in 0..vms.len() {
                    if let Some(out) = vms[i].step() {
                        vms[(i + 1) % 5].input.push_front(out);
                    }
                }
            }
            assert!(vms[0].input.len() == 1);
            *vms[0].input.back().unwrap()
        }).max().unwrap();
    println!("Part 2: {}", best);
}
