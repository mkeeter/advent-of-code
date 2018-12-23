use std::io::{self, Read};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::cmp::min;

// Bounds represent a region with the bounds
//   x + y + z <= bounds[0]
//  -x + y + z <= bounds[1]
//   x - y + z <= bounds[2]
//  -x - y + z <= bounds[3]
//   x + y - z <= bounds[4]
//  etc
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Bounds([i64; 8]);

impl Bounds {
    fn from_pt(pt: &(i64, i64, i64), r: i64) -> Bounds {
        let (x, y, z) = pt;
        let mut out = [0; 8];
        for i in 0..8 {
            out[i] = r + x * Self::sign(i, 0)
                       + y * Self::sign(i, 1)
                       + z * Self::sign(i, 2);
        }
        Bounds(out)
    }

    fn sign(i: usize, axis: usize) -> i64 {
        if i & (1 << axis) != 0 {
             1
        } else {
            -1
        }
    }

    fn opposite(i: usize) -> usize {
        (!i) & 0b111
    }

    fn contains(&self, pt: &(i64, i64, i64)) -> bool {
        let (x, y, z) = pt;
        (0..8).all(|i| x * Self::sign(i, 0)
                     + y * Self::sign(i, 1)
                     + z * Self::sign(i, 2) <= self.0[i])
    }

    fn is_empty(&self) -> bool {
        (0..8).any(|i| self.0[i] < -self.0[Self::opposite(i)])
    }

    fn intersection(&self, other: &Bounds) -> Bounds {
        let mut out = [0; 8];
        for i in 0..8 {
            out[i] = min(self.0[i], other.0[i]);
        }
        Bounds(out)
    }
}

////////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let re = Regex::new(r"(-*\d+)").unwrap();

    let pts = buffer
        .lines()
        .map(|line| re.captures_iter(line)
                .map(|i| str::parse::<i64>(&i[1]).unwrap())
                .collect::<Vec<i64>>())
        .map(|v| ((v[0], v[1], v[2]), v[3]))
        .collect::<Vec<((i64, i64, i64), i64)>>();

    let ((x, y, z), r) = pts.iter().max_by_key(|pt| pt.1).unwrap();
    let n = pts.iter()
        .filter(|(pt, _)| (pt.0 - x).abs() +
                          (pt.1 - y).abs() +
                          (pt.2 - z).abs() <= *r)
        .count();
    println!("Part 1: {}", n);

    ////////////////////////////////////////////////////////////////////////////
    let mut bounds = pts.iter()
        .map(|(pt, r)| Bounds::from_pt(pt, *r))
        .collect::<Vec<_>>();

    let mut intersects = HashSet::new();
    for (i, a) in bounds.iter().enumerate() {
        for (j, b) in bounds.iter().enumerate() {
            if (!a.intersection(&b).is_empty()) {
                intersects.insert((i, j));
                print!("X");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }

    let mut cliques = Vec::new();
    for i in 0..bounds.len() {
        cliques.push(vec![i]);
    }

    loop {
        let mut new_cliques = Vec::new();
        for clique in cliques.iter() {
            for i in 0..bounds.len() {
                if clique.iter().all(|c| intersects.contains(&(*c, i)) && *c != i) {
                    let mut c = clique.clone();
                    c.push(i);
                    c.sort();
                    c.dedup();
                    new_cliques.push(c);
                }
            }
        }
        new_cliques.sort();
        new_cliques.dedup();
        println!("{:?}", new_cliques);
        if cliques == new_cliques {
            break;
        }
        cliques = new_cliques;
    }

    // 129293600 is too high
}
