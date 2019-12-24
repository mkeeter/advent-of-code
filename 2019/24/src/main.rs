use std::collections::HashSet;
use std::io::BufRead;

const SIZE: usize = 5;

fn next(world: u32) -> u32 {
    let get = |x, y| -> bool {
        if x >= 0 && y >= 0 && x < SIZE as i32 && y < SIZE as i32 {
            world & (1u32 << (x + y * SIZE as i32)) != 0
        } else {
            false
        }
    };

    let mut out = 0;
    let neighbors = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let bugs = neighbors.iter()
                .filter(|(dx, dy)| get(x as i32 + dx, y as i32 + dy))
                .count();
            let spawn = if get(x as i32, y as i32) {
                bugs == 1
            } else {
                bugs == 1 || bugs == 2
            };
            if spawn {
                out |= 1 << (x + y * SIZE);
            }
        }
    }
    out
}

fn main() {
    let mut world: u32 = 0;
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                world |= 1 << (x + y * SIZE);
            }
        }
    }

    let mut seen = HashSet::new();
    loop {
        if seen.contains(&world) {
            println!("Part 1: {}", world);
            break;
        }
        seen.insert(world);
        world = next(world);
    }
}
