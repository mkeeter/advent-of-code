use std::io::{self, Read};
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
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

    fn corners(&self) -> Vec<(i64, i64, i64)> {
        let mut out = Vec::new();
        for axis in [1, 2, 4].iter() {
            for offset in [false, true].iter() {
                let mut x = 0;
                let mut y = 0;
                let mut z = 0;

                for i in 0..8 {
                    if ((i & axis) != 0) == *offset {
                        x += self.0[i] * Self::sign(i, 0);
                        y += self.0[i] * Self::sign(i, 1);
                        z += self.0[i] * Self::sign(i, 2);
                    }
                }
                let xs = if x % 4 == 0 { vec![x / 4] }
                         else { vec![x / 4, (x + 3) / 4] };
                let ys = if y % 4 == 0 { vec![y / 4] }
                         else { vec![y / 4, (y + 3) / 4] };
                let zs = if z % 4 == 0 { vec![z / 4] }
                         else { vec![z / 4, (z + 3) / 4] };

                for x in xs.iter() {
                    for y in ys.iter() {
                        for z in zs.iter() {
                            out.push((*x, *y, *z));
                        }
                    }
                }
            }
        }
        return out;
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

    let mut test_points = Vec::new();
    for p in bounds.iter() {
        for c in p.corners() {
            test_points.push(c);
        }
        for q in bounds.iter() {
            let r = p.intersection(&q);
            if !r.is_empty() {
                for c in r.corners() {
                    test_points.push(c);
                }
            }
        }
    }
    test_points.sort();
    test_points.dedup();
    println!("Got {} test points", test_points.len());

    let mut best_score = 0;
    let mut best_dist = std::i64::MAX;
    for (x, y, z) in test_points.iter() {
        let n = pts.iter()
            .filter(|(pt, r)| (pt.0 - x).abs() +
                              (pt.1 - y).abs() +
                              (pt.2 - z).abs() <= *r)
            .count();


        let dist = x + y + z;
        if n > best_score {
            best_score = n;
            best_dist = dist;
        }

        if n == best_score {
            if dist <= best_dist {
                best_dist = dist;
                println!("{}, ({}, {}, {}), {}", n, x, y, z, best_dist);
            }
        }
    }
}
