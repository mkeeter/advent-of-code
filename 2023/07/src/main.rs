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

/// Converts from card count to group count
fn count(seen: [u8; 13]) -> [u8; 5] {
    let mut seen_count = [0u8; 5];
    for s in seen {
        if s > 0 {
            seen_count[s as usize - 1] += 1;
        }
    }
    seen_count
}

fn score(count: [u8; 5]) -> u8 {
    match count {
        [5, 0, 0, 0, 0] => 0, // high card
        [3, 1, 0, 0, 0] => 1, // pair
        [1, 2, 0, 0, 0] => 2, // two pair
        [2, 0, 1, 0, 0] => 3, // three of a kind
        [0, 1, 1, 0, 0] => 4, // full house
        [1, 0, 0, 1, 0] => 5, // four of a kind
        [0, 0, 0, 0, 1] => 6, // four of a kind
        _ => panic!("oh no: {:?}", count),
    }
}

fn score_wild(jokers: u8, mut count: [u8; 5]) -> u8 {
    if jokers > 0 {
        // Upgrade the highest possible group (e.g. high card -> pair)
        if let Some(i) = (0..5).rev().find(|i| count[*i] > 0) {
            count[i] -= 1;
            count[i + jokers as usize] += 1;
        } else {
            count[jokers as usize - 1] += 1;
        }
    }
    score(count)
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
                c => c.to_digit(10).unwrap() - 2,
            };
            seen[i as usize] += 1;
            cards = cards * 13 + i;
        }
        let score = score(count(seen));
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
                c => c.to_digit(10).unwrap() - 1,
            };
            seen[i as usize] += 1;
            cards = cards * 13 + i;
        }
        let jokers = std::mem::take(&mut seen[0]);
        let score = score_wild(jokers, count(seen));
        let bid = iter.next().unwrap().parse::<u64>().unwrap();
        hands.push(Hand { cards, score, bid });
    }
    println!("Part 2: {}", winnings(hands));

    Ok(())
}
