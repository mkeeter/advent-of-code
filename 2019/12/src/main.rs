use std::io::BufRead;
use std::str::FromStr;
use std::collections::HashSet;
use num::Integer;

fn run(mut pos: Vec<[i32; 3]>, mut vel: Vec<[i32; 3]>, n: usize) -> i32 {
    for _ in 0..n {
        for i in 0..pos.len() {
            for j in 0..i {
                if i == j {
                    continue;
                }
                for k in 0..3 {
                    if pos[i][k] < pos[j][k] {
                        vel[i][k] += 1;
                        vel[j][k] -= 1;
                    } else if pos[i][k] > pos[j][k] {
                        vel[i][k] -= 1;
                        vel[j][k] += 1;
                    }
                }
            }
        }
        for i in 0..pos.len() {
            for k in 0..3 {
                pos[i][k] += vel[i][k];
            }
        }
    }
    pos.iter()
        .zip(vel.iter())
        .map(|(p, v)| p.iter().map(|i| i.abs()).sum::<i32>() *
                      v.iter().map(|i| i.abs()).sum::<i32>())
        .sum()
}

fn main() {
    let input = std::io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.replace(|c| !char::is_numeric(c) && c != '-', " "))
        .map(|line| line.split(' ')
                 .filter_map(|i| i32::from_str(i).ok())
                 .collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let mut pos = input.iter()
        .map(|p| [p[0], p[1], p[2]])
        .collect::<Vec<_>>();
    let mut vel = vec![[0, 0, 0]; pos.len()];
    println!("Part 1: {}", run(pos.clone(), vel.clone(), 1000));

    let mut cycle = [0, 0, 0];
    for k in 0..3 {
        let mut seen = HashSet::new();
        seen.insert(pos.iter()
                    .zip(vel.iter())
                    .map(|(p, v)| (p[k], v[k]))
                    .collect::<Vec<(i32, i32)>>());
        // Loop searching for a repeat on this axis
        for n in 0.. {
            for i in 0..pos.len() {
                for j in 0..i {
                    if i == j {
                        continue;
                    }
                    if pos[i][k] < pos[j][k] {
                        vel[i][k] += 1;
                        vel[j][k] -= 1;
                    } else if pos[i][k] > pos[j][k] {
                        vel[i][k] -= 1;
                        vel[j][k] += 1;
                    }
                }
            }
            for i in 0..pos.len() {
                pos[i][k] += vel[i][k];
            }
            let key = pos.iter()
                        .zip(vel.iter())
                        .map(|(p, v)| (p[k], v[k]))
                        .collect::<Vec<(i32, i32)>>();
            if seen.contains(&key) {
                cycle[k] = n + 1 as usize;
                break;
            } else {
                seen.insert(key);
            }
        }
    }
    println!("Part 2: {}", cycle[0].lcm(&cycle[1]).lcm(&cycle[2]));
}
