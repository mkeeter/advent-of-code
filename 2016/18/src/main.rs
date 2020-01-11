use std::collections::HashSet;
use std::io::Read;

fn run(mut traps: HashSet<i32>, rows: usize, cols: usize) -> usize {
    let mut safe = 0;
    for _ in 0..rows {
        safe += cols - traps.len();
        let mut next = HashSet::new();
        for x in 0..cols {
            let x = x as i32;
            let left = traps.contains(&(x - 1));
            let center = traps.contains(&x);
            let right = traps.contains(&(x + 1));
            let is_trap =  (left && center && !right) ||
                           (center && right && !left) ||
                           (left && !right && !center) ||
                           (right && !left && !center);
            if is_trap {
                next.insert(x);
            }
        }
        traps = next;
    }
    safe
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();

    let traps = input.chars()
        .enumerate()
        .filter(|&(_i, b)| b == '^')
        .map(|(i, _b)| i as i32)
        .collect::<HashSet<i32>>();
    let cols = input.len();

    println!("Part 1: {}", run(traps.clone(), 40, cols));
    println!("Part 2: {}", run(traps.clone(), 400000, cols));
}
