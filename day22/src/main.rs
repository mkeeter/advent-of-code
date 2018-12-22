use std::collections::HashMap;

enum Tile { Rocky, Wet, Narrow }
use crate::Tile::*;

fn main() {
    let (depth, target) = (8112, (13, 743));
    let (depth, target) = (510, (10, 10));

    let mut scores: HashMap<(usize, usize), usize> = HashMap::new();
    let mut map: HashMap<(usize, usize), Tile> = HashMap::new();
    for y in 0..=(2 * target.1) {
        for x in 0..=(2 * target.0) {
            let g =
                if x == 0 && y == 0 {
                    0
                } else if x == target.0 && y == target.1 {
                    0
                } else if y == 0 {
                    x * 16807
                } else if x == 0 {
                    y * 48271
                } else {
                    (scores.get(&(x - 1, y)).unwrap() *
                     scores.get(&(x, y - 1)).unwrap())
                };
            let erosion_level = (g + depth) % 20183;

            scores.insert((x, y), erosion_level);
            map.insert((x, y), match erosion_level % 3 {
                0 => Rocky,
                1 => Wet,
                2 => Narrow,
                _ => unreachable!(),
            });
        }
    }

    let mut risk = 0;
    for y in 0..=target.1 {
        for x in 0..=target.0 {
            risk += scores.get(&(x, y)).unwrap() % 3;
        }
    }
    println!("Hello, world! {}", risk);
}
