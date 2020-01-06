use std::collections::HashSet;
use std::io::BufRead;

use itertools::iproduct;

const SIZE: i32 = 100;
type Point = (i32, i32);

fn step(g: HashSet<Point>) -> HashSet<Point> {
    iproduct!(0..SIZE, 0..SIZE)
        .filter(|&(x, y)| {
            let neighbors = [(0, 1), (0, -1), (1, 0), (-1, 0),
                             (1, 1), (1, -1), (-1, 1), (-1, -1)]
                .iter()
                .filter(|(dx, dy)| g.contains(&((x + dx), (y + dy))))
                .count();
            if g.contains(&(x, y)) {
                neighbors == 2 || neighbors == 3
            } else {
                neighbors == 3
            }
        })
        .collect()
}

fn main() {
    let mut map = HashSet::new();
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                map.insert((x as i32, y as i32));
            }
        }
    }

    let mut part1 = map.clone();
    for _i in 0..100 {
        part1 = step(part1);
    }
    println!("Part 1: {}", part1.len());

    let mut part2 = map;
    let stuck = [(0, 0), (0, SIZE - 1), (SIZE - 1, 0), (SIZE - 1, SIZE - 1)]
        .iter()
        .cloned()
        .collect();
    for _i in 0..100 {
        part2 = step(part2.union(&stuck).cloned().collect());
    }
    println!("Part 2: {}", part2.union(&stuck).count());
}
