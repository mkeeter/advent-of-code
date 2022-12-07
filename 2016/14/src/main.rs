use std::collections::{HashSet, VecDeque};
use std::io::Read;

fn run(itr: impl Iterator<Item = String>) -> usize {
    let mut itr = itr
        .map(|s| s.chars().collect::<Vec<_>>())
        .enumerate()
        .map(|(n, s)| {
            let triple = s
                .windows(3)
                .filter(|w| w.iter().all(|&c| c == w[0]))
                .map(|w| w[0])
                .next();
            let quintics = s
                .windows(5)
                .filter(|w| w.iter().all(|&c| c == w[0]))
                .map(|w| w[0])
                .collect::<HashSet<char>>();
            (n, triple, quintics)
        });

    let mut q = VecDeque::new();
    for _i in 0..=1000 {
        q.push_back(itr.next().unwrap());
    }

    let mut found = 0;
    while let Some((n, triple, _quintics)) = q.pop_front() {
        if let Some(t) = triple {
            if q.iter().any(|v| v.2.contains(&t)) {
                found += 1;
                if found == 64 {
                    return n;
                }
            }
        }
        q.push_back(itr.next().unwrap());
    }
    unreachable!();
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();

    let hashes = (0..)
        .map(|i| input.to_string() + &i.to_string())
        .map(md5::compute)
        .map(|i| format!("{:x}", i));
    println!("Part 1: {}", run(hashes));

    let hashes = (0..)
        .map(|i| input.to_string() + &i.to_string())
        .filter_map(|s| {
            std::iter::successors(Some(s), |s| Some(format!("{:x}", md5::compute(s)))).nth(2017)
        });
    println!("Part 2: {}", run(hashes));
}
