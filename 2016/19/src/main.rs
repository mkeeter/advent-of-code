use std::collections::VecDeque;
use std::str::FromStr;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let count = usize::from_str(input.trim()).unwrap();

    let mut elves = VecDeque::new();
    for i in 0..count {
        elves.push_back((i + 1, 1));
    }
    while elves.len() > 1 {
        let mut robber = elves.pop_front().unwrap();
        let victim = elves.pop_front().unwrap();
        robber.1 += victim.1;
        elves.push_back(robber);
    }
    let winner = elves.pop_front().unwrap().0;
    println!("Part 1: {}", winner);

    // Use a Vec-backed linked list
    let mut elves = Vec::new();
    for i in 0..count {
        elves.push((i + 1, 1, (i + 1) % count));
    }
    let mut remaining = count;
    let mut robber = 0;
    while remaining > 1 {
        if remaining % 1000 == 0 {
            println!("{}", remaining);
        }
        let mut prev = 0;
        let mut victim = robber;
        for j in 0..(remaining / 2) {
            prev = victim;
            victim = elves[victim].2;
        }
        elves[prev].2 = elves[victim].2;
        elves[robber].1 += elves[victim].1;
        elves[victim].1 = 0;

        robber = elves[robber].2;
        remaining -= 1;
    }
    /*
    println!("Part 2: {}", winner);
    let mut elves = vec![1; count];
    let mut remaining = count;
    while elves.iter().filter(|p| **p > 0).count() > 1 {
        for i in 0..elves.len() {
            if elves[i] == 0 {
                continue;
            }
            let next = (i..)
                .filter(|j| elves[j % count] > 0)
                .nth(remaining / 2)
                .unwrap();
            elves[i] += elves[next % count];
            elves[next % count] = 0;
            remaining -= 1;
        }
    }
    let winner = elves.iter()
        .enumerate()
        .find(|(i, p)| **p > 0)
        .unwrap().0;
    println!("Part 2: {}", winner + 1);
    */
}
