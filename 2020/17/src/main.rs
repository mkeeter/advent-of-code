use std::io::BufRead;
use std::collections::HashSet;

use itertools::{Itertools, iproduct};

fn main() {
    let mut input: HashSet<(i64, i64)> = HashSet::new();
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                input.insert((x as i64, y as i64));
            }
        }
    }

    // Convert into a 3D grid by setting z = 0;
    let mut grid: HashSet<(i64, i64, i64)> = input.iter()
        .map(|&(x, y)| (x, y, 0))
        .collect();

    // Consistent ordering of n-dimensional neighbors!
    let tern = |i: usize, d: u32| [-1, 1, 0][(i / 3_usize.pow(d)) % 3];

    for _i in 0..6 {
        let xm = grid.iter().map(|p| p.0).minmax().into_option().unwrap();
        let ym = grid.iter().map(|p| p.1).minmax().into_option().unwrap();
        let zm = grid.iter().map(|p| p.2).minmax().into_option().unwrap();

        grid = iproduct!(
                (xm.0 - 1)..=(xm.1 + 1),
                (ym.0 - 1)..=(ym.1 + 1),
                (zm.0 - 1)..=(zm.1 + 1))
            .filter_map(|(x, y, z)| {
                let count = (0..(3_usize.pow(3) - 1)).filter(|&n| {
                    let dx = tern(n, 0);
                    let dy = tern(n, 1);
                    let dz = tern(n, 2);
                    grid.contains(&(x + dx, y + dy, z + dz))
                }).count();
                Some((x, y, z)).filter(|_|
                    (grid.contains(&(x, y, z)) && count == 2)
                    || (count == 3))
            })
            .collect();
    }
    println!("Part 1: {}", grid.len());

    // Convert into a 4D grid by setting w = 0;
    let mut grid: HashSet<(i64, i64, i64, i64)> = input.iter()
        .map(|&(x, y)| (x, y, 0, 0))
        .collect();

    for _i in 0..6 {
        let xm = grid.iter().map(|p| p.0).minmax().into_option().unwrap();
        let ym = grid.iter().map(|p| p.1).minmax().into_option().unwrap();
        let zm = grid.iter().map(|p| p.2).minmax().into_option().unwrap();
        let wm = grid.iter().map(|p| p.3).minmax().into_option().unwrap();

        grid = iproduct!(
                (xm.0 - 1)..=(xm.1 + 1),
                (ym.0 - 1)..=(ym.1 + 1),
                (zm.0 - 1)..=(zm.1 + 1),
                (wm.0 - 1)..=(wm.1 + 1))
            .filter_map(|(x, y, z, w)| {
                let count = (0..(3_usize.pow(4) - 1)).filter(|&n| {
                    let dx = tern(n, 0);
                    let dy = tern(n, 1);
                    let dz = tern(n, 2);
                    let dw = tern(n, 3);
                    grid.contains(&(x + dx, y + dy, z + dz, w + dw))
                }).count();
                Some((x, y, z, w)).filter(|_|
                    (grid.contains(&(x, y, z, w)) && count == 2)
                    || (count == 3))
                }).collect();
    }
    println!("Part 2: {}", grid.len());
}
