use std::collections::{HashMap, HashSet};
use std::io::BufRead;

struct Orbits {
    parents: HashMap<String, String>,
    distances: HashMap<String, usize>,
}

impl Orbits {
    fn distance(&mut self, s: &str) -> usize {
        if let Some(d) = self.distances.get(s) {
            *d
        } else {
            let parent = self.parents.get(s).unwrap().to_owned();
            let d = self.distance(&parent) + 1;
            self.distances.insert(s.to_string(), d);
            d
        }
    }

    fn checksum(&mut self) -> usize {
        let mut out = 0;
        let keys = self.parents.keys().cloned().collect::<Vec<_>>();
        for p in keys {
            out += self.distance(&p);
        }
        out
    }

    fn parents_of(&self, s: &str) -> HashSet<String> {
        let mut out = HashSet::new();
        let mut s = s;
        while let Some(p) = self.parents.get(s) {
            out.insert(p.to_owned());
            s = p;
        }
        out
    }
}

fn main() {
    let parents = std::io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| {
            println!("{}", line);
            let mut itr = line.split(')');
            let orbitee = itr.next().unwrap().to_owned();
            let orbiter = itr.next().unwrap().to_owned();
            (orbiter, orbitee) })
        .collect::<HashMap<_,_>>();

    let mut orbits = Orbits { parents: parents, distances: HashMap::new() };
    orbits.distances.insert("COM".to_string(), 0);
    println!("Part 1: {}", orbits.checksum());

    let common = orbits.parents_of("YOU")
        .intersection(&orbits.parents_of("SAN"))
        .map(|o| {println!("{:?}", o); o})
        .max_by_key(|o| orbits.distances.get(*o).unwrap())
        .unwrap()
        .to_owned();
    println!("Part 2: {}", orbits.distance("YOU") + orbits.distance("SAN") - 2*orbits.distance(&common) - 2);
}
