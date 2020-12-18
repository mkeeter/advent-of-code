use std::io::BufRead;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Token {
    Int(i64),
    ParenLeft,
    ParenRight,
    Add,
    Mul,
}

fn next<I>(iter: &mut std::iter::Peekable<I>) -> Option<Token>
    where I: Iterator<Item=char>
{
    // Eat all whitespace
    while iter.peek() == Some(&' ') {
        iter.next();
    }

    // First, tokenize any numbers
    let mut num = None;
    while let Some(c) = iter.peek() {
        if c.is_numeric() {
            num = Some(num.unwrap_or(0) * 10 + c.to_digit(10).unwrap() as i64);
            iter.next();
        } else {
            break;
        }
    }
    if let Some(n) = num {
        return Some(Token::Int(n));
    }

    while let Some(c) = iter.next() {
        match c {
            '(' =>  return Some(Token::ParenLeft),
            ')' =>  return Some(Token::ParenRight),
            '+' =>  return Some(Token::Add),
            '*' =>  return Some(Token::Mul),
            _ => panic!("Invalid char: {}", c),
        }
    }
    None
}

fn tokenize<'a>(s: &'a str) -> impl Iterator<Item=Token> + 'a {
    let mut iter = s.chars().peekable();
    std::iter::from_fn(move || next(&mut iter))
}

fn eval<I: Iterator<Item=Token>>(iter: &mut I) -> i64 {
    let mut acc = None;
    let mut op = None;

    while let Some(token) = iter.next() {
        let num = match token {
            Token::ParenLeft => Some(eval(iter)),
            Token::ParenRight => return acc.unwrap(),
            Token::Add | Token::Mul => {
                assert!(op == None);
                op = Some(token);
                None
            },
            Token::Int(i) => Some(i),
        };
        if let Some(n) = num {
            if let Some(a) = acc {
                match op.unwrap() {
                    Token::Add => acc = Some(a + n),
                    Token::Mul => acc = Some(a * n),
                    _ => panic!("Invalid op {:?}", op),
                }
                op = None;
            } else {
                assert!(op == None);
                acc = Some(n);
            }
        }
    }
    acc.unwrap()
}

fn eval_sy<I: Iterator<Item=Token>>(iter: &mut I) -> i64 {
    let mut output = VecDeque::new();
    let mut ops = VecDeque::new();


    // Basic shunting-yard parser
    while let Some(token) = iter.next() {
        match token {
            Token::Int(i) => output.push_front(token),
            Token::ParenLeft => ops.push_back(token),
            Token::ParenRight => {
                while *ops.back().unwrap() != Token::ParenLeft {
                    output.push_front(ops.pop_back().unwrap());
                }
                if *ops.back().unwrap() == Token::ParenLeft {
                    ops.pop_back();
                }
            },
            Token::Add => {
                ops.push_back(token);
            },
            Token::Mul => {
                while *ops.back().unwrap_or(&Token::Mul) == Token::Add {
                    output.push_front(ops.pop_back().unwrap());
                }
                ops.push_back(token);
            },
        }
    }
    while let Some(c) = ops.pop_back() {
        output.push_front(c);
    }
    let mut stack = Vec::new();
    while let Some(c) = output.pop_back() {
        match c {
            Token::Add => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            },
            Token::Mul => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            },
            Token::Int(i) => {
                stack.push(i);
            },
            _ => panic!("Invalid op in output: {:?}", c),
        }
    }
    assert!(stack.len() == 1);
    return stack.pop().unwrap();
}

fn main() {
    let input: Vec<Vec<Token>> = std::io::stdin().lock().lines()
        .map(|line| tokenize(&line.unwrap()).collect())
        .collect();

    let out: i64 = input.iter()
        .map(|line| eval(&mut line.iter().copied()))
        .sum();
    println!("Part 1: {}", out);

    let out: i64 = input.iter()
        .map(|line| eval_sy(&mut line.iter().copied()))
        .sum();
    println!("Part 2: {}", out);
}
