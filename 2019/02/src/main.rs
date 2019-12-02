use std::io::Read;
use std::str::FromStr;

fn run(noun: usize, verb: usize, mut mem: Vec<usize>) -> usize {
    mem[1] = noun;
    mem[2] = verb;

    let mut ip = 0;
    loop {
        match mem[ip] {
            1 | 2 => {
                let lhs = mem[mem[ip + 1]];
                let rhs = mem[mem[ip + 2]];
                let out = mem[ip + 3];
                mem[out] = match mem[ip] {
                    1 => lhs + rhs,
                    2 => lhs * rhs,
                    _ => unreachable!(),
                };
                ip += 4;
            }
            99 => break,
            _ => panic!("Invalid opcode"),
        }
    }
    mem[0]
}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let mem = buffer.trim()
        .split(',')
        .map(|i| usize::from_str(i))
        .map(|r| r.expect("Could not parse int"))
        .collect::<Vec<_>>();

    println!("Part 1: {}", run(12, 2, mem.clone()));

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            if run(noun, verb, mem.clone()) == 1969_07_20 {
                println!("Part 2: {}", 100 * noun + verb);
                break 'outer;
            }
        }
    }

    Ok(())
}
