use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::io::BufRead;

#[derive(PartialEq, Eq)]
struct Molecule(String);

impl Ord for Molecule {
    // We're using a max-heap, so the shortest molecule is the best
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.len().cmp(&other.0.len()) {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => self.0.cmp(&other.0),
        }
    }
}

impl PartialOrd for Molecule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn react(mol: &str, lhs: &str, rhs: &str) -> HashSet<String> {
    let parts = mol.split(lhs).collect::<Vec<_>>();
    let mut out = HashSet::new();
    for i in 0..(parts.len() - 1) {
        let mut m = String::new();
        for (j, p) in parts.iter().enumerate() {
            m += p;
            if i == j {
                m += rhs;
            } else if j != parts.len() - 1 {
                m += lhs;
            }
        }
        out.insert(m);
    }
    out
}

fn main() {
    let mut reactions = Vec::new();
    let mut molecule = String::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.contains(" => ") {
            let mut itr = line.split(" => ");
            let lhs = itr.next().unwrap();
            let rhs = itr.next().unwrap();
            reactions.push((lhs.to_owned(), rhs.to_owned()));
        } else if !line.is_empty() {
            molecule = line.to_owned();
        }
    }

    let mut out = HashSet::new();
    for (lhs, rhs) in reactions.iter() {
        for i in react(&molecule, lhs, rhs) {
            out.insert(i);
        }
    }
    println!("Part 1: {}", out.len());

    let mut todo = BinaryHeap::new();
    todo.push((Molecule(molecule), 0));
    while let Some((m, stage)) = todo.pop() {
        if m.0 == "e" {
            println!("Part 2: {}", stage);
            break;
        }

        let mut next = HashSet::new();
        for (lhs, rhs) in reactions.iter() {
            for o in react(&m.0, rhs, lhs) {
                next.insert(o);
            }
        }
        for m in next.into_iter() {
            todo.push((Molecule(m), stage + 1));
        }
    }
}
