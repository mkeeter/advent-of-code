use std::collections::{HashMap, HashSet};
use std::io::BufRead;

struct Orbits {
    names: HashMap<String, usize>,
    parents: Vec<Option<usize>>,
    distances: Vec<Option<usize>>,
}

impl Orbits {
    fn index(&mut self, s: &str) -> usize {
        if let Some(i) = self.names.get(s) {
            *i
        } else {
            let i = self.names.len();
            self.names.insert(s.to_owned(), i);
            self.parents.push(None);
            self.distances.push(None);
            i
        }
    }

    fn new(parents: &HashMap<String, String>) -> Orbits {
        let mut out = Orbits {
            names: HashMap::new(),
            parents: Vec::new(),
            distances: Vec::new(),
        };
        for (k,v) in parents.iter() {
            let k = out.index(k);
            let v = out.index(v);
            out.parents[k] = Some(v);
        }
        let com = out.index("COM");
        out.distances[com] = Some(0);
        out
    }

    fn _distance(&mut self, s: usize) -> usize {
        if let Some(d) = self.distances[s] {
            d
        } else {
            let parent = self.parents[s].unwrap().to_owned();
            let d = self._distance(parent) + 1;
            self.distances[s] = Some(d);
            d
        }
    }

    fn checksum(&mut self) -> usize {
        let mut out = 0;
        for i in 0..self.names.len() {
            out += self._distance(i);
        }
        out
    }

    fn _parents_of(&self, mut s: usize) -> HashSet<usize> {
        let mut out = HashSet::new();
        while let Some(p) = self.parents[s] {
            out.insert(p);
            s = p;
        }
        out
    }

    fn transfer(&self, a: &str, b: &str) -> usize {
        let a = *self.names.get(a).unwrap();
        let b = *self.names.get(b).unwrap();
        let c = *self._parents_of(a)
            .intersection(&self._parents_of(b))
            .max_by_key(|o| self.distances[**o].unwrap())
            .unwrap();

        self.distances[a].unwrap() +
        self.distances[b].unwrap() -
        2*self.distances[c].unwrap() - 2
    }
}

fn main() {
    let parents = std::io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut itr = line.split(')');
            let orbitee = itr.next().unwrap().to_owned();
            let orbiter = itr.next().unwrap().to_owned();
            (orbiter, orbitee) })
        .collect::<HashMap<_,_>>();

    let mut orbits = Orbits::new(&parents);
    println!("Part 1: {}", orbits.checksum());
    println!("Part 2: {}", orbits.transfer("YOU", "SAN"));
}
