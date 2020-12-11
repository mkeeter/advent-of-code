use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let mut input: HashMap<(i32, i32), bool> = HashMap::new();
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            if c == 'L' {
                input.insert((x as i32, y as i32), false);
            }
        }
    }

    const NEIGHBORS: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1),
                                        ( 0, -1), /* 0, */ ( 0, 1),
                                        ( 1, -1), ( 1, 0), ( 1, 1)];
    let mut chairs = input.clone();
    let mut next = HashMap::new();
    let mut changed = true;
    while changed {
        changed = false;
        for ((x, y), filled) in chairs.iter() {
            let count = NEIGHBORS.iter()
                .filter_map(|(dx, dy)| chairs.get(&(x + dx, y + dy)))
                .filter(|f| **f)
                .count();

            let val = (*filled && !(count >= 4)) ||
                      (!*filled && count == 0);
            changed |= val != *filled;
            next.insert((*x, *y), val);
        }
        std::mem::swap(&mut chairs, &mut next);
    }
    println!("Part 1: {}", chairs.iter().filter(|(_, v)| **v).count());

    let mut changed = true;
    let mut chairs = input.clone();
    let mut next = HashMap::new();
    while changed {
        changed = false;
        for x in 0..10 {
            for y in 0..10 {
                if let Some(c) = chairs.get(&(x, y)) {
                    print!("{}", if *c { '#' } else { 'L' });
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();

        for ((x, y), filled) in chairs.iter() {
            let count = NEIGHBORS.iter()
                .filter_map(|(dx, dy)| (1..150).filter_map(|i| chairs.get(&(x + i*dx, y + i*dy))).next())
                .filter(|f| **f)
                .count();

            let val = (*filled && !(count >= 5)) ||
                      (!*filled && count == 0);
            changed |= val != *filled;
            next.insert((*x, *y), val);
        }
        std::mem::swap(&mut chairs, &mut next);
    }
    println!("Part 2: {}", chairs.iter().filter(|(_, v)| **v).count());
}
