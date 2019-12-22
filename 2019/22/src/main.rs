use std::io::BufRead;
use std::str::FromStr;
use std::collections::VecDeque;

#[derive(Debug)]
enum Action {
    CutPos(usize),
    CutNeg(usize),
    DealIncrement(usize),
    DealStack(),
}

impl Action {
    fn new(s: &str) -> Action {
        let i = s.split(' ')
            .filter_map(|i| i32::from_str(i).ok())
            .collect::<Vec<i32>>();
        if s.starts_with("cut") {
            if i[0] < 0 {
                Action::CutNeg(-i[0] as usize)
            } else {
                Action::CutPos(i[0] as usize)
            }
        } else if s.starts_with("deal with increment") {
            Action::DealIncrement(i[0] as usize)
        } else if s == "deal into new stack" {
            Action::DealStack()
        } else {
            panic!("Invalid line: {}", s);
        }
    }

    fn index(&self, c: usize, deck_size: usize) -> usize {
        match self {
            Action::CutPos(i) => {
                (c + i) % deck_size
            },
            Action::CutNeg(i) => {
                (c + deck_size - i) % deck_size
            }
            Action::DealIncrement(i) => {
                (0..).filter(|j| (c + (j * deck_size)) % *i == 0)
                    .map(|j| (c + (j * deck_size)) / *i)
                    .nth(0)
                    .unwrap()
            }
            Action::DealStack() => {
                deck_size - c - 1
            }
        }
    }
}

fn main() {
    let actions = std::io::stdin().lock()
        .lines()
        .map(|line| Action::new(&line.unwrap()))
        .collect::<Vec<Action>>();

    let deck_size = 10007;
    let mut cards = (0..deck_size).collect::<Vec<usize>>();
    let mut next = cards.clone();
    for a in actions.iter() {
        for i in 0..deck_size {
            next[i] = cards[a.index(i, deck_size)];
        }
        std::mem::swap(&mut next, &mut cards);
    }
    for i in 0..deck_size {
        if cards[i] == 2019 {
            println!("Part 1: {}", i);
            break;
        }
    }

    ////////////////////////////////////////////////////////////////////////////////

    let deck_size: usize = 119315717514047;
    let mut c: usize = 2020;
    for a in actions.iter() {
    }
}
