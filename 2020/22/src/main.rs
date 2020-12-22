use std::collections::VecDeque;
use std::io::Read;
use std::cmp::Ordering;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let decks = input.split("\n\n")
        .map(|deck| deck.lines()
            .skip(1)
            .map(|i| i.parse().unwrap())
            .collect::<VecDeque<i64>>())
        .collect::<Vec<_>>();

    let mut deck_a = decks[0].clone();
    let mut deck_b = decks[1].clone();

    while !deck_a.is_empty() && !deck_b.is_empty() {
        let a = deck_a.pop_front().unwrap();
        let b = deck_b.pop_front().unwrap();
        match a.cmp(&b) {
            Ordering::Less => {
                deck_b.push_back(b);
                deck_b.push_back(a);
            },
            Ordering::Greater => {
                deck_a.push_back(a);
                deck_a.push_back(b);
            },
            _ => panic!("Invalid ordering, cards must be unique"),
        }
    }

    let score = deck_a.iter().rev()
        .chain(deck_b.iter().rev())
        .zip(1..)
        .map(|(i, c)| i * c)
        .sum::<i64>();
    println!("Part 1: {}", score);
}
