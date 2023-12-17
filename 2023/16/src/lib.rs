use rayon::prelude::*;
use util::{DenseGrid, Direction};

fn recurse(
    mut pos: (i64, i64),
    mut dir: Direction,
    map: &DenseGrid,
    seen: &mut [u8],
) {
    while let Some(i) = map.index(pos) {
        let prev = &mut seen[i];
        if *prev & dir.bit() != 0 {
            return;
        }
        *prev |= dir.bit();
        match map.get_by_index(i) {
            Some('/') => {
                dir = match dir {
                    Direction::North => Direction::East,
                    Direction::South => Direction::West,
                    Direction::East => Direction::North,
                    Direction::West => Direction::South,
                }
            }
            Some('\\') => {
                dir = match dir {
                    Direction::North => Direction::West,
                    Direction::South => Direction::East,
                    Direction::East => Direction::South,
                    Direction::West => Direction::North,
                }
            }
            Some('-') => {
                if matches!(dir, Direction::North | Direction::South) {
                    recurse(pos, Direction::East, map, seen);
                    dir = Direction::West;
                }
            }
            Some('|') => {
                if matches!(dir, Direction::East | Direction::West) {
                    recurse(pos, Direction::North, map, seen);
                    dir = Direction::South;
                }
            }
            Some(c) => panic!("invalid character {c}"),
            None => (),
        }
        pos = dir.next(pos);
    }
}

fn run(pos: (i64, i64), dir: Direction, grid: &DenseGrid) -> usize {
    let mut seen = vec![0; grid.width() * grid.height()];
    recurse(pos, dir, grid, &mut seen);
    seen.into_iter().filter(|v| *v != 0).count()
}

pub fn solve(s: &str) -> (String, String) {
    let grid = DenseGrid::new(s);

    let p1 = run((0, 0), Direction::East, &grid);

    let w = (0..grid.width())
        .into_par_iter()
        .map(|x| {
            [
                ((x as i64, 0i64), Direction::South),
                ((x as i64, grid.height() as i64 - 1), Direction::North),
            ]
        })
        .flatten();
    let h = (0..grid.height())
        .into_par_iter()
        .map(|y| {
            [
                ((0i64, y as i64), Direction::East),
                ((grid.width() as i64 - 1, y as i64), Direction::West),
            ]
        })
        .flatten();
    let p2 = w
        .chain(h)
        .map(|(pos, dir)| run(pos, dir, &grid))
        .max()
        .unwrap();

    (p1.to_string(), p2.to_string())
}
