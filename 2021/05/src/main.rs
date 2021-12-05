use parse_display::{Display, FromStr};
use std::collections::HashMap;
use std::io::BufRead;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{xa},{ya} -> {xb},{yb}")]
struct Line {
    xa: i64,
    ya: i64,
    xb: i64,
    yb: i64,
}
impl Line {
    fn size(&self) -> i64 {
        (self.xb - self.xa).abs().max((self.yb - self.ya).abs()) + 1
    }
    fn at(&self, i: i64) -> (i64, i64) {
        (
            self.xa + (self.xb - self.xa).signum() * i,
            self.ya + (self.yb - self.ya).signum() * i,
        )
    }
    fn orthogonal(&self) -> bool {
        self.xa == self.xb || self.ya == self.yb
    }
}

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect::<Vec<Line>>();

    let mut seen = HashMap::new();
    lines
        .iter()
        .filter(|line| line.orthogonal())
        .flat_map(|line| (0..line.size()).map(move |i| line.at(i)))
        .for_each(|pt| *seen.entry(pt).or_insert(0) += 1);
    println!("Part 1: {}", seen.values().filter(|&i| i > &1).count());

    let mut seen = HashMap::new(); // Once more, with diagonals!
    lines
        .iter()
        .flat_map(|line| (0..line.size()).map(move |i| line.at(i)))
        .for_each(|pt| *seen.entry(pt).or_insert(0) += 1);
    println!("Part 2: {}", seen.values().filter(|&i| i > &1).count());
}
