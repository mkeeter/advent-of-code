use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug)]
enum Action {
    CutPos(i64),
    CutNeg(i64),
    DealIncrement(i64),
    DealStack(),
}

impl Action {
    fn new(s: &str) -> Action {
        let i = s.split(' ')
            .filter_map(|i| i64::from_str(i).ok())
            .collect::<Vec<i64>>();
        if s.starts_with("cut") {
            if i[0] < 0 {
                Action::CutNeg(-i[0])
            } else {
                Action::CutPos(i[0])
            }
        } else if s.starts_with("deal with increment") {
            Action::DealIncrement(i[0])
        } else if s == "deal into new stack" {
            Action::DealStack()
        } else {
            panic!("Invalid line: {}", s);
        }
    }

    // returns A, B such that A*c + B is the new position
    fn math(&self, deck_size: i64) -> (i64, i64) {
        match self {
            Action::CutPos(i) => {
                (1, -*i)
            },
            Action::CutNeg(i) => {
                (1, deck_size + *i)
            }
            Action::DealIncrement(i) => {
                (*i, 0)
            }
            Action::DealStack() => {
                (-1, deck_size - 1)
            }
        }
    }
}

fn fuse((a, b): (i64, i64), (c, d): (i64, i64), deck_size: i64) -> (i64, i64) {
    let mut a = c as i128 * a as i128;
    let mut b = c as i128 * b as i128 + d as i128;
    while a < 0 {
        a += deck_size as i128;
    }
    while b < 0 {
        b += deck_size as i128;
    }
    ((a % deck_size as i128) as i64, (b % deck_size as i128) as i64)
}

fn main() {
    let actions = std::io::stdin().lock()
        .lines()
        .map(|line| Action::new(&line.unwrap()))
        .collect::<Vec<Action>>();

    let build = |deck_size: i64| {
        actions.iter()
            .fold((1, 0), |acc, action|
                  fuse(acc, action.math(deck_size), deck_size))
    };

    let deck_size: i64 = 10007;
    let t = build(deck_size);
    println!("Part 1: {}", (t.0 * 2019 + t.1) % deck_size);

    ////////////////////////////////////////////////////////////////////////////////

    let deck_size: i64 = 119315717514047;
    let num_passes: i64 = 101741582076661;
    let mut t = build(deck_size);

    let mut i = 1;
    let mut v = Vec::new();
    while i <= num_passes {
        v.push(t.clone());
        i *= 2;
        t = fuse(t, t, deck_size);
    }

    let mut t = (1, 0);
    let mut total: usize = 0;
    for (i, p) in v.iter().enumerate() {
        if (num_passes & (1 << i)) != 0 {
            t = fuse(t, *p, deck_size);
            total += (1 << i);
        }
    }
    println!("Part 2: Ask Wolfram Alpha to solve ({} * i + {}) % {} = 0", t.0, t.1 - 2020, deck_size);
}
