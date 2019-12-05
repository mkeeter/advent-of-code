use std::io::Read;
use std::str::FromStr;

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
            1 | 2 => {
                let lhs = param(&mem, ip, 0);
                let rhs = param(&mem, ip, 1);
                let out = mem[ip + 3] as usize;
                mem[out] = match opcode {
                    1 => lhs + rhs,
                    2 => lhs * rhs,
                    _ => unreachable!(),
                };
                ip += 4;
            }
            3 => {
                let out = mem[ip + 1];
                mem[out as usize] = input;
                ip += 2;
            }
            4 => {
                output.push(param(&mem, ip, 0));
                ip += 2;
            }
            99 => break,
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
