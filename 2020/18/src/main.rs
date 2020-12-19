use std::io::BufRead;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Token {
    Num(i64),
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
        return Some(Token::Num(n));
    }

    iter.next().map(|c|
        match c {
            '(' => Token::ParenLeft,
            ')' => Token::ParenRight,
            '+' => Token::Add,
            '*' => Token::Mul,
            _ => panic!("Invalid char: {}", c),
        })
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
            Token::Num(i) => Some(i),
        };
        // Apply operations as soon as they're available
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
    let mut ops = VecDeque::new();

    let mut stack = Vec::new();
    let mut output = |c| {
        match c {
            Token::Num(i) => stack.push(i),
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
            _ => panic!("Invalid op in output: {:?}", c),
        }
    };

    // Basic shunting-yard parser
    for token in iter {
        match token {
            Token::Num(_) => output(token),
            Token::ParenLeft => ops.push_front(token),
            Token::ParenRight => {
                while *ops.front().unwrap() != Token::ParenLeft {
                    output(ops.pop_front().unwrap());
                }
                ops.pop_front().unwrap(); // Remove matching paren
            },
            Token::Add => ops.push_front(token),
            Token::Mul => {
                while *ops.front().unwrap_or(&Token::Mul) == Token::Add {
                    output(ops.pop_front().unwrap());
                }
                ops.push_front(token);
            },
        }
    }
    ops.into_iter().for_each(output);

    assert!(stack.len() == 1);
    stack.pop().unwrap()
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
