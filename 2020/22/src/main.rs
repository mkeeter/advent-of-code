use std::collections::{HashSet, VecDeque};
use std::io::Read;
use std::cmp::Ordering;

#[derive(Debug)]
enum Winner { PlayerA, PlayerB }
type Deck = VecDeque<u8>;

fn play_recursive(mut deck_a: Deck, mut deck_b: Deck) -> (Winner, Deck) {
    let mut seen: HashSet<(Deck, Deck)> = HashSet::new();

    while !deck_a.is_empty() && !deck_b.is_empty() {

        if !seen.insert((deck_a.clone(), deck_b.clone())) {
            return (Winner::PlayerA, deck_a);
        }

        let a = deck_a.pop_front().unwrap();
        let b = deck_b.pop_front().unwrap();

        let won = if deck_a.len() >= a as usize && deck_b.len() >= b as usize {
            let deck_a = deck_a.iter().take(a as usize).copied().collect();
            let deck_b = deck_b.iter().take(b as usize).copied().collect();
            play_recursive(deck_a, deck_b).0
        } else {
            match a.cmp(&b) {
                Ordering::Less => Winner::PlayerB,
                Ordering::Greater => Winner::PlayerA,
                _ => panic!("Invalid comparison; cards must be unique"),
            }
        };
        match won {
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
        (Winner::PlayerA, deck_a)
    } else {
        (Winner::PlayerB, deck_b)
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
        .map(|(i, c)| *i as u32 * c)
        .sum::<u32>();
    println!("Part 1: {}", score);

    let deck_a = decks[0].clone();
    let deck_b = decks[1].clone();
    let winner = play_recursive(deck_a, deck_b);
    let score = winner.1.iter().rev()
        .zip(1..)
        .map(|(i, c)| *i as u32 * c)
        .sum::<u32>();
    println!("Part 2: {}", score);
}
