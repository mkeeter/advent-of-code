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
        .enumerate()
        .map(|(i, (pt, r))| ([i].iter().cloned().collect(),
                             Bounds::from_pt(pt, *r)))
        .collect::<Vec<(Vec<usize>, Bounds)>>();

    let mut seen = HashSet::new();
    for max_rank in 0.. {
        for (ca, _) in bounds.iter() {
            print!("{:?}", ca);
        }print!("\n");

        let mut next = Vec::new();
        for (i, (ca, ba)) in bounds.iter().enumerate() {
            println!("{} / {}", i, bounds.len());
            for (j, (cb, bb))  in bounds.iter().enumerate() {
                if j >= i {
                    break;
                }
                if ca.len() <= max_rank && cb.len() <= max_rank {
                    continue;
                }
                let bc = ba.intersection(bb);
                if bc.is_empty() {
                    continue;
                }

                let mut cc = Vec::new();
                for x in ca { cc.push(x.clone()); }
                for x in cb { cc.push(x.clone()); }
                cc.sort();
                cc.dedup();

                if cc.len() <= ca.len() || cc.len() <= cb.len() {
                    continue;
                }
                if seen.contains(&cc) {
                    continue;
                }
                seen.insert(cc.clone());
                next.push((cc, bc));
            }
        }
        if next.len() == 0 {
            break;
        }
        bounds = next;
    }

    let ba = Bounds::from_pt(&(0, 5, 0), 11);
    let bb = Bounds::from_pt(&(6, 0, 0), 1);
    let bc = ba.intersection(&bb);

    // 129293600 is too high
}
