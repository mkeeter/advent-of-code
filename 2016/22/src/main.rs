use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::collections::{HashSet, HashMap, VecDeque};
use std::io::BufRead;

use itertools::iproduct;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Node {
    size: usize,
    used: usize,
    avail: usize,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.used.hash(state);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Grid {
    nodes: Vec<Node>,
    height: usize,
    width: usize,
    target: (i32, i32),
}

impl Grid {
    fn index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(y * self.width + x)
        }
    }

    // Returns a new Grid with data moved from one node to another
    fn transfer(&self, tx: i32, ty: i32, rx: i32, ry: i32) -> Option<Self> {
        let i = self.index(tx, ty);
        let j = self.index(rx, ry);
        if i.is_none() || j.is_none() {
            return None;
        }
        let i = i.unwrap();
        let j = j.unwrap();
        // Check that there is enough space
        let size = self.nodes[i].used;
        if size > self.nodes[j].avail {
            return None;
        }

        let mut next = self.clone();
        next.nodes[i].used  -= size;
        next.nodes[i].avail += size;
        next.nodes[j].used  += size;
        next.nodes[j].avail -= size;

        if next.target == (tx, ty) {
            next.target = (rx, ry);
        }
        Some(next)
    }
}

fn main() {
    let mut nodes = HashMap::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let i = line.replace('T', "")
            .replace("-x", " ")
            .replace("-y", " ")
            .split(' ')
            .filter_map(|w| usize::from_str(w).ok())
            .collect::<Vec<usize>>();
        if i.len() == 5 {
            // (x, y) -> (size, used, avail)
            nodes.insert((i[0], i[1]), (i[2], i[3], i[4]));
        }
    }

    let (width, height) = *nodes.keys().max().unwrap();
    let mut grid = Grid {
        nodes: Vec::new(),
        height: height + 1,
        width: width + 1,
        target: (width as i32, 0)
    };
    for y in 0..=height {
        for x in 0..=width {
            let (size, used, avail) = *nodes.get(&(x, y)).unwrap();
            grid.nodes.push(Node { size, used, avail });
        }
    }

    let mut viable = 0;
    for (i, a) in grid.nodes.iter().enumerate() {
        for (j, b) in grid.nodes.iter().enumerate() {
            // Skip if both are the same node
            if i == j {
                continue;
            }
            // Get (size, used, avail) from hashmap
            if a.used != 0 && a.used <= b.avail {
                viable += 1;
            }
        }
    }
    println!("Part 1: {}", viable);

    let mut todo = VecDeque::new();
    todo.push_back((grid, 0));
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut seen = HashSet::new();
    while let Some((grid, steps)) = todo.pop_front() {
        if grid.target == (0, 0) {
            println!("Made it in {} steps", steps);
            break;
        }
        // Skip previously seen states
        if seen.contains(&grid) {
            continue;
        } else {
            seen.insert(grid.clone());
        }

        iproduct!(0..grid.width, 0..grid.height, directions.iter())
            .filter_map(|(x, y, (dx, dy))| {
                let x = x as i32;
                let y = y as i32;
                grid.transfer(x, y, x + dx, y + dy)
            })
            .for_each(|grid| todo.push_back((grid, steps + 1)));
    }
}
