use std::io::{self, Read};
use std::collections::{HashSet, HashMap, VecDeque};

type Pt = (i32, i32);

#[derive(Debug)]
struct Map {
    graph: HashMap<Pt, HashSet<Pt>>,
}

impl Map {
    fn from_str(s: &str) -> Map {
        let mut itr = s.chars();

        let mut start = HashSet::new();
        start.insert((0, 0));

        let mut out = Map { graph: HashMap::new() };
        out.from_iter(&mut itr, &start);
        out
    }

    fn from_iter<I>(&mut self, s: &mut I, start: &HashSet<Pt>) -> HashSet<Pt>
        where I: Iterator<Item=char>
    {
        let mut heads = start.clone();
        let mut out = HashSet::new();
        while let Some(c) = s.next() {
            match c {
                '^' => continue,
                '$' => return HashSet::new(),
                'N'|'S'|'E'|'W' => {
                    heads = self.step(&heads, c);
                },
                '|' => { // Reset to initial heads and store final heads
                    for h in heads { out.insert(h.clone()); }
                    heads = start.iter().cloned().collect();
                }
                '(' => { // Recursive call
                    for h in self.from_iter(s, &heads) {
                        heads.insert(h);
                    }
                }
                ')' => { // Return from recursive call
                    for h in heads { out.insert(h.clone()); }
                    return out;
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
        heads.iter().map(|a| {
            let b = (a.0 + d.0, a.1 + d.1);
            self.link(*a, b);
            b }).collect()
    }
    fn link(&mut self, a: Pt, b: Pt) {
        self.graph.entry(a)
            .or_insert(HashSet::new())
            .insert(b);
        self.graph.entry(b)
            .or_insert(HashSet::new())
            .insert(a);
    }

    fn flood(&self) -> HashMap<Pt, usize> {
        let mut todo = VecDeque::new();
        todo.push_back((0, 0));

        let mut distances = HashMap::new();
        distances.insert((0, 0), 0);

        while let Some(p) = todo.pop_front() {
            for next in self.graph.get(&p).unwrap() {
                if !distances.contains_key(next) {
                    distances.insert(*next, distances.get(&p).unwrap() + 1);
                    todo.push_back(*next);
                }
            }
        }
        distances
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let m = Map::from_str(&buffer);

    let distances = m.flood();
    let max_distance = distances.values().max().unwrap();
    let over1k = distances.values().filter(|i| **i >= 1000).count();
    println!("{}, {}", max_distance, over1k);

}
