use std::str::FromStr;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let count = usize::from_str(input.trim()).unwrap();
    let mut elves = vec![1; count];
    while elves.iter().filter(|p| **p > 0).count() > 1 {
        for i in 0..elves.len() {
            if elves[i] == 0 {
                continue;
            }
            let next = (i + 1..).find(|j| elves[j % count] > 0).unwrap();
            elves[i] += elves[next % count];
            elves[next % count] = 0;
        }
    }
    let winner = elves.iter()
        .enumerate()
        .find(|(i, p)| **p > 0)
        .unwrap().0;
    println!("Part 1: {}", winner + 1);

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
}
