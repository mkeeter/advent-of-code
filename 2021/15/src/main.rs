use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::BufRead;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Task {
    score: Reverse<usize>,
    pos: (usize, usize),
}

struct Tile {
    weight: usize,
    score: Option<usize>,
}

fn search(map: &[Vec<u8>]) -> usize {
    let mut map: Vec<Vec<Tile>> = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|weight| Tile {
                    weight: *weight as usize,
                    score: None,
                })
                .collect()
        })
        .collect();

    let mut todo = BinaryHeap::new();
    todo.push(Task {
        score: Reverse(0),
        pos: (0, 0),
    });

    let xmax = map[0].len() as i64 - 1;
    let ymax = map.len() as i64 - 1;

    while let Some(task) = todo.pop() {
        let (x, y) = task.pos;
        let tile = &mut map[y][x];
        if let Some(s) = tile.score {
            assert!(s <= task.score.0);
            continue;
        }
        tile.score = Some(task.score.0);

        for (dx, dy) in &[(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let (x, y) = (x as i64 + dx, y as i64 + dy);
            if x < 0 || y < 0 || x > xmax || y > ymax {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            todo.push(Task {
                score: Reverse(task.score.0 + map[y][x].weight),
                pos: (x, y),
            });
        }
    }
    map[ymax as usize][ymax as usize].score.unwrap()
}

fn main() {
    let minimap = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().bytes().map(|c| c - b'0').collect())
        .collect::<Vec<Vec<u8>>>();

    println!("Part 1: {}", search(&minimap));

    let xsize = minimap[0].len();
    let ysize = minimap.len();
    let mut megamap = vec![vec![0; xsize * 5]; xsize * 5];
    for (y, row) in megamap.iter_mut().enumerate() {
        for (x, c) in row.iter_mut().enumerate() {
            let risk = minimap[y % ysize][x % xsize];
            *c = ((risk + (x / xsize + y / ysize) as u8 - 1) % 9) + 1;
        }
    }
    println!("Part 2: {}", search(&megamap));
}
