use std::collections::{BinaryHeap, HashMap};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Tile {
    Rocky,
    Wet,
    Narrow,
}
use crate::Tile::*;
impl Tile {
    fn compatible(&self, tool: &Tool) -> bool {
        match self {
            Rocky => return *tool == Torch || *tool == Gear,
            Wet => return *tool == Neither || *tool == Gear,
            Narrow => return *tool == Neither || *tool == Torch,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
enum Tool {
    Torch,
    Gear,
    Neither,
}
use crate::Tool::*;

fn main() {
    let (depth, target) = (8112, (13, 743));
    //let (depth, target) = (510, (10, 10));

    // We build the graph on an expanded grid, in case there are approaches
    // that come from beyond the target's position
    let width = target.0 * 16;
    let height = target.1 * 4;

    let mut scores: HashMap<(i64, i64), usize> = HashMap::new();
    let mut map: HashMap<(i64, i64), Tile> = HashMap::new();
    for y in 0..=height {
        for x in 0..=width {
            let g = if x == 0 && y == 0 {
                0
            } else if x == target.0 && y == target.1 {
                0
            } else if y == 0 {
                (x * 16807) as usize
            } else if x == 0 {
                (y * 48271) as usize
            } else {
                (scores.get(&(x - 1, y)).unwrap() * scores.get(&(x, y - 1)).unwrap())
            };
            let erosion_level = (g + depth) % 20183;

            scores.insert((x, y), erosion_level);
            map.insert(
                (x, y),
                match erosion_level % 3 {
                    0 => Rocky,
                    1 => Wet,
                    2 => Narrow,
                    _ => unreachable!(),
                },
            );
        }
    }

    let mut risk = 0;
    for y in 0..=target.1 {
        for x in 0..=target.0 {
            risk += scores.get(&(x, y)).unwrap() % 3;
        }
    }
    println!("Part 1: {}", risk);

    ///////////////////////////////////////////////////////////////////////////

    // Build the graph, using negative edges since the heap is a max-heap
    type Pt = (i64, i64, Tool);
    type Edge = (Pt, u32);
    let mut graph: HashMap<Pt, Vec<Edge>> = HashMap::new();

    for y in 0..=height {
        for x in 0..=width {
            for tool in [Torch, Gear, Neither].iter() {
                // Try to move horizontally with the same tool
                for n in [(0, -1), (0, 1), (-1, 0), (1, 0)].iter() {
                    if let Some(next) = map.get(&(x + n.0, y + n.1)) {
                        if next.compatible(tool) {
                            graph
                                .entry((x, y, *tool))
                                .or_insert(Vec::new())
                                .push(((x + n.0, y + n.1, *tool), 1));
                        }
                    }
                }
                // Try to switch tools while staying in the same position
                for n in [Torch, Gear, Neither].iter() {
                    if n != tool && map.get(&(x, y)).unwrap().compatible(n) {
                        graph
                            .entry((x, y, *tool))
                            .or_insert(Vec::new())
                            .push(((x, y, *n), 7));
                    }
                }
            }
        }
    }

    let mut distance: HashMap<Pt, i64> = HashMap::new();
    let mut heap = BinaryHeap::new();

    let start: Pt = (0, 0, Torch);
    distance.insert(start, 0);
    heap.push((0, start));

    let target = (target.0, target.1, Torch);
    while let Some((t, current)) = heap.pop() {
        // We may have already solved this position; skip if
        // there's a better score in place already
        let d = distance.get(&current).unwrap().clone();
        if d > t {
            continue;
        }

        // Iterate over all edges
        for (next, weight) in graph.get(&current).unwrap().iter() {
            let tentative = d - (*weight as i64);
            if *distance.get(next).unwrap_or(&std::i64::MIN) < tentative {
                distance.insert(*next, tentative);
                heap.push((tentative, *next));
            }
        }

        // Only loop until we've solved for the target position
        if current == target {
            break;
        }
    }

    println!("Part 2: {}", distance.get(&target).unwrap());
}
