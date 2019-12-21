use std::collections::{HashMap, VecDeque};
use std::io::BufRead;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum Arg {
    Lit(u16),
    Ref(String),
}

#[derive(Clone, Debug)]
enum Op {
    Not(Arg),
    Or(Arg, Arg),
    And(Arg, Arg),
    RShift(Arg, Arg),
    LShift(Arg, Arg),
    Dup(Arg),
}

#[derive(Clone, Debug)]
struct Gate(Op, String);

fn arg(s: &str) -> Arg {
    if let Ok(i) = u16::from_str(s) {
        Arg::Lit(i)
    } else {
        Arg::Ref(s.to_owned())
    }
}

fn parse(s: &str) -> Gate {
    let words = s.split(' ').collect::<Vec<_>>();
    let op = if words[0] == "NOT" {
        Op::Not(arg(words[1]))
    } else if words[1] == "OR" {
        Op::Or(arg(words[0]), arg(words[2]))
    } else if words[1] == "AND" {
        Op::And(arg(words[0]), arg(words[2]))
    } else if words[1] == "RSHIFT" {
        Op::RShift(arg(words[0]), arg(words[2]))
    } else if words[1] == "LSHIFT" {
        Op::LShift(arg(words[0]), arg(words[2]))
    } else {
        Op::Dup(arg(words[0]))
    };
    Gate(op, words.last().unwrap().to_string())
}

fn run(gates: &[Gate]) -> u16 {
    let mut gates = gates.iter().cloned().collect::<VecDeque<_>>();
    let mut state: HashMap<String, u16> = HashMap::new();
    while let Some(Gate(op, out)) = gates.pop_front() {
        let get = |arg: &Arg| match arg {
            Arg::Lit(i) => Some(*i),
            Arg::Ref(s) => state.get(s).copied(),
        };
        if let Some(z) = match &op {
            Op::Not(a) => get(a).map(|i| !i),
            Op::Or(a,  b) => get(a).and_then(|i| get(b).map(|j| i | j)),
            Op::And(a, b) => get(a).and_then(|i| get(b).map(|j| i & j)),
            Op::RShift(a, b) => get(a).and_then(|i| get(b).map(|j| i >> j)),
            Op::LShift(a, b) => get(a).and_then(|i| get(b).map(|j| i << j)),
            Op::Dup(a) => get(a),
        }{
            state.insert(out, z);
        } else {
            gates.push_back(Gate(op, out));
        }
    }
    *state.get("a").unwrap()
}


fn main() {
    let gates = std::io::stdin().lock()
        .lines()
        .map(|line| parse(&line.unwrap()))
        .collect::<Vec<Gate>>();

    let a = run(&gates[..]);
    println!("Part 1: {}", a);

    let mut gates = gates;
    for Gate(op, out) in gates.iter_mut() {
        if out == "b" {
            *op = Op::Dup(Arg::Lit(a));
        }
    }
    println!("Part 2: {}", run(&gates[..]));
}
