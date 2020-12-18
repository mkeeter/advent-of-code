use std::io::BufRead;

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
        println!("Got token {:?}, acc {:?}, op {:?}", token, acc, op);
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
    println!("Returning {:?}", acc);
    acc.unwrap()
}

fn main() {
    let input: Vec<Vec<Token>> = std::io::stdin().lock().lines()
        .map(|line| tokenize(&line.unwrap()).collect())
        .collect();

    let out: i64 = input.iter().map(|line| eval(&mut line.iter().copied())).sum();
    println!("Part 1: {}", out);
}
