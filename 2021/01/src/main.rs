use itertools::Itertools;
use std::io::BufRead;

fn main() {
    let nums = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect::<Vec<i32>>();

    fn count<I: Iterator<Item = i32>>(itr: I) -> usize {
        itr.tuple_windows().filter(|(a, b)| b > a).count()
    }
    let a = count(nums.iter().cloned());
    println!("Part 1: {}", a);

    let b = count(nums.iter().tuple_windows().map(|(a, b, c)| a + b + c));
    println!("Part 2: {}", b);
}
