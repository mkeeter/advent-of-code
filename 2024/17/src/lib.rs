use std::fmt::Write;
use std::io::Write as _;

use util::get_integers;

#[derive(Clone)]
struct Vm {
    a: u64,
    b: u64,
    c: u64,
    ip: usize,
    tape: Vec<u8>,
}

#[derive(Debug)]
enum Step {
    Output(u8),
    Continue,
    Done,
}

#[derive(Debug)]
enum Opcode {
    Adv(Combo),
    Bxl(u8),
    Bst(Combo),
    Jnz(u8),
    Bxc,
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

impl Opcode {
    fn new(opcode: u8, operand: u8) -> Self {
        match opcode {
            0 => Opcode::Adv(Combo::new(operand)),
            1 => Opcode::Bxl(operand),
            2 => Opcode::Bst(Combo::new(operand)),
            3 => Opcode::Jnz(operand),
            4 => Opcode::Bxc,
            5 => Opcode::Out(Combo::new(operand)),
            6 => Opcode::Bdv(Combo::new(operand)),
            7 => Opcode::Cdv(Combo::new(operand)),
            _ => panic!("invalid opcode {opcode}"),
        }
    }
}

#[derive(Debug)]
enum Combo {
    A,
    B,
    C,
    Lit(u64),
}

impl Combo {
    fn new(v: u8) -> Self {
        match v {
            0..=3 => Combo::Lit(u64::from(v)),
            4 => Combo::A,
            5 => Combo::B,
            6 => Combo::C,
            _ => panic!("invalid combo value {v}"),
        }
    }
}

impl Vm {
    fn new(s: &str) -> Self {
        let mut iter = s.lines();
        let regs: [u64; 3] = std::array::from_fn(|_| {
            iter.next().and_then(|i| get_integers(i).next()).unwrap()
        });
        let blank = iter.next().unwrap();
        assert!(blank.is_empty());

        let t = iter.next().unwrap();
        let tape: Vec<u8> = get_integers(t).collect();

        Vm {
            a: regs[0],
            b: regs[1],
            c: regs[2],
            ip: 0,
            tape,
        }
    }

    fn run_to_completion(&mut self) -> Vec<u8> {
        let mut out = vec![];
        loop {
            match self.step() {
                Step::Output(i) => out.push(i),
                Step::Continue => (),
                Step::Done => return out,
            }
        }
    }
    fn step(&mut self) -> Step {
        if self.ip >= self.tape.len() {
            return Step::Done;
        }
        let op = Opcode::new(self.tape[self.ip], self.tape[self.ip + 1]);
        self.ip += 2;
        self.apply(op)
    }

    fn apply(&mut self, op: Opcode) -> Step {
        match op {
            Opcode::Adv(v) => self.a >>= self.combo(v),
            Opcode::Bxl(v) => self.b ^= u64::from(v),
            Opcode::Bst(v) => self.b = self.combo(v) & 0b111,
            Opcode::Jnz(v) if self.a != 0 => self.ip = usize::from(v),
            Opcode::Jnz(_) => (), // non jump
            Opcode::Bxc => self.b ^= self.c,
            Opcode::Out(v) => return Step::Output(self.combo(v) as u8 & 0b111),
            Opcode::Bdv(v) => self.b = self.a >> self.combo(v),
            Opcode::Cdv(v) => self.c = self.a >> self.combo(v),
        }
        Step::Continue
    }

    fn combo(&self, v: Combo) -> u64 {
        match v {
            Combo::Lit(v) => v,
            Combo::A => self.a,
            Combo::B => self.b,
            Combo::C => self.c,
        }
    }
}

struct Writer<'a> {
    ia: usize,
    ib: usize,
    ic: usize,
    t: usize,
    ip: usize,
    tape: &'a [u8],
    lines: Vec<String>,
}

