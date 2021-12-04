use std::io::BufRead;

/// Returns true if the most common digit at position `bit` is `1` or a tie.
fn common(nums: &[i32], bit: usize) -> bool {
    0 <= nums
        .iter()
        .fold(0, |sum, i| sum + (((i >> bit) & 1) * 2 - 1))
}

fn rating(mut nums: Vec<i32>, bits: usize, mode: bool) -> i32 {
    for bit in (0..bits).rev() {
        let target = (mode ^ common(&nums, bit)) as i32;
        nums = nums
            .into_iter()
            .filter(|n| ((n >> bit) & 1) == target)
            .collect();
        if nums.len() == 1 {
            return nums[0];
        }
    }
    panic!("Failed to reduce nums");
}

fn main() {
    let stdin = std::io::stdin();
    let mut iter = stdin.lock().lines().map(|line| line.unwrap()).peekable();

    let bits = iter.peek().unwrap().len();
    let nums = iter
        .map(|line| i32::from_str_radix(&line, 2).unwrap())
        .collect::<Vec<i32>>();

    let gamma = (0..bits)
        .filter(|b| common(&nums, *b))
        .fold(0, |out, b| out | (1 << b));
    let epsilon = (!gamma) & ((1 << bits) - 1);

    println!("Part 1: {}", gamma * epsilon);
    println!(
        "Part 2: {}",
        rating(nums.clone(), bits, true) * rating(nums, bits, false)
    );
}
