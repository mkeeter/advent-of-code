use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{map, recognize},
    multi::many1,
    sequence::{delimited, separated_pair},
    IResult,
};
use std::io::BufRead;

fn parse_num(input: &str) -> IResult<&str, u8> {
    map(recognize(many1(one_of("0123456789"))), |s: &str| {
        s.parse().unwrap()
    })(input)
}
fn parse_pair(input: &str) -> IResult<&str, (Number, Number)> {
    delimited(
        tag("["),
        separated_pair(parse_value, tag(","), parse_value),
        tag("]"),
    )(input)
}
fn parse_value(input: &str) -> IResult<&str, Number> {
    alt((
        map(parse_num, Number::Value),
        map(parse_pair, |(a, b)| Number::Pair(Box::new((a, b)))),
    ))(input)
}

#[derive(Clone, Debug)]
enum Number {
    Value(u8),
    Pair(Box<(Number, Number)>),
}
impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Value(u) => write!(f, "{}", u),
            Number::Pair(p) => write!(f, "[{},{}]", p.0, p.1),
        }
    }
}

enum Output {
    None,
    Done,
    Explode(u8, u8),
    AddLeft(u8),
    AddRight(u8),
}
impl Number {
    fn simplify(&mut self) {
        loop {
            while !matches!(self.simplify_explode(0), Output::None) {}
            if !self.simplify_split() {
                break;
            }
        }
    }
    fn simplify_split(&mut self) -> bool {
        match self {
            Number::Value(v) => {
                if *v >= 10 {
                    *self = Number::Pair(Box::new((
                        Number::Value(*v / 2),
                        Number::Value((*v + 1) / 2),
                    )));
                    true
                } else {
                    false
                }
            }
            Number::Pair(p) => p.0.simplify_split() || p.1.simplify_split(),
        }
    }
    fn value(&self) -> Option<u8> {
        if let Number::Value(a) = self {
            Some(*a)
        } else {
            None
        }
    }
    fn simplify_explode(&mut self, depth: u8) -> Output {
        match self {
            Number::Value(_) => return Output::None,
            Number::Pair(p) => {
                if depth >= 4 {
                    let a = p.0.value().unwrap();
                    let b = p.1.value().unwrap();
                    *self = Number::Value(0);
                    return Output::Explode(a, b);
                }
                match p.0.simplify_explode(depth + 1) {
                    Output::None => (),
                    Output::Explode(a, b) => {
                        p.1.add_left(b);
                        return Output::AddLeft(a);
                    }
                    Output::AddRight(a) => {
                        p.1.add_left(a);
                        return Output::Done;
                    }
                    Output::Done => return Output::Done,
                    Output::AddLeft(a) => return Output::AddLeft(a),
                };
                match p.1.simplify_explode(depth + 1) {
                    Output::None => (),
                    Output::Explode(a, b) => {
                        p.0.add_right(a);
                        return Output::AddRight(b);
                    }
                    Output::AddLeft(a) => {
                        p.0.add_right(a);
                        return Output::Done;
                    }
                    Output::Done => return Output::Done,
                    Output::AddRight(a) => return Output::AddRight(a),
                };
            }
        }
        Output::None
    }

    fn add_left(&mut self, a: u8) {
        match self {
            Number::Value(v) => *v += a,
            Number::Pair(p) => p.0.add_left(a),
        }
    }
    fn add_right(&mut self, a: u8) {
        match self {
            Number::Value(v) => *v += a,
            Number::Pair(p) => p.1.add_right(a),
        }
    }
    fn magnitude(&self) -> usize {
        match self {
            Number::Value(v) => *v as usize,
            Number::Pair(p) => 3 * p.0.magnitude() + 2 * p.1.magnitude(),
        }
    }
}
impl std::ops::Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut sum = Number::Pair(Box::new((self, other)));
        sum.simplify();
        sum
    }
}

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(|line| parse_value(&line.unwrap()).unwrap().1)
        .collect::<Vec<Number>>();

    let out = lines[1..]
        .iter()
        .fold(lines[0].clone(), |a, b| a + b.clone());
    println!("Part 1: {}", out.magnitude());

    let best = lines
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            lines
                .iter()
                .enumerate()
                .filter(move |(j, _)| *j != i)
                .map(move |(_, b)| (a.clone() + b.clone()).magnitude())
        })
        .max()
        .unwrap();
    println!("Part 2: {}", best);
}
