use std::io::BufRead;
use std::str::FromStr;
use std::collections::HashSet;
use std::cmp::Ordering;

use num::Integer;
use smallvec::SmallVec;

#[derive(Clone, Hash, PartialEq, Eq)]
struct State {
    pos: i32,
    vel: i32,
}
struct Axis {
    state: SmallVec<[State; 4]>,
}

impl Axis {
    fn from_array(input: &[Vec<i32>], i: usize) -> Axis {
        Axis { state: input.iter()
            .map(|p| State { pos: p[i], vel: 0 })
            .collect() }
    }

    fn pos(&self, i: usize) -> i32 {
        self.state[i].pos
    }

    fn vel(&self, i: usize) -> i32 {
        self.state[i].vel
    }

    fn step(&mut self) {
        for i in 0..self.state.len() {
            for j in 0..i {
                match self.pos(i).cmp(&self.pos(j)) {
                    Ordering::Less => {
                        self.state[i].vel += 1;
                        self.state[j].vel -= 1;
                    },
                    Ordering::Greater => {
                        self.state[i].vel -= 1;
                        self.state[j].vel += 1;
                    },
                    _ => (),
                }
            }
        }
        for a in self.state.iter_mut() {
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
        seen.insert(self.state.clone());

        for n in 0.. {
            self.step();
            let state = self.state.clone();
            if seen.contains(&state) {
                return n + 1;
            } else {
                seen.insert(state);
            }
        }
        unreachable!();
    }
}

fn main() {
    let input = std::io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.replace(|c| !char::is_numeric(c) && c != '-', " "))
        .map(|line| line.split(' ')
                 .filter_map(|i| i32::from_str(i).ok())
                 .collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let mut axes = [Axis::from_array(&input, 0),
                    Axis::from_array(&input, 1),
                    Axis::from_array(&input, 2)];

    for a in axes.iter_mut() {
        a.run(1000);
    }
    let energy: i32 = (0..input.len())
        .map(|i| axes.iter().map(|a| a.pos(i).abs()).sum::<i32>() *
                 axes.iter().map(|a| a.vel(i).abs()).sum::<i32>())
        .sum();
    println!("Part 1: {}", energy);
    println!("Part 2: {}", axes.iter_mut().fold(1, |acc, a| acc.lcm(&a.cycle())));
}
