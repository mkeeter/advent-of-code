use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Value {
    Lit(i32),
    Reg(usize)
}

impl Value {
    fn reg(self) -> Option<usize> {
        match self {
            Value::Lit(_) => None,
            Value::Reg(c) => Some(c),
        }
    }
}

impl FromStr for Value {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        if let Ok(i) = i32::from_str(s) {
            Ok(Self::Lit(i))
        } else if s.len() == 1 {
            let c = s.chars().nth(0).unwrap();
            if c >= 'a' && c <= 'd' {
                Ok(Self::Reg((c as u32 - b'a' as u32) as usize))
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[allow(non_camel_case_types)]
enum Instruction {
    cpy(Value, Value),
    inc(Value),
    dec(Value),
    jnz(Value, Value),
    tgl(Value),
    out(Value),
}
use Instruction::*;

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut itr = s.split(' ');
        let op = itr.next();
        if op.is_none() {
            return Err(());
        }

        let op = op.unwrap();
        let args = itr.filter_map(|v| Value::from_str(v).ok())
            .collect::<Vec<_>>();
        match op {
            "cpy" => Ok(cpy(args[0], args[1])),
            "inc" => Ok(inc(args[0])),
            "dec" => Ok(dec(args[0])),
            "jnz" => Ok(jnz(args[0], args[1])),
            "tgl" => Ok(tgl(args[0])),
            "out" => Ok(out(args[0])),
            _ => Err(()),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Vm {
    instructions: Vec<Instruction>,
    ip: i32,
    pub regs: [i32; 4],
}

impl FromStr for Vm {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let instructions = s.lines()
            .filter_map(|line| Instruction::from_str(&line).ok())
            .collect::<Vec<_>>();
        Ok(Vm { instructions, ip: 0, regs: [0; 4] })
    }
}

impl Vm {
    fn get(&self, v: Value) -> i32 {
        match v {
            Value::Lit(i) => i,
            Value::Reg(r) => self.regs[r]
        }
    }

    fn delta(&self, start: i32, end: i32) -> Option<[i32; 4]> {
        let mut d = [0; 4];
        println!("JIT target:");
        for i in start..=end {
            println!("   {:?}", self.instructions[i as usize]);
        }
        for i in (start..end).rev() {
            match self.instructions[i as usize] {
                inc(a) => a.reg().map(|a| d[a] += 1),
                dec(a) => a.reg().map(|a| d[a] -= 1),
                _ => return None,
            };
        }
        Some(d)
    }

    // This will optimize out simple loops like
    //      inc(Reg(0))
    //      dec(Reg(2))
    //      jnz(Reg(2), Lit(-2))
    fn jit(&mut self, start: i32, end: i32) -> bool {
        if let Some(d) = self.delta(start, end) {
            let r = match self.instructions[end as usize] {
                jnz(a, _) => match a.reg() {
                    Some(a) => a,
                    _ => return false,
                },
                _ => panic!(),
            };
            if d[r] == 0 {
                return false;
            }
            let v = self.regs[r];
            let iterations = (v + -d[r] - 1) / -d[r];
            for (i, r) in self.regs.iter_mut().enumerate() {
                *r += iterations * d[i];
            }
            return true;
        } else {
            return false;
        }
    }

    fn apply(&mut self, i: Instruction) -> Option<i32> {
        match i {
            cpy(a, b) => {
                if let Some(b) = b.reg() {
                    self.regs[b] = self.get(a);
                }
                self.ip += 1;
                None
            },
            inc(a) => {
                if let Some(a) = a.reg() {
                    self.regs[a] += 1;
                }
                self.ip += 1;
                None
            }
            dec(a) => {
                if let Some(a) = a.reg() {
                    self.regs[a] -= 1;
                }
                self.ip += 1;
                None
            }
            jnz(a, b) => {
                if self.get(a) != 0 {
                    // This is the jump target
                    let target = self.ip + self.get(b);

                    // If b is a literal value, then try to JIT the loop
                    if b.reg() != None || !self.jit(target, self.ip) {
                        self.ip = target;
                    }
                } else {
                    self.ip += 1;
                };
                None
            }
            tgl(a) => {
                let target = self.ip + self.get(a);
                if target >= 0 && target < self.instructions.len() as i32 {
                    let i = match self.instructions[target as usize] {
                        cpy(a, b) => jnz(a, b),
                        inc(a) => dec(a),
                        dec(a) => inc(a),
                        jnz(a, b) => cpy(a, b),
                        tgl(a) => inc(a),
                        out(a) => inc(a),
                    };
                    self.instructions[target as usize] = i;
                }
                self.ip += 1;
                None
            }
            out(a) => {
                self.ip += 1;
                Some(self.get(a))
            }
        }
    }
    pub fn run(&mut self) {
        while self.ip >= 0 && (self.ip as usize) < self.instructions.len() {
            let i = self.instructions[self.ip as usize];
            self.apply(i);
        }
    }

    pub fn next(&mut self) -> Option<i32> {
        while self.ip >= 0 && (self.ip as usize) < self.instructions.len() {
            let i = self.instructions[self.ip as usize];
            if let Some(o) = self.apply(i) {
                return Some(o);
            }
        }
        None
    }
}
