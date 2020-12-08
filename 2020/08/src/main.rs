use std::io::BufRead;
use std::str::FromStr;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Opcode {
    nop,
    acc,
    jmp,
}

#[derive(Clone)]
struct Program(Vec<(Opcode, i64)>);
impl Program {
    fn parse_line(s: &str) -> (Opcode, i64) {
        let mut itr = s.split(' ');

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

    fn step(&self, (ip, ac) : (i64, i64)) -> (i64, i64) {
        let (op, v) = &self.0[ip as usize];
        match op {
            Opcode::nop => (ip + 1, ac),
            Opcode::acc => (ip + 1, v + ac),
            Opcode::jmp => (ip + v, ac),
        }
    }

    // Returns Okay(ac) if the program terminates, and Err(ac) if not
    fn run(&self) -> Result<i64, i64> {
        let mut seen = vec![false; self.0.len()];
        let mut state = (0, 0); // ip, ac
        loop {
            let ip = state.0 as usize;
            if ip == self.0.len() {
                return Ok(state.1);
            } else if seen[ip] {
                return Err(state.1);
            }
            seen[ip] = true;
            state = self.step(state);
        }
    }
}


fn main() {
    let p = Program(std::io::stdin().lock().lines()
        .map(|line| Program::parse_line(&line.unwrap()))
        .collect::<Vec<_>>());

    let p1 = p.run().unwrap_err();
    println!("Part 1: {}", p1);

    for i in 0..p.0.len() {
        let op = match p.0[i].0 {
            Opcode::jmp => Opcode::nop,
            Opcode::nop => Opcode::jmp,
            Opcode::acc => continue,
        };
        let mut p_ = p.clone();
        p_.0[i].0 = op;

        if let Ok(p2) = p_.run() {
            println!("Part 2: {}", p2);
            break;
        }
    }
}
