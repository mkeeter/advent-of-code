use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::BufRead;
use std::str::FromStr;

use num::Integer;
use smallvec::SmallVec;

#[derive(Clone, Hash, PartialEq, Eq)]
struct State {
    pos: i32,
    vel: i32,
}
struct Axis(SmallVec<[State; 4]>);

impl Axis {
    fn from_array(input: &[Vec<i32>], i: usize) -> Axis {
        Axis(input.iter().map(|p| State { pos: p[i], vel: 0 }).collect())
    }

    fn step(&mut self) {
        for i in 0..self.0.len() {
            for j in 0..i {
                match self.0[i].pos.cmp(&self.0[j].pos) {
                    Ordering::Less => {
                        self.0[i].vel += 1;
                        self.0[j].vel -= 1;
                    }
                    Ordering::Greater => {
                        self.0[i].vel -= 1;
                        self.0[j].vel += 1;
                    }
                    _ => (),
                }
            }
        }
        for a in self.0.iter_mut() {
            a.pos += a.vel;
        }
    }

    fn run(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    fn cycle(&mut self) -> usize {
        let mut seen = HashSet::new();

        for n in 0.. {
            if seen.contains(&self.0) {
                return n;
            }
            seen.insert(self.0.clone());
            self.step();
        }
        unreachable!();
    }
}

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.replace(|c| !char::is_numeric(c) && c != '-', " "))
        .map(|line| {
            line.split(' ')
                .filter_map(|i| i32::from_str(i).ok())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut axes = [
        Axis::from_array(&input, 0),
        Axis::from_array(&input, 1),
        Axis::from_array(&input, 2),
    ];

    for a in axes.iter_mut() {
        a.run(1000);
    }
    let energy: i32 = (0..input.len())
        .map(|i| {
            axes.iter().map(|a| a.0[i].pos.abs()).sum::<i32>()
                * axes.iter().map(|a| a.0[i].vel.abs()).sum::<i32>()
        })
        .sum();
    println!("Part 1: {}", energy);
    println!(
        "Part 2: {}",
        axes.iter_mut().fold(1, |acc, a| acc.lcm(&a.cycle()))
    );
}
