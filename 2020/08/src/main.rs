use std::io::BufRead;
use std::str::FromStr;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Opcode {
    nop,
    acc,
    jmp,
}

struct Program(Vec<(Opcode, i64)>);
impl Program {
    fn parse_line(s: &str) -> (Opcode, i64) {
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

    fn step(&self, ip: i64, ac: i64) -> (i64, i64) {
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
        let mut ip = 0;
        let mut ac = 0;
        loop {
            let ip_ = ip as usize;
            if ip_ == self.0.len() + 1 {
                return Ok(ac);
            } else if seen[ip_] {
                return Err(ac);
            } else {
                seen[ip_] = true;
            }
            let next = self.step(ip, ac);
            ip = next.0;
            ac = next.1;
        }
    }
}


fn main() {
    let p = Program(std::io::stdin().lock().lines()
        .map(|line| Program::parse_line(&line.unwrap()))
        .collect::<Vec<_>>());

    let p1 = p.run().unwrap_err();
    println!("Part 1: {}", p1);

}
