use std::io::Read;
use std::str::FromStr;

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

fn param(mem: &Vec<i64>, ip: usize, index: u32) -> i64 {
    let m = (mem[ip] / 10_i64.pow(index + 1)) % 10;
    let arg = ip + index as usize;
    match m {
        MODE_POSITION  => mem[mem[arg] as usize],
        MODE_IMMEDIATE => mem[arg],
        _ => panic!(),
    }
}

fn run<'a, I>(mut mem: Vec<i64>, mut input: I) -> Vec<i64>
    where I: Iterator<Item=&'a i64>
{
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
                mem[out] = *input.next().unwrap();
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
        .map(|i| i64::from_str(i))
        .map(|r| r.expect("Could not parse int"))
        .collect::<Vec<_>>();

    let mut phases = [0,1,2,3,4];
    let mut best = 0;
    permutohedron::heap_recursive(&mut phases, |ps| {
        let mut input = 0;
        for i in 0..5 {
            input = run(mem.clone(), [ps[i], input].iter())[0];
        }
        if input > best {
            println!("{} {:?}", input, ps);
            best = input;
        }
    });
}
