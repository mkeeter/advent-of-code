use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let mut input = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let ints = line
                .unwrap()
                .replace('.', "")
                .split(' ')
                .filter_map(|w| usize::from_str(w).ok())
                .collect::<Vec<_>>();
            (ints[0], ints[1])
        })
        .collect::<Vec<(usize, usize)>>();

    let part1 = (0..)
        .find(|t| {
            input
                .iter()
                .enumerate()
                .all(|(i, (pos, offset))| (offset + t + i + 1) % pos == 0)
        })
        .unwrap();
    println!("Part 1: {}", part1);

    input.push((11, 0));
    let part2 = (0..)
        .find(|t| {
            input
                .iter()
                .enumerate()
                .all(|(i, (pos, offset))| (offset + t + i + 1) % pos == 0)
        })
        .unwrap();
    println!("Part 2: {}", part2);
}
