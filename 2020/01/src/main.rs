use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let nums = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect::<Vec<i32>>();

    // Build a map from sum -> product for all pairs in the input
    let sums: HashMap<i32, i32> = nums
        .iter()
        .flat_map(|a| nums.iter().map(move |b| (a + b, a * b)))
        .collect();

    // The solution to part 1 is just stored in the table:
    println!("Part 1: {}", sums.get(&2020).expect("Could not find sum"));

    // Do an O(n) sweep through the table to find the triple
    let b = nums
        .iter()
        .flat_map(|a| sums.get(&(2020 - a)).map(|b| a * b))
        .next()
        .expect("Could not find three-item match");
    println!("Part 2: {}", b);
}
