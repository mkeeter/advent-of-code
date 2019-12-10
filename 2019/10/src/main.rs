use std::io::BufRead;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use core::f32::consts::PI;

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
             .filter(|(_x, c)| *c != '.')
             .map(|(x, _c)| (x as i32, y as i32))
             .collect::<Vec<_>>())
        .collect::<HashSet<(i32, i32)>>();

    let mut best = 0;
    let mut pos = (0, 0);
    for a in asteroids.iter() {
        let mut seen = HashSet::new();
        for b in asteroids.iter() {
            if a != b {
                let delta = (b.0 - a.0, b.1 - a.1);
                let angle = reduce(delta);
                seen.insert(angle);
            }
        }
        if seen.len() > best {
            pos = *a;
            best = seen.len();
        }
    }
    println!("\nPart 1: {:?} can see {}", pos, best);

    let mut vaporized = 0;
    let mut asteroids = asteroids;
    while asteroids.len() > 1 {
        let mut seen = HashMap::new();
        for b in asteroids.iter() {
            if pos != *b {
                let delta = (b.0 - pos.0, b.1 - pos.1);
                let distance = delta.0.pow(2) + delta.1.pow(2);
                let angle = reduce(delta);
                let v = seen.entry(angle).or_insert((distance, b));
                if distance < v.0 {
                    *v = (distance, b);
                }
            }
        }
        let mut targets = seen
            .into_iter()
            .map(|((dx, dy), (_distance, pos))|
                 ((dx as f32).atan2(-dy as f32), *pos))
            .map(|(angle, pos)| if angle < 0f32 {
                (angle + 2f32 * PI, pos)
            } else {
                (angle, pos)
            })
            .collect::<Vec<_>>();
        targets.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        for (i, (_, p)) in targets.iter().enumerate() {
            if i + vaporized + 1 == 200 {
                println!("Part 2: {}", p.0 * 100 + p.1);
            }
            asteroids.remove(p);
        }
        vaporized += targets.len();
    }
}
