use std::io::BufRead;
use std::cmp::min;
use std::collections::HashMap;

fn reduce((i, j): (i32, i32)) -> (i32, i32) {
    if i == 0 {
        (0, j / j.abs())
    } else if j == 0 {
        (i / i.abs(), 0)
    } else {
        for q in 2..=min(i.abs(), j.abs()) {
            if i % q == 0 && j % q == 0 {
                return reduce((i / q, j / q))
            }
        }
        (i, j)
    }
}

fn main() {
    let asteroids = std::io::stdin().lock()
        .lines()
        .map(|line| line.unwrap())
        .enumerate()
        .flat_map(|(y, line)| line.chars()
             .enumerate()
             .filter(|(_x, c)| *c == '#')
             .map(|(x, _c)| (x as i32, y as i32))
             .collect::<Vec<_>>())
        .collect::<Vec<(i32, i32)>>();

    let mut best = 0;
    let mut best_pos = (0, 0);
    for a in asteroids.iter() {
        let mut seen = HashMap::new();
        for b in asteroids.iter() {
            if a != b {
                let delta = (b.0 - a.0, b.1 - a.1);
                let distance = delta.0.pow(2) + delta.1.pow(2);
                let angle = reduce(delta);
                let v = seen.entry(angle).or_insert(distance);
                if *v < distance {
                    *v = distance;
                }
            }
        }
        println!("Asteroid at {:?} can see {}", a, seen.len());
        if seen.len() > best {
            best_pos = *a;
            best = seen.len();
        }
    }
    println!("\nPart 1: {:?} can see {}", best_pos, best);
}
