use std::io::{self, Read};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::cmp::min;

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

    fn empty(&self) -> bool {
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

    // Bounds represent a region with the bounds
    //   x + y + z <= bounds[0]
    //  -x + y + z <= bounds[1]
    //   x - y + z <= bounds[2]
    //  -x - y + z <= bounds[3]
    //   x + y - z <= bounds[4]
    //  etc
    let bounds = pts.iter()
        .map(|(pt, r)| Bounds::from_pt(pt, *r))
        .collect::<Vec<Bounds>>();

    let ba = Bounds::from_pt(&(0, 0, 0), 10);
    let bb = Bounds::from_pt(&(11, 0, 0), 10);
    let bc = ba.intersection(&bb);
    println!("{:?}", bc.empty());

    //assert!(best_pts.len() == 1);
    let mut target = best_pts.iter().next().unwrap().clone();

    println!("walking from {:?}", target);
    let mut improved = true;
    while improved {
        improved = false;
        let offset = |i| match i % 3 {
            0 => -1,
            1 =>  0,
            2 =>  1,
            _ => unreachable!(),
        };

        for i in 0..27 {
            let next = (target.0 + offset(i),
                        target.1 + offset(i/3),
                        target.2 + offset(i/9));
            if next.0.abs() + next.1.abs() + next.2.abs() >=
               target.0.abs() + target.1.abs() + target.2.abs()
            {
                continue;
            } else if pts.iter()
                .filter(|((x, y, z), r)| (next.0 - x).abs() +
                                         (next.1 - y).abs() +
                                         (next.2 - z).abs() <= *r)
                .count() == best_score
            {
                target = next;
                improved = true;
            }        }
        if improved {
            println!("{:?}", target);
        }
    }
    println!("{:?}, {:?}", best_score, best_pts);
    // 129293600 is too high
}
