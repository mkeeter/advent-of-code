use std::collections::{HashSet, VecDeque};
use std::io::Read;
use std::cmp::Ordering;

#[derive(Debug)]
enum Winner { PlayerA, PlayerB }
type Deck = VecDeque<u8>;

fn score(deck: Deck) -> u32 {
    deck.into_iter()
        .rev()
        .zip(1..)
        .map(|(i, c)| i as u32 * c)
        .sum::<u32>()
}

fn play<F>(mut deck_a: Deck, mut deck_b: Deck, check: F) -> (Winner, u32)
    where F: Fn(u8, u8, &Deck, &Deck) -> Winner
{
    let mut seen: HashSet<(Deck, Deck)> = HashSet::new();

    while !deck_a.is_empty() && !deck_b.is_empty() {
        // Break the infinite loop
        if !seen.insert((deck_a.clone(), deck_b.clone())) {
            return (Winner::PlayerA, score(deck_a));
        }

        let a = deck_a.pop_front().unwrap();
        let b = deck_b.pop_front().unwrap();
        match check(a, b, &deck_a, &deck_b) {
            Winner::PlayerA => {
                deck_a.push_back(a);
                deck_a.push_back(b);
            },
            Winner::PlayerB => {
                deck_b.push_back(b);
                deck_b.push_back(a);
            },
        }
    }

    if deck_b.is_empty() {
        (Winner::PlayerA, score(deck_a))
    } else {
        (Winner::PlayerB, score(deck_b))
    }
}

fn check_normal(a: u8, b: u8, _deck_a: &Deck, _deck_b: &Deck) -> Winner {
    match a.cmp(&b) {
        Ordering::Less => Winner::PlayerB,
        Ordering::Greater => Winner::PlayerA,
        _ => panic!("Invalid comparison; cards must be unique"),
    }
}

fn check_recursive(a: u8, b: u8, deck_a: &Deck, deck_b: &Deck) -> Winner {
    if deck_a.len() >= a as usize && deck_b.len() >= b as usize {
        let deck_a = deck_a.iter().take(a as usize).copied().collect();
        let deck_b = deck_b.iter().take(b as usize).copied().collect();
        play(deck_a, deck_b, check_recursive).0
    } else {
        check_normal(a, b, deck_a, deck_b)
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let decks = input.split("\n\n")
        .map(|deck| deck.lines()
            .skip(1)
            .map(|i| i.parse().unwrap())
            .collect::<Deck>())
        .collect::<Vec<_>>();

    let p1 = play(decks[0].clone(), decks[1].clone(), check_normal).1;
    println!("Part 1: {}", p1);

    let p2 = play(decks[0].clone(), decks[1].clone(), check_recursive).1;
    println!("Part 2: {}", p2);
}
