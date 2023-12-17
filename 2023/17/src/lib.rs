use std::collections::BTreeSet;
use util::Direction;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct State {
    loss: u64,
    pos: (i64, i64),
    dir: Direction,
    momentum: u8,
}

pub fn solve(s: &str) -> (String, String) {
    let map: Vec<Vec<u8>> = s
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    let get = |pos: (i64, i64)| -> Option<u8> {
        let x: usize = pos.0.try_into().ok()?;
        let y: usize = pos.1.try_into().ok()?;
        map.get(y).and_then(|row: &Vec<u8>| row.get(x)).cloned()
    };

    let end = (map[0].len() as i64 - 1, map.len() as i64 - 1);

    let start = || {
        [Direction::East, Direction::South]
            .into_iter()
            .map(|dir| State {
                loss: 0,
                pos: (0, 0),
                dir,
                momentum: 0,
            })
            .collect::<BTreeSet<_>>()
    };

    let run = |min_momentum, max_momentum| {
        // Map of total heat loss -> current state, acting as a priority queue
        let mut paths = start();
        let mut seen = BTreeSet::new();
        while let Some(p) = paths.pop_first() {
            if p.pos == end {
                return p.loss;
            }
            if !seen.insert((p.pos, p.dir, p.momentum)) {
                continue;
            }
            let turns = match p.dir {
                Direction::North | Direction::South => {
                    [Direction::East, Direction::West]
                }
                Direction::East | Direction::West => {
                    [Direction::North, Direction::South]
                }
            };
            for (dir, momentum) in turns
                .into_iter()
                .map(|d| {
                    if p.momentum >= min_momentum {
                        Some((d, 1))
                    } else {
                        None
                    }
                })
                .chain(std::iter::once(if p.momentum < max_momentum {
                    Some((p.dir, p.momentum + 1))
                } else {
                    None
                }))
                .flatten()
            {
                let pos = dir.next(p.pos);
                if let Some(new_loss) = get(pos) {
                    paths.insert(State {
                        loss: p.loss + new_loss as u64,
                        pos,
                        dir,
                        momentum,
                    });
                }
            }
        }
        panic!("no path found")
    };

    let p1 = run(0, 3);
    let p2 = run(4, 10);

    (p1.to_string(), p2.to_string())
}
