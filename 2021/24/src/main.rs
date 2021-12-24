use std::collections::HashSet;
use std::io::BufRead;

#[derive(Clone, Eq, PartialEq)]
enum Value {
    Invalid,
    Register(u8),
    Constant(i64),
    Input(u8),
    Add(Box<Value>, Box<Value>),
    Mul(Box<Value>, Box<Value>),
    Div(Box<Value>, Box<Value>),
    Mod(Box<Value>, Box<Value>),
    Eql(Box<Value>, Box<Value>),
}
impl Value {}

#[derive(Clone, Eq, PartialEq)]
struct Register {
    value: Value,
    range: (i64, i64),
}
impl Register {
    fn is_constant(&self) -> bool {
        matches!(self.value, Value::Constant(_))
    }
}
impl std::fmt::Debug for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?} [{}, {}]", self.value, self.range.0, self.range.1)
    }
}
impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Value::Invalid => panic!("Can't print Invalid"),
            Value::Constant(c) => write!(f, "{}", c),
            Value::Input(u) => write!(f, "Input({})", u),
            Value::Register(u) => write!(f, "Register({})", u),
            Value::Add(a, b) => write!(f, "({:?} + {:?})", a, b),
            Value::Mul(a, b) => write!(f, "({:?} * {:?})", a, b),
            Value::Div(a, b) => write!(f, "({:?} / {:?})", a, b),
            Value::Mod(a, b) => write!(f, "({:?} % {:?})", a, b),
            Value::Eql(a, b) => write!(f, "({:?} == {:?})", a, b),
        }
    }
}

#[derive(Debug)]
struct Vm {
    registers: [Register; 4],
    index: u8,

    /// Input range for the current block
    input: [(i64, i64); 4],
}

#[derive(Debug)]
struct Block {
    input: [(i64, i64); 4],
    output: [Register; 4],
}

