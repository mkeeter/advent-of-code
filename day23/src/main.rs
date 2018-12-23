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

    let target = (0..bounds.len()).collect::<Vec<_>>();
    let mut best_score = 0;
    let mut seen: HashSet<Vec<usize>> = HashSet::new();
    let mut todo = VecDeque::new();
    todo.push_back(target);

    while let Some(next) = todo.pop_front() {
        println!("Testing group of size {}", next.len());
        if next.len() < best_score || seen.contains(&next) {
            continue;
        }

        let mut bad_points = next.iter().map(|a|
            (next.iter().filter(|b| !intersects.contains(&(*a, **b))).count(),
             a))
            .filter(|(score, _)| *score > 0)
            .collect::<Vec<_>>();
        bad_points.sort();

        if bad_points.len() == 0 {
            println!("Success!\n");
            best_score = next.len();
            break;
        } else {
            println!("Found {} bad points to try removing\n", bad_points.len());
        }

        for (_, t) in bad_points.iter().rev() {
            let next = next.iter().filter(|c| c != t).cloned().collect();
            todo.push_back(next);
        }
    }

    // 129293600 is too high
}

fn run(target: &Vec<usize>, intersects: &HashSet<(usize, usize)>,
       best_score: &mut usize,
       seen: &mut HashSet<Vec<usize>>)
{
    if target.len() < *best_score {
        return;
    }
    if seen.contains(target) {
        return;
    }
    seen.insert(target.clone());
    println!("Testing group of size {}", target.len());

    let mut bad_points = target.iter().map(|a|
        (target.iter().filter(|b| !intersects.contains(&(*a, **b))).count(),
         a))
        .filter(|(score, _)| *score > 0)
        .collect::<Vec<_>>();
    bad_points.sort();

    if bad_points.len() == 0 {
        println!("Success!\n");
        *best_score = target.len();
        return;
    }

    if target.len() == *best_score {
        return;
    }

    for (_, t) in bad_points.iter().rev() {
        let next = target.iter().filter(|c| c != t).cloned().collect();
        run(&next, intersects, best_score, seen);
    }
}
