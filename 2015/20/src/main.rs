use std::io::Read;
use std::str::FromStr;

fn part1(n: usize) -> Vec<usize> {
    let mut houses = vec![0; n];
    for i in 0..n {
        for j in (i..n).step_by(i + 1) {
            houses[j] += 10 * (i + 1);
        }
    }
    houses
}

fn part2(n: usize) -> Vec<usize> {
    let mut houses = vec![0; n];
    for i in 0..n {
        for j in (i..n).step_by(i + 1).take(50) {
            houses[j] += 11 * (i + 1);
        }
    }
    houses
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let presents = usize::from_str(&input).unwrap();

    let mut size = 1000;
    loop {
        let houses = part1(size);
        if let Some(h) = houses.iter()
            .enumerate()
            .filter(|(_i, p)| **p >= presents)
            .next()
        {
            println!("Part 1: {}", h.0 + 1);
            break;
        }
        size *= 2;
    }

    loop {
        let houses = part2(size);
        if let Some(h) = houses.iter()
            .enumerate()
            .filter(|(_i, p)| **p >= presents)
            .next()
        {
            println!("Part 2: {}", h.0 + 1);
            break;
        }
        size *= 2;
    }
}
