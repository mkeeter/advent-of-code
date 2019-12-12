use std::io::BufRead;
use std::str::FromStr;

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

    for _ in 0..1000 {
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
    let energy = pos.iter()
        .zip(vel.iter())
        .map(|(p, v)| p.iter().map(|i| i.abs()).sum::<i32>() *
                      v.iter().map(|i| i.abs()).sum::<i32>())
        .sum::<i32>();
    println!("Part 1: {}", energy);
}
