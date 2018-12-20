use std::io::{self, Read};
use std::collections::{HashSet, HashMap, VecDeque};

type Pt = (i32, i32);

#[derive(Debug)]
struct Map {
    graph: HashMap<Pt, HashSet<Pt>>,
}

impl Map {
    fn from_str(s: &str, start: Pt) -> Map {
        let mut out = Map { graph: HashMap::new() };
        out.walk(&mut s.chars(), &[start].iter().cloned().collect());
        out
    }

    fn walk<I>(&mut self, s: &mut I, start: &HashSet<Pt>) -> HashSet<Pt>
        where I: Iterator<Item=char>
    {
        let mut heads = start.clone();
        let mut out = HashSet::new();
        while let Some(c) = s.next() {
            match c {
                '^' => continue,
                '$' => return HashSet::new(), // Termination condition
                'N'|'S'|'E'|'W' => {
                    heads = self.step(&heads, c);
                },
                '|' => { // Store new heads, then reset to original
                    out = out.union(&heads).cloned().collect();
                    heads = start.clone();
                }
                '(' => { // Recursive call, returning more heads
                    heads = heads.union(&self.walk(s, &heads))
                                 .cloned().collect();
                }
                ')' => { // Return from recursive call
                    return out.union(&heads).cloned().collect();
                }
                 _  => unreachable!("oh no"),
            }
        }
        unreachable!("oh no");
    }

    fn step(&mut self, heads: &HashSet<Pt>, dir: char) -> HashSet<Pt> {
        let d = match dir {
            'N' => ( 0,  1),
            'E' => ( 1,  0),
            'S' => ( 0, -1),
            'W' => (-1,  0),
             _  => unreachable!("invalid direction"),
        };
        heads.iter()
            .map(|a| self.link(*a, (a.0 + d.0, a.1 + d.1)))
            .collect()
    }

    fn link(&mut self, a: Pt, b: Pt) -> Pt {
        self.graph.entry(a)
            .or_insert(HashSet::new())
            .insert(b);
        self.graph.entry(b)
            .or_insert(HashSet::new())
            .insert(a);
        b
    }

    fn bfs(&self, start: Pt) -> HashMap<Pt, usize> {
        let mut todo: VecDeque<_> = [start].iter().cloned().collect();

        let mut distances: HashMap<Pt, usize> = [
            (start, 0)
        ].iter().cloned().collect();

        while let Some(p) = todo.pop_front() {
            for next in self.graph.get(&p).unwrap() {
                let d = distances.get(&p).unwrap().clone();
                distances.entry(*next)
                    .or_insert_with(|| {
                        todo.push_back(*next);
                        d + 1 });
            }
        }
        distances
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let start = (0, 0);
    let m = Map::from_str(&buffer, start);
    let distances = m.bfs(start);

    let max_distance = distances.values().max().unwrap();
    let over1k = distances.values().filter(|i| **i >= 1000).count();
    println!("{}, {}", max_distance, over1k);
}
