use std::io::BufRead;
use std::str::FromStr;

use modinverse::modinverse;

// Represents T.0*x + T.1
type Transform = (i128, i128);

#[derive(Debug)]
enum Action {
    Cut(i128),
    DealIncrement(i128),
    DealStack(),
}

impl Action {
    fn new(s: &str) -> Action {
        let i = s.split(' ')
            .filter_map(|i| i128::from_str(i).ok())
            .next().unwrap_or(0);
        if s.starts_with("cut") {
            Action::Cut(i)
        } else if s.starts_with("deal with increment") {
            Action::DealIncrement(i)
        } else if s == "deal into new stack" {
            Action::DealStack()
        } else {
            panic!("Invalid line: {}", s);
        }
    }

    // returns A, B such that A*i + B is the new position
    fn math(&self, deck_size: i128) -> Transform {
        match self {
            Action::Cut(i) => (1, -*i),
            Action::DealIncrement(i) => (*i, 0),
            Action::DealStack() => (-1, deck_size - 1),
        }
    }
}

fn fuse((a, b): Transform, (c, d): Transform, deck_size: i128) -> Transform {
    let a = c * a;
    let b = c * b + d;
    (a.rem_euclid(deck_size), b.rem_euclid(deck_size))
}

fn main() {
    let actions = std::io::stdin().lock()
        .lines()
        .map(|line| Action::new(&line.unwrap()))
        .collect::<Vec<Action>>();

    let build = |deck_size: i128| {
        actions.iter()
            .fold((1, 0), |acc, action|
                  fuse(acc, action.math(deck_size), deck_size))
    };

    let deck_size: i128 = 10007;
    let t = build(deck_size);
    println!("Part 1: {}", (t.0 * 2019 + t.1) % deck_size);

    ////////////////////////////////////////////////////////////////////////////////

    let deck_size: i128 = 119315717514047;
    let num_passes: i128 = 101741582076661;
    let mut t = build(deck_size);

    let mut i = 1;
    let mut v = Vec::new();
    while i <= num_passes {
        v.push(t.clone());
        i *= 2;
        t = fuse(t, t, deck_size);
    }

    let mut t = (1, 0);
    for (i, p) in v.iter().enumerate() {
        if (num_passes & (1 << i)) != 0 {
            t = fuse(t, *p, deck_size);
        }
    }

    let m = modinverse(t.0, deck_size).unwrap();
    println!("Part 2: {}", ((2020 - t.1) * m).rem_euclid(deck_size));
}
