use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let sizes = std::io::stdin().lock()
        .lines()
        .filter_map(|line| usize::from_str(&line.unwrap()).ok())
        .collect::<Vec<usize>>();

    let target = 150;
    let sol = (0..(1 << sizes.len()))
        .filter(|i| target == (0..sizes.len())
             .filter(|j| (i & (1usize << j)) != 0)
             .map(|j| sizes[j])
             .sum::<usize>())
        .map(|i| i.count_ones())
        .collect::<Vec<u32>>();
    println!("Part 1: {}", sol.len());

    let min_ones = sol.iter().min().unwrap();
    let count = sol.iter().filter(|&i| i == min_ones).count();
    println!("Part 2: {}", count);
}
