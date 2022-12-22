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
    size: i64,
}

impl Side {
    fn rotate(&self, pos: Pos, rot: Rotation) -> Pos {
        let origin = self.origin
            + match (self.dir.dx, self.dir.dy) {
                (1, 0) => Dir { dx: 0, dy: 0 },
                (0, 1) => Dir {
                    dx: -self.size,
                    dy: 0,
                },
                (-1, 0) => Dir {
                    dx: -self.size,
                    dy: -self.size,
                },
                (0, -1) => Dir {
                    dx: 0,
                    dy: -self.size,
                },
                _ => panic!("aaah"),
            };

        let dx = pos.x - origin.x;
        let dy = pos.y - origin.y;
        let vec = Dir { dx, dy }.rotate(rot);
        let corner = match rot {
            Rotation::Left => Pos {
                x: self.origin.x,
                y: self.origin.y + self.size - 1,
            },
            Rotation::Right => Pos {
                x: self.origin.x + self.size - 1,
                y: self.origin.y,
            },
            Rotation::Flip => Pos {
                x: self.origin.x + self.size - 1,
                y: self.origin.y + self.size - 1,
            },
            Rotation::Null => self.origin,
        };
        corner + vec
    }
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
                        // WARP
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
        size: side_length,
    });

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
                        size: side_length,
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
    for (i, s) in sides.iter().enumerate() {
        println!("{i}: {s:?}");
    }
    println!();

    //let mut tiles = BTreeMap::new();
    for (side_index, side_a) in sides.iter().enumerate() {
        println!("{side_a:?}");
        for (edge_index, dir) in DIRS.iter().enumerate() {
            let (side_b_index, rot_b) = NEIGHBORS[side_index][edge_index];
            let side_b = sides[side_b_index];
            println!("  {side_a:?}");

            let offset_a = side_a.origin + side_a.dir * (side_length - 1);
            let axis_a = side_a.dir.rotate(Rotation::Right);

            println!("  side b origin: {:?}", side_b.origin);
            println!("  rot_b: {rot_b:?}");
            let offset_b = side_b.rotate(side_b.origin, rot_b);
            let axis_b = side_b.dir.rotate(rot_b).rotate(Rotation::Right);
            println!("    offset_a: {offset_a:?}");
            println!("    axis_a:   {axis_a:?}");
            println!("    offset_b: {offset_b:?}");
            println!("    axis_b:   {axis_b:?}");

            for pos in 0..side_length {
                let p = side_a.rotate(offset_a + axis_a * pos as i64, *dir);
                let q = side_b.rotate(offset_b + axis_b * pos as i64, *dir);
                println!(
                    "       {p:?} + {:?} => {q:?}",
                    side_a.dir.rotate(*dir)
                );
            }
            panic!();
        }
    }

    let mut pos = (start_x, 0);
    assert_eq!(start_x % side_length, 0);

    let mut side = 0;
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
                        println!(
                            "Departing from side {side} at {pos:?}, {dir:?}"
                        );
                        panic!();
                        // WARP
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

    Ok(())
}
