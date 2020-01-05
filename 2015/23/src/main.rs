use std::io::BufRead;
use std::str::FromStr;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Opcode {
    hlf(bool),
    tpl(bool),
    inc(bool),
    jmp(i32),
    jie(bool, i32),
    jio(bool, i32),
}

fn parse(s: &str) -> Opcode {
    let w = s.split(' ').collect::<Vec<&str>>();
    match w[0] {
        "hlf" => Opcode::hlf(w[1] == "a"),
        "tpl" => Opcode::tpl(w[1] == "a"),
        "inc" => Opcode::inc(w[1] == "a"),
        "jmp" => Opcode::jmp(i32::from_str(w[1]).unwrap()),
        "jie" => Opcode::jie(w[1] == "a,", i32::from_str(w[2]).unwrap()),
        "jio" => Opcode::jio(w[1] == "a,", i32::from_str(w[2]).unwrap()),
        _ => panic!("Invalid line {:?}", w),
    }
}

fn main() {
    let prog = std::io::stdin().lock()
        .lines()
        .map(|line| parse(&line.unwrap()))
        .collect::<Vec<Opcode>>();

    let run = |a: i32, b: i32| {
        let mut ip: i32 = 0;
        let mut regs = [b, a];
        use Opcode::*;
        while let Some(op) = prog.get(ip as usize) {
            match op {
                hlf(r) => { regs[*r as usize] /= 2; ip += 1 },
                tpl(r) => { regs[*r as usize] *= 3; ip += 1 },
                inc(r) => { regs[*r as usize] += 1; ip += 1 },
                jmp(j) => ip += *j,
                jie(r, j) => ip += if regs[*r as usize] % 2 == 0
                                   { *j } else { 1 },
                jio(r, j) => ip += if regs[*r as usize] == 1
                                   { *j } else { 1 },
            }
        }
        (regs[1], regs[0])
    };
    let (_a, b) = run(0, 0);
    println!("Part 1: {}", b);

    let (_a, b) = run(1, 0);
    println!("Part 2: {}", b);
}
