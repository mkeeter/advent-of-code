use std::io::BufRead;
use std::str::FromStr;
use std::collections::VecDeque;

#[derive(Debug)]
enum Action {
    Cut(i32),
    DealIncrement(i32),
    DealStack(),
}

impl Action {
    fn new(s: &str) -> Action {
        let i = s.split(' ')
            .filter_map(|i| i32::from_str(i).ok())
            .collect::<Vec<i32>>();
        if s.starts_with("cut") {
            Action::Cut(i[0])
        } else if s.starts_with("deal with increment") {
            Action::DealIncrement(i[0])
        } else if s == "deal into new stack" {
            Action::DealStack()
        } else {
            panic!("Invalid line: {}", s);
        }
    }
}

fn main() {
    let actions = std::io::stdin().lock()
        .lines()
        .map(|line| Action::new(&line.unwrap()))
        .collect::<Vec<Action>>();

    let deck_size = 10007;
    let mut cards = (0..deck_size).collect::<VecDeque<usize>>();
    for a in actions {
        match a {
            Action::Cut(i) => {
                if i > 0 {
                    cards.rotate_left(i as usize);
                } else {
                    cards.rotate_right(-i as usize);
                }
            },
            Action::DealIncrement(i) => {
                let mut tmp = vec![0; deck_size];
                for j in 0..deck_size {
                    tmp[(j * i as usize) % deck_size] = cards[j];
                }
                cards = tmp.into_iter().collect();
            },
            Action::DealStack() => {
                // Reverse the deck order
                for j in 0..cards.len() / 2 {
                    cards.swap(j, cards.len() - j - 1)
                }
            },
        }
    }
    for i in 0..deck_size {
        if cards[i] == 2019 {
            println!("Part 1: {}", i);
            break;
        }
    }
}
