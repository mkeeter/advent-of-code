use std::io::BufRead;
use std::str::FromStr;
use std::collections::HashSet;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Opcode {
    nop,
    acc,
    jmp,
}

fn parse(s: &str) -> (Opcode, i64) {
    let mut itr = s.split(" ");

    let op_str = itr.next().unwrap();
    let op = match op_str {
        "nop" => Opcode::nop,
        "acc" => Opcode::acc,
        "jmp" => Opcode::jmp,
        _ => panic!("Unknown opcode {}", op_str),
    };

    let num = i64::from_str(itr.next().unwrap()).unwrap();

    (op, num)
}

fn main() {
    let program = std::io::stdin().lock().lines()
        .map(|line| parse(&line.unwrap()))
        .collect::<Vec<_>>();
    println!("{:?}", program);

    let step = |(ip, ac) : (i64, i64)| {
        let (op, v) = &program[ip as usize];
        match op {
            Opcode::nop => (ip + 1, ac),
            Opcode::acc => (ip + 1, v + ac),
            Opcode::jmp => (ip + v, ac),
        }
    };

    let mut seen = HashSet::new();
    let p1 = std::iter::successors(Some((0, 0)), |i| Some(step(*i)))
        .find(move |(ip, _)| !seen.insert(*ip))
        .unwrap();
    println!("{:?}", p1);

}
