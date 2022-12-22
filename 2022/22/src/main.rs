use anyhow::{bail, Result};
use std::{collections::BTreeMap, io::BufRead};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Dir {
    dx: i64,
    dy: i64,
}

impl std::ops::Add<Dir> for Pos {
    type Output = Self;
    fn add(self, other: Dir) -> Self {
        Pos {
            x: self.x + other.dx,
            y: self.y + other.dy,
        }
    }
}

impl std::ops::Sub<Dir> for Pos {
    type Output = Self;
    fn sub(self, other: Dir) -> Self {
        Pos {
            x: self.x - other.dx,
            y: self.y - other.dy,
        }
    }
}

impl std::ops::Mul<i64> for Dir {
    type Output = Self;
    fn mul(self, other: i64) -> Self {
        Dir {
            dx: self.dx * other,
            dy: self.dy * other,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Rotation {
    Null,
    Left,
    Right,
    Flip,
}

impl std::ops::Neg for Rotation {
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            Rotation::Null | Rotation::Flip => self,
            Rotation::Left => Rotation::Right,
            Rotation::Right => Rotation::Left,
        }
    }
}

impl Dir {
    fn rotate(self, rot: Rotation) -> Self {
        match rot {
            Rotation::Null => self,
            Rotation::Left => Dir {
                dx: self.dy,
                dy: -self.dx,
            },
            Rotation::Right => Dir {
                dx: -self.dy,
                dy: self.dx,
            },
            Rotation::Flip => {
                self.rotate(Rotation::Left).rotate(Rotation::Left)
            }
        }
    }
    fn score(&self) -> i64 {
        match (self.dx, self.dy) {
            (1, 0) => 0,
            (0, 1) => 1,
            (-1, 0) => 2,
            (0, -1) => 3,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Side {
    origin: Pos,
    dir: Dir,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
enum Tile {
    Empty,
    Rock,
}

#[derive(Copy, Clone, Debug)]
enum Action {
    Move(usize),
    Left,
    Right,
}

////////////////////////////////////////////////////////////////////////////////

fn main() -> Result<()> {
    let mut map = BTreeMap::new();
    let mut actions = vec![];
    let mut load_directions = false;
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        let line = line.unwrap();
        if line.is_empty() {
            load_directions = true;
            continue;
        } else if load_directions {
            let mut i: Option<usize> = None;
            for c in line.chars() {
                match c {
                    'L' => {
                        if let Some(i) = i.take() {
                            actions.push(Action::Move(i));
                        }
                        actions.push(Action::Left);
                    }
                    'R' => {
                        if let Some(i) = i.take() {
                            actions.push(Action::Move(i));
                        }
                        actions.push(Action::Right);
                    }
                    c if c.is_numeric() => {
                        let digit = c.to_digit(10).unwrap() as usize;
                        i = Some(i.unwrap_or(0) * 10 + digit);
                    }
                    _ => bail!("Unknown action '{c}'"),
                }
            }
            if let Some(i) = i.take() {
                actions.push(Action::Move(i));
            }
        } else {
            for (x, c) in line.chars().enumerate() {
                let t = match c {
                    ' ' => continue,
                    '.' => Tile::Empty,
                    '#' => Tile::Rock,
                    _ => bail!("Unknown symbol '{c}'"),
                };
                map.insert(
                    Pos {
                        x: x as i64,
                        y: y as i64,
                    },
                    t,
                );
            }
        }
    }

    let start_x = (0..)
        .find(|x| matches!(map.get(&Pos { x: *x, y: 0 }), Some(Tile::Empty)))
        .unwrap();

    let size = map.keys().map(|k| k.x.max(k.y)).max().unwrap_or(0) + 1;

    let mut pos = Pos { x: start_x, y: 0 };
    let mut dir = Dir { dx: 1, dy: 0 };

    for m in &actions {
        match m {
            Action::Left => dir = dir.rotate(Rotation::Left),
            Action::Right => dir = dir.rotate(Rotation::Right),
            Action::Move(n) => {
                for _ in 0..*n {
                    let next = pos + dir;

                    let next = if map.contains_key(&next) {
                        next
                    } else {
                        // Let's do a warp!
                        let mut next = pos;
                        let mut last = None;
                        while next.x >= 0
                            && next.x <= size
                            && next.y >= 0
                            && next.y <= size
                        {
                            if map.contains_key(&next) {
                                last = Some(next);
                            }
                            next = next - dir;
                        }
                        last.unwrap()
                    };

                    if let Some(t) = map.get(&next) {
                        match t {
                            Tile::Rock => break,
                            Tile::Empty => pos = next,
                        }
                    }
                }
            }
        }
    }
    let row = pos.y + 1;
    let col = pos.x + 1;
    let facing = dir.score();
    println!("Part 1: {}", row * 1000 + col * 4 + facing);

    ////////////////////////////////////////////////////////////////////////////

    let side_length = ((map.len() / 6) as f64).sqrt() as i64;

    // There are six sides, which are identified as 0-6
    // The canonical ordering is the following
    //      0
    //     123
    //      4
    //      5
    // We declare that our starting position is the top-left corner of side 0,
    // which is in the Normal orientation.

    // Each side is defined by what it connects to, and how to rotate it
    const DIRS: [Rotation; 4] = [
        Rotation::Null,
        Rotation::Right,
        Rotation::Flip,
        Rotation::Left,
    ];
    const NEIGHBORS: [[(usize, Rotation); 4]; 6] = [
        [
            // Side 0
            (3, Rotation::Right),
            (2, Rotation::Null),
            (1, Rotation::Left),
            (5, Rotation::Null),
        ],
        [
            // Side 1
            (2, Rotation::Null),
            (4, Rotation::Left),
            (5, Rotation::Flip),
            (0, Rotation::Right),
        ],
        [
            // Side 2
            (3, Rotation::Null),
            (4, Rotation::Null),
            (1, Rotation::Null),
            (0, Rotation::Null),
        ],
        [
            // Side 3
            (5, Rotation::Flip),
            (4, Rotation::Right),
            (2, Rotation::Null),
            (0, Rotation::Left),
        ],
        [
            // Side 4
            (3, Rotation::Left),
            (5, Rotation::Null),
            (1, Rotation::Right),
            (2, Rotation::Null),
        ],
        [
            // Side 5
            (3, Rotation::Flip),
            (0, Rotation::Null),
            (1, Rotation::Flip),
            (4, Rotation::Null),
        ],
    ];

    let mut sides = [None; 6];
    sides[0] = Some(Side {
        origin: Pos { x: start_x, y: 0 },
        dir: Dir { dx: 1, dy: 0 },
    });

    // Find the canonical unwinding of the map
    while sides.iter().any(Option::is_none) {
        for i in 0..sides.len() {
            let s = match sides[i] {
                Some(s) => s,
                None => continue,
            };
            for (j, rot) in DIRS.iter().enumerate() {
                let corner = s.origin + s.dir.rotate(*rot) * side_length;
                if map.contains_key(&corner) {
                    let (new_side_index, new_side_rot) = NEIGHBORS[i][j];
                    let new_dir = s.dir.rotate(-new_side_rot);
                    let new_dx = new_dir;
                    let new_dy = new_dir.rotate(Rotation::Right);

                    // Snap to the original grid
                    let mut new_origin = Pos {
                        x: corner.x / side_length * side_length,
                        y: corner.y / side_length * side_length,
                    };
                    if new_dx.dx < 0 || new_dx.dy < 0 {
                        new_origin = new_origin + new_dx * (-side_length + 1);
                    }
                    if new_dy.dx < 0 || new_dy.dy < 0 {
                        new_origin = new_origin + new_dy * (-side_length + 1);
                    }

                    let new_side = Side {
                        origin: new_origin,
                        dir: new_dir,
                    };
                    if let Some(prev_new_side) = sides[new_side_index] {
                        assert_eq!(new_side, prev_new_side);
                    }
                    sides[new_side_index] = Some(new_side);
                }
            }
        }
    }
    let sides = sides.map(Option::unwrap);

    let mut flat_map = BTreeMap::new();
    let mut flat_to_orig = BTreeMap::new();
    for (side, (offset_x, offset_y)) in sides
        .iter()
        .zip([(1, 0), (0, 1), (1, 1), (2, 1), (1, 2), (1, 3)].into_iter())
    {
        let offset_x = offset_x * side_length;
        let offset_y = offset_y * side_length;
        let origin = side.origin;
        let dx = side.dir;
        let dy = side.dir.rotate(Rotation::Right);
        for x in 0..side_length {
            for y in 0..side_length {
                let pos_orig = origin + dx * x + dy * y;
                let pos_new = Pos {
                    x: offset_x + x,
                    y: offset_y + y,
                };
                flat_map.insert(pos_new, map.get(&pos_orig).unwrap());
                flat_to_orig.insert(pos_new, pos_orig);
            }
        }
    }

    let mut warp_zones = BTreeMap::new();
    let mut make_warp = |a: (Pos, Dir), b: (Pos, Dir)| {
        assert!(flat_map.contains_key(&a.0));
        assert!(flat_map.contains_key(&b.0));
        assert!(!flat_map.contains_key(&(a.0 + a.1)));
        assert!(!flat_map.contains_key(&(b.0 + b.1 * -1)));
        warp_zones.insert(a, b);
        warp_zones.insert(
            (b.0, b.1.rotate(Rotation::Flip)),
            (a.0, a.1.rotate(Rotation::Flip)),
        );
    };
    for i in 0..side_length {
        // 0-5
        make_warp(
            (
                Pos {
                    x: i + side_length,
                    y: 0,
                },
                Dir { dx: 0, dy: -1 },
            ),
            (
                Pos {
                    x: i + side_length,
                    y: side_length * 4 - 1,
                },
                Dir { dx: 0, dy: -1 },
            ),
        );
        // 0-1
        make_warp(
            (
                Pos {
                    x: i,
                    y: side_length,
                },
                Dir { dx: 0, dy: -1 },
            ),
            (
                Pos {
                    x: side_length,
                    y: i,
                },
                Dir { dx: 1, dy: 0 },
            ),
        );
        // 0-3
        make_warp(
            (
                Pos {
                    x: side_length * 2 + i,
                    y: side_length,
                },
                Dir { dx: 0, dy: -1 },
            ),
            (
                Pos {
                    x: side_length * 2 - 1,
                    y: side_length - 1 - i,
                },
                Dir { dx: -1, dy: 0 },
            ),
        );
        // 1-5
        make_warp(
            (
                Pos {
                    x: 0,
                    y: side_length + i,
                },
                Dir { dx: -1, dy: 0 },
            ),
            (
                Pos {
                    x: side_length,
                    y: side_length * 4 - 1 - i,
                },
                Dir { dx: 1, dy: 0 },
            ),
        );
        // 3-5
        make_warp(
            (
                Pos {
                    x: side_length * 3 - 1,
                    y: side_length + i,
                },
                Dir { dx: 1, dy: 0 },
            ),
            (
                Pos {
                    x: side_length * 2 - 1,
                    y: side_length * 4 - 1 - i,
                },
                Dir { dx: -1, dy: 0 },
            ),
        );
        // 1-4
        make_warp(
            (
                Pos {
                    x: i,
                    y: side_length * 2 - 1,
                },
                Dir { dx: 0, dy: 1 },
            ),
            (
                Pos {
                    x: side_length,
                    y: side_length * 3 - 1 - i,
                },
                Dir { dx: 1, dy: 0 },
            ),
        );
        // 3-4
        make_warp(
            (
                Pos {
                    x: side_length * 2 + i,
                    y: side_length * 2 - 1,
                },
                Dir { dx: 0, dy: 1 },
            ),
            (
                Pos {
                    x: side_length * 2 - 1,
                    y: side_length * 2 + i,
                },
                Dir { dx: -1, dy: 0 },
            ),
        );
    }

    let mut pos = Pos {
        x: side_length,
        y: 0,
    };
    let mut dir = Dir { dx: 1, dy: 0 };
    for m in &actions {
        match m {
            Action::Left => dir = dir.rotate(Rotation::Left),
            Action::Right => dir = dir.rotate(Rotation::Right),
            Action::Move(n) => {
                for _ in 0..*n {
                    let next = pos + dir;

                    if let Some(t) = flat_map.get(&next) {
                        match t {
                            Tile::Rock => break,
                            Tile::Empty => pos = next,
                        }
                    } else {
                        let (next_pos, next_dir) =
                            warp_zones.get(&(pos, dir)).unwrap();
                        match flat_map.get(next_pos).unwrap() {
                            Tile::Rock => break,
                            Tile::Empty => {
                                pos = *next_pos;
                                dir = *next_dir;
                            }
                        }
                    }
                }
            }
        }
    }

    let final_side = match (pos.x / side_length, pos.y / side_length) {
        (1, 0) => 0,
        (0, 1) => 1,
        (1, 1) => 2,
        (2, 1) => 3,
        (1, 2) => 4,
        (1, 3) => 5,
        _ => panic!("Invalid final position"),
    };
    // Unwind the rotation back to our global frame
    let mut dir_local = sides[final_side].dir;
    while dir_local != (Dir { dx: 1, dy: 0 }) {
        dir_local = dir_local.rotate(Rotation::Left);
        dir = dir.rotate(Rotation::Left);
    }

    let pos = flat_to_orig.get(&pos).unwrap();
    let row = pos.y + 1;
    let col = pos.x + 1;
    let facing = dir.score();
    println!("Part 2: {}", row * 1000 + col * 4 + facing);

    Ok(())
}
