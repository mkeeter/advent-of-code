use std::collections::VecDeque;
use std::io::BufRead;

fn step(m: &mut Vec<Vec<u32>>) -> usize {
    let mut todo = VecDeque::new();
    for (y, row) in m.iter_mut().enumerate() {
        for (x, c) in row.iter_mut().enumerate() {
            *c += 1;
            if *c > 9 {
                todo.push_back((x, y));
            }
        }
    }

    let mut flashes = 0;
    let xmax = m[0].len() - 1;
    let ymax = m.len() - 1;
    while let Some((x, y)) = todo.pop_front() {
        if m[y][x] == 0 {
            continue;
        }
        m[y][x] = 0;
        flashes += 1;
        for y in y.saturating_sub(1)..=(y + 1).min(ymax) {
            for x in x.saturating_sub(1)..=(x + 1).min(xmax) {
                if m[y][x] != 0 {
                    m[y][x] += 1;
                }
                if m[y][x] > 9 {
                    todo.push_back((x, y));
                }
            }
        }
    }
    flashes
}

fn main() {
    let mut octopus = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c as u32 - '0' as u32)
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut flashes = 0;
    for i in 0.. {
        let f = step(&mut octopus);
        flashes += f;
        if i == 99 {
            println!("Part 1: {}", flashes);
        }
        if f == octopus.len() * octopus[0].len() {
            println!("Part 2: {}", i + 1);
            break;
        }
    }
}
