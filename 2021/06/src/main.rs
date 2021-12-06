/// Returns the population of fish spawned by a single starting fish whose
/// internal timer hits 0 on day `fish`
fn pop(time: usize, fish: usize) -> usize {
    let children = (time.saturating_sub(fish) + 6) / 7;
    1 + (0..children).map(|i| pop(time, fish + i*7 + 9)).sum::<usize>()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let nums = input.trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<usize>>();

    println!("Part 1: {}", nums.iter().map(|&fish| pop(80, fish)).sum::<usize>());
}
