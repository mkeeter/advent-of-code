use std::collections::{HashSet, HashMap};
use std::io::BufRead;

const SIZE: i32 = 5;

fn main() {
    let mut world: HashMap<(i32, i32), char> = HashMap::new();
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            world.insert((x as i32, y as i32), c);
        }
    }

    let neighbors = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut seen = HashSet::new();
    loop {
        let w = world.iter()
            .filter(|(_k, v)| **v == '#')
            .fold(0, |acc, ((x, y), _v)| acc | (1 << (x + y * SIZE)));
        if seen.contains(&w) {
            println!("Part 1: {}", w);
            break;
        }
        seen.insert(w);

        world = world.iter()
            .map(|((x, y), v)| {
                let bugs = neighbors.iter()
                    .filter_map(|(dx, dy)| world.get(&(x + dx, y + dy)))
                    .filter(|c| **c == '#')
                    .count();
                let spawn = if *v == '#' {
                    bugs == 1
                } else {
                    bugs == 1 || bugs == 2
                };
                ((*x, *y), if spawn { '#' } else { '.' })
            })
            .collect();
    }
}
