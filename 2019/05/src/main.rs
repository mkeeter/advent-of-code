use std::io::Read;
use std::str::FromStr;

const OP_ADD: i32 = 1;
const OP_SUB: i32 = 2;
const OP_INPUT: i32 = 3;
const OP_OUTPUT: i32 = 4;
const OP_BREAK: i32 = 99;

fn param(mem: &Vec<i32>, ip: usize, index: u32) -> i32 {
    let m = (mem[ip] / 10_i32.pow(index + 2)) % 10;
    let arg = 1 + ip + index as usize;
    match m {
        0 => mem[mem[arg] as usize],
        1 => mem[arg],
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
                let lhs = param(&mem, ip, 0);
                let rhs = param(&mem, ip, 1);
                let out = mem[ip + 3] as usize;
                mem[out] = lhs + rhs;
                ip += 4;
            }
            OP_SUB => {
                let lhs = param(&mem, ip, 0);
                let rhs = param(&mem, ip, 1);
                let out = mem[ip + 3] as usize;
                mem[out] = lhs * rhs;
                ip += 4;
            }
            OP_INPUT => {
                let out = mem[ip + 1];
                mem[out as usize] = input;
                ip += 2;
            }
            OP_OUTPUT => {
                output.push(param(&mem, ip, 0));
                ip += 2;
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
    let out = run(mem, 1);
    println!("{:?}", out);
}
