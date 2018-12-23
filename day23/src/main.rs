use std::io::{self, Read};
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::{min, max};

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Region {
    xmin: i64, xmax: i64,
    ymin: i64, ymax: i64,
    zmin: i64, zmax: i64,
}

impl Region {
    fn could_contain(&self, p: &Point) -> bool {
        !self.could_not_contain(p)
    }

    fn could_not_contain(&self, p: &Point) -> bool {
        p.xmax() < self.xmin || p.xmin() > self.xmax ||
        p.ymax() < self.ymin || p.ymin() > self.ymax ||
        p.zmax() < self.zmin || p.zmin() > self.zmax
    }

    fn split(&self) -> Vec<Region> {
        let xmid = (self.xmin + self.xmax) / 2;
        let ymid = (self.ymin + self.ymax) / 2;
        let zmid = (self.zmin + self.zmax) / 2;
        (0..8).map(|i|
            Region { xmin: if (i & 1) != 0 { self.xmin } else { xmid + 1 },
                     xmax: if (i & 1) != 0 {      xmid } else { self.xmax },
                     ymin: if (i & 2) != 0 { self.ymin } else { ymid + 1 },
                     ymax: if (i & 2) != 0 {      ymid } else { self.ymax },
                     zmin: if (i & 4) != 0 { self.zmin } else { zmid + 1 },
                     zmax: if (i & 4) != 0 {      zmid } else { self.zmax },
        }).collect()
    }

    fn max_side(&self) -> i64 {
        [(self.xmax - self.xmin),
         (self.ymax - self.ymin),
         (self.zmax - self.zmin)].iter().max().unwrap().clone()
    }
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}

impl Point {
    fn xmin(&self) -> i64 { self.x - self.r }
    fn xmax(&self) -> i64 { self.x + self.r }
    fn ymin(&self) -> i64 { self.y - self.r }
    fn ymax(&self) -> i64 { self.y + self.r }
    fn zmin(&self) -> i64 { self.z - self.r }
    fn zmax(&self) -> i64 { self.z + self.r }
}

struct Worker {
    pts: Vec<Point>,
    best_score: usize,
    best_pos: Vec<(i64, i64, i64)>,
}

impl Worker {
    fn new(pts: Vec<Point>, starting_score: usize) -> Worker {
        Worker { pts: pts, best_score: starting_score, best_pos: vec![] }
    }

    fn get_active(&self, r: &Region, active: &Vec<usize>) -> Vec<usize> {
        active.iter()
            .filter(|i| r.could_contain(&self.pts[**i]))
            .cloned()
            .collect()
    }

    fn pointwise(&mut self, r: &Region, active: &Vec<usize>) {
        let mut my_best_score = 0;
        for x in r.xmin..=r.xmax {
            for y in r.ymin..=r.ymax {
                for z in r.zmin..=r.zmax {
                    let score = active.iter()
                        .map(|i| &self.pts[*i])
                        .filter(|p| (x - p.x).abs() +
                                    (y - p.y).abs() +
                                    (z - p.z).abs() <= p.r)
                        .count();
                    my_best_score = max(score, my_best_score);
                    if score > self.best_score {
                        println!("Found new best score {} at {} {} {}", score, x, y, z);
                        self.best_score = score;
                        self.best_pos.clear();
                    }
                    if score >= self.best_score {
                        println!("Found matching best score {} at {} {} {}", score, x, y, z);
                        self.best_pos.push((x, y, z));
                    }
                }
            }
        }
    }

    fn run(&mut self) {
        let r = Region {
            xmin: self.pts.iter().map(Point::xmin).min().unwrap(),
            xmax: self.pts.iter().map(Point::xmax).max().unwrap(),
            ymin: self.pts.iter().map(Point::ymin).min().unwrap(),
            ymax: self.pts.iter().map(Point::ymax).max().unwrap(),
            zmin: self.pts.iter().map(Point::zmin).min().unwrap(),
            zmax: self.pts.iter().map(Point::zmax).max().unwrap(),
        };

        let active = (0..self.pts.len()).into_iter().collect();
        self.recurse(&r, &active);
    }

    fn recurse(&mut self, r: &Region, active: &Vec<usize>) {
        if active.len() < self.best_score {
            return;
        }

        if r.max_side() < 10 {
            self.pointwise(r, active);
            return;
        }

        let mut rs: Vec<_> = r.split().into_iter()
            .map(|r| (self.get_active(&r, active), r))
            .collect();
        rs.sort_by_key(|(a, _)| a.len());

        for (a, r) in rs.iter().rev() {
            self.recurse(r, a);
        }
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
        .map(|v| Point { x: v[0], y: v[1], z: v[2], r: v[3] })
        .collect::<Vec<_>>();

    let p = pts.iter().max_by_key(|p| p.r).unwrap();
    let n = pts.iter()
        .filter(|q| (p.x - q.x).abs() +
                    (p.y - q.y).abs() +
                    (p.z - q.z).abs() <= p.r)
        .count();
    println!("Part 1: {}", n);

    ////////////////////////////////////////////////////////////////////////////
    let starting_score = pts.iter()
        .flat_map(|p| vec![(p.x + p.r, p.y, p.z), (p.x - p.r, p.y, p.z),
                           (p.x, p.y + p.r, p.z), (p.x, p.y - p.r, p.z),
                           (p.x, p.y, p.z + p.r), (p.x, p.y, p.z - p.r)]
                           .into_iter())
        .map(|(x, y, z)| pts.iter().filter(|q| (x - q.x).abs() +
                                               (y - q.y).abs() +
                                               (z - q.z).abs() <= q.r)
                           .count())
        .max().unwrap();
    println!("Got lower bound {}", starting_score);
    let mut w = Worker::new(pts, starting_score);
    w.run();

    // 129293600 is too high
}
