use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input")
        .lines()
        .map(|line| {
            line.chars()
                .filter(|&c| c == '#' || c == '.')
                .map(|c| c == '#')
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    let mut state = input[0]
        .iter()
        .enumerate()
        .filter(|p| *p.1)
        .map(|p| p.0 as i64)
        .collect::<HashSet<i64>>();

    let table = input[2..]
        .iter()
        .map(|v| {
            (
                v[5],
                v[..5]
                    .iter()
                    .enumerate()
                    .map(|(i, &b)| if b { 1 << i } else { 0 })
                    .fold(0, |a, b| a | b),
            )
        })
        .filter(|p| p.0)
        .map(|p| p.1)
        .collect::<HashSet<i64>>();

    const N: usize = 50000000000;
    let mut seen = HashMap::new();

    for i in 0..N {
        let start = state.iter().min().unwrap() - 2;
        let end = state.iter().max().unwrap() + 2;

        let score = state.iter().sum::<i64>();

        let (key, offset) = {
            let mut flat = state.iter().cloned().collect::<Vec<i64>>();
            let m = flat.iter().min().unwrap().clone();
            for f in &mut flat {
                *f -= m;
            }
            flat.sort();
            (flat, m)
        };
        if let Some((j, prev_offset)) = seen.get(&key) {
            println!("Found cycle from {:?} to {:?}", j, i);
            assert!(*j == i - 1);
            assert!(offset - *prev_offset == 1);
            println!(
                "Final score estimated at {}",
                score + ((N - i) * state.len()) as i64
            );
        }
        seen.insert(key, (i, offset));

        let next = (start..=end)
            .filter(|j| {
                let k = ((j - 2)..=(j + 2))
                    .enumerate()
                    .filter(|(_, p)| state.contains(p))
                    .map(|(i, _)| (1 << i))
                    .fold(0, |a, b| a | b);
                table.contains(&k)
            })
            .collect::<HashSet<i64>>();

        state = next;
    }

    println!("Final score: {}", state.iter().sum::<i64>());
}
