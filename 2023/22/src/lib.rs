use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: u16,
    y: u16,
    z: u16,
}

impl std::str::FromStr for Pos {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',').map(|v| v.parse::<_>().unwrap());
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
    fn top(&self) -> impl Iterator<Item = Pos> + '_ {
        (self.start.x..=self.end.x).flat_map(move |x| {
            (self.start.y..=self.end.y).map(move |y| Pos {
                x,
                y,
                z: self.end.z,
            })
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
        .flat_map(|(i, b)| b.top().map(move |p| (p, i)))
        .collect();

    let mut changed = true;
    let mut stable = vec![false; bricks.len()];
    while changed {
        changed = false;
        for (i, b) in bricks.iter_mut().enumerate() {
            if stable[i] {
                continue;
            }
            // Drop this brick as far as possible, marking as stable if it ends
            // up supported by a stable brick.
            let orig = *b;
            loop {
                if b.start.z == 1 {
                    stable[i] = true;
                    break;
                }
                let mut supported = false;
                for p in b.bottom() {
                    // If the bottom is supported, then we can't fall
                    if let Some(j) = map.get(&Pos { z: p.z - 1, ..p }) {
                        stable[i] |= stable[*j];
                        supported = true;
                    }
                }
                if supported {
                    break;
                } else {
                    // Otherwise, fall
                    b.start.z -= 1;
                    b.end.z -= 1;
                }
            }
            // If we fell, then update the top map
            if orig.start.z != b.start.z {
                // Remove the previous brick positions
                for p in orig.top() {
                    let prev = map.remove(&p).unwrap();
                    assert_eq!(prev, i);
                }
                // Add the new brick positions
                for p in b.top() {
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

    ////////////////////////////////////////////////////////////////////////////

    // Flip the `supported_by` array!
    // `supporting[i]` returns bricks that brick `i` is supporting
    let mut supporting = (0..bricks.len())
        .map(|_| Vec::new())
        .collect::<Vec<Vec<usize>>>();
    for (i, bs) in supported_by.iter().enumerate() {
        for s in bs {
            supporting[*s].push(i);
        }
    }

    let p2: usize = critical
        .par_iter()
        .map(|&c| {
            let mut falling = HashSet::new();
            let mut todo = VecDeque::new();
            todo.push_front(c);
            while let Some(c) = todo.pop_front() {
                if !falling.insert(c) {
                    continue;
                }
                for &s in &supporting[c] {
                    let sup = &supported_by[s];
                    if !sup.is_empty()
                        && sup.iter().all(|b| falling.contains(b))
                    {
                        todo.push_back(s);
                    }
                }
            }
            falling.len() - 1
        })
        .sum();

    (p1.to_string(), p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_jenga() {
        let s = indoc::indoc! {"
            1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9
        "};
        let (p1, p2) = solve(s);
        assert_eq!(p1, "5");
        assert_eq!(p2, "7");
    }
}
