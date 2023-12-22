use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
    z: usize,
}

impl std::str::FromStr for Pos {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',').map(|v| v.parse::<usize>().unwrap());
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();
        Ok(Self { x, y, z })
    }
}

#[derive(Copy, Clone, Debug)]
struct Brick {
    start: Pos,
    end: Pos,
}

impl Brick {
    fn pos(&self) -> impl Iterator<Item = Pos> + '_ {
        (self.start.x..=self.end.x)
            .flat_map(|x| (self.start.y..=self.end.y).map(move |y| (x, y)))
            .flat_map(|(x, y)| {
                (self.start.z..=self.end.z).map(move |z| Pos { x, y, z })
            })
    }
    fn bottom(&self) -> impl Iterator<Item = Pos> + '_ {
        (self.start.x..=self.end.x).flat_map(move |x| {
            (self.start.y..=self.end.y).map(move |y| Pos {
                x,
                y,
                z: self.start.z,
            })
        })
    }
}

impl std::str::FromStr for Brick {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('~').map(|v| v.parse::<Pos>().unwrap());
        let start = iter.next().unwrap();
        let end = iter.next().unwrap();
        assert!(start.x <= end.x);
        assert!(start.y <= end.y);
        assert!(start.z <= end.z);
        Ok(Self { start, end })
    }
}

pub fn solve(s: &str) -> (String, String) {
    let mut bricks = s
        .lines()
        .map(|b| b.parse::<Brick>().unwrap())
        .collect::<Vec<_>>();
    let mut map: HashMap<Pos, usize> = bricks
        .iter()
        .enumerate()
        .flat_map(|(i, b)| b.pos().map(move |p| (p, i)))
        .collect();

    let mut changed = true;
    while changed {
        changed = false;
        for (i, b) in bricks.iter_mut().enumerate() {
            // Can this brick fall?
            if b.bottom().all(|p| {
                p.z != 1 && !map.contains_key(&Pos { z: p.z - 1, ..p })
            }) {
                // Remove the previous brick positions
                for p in b.pos() {
                    let prev = map.remove(&p).unwrap();
                    assert_eq!(prev, i);
                }
                // Adjust the brick downward
                b.start.z -= 1;
                b.end.z -= 1;
                // Add the new brick positions
                for p in b.pos() {
                    let prev = map.insert(p, i);
                    assert!(prev.is_none());
                }
                changed = true;
            }
        }
    }
    // At this point, all bricks are in their lowest-energy state

    // Find which bricks are supported by others
    let mut supported_by = vec![];
    for b in bricks.iter() {
        let supporting = b
            .bottom()
            .flat_map(|p| map.get(&Pos { z: p.z - 1, ..p }))
            .cloned()
            .collect::<HashSet<usize>>();
        if supporting.is_empty() {
            assert_eq!(b.start.z, 1);
        }
        supported_by.push(supporting);
    }

    let critical = supported_by
        .iter()
        .filter(|s| s.len() == 1)
        .flatten()
        .cloned()
        .collect::<HashSet<usize>>();
    let p1 = bricks.len() - critical.len();

    let mut out = 0;
    for c in critical {
        let mut falling = HashSet::new();
        falling.insert(c);
        let mut changed = true;
        while changed {
            changed = false;
            for (i, sup) in supported_by.iter().enumerate() {
                if !sup.is_empty()
                    && sup.iter().all(|b| falling.contains(b))
                    && falling.insert(i)
                {
                    changed = true;
                }
            }
        }
        out += falling.len() - 1;
    }
    let p2 = out;

    (p1.to_string(), p2.to_string())
}
