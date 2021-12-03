use std::io::BufRead;

fn rating(mut nums: Vec<i32>, bits: usize, mode: bool) -> i32 {
    for bit in (0..bits).rev() {
        if nums.len() == 1 {
            break;
        }
        let score = nums
            .iter()
            .fold(0, |sum, i| sum + (((i >> bit) & 1) * 2 - 1));
        let target = (mode ^ (score >= 0)) as i32;
        nums = nums
            .into_iter()
            .filter(|n| ((n >> bit) & 1) == target)
            .collect();
    }
    assert!(nums.len() == 1);
    nums[0]
}

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let bits = lines[0].len();
    let nums = lines
        .into_iter()
        .map(|line| i32::from_str_radix(&line, 2).unwrap())
        .collect::<Vec<i32>>();

    let count = nums
        .iter()
        .flat_map(|n| (0..bits).map(move |b| (b, ((n >> b) & 1) * 2 - 1)))
        .scan([0; 32], |accum, (b, d)| {
            accum[b] += d;
            Some(*accum)
        })
        .last()
        .unwrap();

    let gamma = (0..bits)
        .map(|i| ((count[i] > 0) as u32) << i)
        .fold(0, |a, b| a | b);
    let epsilon = (!gamma) & ((1 << bits) - 1);

    println!("Part 1: {}", gamma * epsilon);
    println!(
        "Part 2: {}",
        rating(nums.clone(), bits, true) * rating(nums, bits, false)
    );
}
