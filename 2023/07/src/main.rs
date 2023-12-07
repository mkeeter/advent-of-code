use anyhow::Result;
use std::io::BufRead;

/// Structure for a single hand, with field order chosen to sort correctly
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    /// Score for the hand
    score: u8,
    /// Hand, converted to a base-13 number
    cards: u32,
    /// Bid (provided in the input)
    bid: u64,
}

fn score_hand(seen: [u8; 13]) -> u8 {
    let mut seen_count = [0u8; 5];
    for s in seen {
        if s > 0 {
            seen_count[s as usize - 1] += 1;
        }
    }
    match seen_count {
        [5, 0, 0, 0, 0] => 0, // high card
        [3, 1, 0, 0, 0] => 1, // pair
        [1, 2, 0, 0, 0] => 2, // two pair
        [2, 0, 1, 0, 0] => 3, // three of a kind
        [0, 1, 1, 0, 0] => 4, // full house
        [1, 0, 0, 1, 0] => 5, // four of a kind
        [0, 0, 0, 0, 1] => 6, // four of a kind
        _ => panic!("oh no"),
    }
}

fn score_wild(seen: [u8; 13]) -> u8 {
    // Jokers are at index 0 in the unsorted list
    if seen[0] > 0 {
        let mut best = 0;
        for i in 1..13 {
            let mut hand = seen;
            hand[0] -= 1;
            hand[i] += 1;
            let s = score_wild(hand);
            if s > best {
                best = s;
                if best == 6 {
                    return best;
                }
            }
        }
        best
    } else {
        score_hand(seen)
    }
}

fn main() -> Result<()> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<String>, _>>()?;

    let mut hands = vec![];
    for line in &lines {
        let mut iter = line.split_whitespace();
        let mut cards = 0;
        let mut seen = [0u8; 13];
        for c in iter.next().unwrap().chars() {
            let i = match c {
                'A' => 12,
                'K' => 11,
                'Q' => 10,
                'J' => 9,
                'T' => 8,
                i => i.to_digit(10).unwrap() - 2,
            };
            seen[i as usize] += 1;
            cards = cards * 13 + i;
        }
        let score = score_hand(seen);
        let bid = iter.next().unwrap().parse::<u64>().unwrap();
        hands.push(Hand { cards, score, bid });
    }

    let winnings = |mut hands: Vec<Hand>| {
        hands.sort();
        hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i as u64 + 1) * h.bid)
            .sum::<u64>()
    };
    println!("Part 1: {}", winnings(hands));

    let mut hands = vec![];
    for line in &lines {
        let mut iter = line.split_whitespace();
        let mut cards = 0;
        let mut seen = [0u8; 13];
        for c in iter.next().unwrap().chars() {
            let i = match c {
                'A' => 12,
                'K' => 11,
                'Q' => 10,
                'T' => 9,
                'J' => 0,
                i => i.to_digit(10).unwrap() - 1,
            };
            seen[i as usize] += 1;
            cards = cards * 13 + i;
        }
        let score = score_wild(seen);
        let bid = iter.next().unwrap().parse::<u64>().unwrap();
        hands.push(Hand { cards, score, bid });
    }
    println!("Part 2: {}", winnings(hands));

    Ok(())
}
