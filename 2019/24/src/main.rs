use itertools::iproduct;
use std::collections::HashSet;
use std::io::BufRead;

const SIZE: i32 = 5;
const CENTER: i32 = SIZE / 2;

type Map = HashSet<(i32, i32)>;
struct RecursiveMap(HashSet<(i32, i32, i32)>);

impl RecursiveMap {
    fn get(&self, x: i32, y: i32, z: i32) -> bool {
        if x == CENTER && y == CENTER {
            panic!("Stared into the abyss");
        }
        self.0.contains(&(x, y, z))
    }

    fn neighbors(&self, x: i32, y: i32, z: i32) -> usize {
        if x == CENTER && y == CENTER {
            panic!("Stared into the abyss");
        }

        let neighbors_left = if x == CENTER + 1 && y == CENTER {
            (0..SIZE).filter(|i| self.get(SIZE - 1, *i, z + 1)).count()
        } else if x > 0 {
            self.get(x - 1, y, z) as usize
        } else {
            self.get(CENTER - 1, CENTER, z - 1) as usize
        };

        let neighbors_right = if x == CENTER - 1 && y == CENTER {
            (0..SIZE).filter(|i| self.get(0, *i, z + 1)).count()
        } else if x < SIZE - 1 {
            self.get(x + 1, y, z) as usize
        } else {
            self.get(CENTER + 1, CENTER, z - 1) as usize
        };

        let neighbors_above = if y == CENTER + 1 && x == CENTER {
            (0..SIZE).filter(|i| self.get(*i, SIZE - 1, z + 1)).count()
        } else if y > 0 {
            self.get(x, y - 1, z) as usize
        } else {
            self.get(CENTER, CENTER - 1, z - 1) as usize
        };

        let neighbors_below = if y == CENTER - 1 && x == CENTER {
            (0..SIZE).filter(|i| self.get(*i, 0, z + 1)).count()
        } else if y < SIZE - 1 {
            self.get(x, y + 1, z) as usize
        } else {
            self.get(CENTER, CENTER + 1, z - 1) as usize
        };

        neighbors_left + neighbors_right + neighbors_above + neighbors_below
    }
}

fn main() {
    let mut input = HashSet::new();
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                input.insert((x as i32, y as i32));
            }
        }
    }

    let mut seen = HashSet::new();
    let mut world: Map = input.clone();
    loop {
        let w = world
            .iter()
            .fold(0, |acc, (x, y)| acc | (1 << (x + y * SIZE)));
        if !seen.insert(w) {
            println!("Part 1: {}", w);
            break;
        }

        world = iproduct!(0..SIZE, 0..SIZE)
            .filter(|&(x, y)| {
                let nearby_bugs = [(0, 1), (1, 0), (0, -1), (-1, 0)]
                    .iter()
                    .filter(|(dx, dy)| world.contains(&(x + dx, y + dy)))
                    .count();
                let is_bug = world.contains(&(x, y));
                nearby_bugs == 1 || (nearby_bugs == 2 && !is_bug)
            })
            .collect();
    }

    let mut world = RecursiveMap(input.iter().map(|&(x, y)| (x, y, 0)).collect());

    for i in 0..200 {
        world = RecursiveMap(
            iproduct!(0..SIZE, 0..SIZE, (-i - 1)..=(i + 1))
                .filter(|&(x, y, _z)| x != CENTER || y != CENTER)
                .filter(|&(x, y, z)| {
                    let nearby_bugs = world.neighbors(x, y, z);
                    let is_bug = world.0.contains(&(x, y, z));
                    nearby_bugs == 1 || (nearby_bugs == 2 && !is_bug)
                })
                .collect(),
        );
    }
    println!("Part 2: {}", world.0.len());
}
