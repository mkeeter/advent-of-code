use std::io::BufRead;

#[derive(Eq, PartialEq)]
enum Tile {
    East,
    South,
    None,
}

fn main() {
    let mut map = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|b| match b {
                    '.' => Tile::None,
                    '>' => Tile::East,
                    'v' => Tile::South,
                    c => panic!("Invalid char {}", c),
                })
                .collect()
        })
        .collect::<Vec<Vec<Tile>>>();

    let width = map[0].len();
    let height = map.len();
    for i in 0.. {
        let mut changed = false;
        let mut todo = vec![];
        for (y, row) in map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == Tile::East && map[y][(x + 1) % width] == Tile::None {
                    todo.push((x, y));
                }
            }
        }
        changed |= !todo.is_empty();
        for (x, y) in todo.into_iter() {
            map[y][x] = Tile::None;
            map[y][(x + 1) % width] = Tile::East;
        }

        let mut todo = vec![];
        for (y, row) in map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == Tile::South && map[(y + 1) % height][x] == Tile::None {
                    todo.push((x, y));
                }
            }
        }
        changed |= !todo.is_empty();
        for (x, y) in todo.into_iter() {
            map[y][x] = Tile::None;
            map[(y + 1) % height][x] = Tile::South;
        }

        if !changed {
            println!("Part 1: {}", i + 1);
            break;
        }
    }
    println!("Part 2: ⭐");
}
