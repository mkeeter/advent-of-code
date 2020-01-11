use std::collections::HashSet;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let favorite = i32::from_str(&input.trim()).unwrap();

    let filled = |x: i32, y: i32| -> bool {
        if x < 0 || y < 0 {
            true
        } else {
            (x*x + 3*x + 2*x*y + y + y*y + favorite).count_ones() % 2 == 1
        }
    };

    let mut todo = Vec::new();
    todo.push((1, 1));

    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut part1 = None;
    let mut part2 = None;
    for steps in 0.. {
        if todo.iter().any(|&(x, y)| x == 31 && y == 39) {
            part1 = Some(steps);
        }
        if steps == 51 {
            part2 = Some(seen.len());
        }
        if part1.is_some() && part2.is_some() {
            break;
        }
        todo = todo.into_iter()
            .filter(|&(x, y)| seen.insert((x, y)))
            .flat_map(|(x, y)| directions.iter()
                      .map(move |(dx, dy)| (x + dx, y + dy)))
            .filter(|(x, y)| !filled(*x, *y))
            .collect();

    }
    println!("Part 1: {}", part1.unwrap());
    println!("Part 2: {}", part2.unwrap());
}
