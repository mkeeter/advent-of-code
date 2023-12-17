use util::Direction;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct State {
    pos: (i64, i64),
    dir: Direction,
}

impl State {
    fn key(&self) -> usize {
        let x: u8 = self.pos.0.try_into().unwrap();
        let y: u8 = self.pos.1.try_into().unwrap();
        let dir = self.dir as u8;
        u32::from_le_bytes([x, y, dir, 0]) as usize
    }
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
        vec![[Direction::East, Direction::South]
            .into_iter()
            .map(|dir| State { pos: (0, 0), dir })
            .collect::<Vec<State>>()]
    };

    let run = |min_momentum, max_momentum| {
        // Map of total heat loss -> current state, acting as a priority queue
        let mut paths = start();
        let mut seen = vec![0u64; 256 * 256 * 4 / 64];
        let mut loss = 0;
        loop {
            let group = std::mem::take(&mut paths[loss]);
            'outer: for mut p in group {
                if p.pos == end {
                    return loss;
                }

                // Check for this value in our bitmasked array
                let key = p.key();
                let (i, mask) = (key / 64, 1 << (key % 64));
                if seen[i] & mask != 0 {
                    continue;
                } else {
                    seen[i] |= mask;
                }

                let mut loss = loss;
                let turns = match p.dir {
                    Direction::North | Direction::South => {
                        [Direction::East, Direction::West]
                    }
                    Direction::East | Direction::West => {
                        [Direction::North, Direction::South]
                    }
                };
                // Walk our minimum distance
                for _ in 0..min_momentum {
                    p.pos = p.dir.next(p.pos);
                    if let Some(nl) = get(p.pos) {
                        loss += nl as usize;
                    } else {
                        continue 'outer;
                    }
                }
                // Then walk through the "allowed-to-turn" steps
                for _ in min_momentum..max_momentum {
                    p.pos = p.dir.next(p.pos);
                    if let Some(nl) = get(p.pos) {
                        loss += nl as usize;
                        for &dir in &turns {
                            if loss >= paths.len() {
                                paths.resize_with(loss + 1, Vec::new);
                            }
                            paths[loss].push(State { dir, ..p });
                        }
                    } else {
                        continue 'outer;
                    }
                }
            }
            loss += 1;
        }
    };

    let (p1, p2) = std::thread::scope(|s| {
        let h1 = s.spawn(|| run(0, 3));
        let h2 = s.spawn(|| run(3, 10));
        (h1.join().unwrap(), h2.join().unwrap())
    });

    (p1.to_string(), p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_crucible() {
        let s = indoc! {"
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        "};
        let (p1, p2) = solve(s);
        assert_eq!(p1, "102");
        assert_eq!(p2, "94");

        let s = indoc! {"
            111111111111
            999999999991
            999999999991
            999999999991
            999999999991
        "};
        let (_p1, p2) = solve(s);
        assert_eq!(p2, "71");
    }
}
