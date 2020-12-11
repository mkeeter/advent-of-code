use std::collections::HashMap;
use std::io::Read;
use arrayvec::ArrayVec;

fn run<F>(ns: &Vec<ArrayVec::<[usize; 8]>>, pred: F) -> usize
    where F: Fn(bool, usize) -> bool
{
    let mut state = vec![true; ns.len()];
    let mut next = vec![false; ns.len()];
    while state != next {
        std::mem::swap(&mut state, &mut next);
        for (i, n) in ns.iter().enumerate() {
            next[i] = pred(state[i], n.iter().filter(|j| state[**j]).count());
        }
    }
    state.iter().filter(|j| **j).count()
}

fn table(chairs: &HashMap<(i32, i32), usize>, max_dist: i32)
    -> Vec<ArrayVec::<[usize; 8]>>
{
    const NEIGHBORS: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1),
                                        ( 0, -1), /* 0, */ ( 0, 1),
                                        ( 1, -1), ( 1, 0), ( 1, 1)];
    chairs.iter().map(|((x, y), _)|
        NEIGHBORS.iter()
            .filter_map(|(dx, dy)| (1..=max_dist)
                .filter_map(|i| chairs.get(&(x + i*dx, y + i*dy)))
                .next())
            .copied()
            .collect())
        .collect()
}

fn main() {
    // Build a map from chairs to positions in a Vec
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Failed to read input");

    // Unpack into a list of chairs
    let mut chairs: HashMap<(i32, i32), usize> = input.lines().enumerate()
        .flat_map(|(y, line)| line.chars().enumerate()
            .filter(|(_, c)| *c == 'L')
            .map(move |(x, _)| (x as i32, y as i32)))
        .zip(std::iter::repeat(0))
        .collect();

    // Assign indexes based on iteration order
    chairs.iter_mut().enumerate().for_each(|(i, c)| *c.1 = i);
    let chairs = chairs; // lock mutability

    // Build a list of neighbors checked by each chair, then solve
    let ns = table(&chairs, 1);
    let p1 = run(&ns, |filled, count| (filled && count < 4) || (count == 0));
    println!("Part 1: {}", p1);

    // Calculate the longest possible distance to travel
    let max = input.lines().map(|line| line.chars().count())
        .chain(std::iter::once(input.lines().count()))
        .max().unwrap() as i32;
    let ns = table(&chairs, max);
    let p2 = run(&ns, |filled, count| (filled && count < 5) || (count == 0));
    println!("Part 2: {}", p2);
}
