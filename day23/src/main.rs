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
        vec![
            ((self.0[0] - self.0[1] + self.0[2] - self.0[3]) / 4,
             (self.0[0] + self.0[1] - self.0[2] - self.0[3]) / 4,
             (self.0[0] + self.0[1] + self.0[2] + self.0[3]) / 4),

            ((self.0[4] - self.0[5] + self.0[6] - self.0[7]) / 4,
             (self.0[4] + self.0[5] - self.0[6] - self.0[7]) / 4,
             (-self.0[4] - self.0[5] - self.0[6] - self.0[7]) / 4),

            ((self.0[0] + self.0[2] + self.0[4] + self.0[6]) / 4,
             (self.0[0] - self.0[2] + self.0[4] - self.0[6]) / 4,
             (self.0[0] + self.0[5] - self.0[4] - self.0[6]) / 4),

            ((-self.0[1] - self.0[3] - self.0[5] - self.0[7]) / 4,
             (self.0[1] - self.0[3] + self.0[5] - self.0[7]) / 4,
             (self.0[1] + self.0[3] - self.0[5] - self.0[7]) / 4),

            ((self.0[0] - self.0[1] + self.0[4] - self.0[5]) / 4,
             (self.0[0] + self.0[1] + self.0[4] + self.0[5]) / 4,
             (self.0[0] + self.0[1] - self.0[4] - self.0[5]) / 4),

            ((self.0[2] - self.0[3] + self.0[6] - self.0[7]) / 4,
             (-self.0[2] - self.0[3] - self.0[6] - self.0[7]) / 4,
             (self.0[2] + self.0[3] - self.0[6] - self.0[7]) / 4)]
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

    let ba = Bounds::from_pt(&(0, 0, 0), 3);
    let bc = Bounds::from_pt(&(3, 0, 0), 3);
    let int = ba.intersection(&bc);
    for i in int.corners() {
        if !ba.contains(&i) {
            println!("OH NO: {:?}", &i);
        }
        if !bc.contains(&i) {
            println!("BO NO: {:?}", &i);
        }
        if !int.contains(&i) {
            println!("HO NO: {:?}", &i);
        }
        println!("----");
    }

    println!("{:?}", int.corners());
    // 129293600 is too high
}
