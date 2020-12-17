use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let mut input: HashSet<(i64, i64, i64)> = HashSet::new();
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                input.insert((x as i64, y as i64, 0));
            }
        }
    }
    let mut grid = input.clone();

    let tern = |i| match i {
        0 => -1,
        1 => 1,
        2 => 0,
        _ => panic!("Invalid tern {}", i),
    };

    for _i in 0..6 {
        let xmin = grid.iter().map(|p| p.0).min().unwrap();
        let xmax = grid.iter().map(|p| p.0).max().unwrap();
        let ymin = grid.iter().map(|p| p.1).min().unwrap();
        let ymax = grid.iter().map(|p| p.1).max().unwrap();
        let zmin = grid.iter().map(|p| p.2).min().unwrap();
        let zmax = grid.iter().map(|p| p.2).max().unwrap();

        let mut next: HashSet<(i64, i64, i64)> = HashSet::new();
        for x in (xmin - 1)..=(xmax + 1) {
            for y in (ymin - 1)..=(ymax + 1) {
                for z in (zmin - 1)..=(zmax + 1) {
                    let count = (0..26).filter(|n| {
                        let dx = tern(n % 3);
                        let dy = tern((n / 3) % 3);
                        let dz = tern((n / 9) % 3);
                        grid.contains(&(x + dx, y + dy, z + dz))
                    }).count();
                    let now_active = (grid.contains(&(x, y, z)) && count == 2)
                        || (count == 3);
                    if now_active {
                        next.insert((x, y, z));
                    }
                }
            }
        }
        std::mem::swap(&mut grid, &mut next);
    }
    println!("{}", grid.len());

    let mut grid: HashSet<(i64, i64, i64, i64)> = input.into_iter()
        .map(|(x, y, z)| (x, y, z, 0))
        .collect();
    for _i in 0..6 {
        let xmin = grid.iter().map(|p| p.0).min().unwrap();
        let xmax = grid.iter().map(|p| p.0).max().unwrap();
        let ymin = grid.iter().map(|p| p.1).min().unwrap();
        let ymax = grid.iter().map(|p| p.1).max().unwrap();
        let zmin = grid.iter().map(|p| p.2).min().unwrap();
        let zmax = grid.iter().map(|p| p.2).max().unwrap();
        let wmin = grid.iter().map(|p| p.3).min().unwrap();
        let wmax = grid.iter().map(|p| p.3).max().unwrap();

        let mut next: HashSet<(i64, i64, i64, i64)> = HashSet::new();
        for x in (xmin - 1)..=(xmax + 1) {
            for y in (ymin - 1)..=(ymax + 1) {
                for z in (zmin - 1)..=(zmax + 1) {
                    for w in (wmin - 1)..=(wmax + 1) {
                        let count = (0..80).filter(|n| {
                            let dx = tern(n % 3);
                            let dy = tern((n / 3) % 3);
                            let dz = tern((n / 9) % 3);
                            let dw = tern((n / 27) % 3);
                            grid.contains(&(x + dx, y + dy, z + dz, w + dw))
                        }).count();
                        let now_active = (grid.contains(&(x, y, z, w)) && count == 2)
                            || (count == 3);
                        if now_active {
                            next.insert((x, y, z, w));
                        }
                    }
                }
            }
        }
        std::mem::swap(&mut grid, &mut next);
    }
    println!("{}", grid.len());
}
