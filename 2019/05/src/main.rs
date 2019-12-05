use std::io::Read;
use std::str::FromStr;

fn imm(opcode: i32, index: u32) -> bool {
    let m = (opcode / 10_i32.pow(2 + index)) % 10;
    match m {
        0 => false,
        1 => true,
        _ => panic!(),
    }
}

fn run(mut mem: Vec<i32>, input: i32) -> Vec<i32> {
    let mut ip = 0;
    let mut output = Vec::new();
    loop {
        let opcode = mem[ip];
        match opcode % 100 {
            1 | 2 => {
                println!("{:?}", &mem[ip..ip+4]);
                let lhs = if imm(opcode, 0) {
                    mem[ip + 1]
                } else {
                    mem[mem[ip + 1] as usize]
                };
                let rhs = if imm(opcode, 1) {
                    mem[ip + 2]
                } else {
                    mem[mem[ip + 2] as usize]
                };
                let out = mem[ip + 3] as usize;
                mem[out] = match opcode % 100 {
                    1 => lhs + rhs,
                    2 => lhs * rhs,
                    _ => unreachable!(),
                };
                ip += 4;
            }
            3 => {
                println!("{:?}", &mem[ip..ip+2]);
                let out = mem[ip + 1] as usize;
                mem[out] = input;
                ip += 2;
            }
            4 => {
                println!("{:?}", &mem[ip..ip+2]);
                if imm(opcode, 0) {
                    println!("immediate\n");
                    output.push(mem[ip + 1]);
                } else {
                    println!("position\n");
                    output.push(mem[mem[ip + 1] as usize]);
                }
                println!("Output: {:?}", output);
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
