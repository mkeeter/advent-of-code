use std::io::BufRead;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum Value {
    Lit(i32),
    Reg(usize)
}

impl Value {
    fn reg(self) -> usize {
        match self {
            Value::Lit(_) => panic!("Cannot get reg from lit"),
            Value::Reg(c) => c,
        }
    }
    fn lit(self) -> i32 {
        match self {
            Value::Lit(i) => i,
            Value::Reg(_) => panic!("Cannot get lit from reg"),
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

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
enum Instruction {
    cpy(Value, Value),
    inc(Value),
    dec(Value),
    jnz(Value, Value),
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
            _ => Err(()),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

fn run(input: &[Instruction], mut regs: [i32; 4]) -> i32 {
    let mut ip: i32 = 0;
    while let Some(i) = input.get(ip as usize) {
        match i {
            cpy(a, b) => {
                regs[b.reg()] = match a {
                    Value::Lit(i) => *i,
                    Value::Reg(r) => regs[*r]
                };
                ip += 1;
            },

            inc(a) => {
                regs[a.reg()] += 1;
                ip += 1;
            }
            dec(a) => {
                regs[a.reg()] -= 1;
                ip += 1;
            }
            jnz(a, b) => {
                let v = match a {
                    Value::Lit(i) => *i,
                    Value::Reg(r) => regs[*r]
                };
                ip += if v != 0 {
                    b.lit()
                } else {
                    1
                }
            }
        }
    }
    regs[0]
}

fn main() {
    let input = std::io::stdin().lock()
        .lines()
        .filter_map(|line| Instruction::from_str(&line.unwrap()).ok())
        .collect::<Vec<_>>();

    println!("Part 1: {}", run(&input, [0, 0, 0, 0]));
    println!("Part 2: {}", run(&input, [0, 0, 1, 0]));
}