impl Vm {
    fn new() -> Self {
        let r = Register {
            value: Value::Constant(0),
            range: (0, 0),
        };
        let mut out = Self {
            registers: [r.clone(), r.clone(), r.clone(), r],
            index: 0,
            input: [(0, 0); 4],
        };
        out.reset();
        out
    }
    fn reset(&mut self) {
        for i in 0..4 {
            self.registers[i].value = Value::Register(i as u8);
            self.input[i] = self.registers[i].range;
        }
    }
    fn reg(&mut self, v: &str) -> &mut Register {
        match v {
            "x" => &mut self.registers[0],
            "y" => &mut self.registers[1],
            "z" => &mut self.registers[2],
            "w" => &mut self.registers[3],
            r => panic!("Invalid register {}", r),
        }
    }
    fn take_reg(&mut self, v: &str) -> Register {
        let mut out = Register {
            value: Value::Invalid,
            range: (0, 0),
        };
        std::mem::swap(&mut out, self.reg(v));
        out
    }
    fn reg_or_value(&mut self, v: &str) -> Register {
        match v {
            "x" => self.registers[0].clone(),
            "y" => self.registers[1].clone(),
            "z" => self.registers[2].clone(),
            "w" => self.registers[3].clone(),
            i => {
                let i = i.parse().unwrap();
                Register {
                    value: Value::Constant(i as i64),
                    range: (i, i),
                }
            }
        }
    }
    fn exec(&mut self, line: &str) {
        let mut words = line.split(' ');
        match words.next().unwrap() {
            "inp" => {
                self.reset();
                *self.reg(words.next().unwrap()) = Register {
                    value: Value::Input(self.index),
                    range: (1, 9),
                };
                self.index += 1;
            }
            "add" => {
                let reg_a = words.next().unwrap();
                let a = self.take_reg(reg_a);
                let b = self.reg_or_value(words.next().unwrap());
                *self.reg(reg_a) = if a.range == (0, 0) {
                    b
                } else if b.range == (0, 0) {
                    a
                } else {
                    Register {
                        value: Value::Add(Box::new(a.value), Box::new(b.value)),
                        range: (a.range.0 + b.range.0, a.range.1 + b.range.1),
                    }
                }
            }
            "mul" => {
                let reg_a = words.next().unwrap();
                let a = self.take_reg(reg_a);
                let b = self.reg_or_value(words.next().unwrap());
                *self.reg(reg_a) = if b.range == (1, 1) {
                    a
                } else if b.range == (0, 0) || a.range == (0, 0) {
                    Register {
                        value: Value::Constant(0),
                        range: (0, 0),
                    }
                } else {
                    let range = [
                        a.range.0 * b.range.0,
                        a.range.0 * b.range.1,
                        a.range.1 * b.range.0,
                        a.range.0 * b.range.1,
                    ];
                    Register {
                        value: Value::Mul(Box::new(a.value), Box::new(b.value)),
                        range: (*range.iter().min().unwrap(), *range.iter().max().unwrap()),
                    }
                }
            }
            "div" => {
                let reg_a = words.next().unwrap();
                let a = self.take_reg(reg_a);
                let b = self.reg_or_value(words.next().unwrap());
                assert!(b.range.0 == b.range.1);
                assert!(b.range.0 > 0);
                *self.reg(reg_a) = if b.range == (1, 1) {
                    a
                } else {
                    Register {
                        value: Value::Div(Box::new(a.value), Box::new(b.value)),
                        range: (a.range.0 / b.range.0, a.range.1 / b.range.1),
                    }
                }
            }
            "mod" => {
                let reg_a = words.next().unwrap();
                let a = self.take_reg(reg_a);
                let b = self.reg_or_value(words.next().unwrap());
                assert!(b.range.0 == b.range.1);
                assert!(b.range.0 > 0);
                assert!(a.range.0 >= 0);
                *self.reg(reg_a) = Register {
                    value: Value::Mod(Box::new(a.value), Box::new(b.value)),
                    range: (0, a.range.1.min(b.range.1)),
                };
            }
            "eql" => {
                let reg_a = words.next().unwrap();
                let a = self.take_reg(reg_a);
                let b = self.reg_or_value(words.next().unwrap());
                *self.reg(reg_a) = if a.range.1 < b.range.0 || a.range.0 > b.range.1 {
                    Register {
                        value: Value::Constant(0),
                        range: (0, 0),
                    }
                } else if a.is_constant() && b.is_constant() && a.range == b.range {
                    Register {
                        value: Value::Constant(1),
                        range: (1, 1),
                    }
                } else {
                    Register {
                        value: Value::Eql(Box::new(a.value), Box::new(b.value)),
                        range: (0, 1),
                    }
                }
            }
            _ => panic!("Invalid instruction {}", line),
        }
        for r in &mut self.registers {
            assert!(r.range.0 <= r.range.1);
            if r.is_constant() {
                assert!(r.range.0 == r.range.1);
            }
            if r.range.0 == r.range.1 {
                r.value = Value::Constant(r.range.0);
            }
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/prog.rs"));

fn main() {
    let mut vm = Vm::new();
    let stdin = std::io::stdin();
    let mut iter = stdin.lock().lines().peekable();
    let mut blocks = vec![];
    while let Some(line) = iter.next() {
        let line = line.unwrap();
        vm.exec(&line);
        println!("{} => {:#?}", line, vm);
        let next = iter.peek();
        if next.is_none() || next.as_ref().unwrap().as_ref().unwrap().starts_with("inp") {
            blocks.push(Block {
                input: vm.input,
                output: vm.registers.clone(),
            });
        }
    }
    println!("{:?}", vm.registers[2]);
    for b in &blocks {
        println!("{:?}", b);
    }
    let last: &Block = blocks.last().unwrap();
    let inputs = (last.input[0].0..=last.input[0].1).flat_map(move |x| {
        (last.input[1].0..=last.input[1].1).flat_map(move |y| {
            (last.input[2].0..=last.input[2].1).flat_map(move |z| {
                (last.input[3].0..=last.input[3].1)
                    .flat_map(move |w| (1..=9).map(move |i| (x, y, z, w, i)))
            })
        })
    });
}
