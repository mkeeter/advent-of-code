use std::io::Read;
use std::str::FromStr;

const OP_ADD:    i32 = 1;
const OP_MUL:    i32 = 2;
const OP_INPUT:  i32 = 3;
const OP_OUTPUT: i32 = 4;
const OP_JIT:    i32 = 5;
const OP_JIF:    i32 = 6;
const OP_LT:     i32 = 7;
const OP_EQ:     i32 = 8;
const OP_BREAK:  i32 = 99;

const MODE_POSITION:  i32 = 0;
const MODE_IMMEDIATE: i32 = 1;

fn param(mem: &Vec<i32>, ip: usize, index: u32) -> i32 {
    let m = (mem[ip] / 10_i32.pow(index + 1)) % 10;
    let arg = ip + index as usize;
    match m {
        MODE_POSITION  => mem[mem[arg] as usize],
        MODE_IMMEDIATE => mem[arg],
        _ => panic!(),
    }
}

fn run(mut mem: Vec<i32>, input: i32) -> Vec<i32> {
    let mut ip = 0;
    let mut output = Vec::new();
    loop {
        let opcode = mem[ip] % 100;
        match opcode {
            OP_ADD => {
                let lhs = param(&mem, ip, 1);
                let rhs = param(&mem, ip, 2);
                let out = mem[ip + 3] as usize;
                mem[out] = lhs + rhs;
                ip += 4;
            }
            OP_MUL => {
                let lhs = param(&mem, ip, 1);
                let rhs = param(&mem, ip, 2);
                let out = mem[ip + 3] as usize;
                mem[out] = lhs * rhs;
                ip += 4;
            }
            OP_INPUT => {
                let out = mem[ip + 1] as usize;
                mem[out] = input;
                ip += 2;
            }
            OP_OUTPUT => {
                output.push(param(&mem, ip, 1));
                ip += 2;
            }
            OP_JIT => {
                let p = param(&mem, ip, 1);
                if p != 0 {
                    ip = param(&mem, ip, 2) as usize;
                } else {
                    ip += 3;
                }
            }
            OP_JIF => {
                let p = param(&mem, ip, 1);
                if p == 0 {
                    ip = param(&mem, ip, 2) as usize;
                } else {
                    ip += 3;
                }
            }
            OP_LT => {
                let lhs = param(&mem, ip, 1);
                let rhs = param(&mem, ip, 2);
                let out = mem[ip + 3] as usize;
                mem[out] = if lhs < rhs { 1 } else { 0 };
                ip += 4;
            }
            OP_EQ => {
                let lhs = param(&mem, ip, 1);
                let rhs = param(&mem, ip, 2);
                let out = mem[ip + 3] as usize;
                mem[out] = if lhs == rhs { 1 } else { 0 };
                ip += 4;
            }
            OP_BREAK => break,
            _ => panic!("Invalid opcode {}", opcode),
        }
    }
    output
}
fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mem = buffer.trim()
        .split(',')
        .map(|i| i32::from_str(i))
        .map(|r| r.expect("Could not parse int"))
        .collect::<Vec<_>>();

    let out = run(mem.clone(), 1);
    for o in out[0..out.len() - 1].iter() {
        assert!(*o == 0);
    }
    println!("Part 1: {}", out[out.len() - 1]);

    let out = run(mem.clone(), 5);
    assert!(out.len() == 1);
    println!("Part 2: {:?}", out[0]);
}
