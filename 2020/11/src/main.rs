use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let input: HashMap<(i32, i32), bool> = input.lines().enumerate()
        .flat_map(|(y, line)| line.chars().enumerate()
            .filter(|(_, c)| *c == 'L')
            .map(move |(x, c)| ((x as i32, y as i32), c)))
        .map(|(xy, _)| (xy, false))
        .collect();

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