impl Writer<'_> {
    fn run(&mut self) {
        while self.ip < self.tape.len() {
            let op = Opcode::new(self.tape[self.ip], self.tape[self.ip + 1]);
            self.ip += 2;
            self.apply(op);
        }
    }

    fn hex<I>(i: I) -> String
    where
        I: std::fmt::LowerHex,
    {
        format!("#x{i:016x}")
    }

    fn apply(&mut self, op: Opcode) {
        match op {
            Opcode::Adv(v) => {
                self.lines.push(format!(
                    "(assert (= a{} (bvlshr a{} {})))",
                    self.ia + 1,
                    self.ia,
                    self.combo(v)
                ));
                self.ia += 1;
            }
            Opcode::Bxl(v) => {
                self.lines.push(format!(
                    "(assert (= b{} (bvxor b{} {})))",
                    self.ib + 1,
                    self.ib,
                    Self::hex(v)
                ));
                self.ib += 1;
            }
            Opcode::Bst(v) => {
                self.lines.push(format!(
                    "(assert (= b{} (bvand {} {})))",
                    self.ib + 1,
                    self.combo(v),
                    Self::hex(0b111),
                ));
                self.ib += 1;
            }
            Opcode::Jnz(_) if self.t == self.tape.len() => self
                .lines
                .push(format!("(assert (= a{} {}))", self.ia, Self::hex(0))),
            Opcode::Jnz(v) => {
                self.ip = usize::from(v);
                self.lines.push(format!(
                    "(assert (not (= a{} {})))",
                    self.ia,
                    Self::hex(0),
                ));
            }
            Opcode::Bxc => {
                self.lines.push(format!(
                    "(assert (= b{} (bvxor b{} c{})))",
                    self.ib + 1,
                    self.ib,
                    self.ic,
                ));
                self.ib += 1;
            }
            Opcode::Out(v) => {
                self.lines.push(format!(
                    "(assert (= (bvand {} {}) {}))",
                    self.combo(v),
                    Self::hex(0b111),
                    Self::hex(self.tape[self.t]),
                ));
                self.t += 1;
            }
            Opcode::Bdv(v) => {
                self.lines.push(format!(
                    "(assert (= b{} (bvlshr a{} {})))",
                    self.ib + 1,
                    self.ia,
                    self.combo(v)
                ));
                self.ib += 1;
            }
            Opcode::Cdv(v) => {
                self.lines.push(format!(
                    "(assert (= c{} (bvlshr a{} {})))",
                    self.ic + 1,
                    self.ia,
                    self.combo(v)
                ));
                self.ic += 1;
            }
        }
    }

    fn combo(&self, v: Combo) -> String {
        match v {
            Combo::Lit(v) => Self::hex(v),
            Combo::A => format!("a{}", self.ia),
            Combo::B => format!("b{}", self.ib),
            Combo::C => format!("c{}", self.ic),
        }
    }
}

pub fn solve(s: &str) -> (String, u64) {
    let vm = Vm::new(s);
    let out = vm.clone().run_to_completion();
    let mut s = String::new();
    for i in out {
        s += &format!("{i},");
    }
    s.pop();

    let mut w = Writer {
        ia: 0,
        ib: 0,
        ic: 0,
        t: 0,
        ip: 0,
        tape: &vm.tape,
        lines: vec![],
    };
    w.run();
    let mut smt = String::new();
    for i in 0..=w.ia {
        writeln!(&mut smt, "(declare-const a{i} (_ BitVec 64))").unwrap();
    }
    for i in 0..=w.ib {
        writeln!(&mut smt, "(declare-const b{i} (_ BitVec 64))").unwrap();
    }
    for i in 0..=w.ic {
        writeln!(&mut smt, "(declare-const c{i} (_ BitVec 64))").unwrap();
    }
    for line in w.lines {
        writeln!(&mut smt, "{line}").unwrap();
    }
    indoc::writedoc!(
        &mut smt,
        "
            (assert (= b0 {}))
            (assert (= c0 {}))
            (minimize a0)
            (check-sat)
            (eval a0)
        ",
        Writer::hex(vm.b),
        Writer::hex(vm.c),
    )
    .unwrap();

    use std::process::{Command, Stdio};
    let mut z3 = Command::new("z3")
        .arg("-in")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to call `z3`; is it installed?");

    let mut stdin = z3.stdin.take().expect("Failed to open stdin");
    stdin
        .write_all(smt.as_bytes())
        .expect("failed to write to z3");
    drop(stdin);
    let output = z3.wait_with_output().expect("Failed to read stdout");
    let out = String::from_utf8(output.stdout).unwrap();
    let Some(v) = out
        .lines()
        .nth(1)
        .and_then(|i| u64::from_str_radix(&i[2..], 16).ok())
    else {
        panic!("z3 failed:\n{}", out);
    };

    (s, v)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const EXAMPLE: &str = indoc::indoc! {"
            Register A: 729
            Register B: 0
            Register C: 0

            Program: 0,1,5,4,3,0
        "};

        let mut vm = Vm::new(EXAMPLE);
        let out = vm.run_to_completion();
        assert_eq!(out, [4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);

        const QUINE: &str = indoc::indoc! {"
            Register A: 2024
            Register B: 0
            Register C: 0

            Program: 0,3,5,4,3,0
        "};
        assert_eq!(solve(QUINE).1, 117440);
    }
}
