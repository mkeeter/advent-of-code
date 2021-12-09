use std::collections::HashSet;
use std::io::BufRead;

const DXDY: [(i64, i64); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn main() {
    let heights = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c as u32 - '0' as u32)
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    let h = |x: i64, y: i64| -> Option<u32> {
        heights
            .get(y as usize)
            .and_then(|row| row.get(x as usize).cloned())
    };
    let mut low = Vec::new();
    for (y, row) in heights.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            if DXDY.iter().all(|(dx, dy)| {
                h(x as i64 + dx, y as i64 + dy)
                    .map(|w| w > *v)
                    .unwrap_or(true)
            }) {
                low.push((x as i64, y as i64));
            }
        }
    }
    println!(
        "Part 1: {}",
        low.iter().map(|&(i, j)| h(i, j).unwrap() + 1).sum::<u32>()
    );

    let mut basins = vec![HashSet::new(); low.len()];
    let mut todo = low.into_iter().enumerate().collect::<Vec<_>>();
    while let Some((i, (x, y))) = todo.pop() {
        if !basins[i].insert((x, y)) {
            continue;
        }
        for (dx, dy) in DXDY {
            if h(x + dx, y + dy).map(|w| w != 9).unwrap_or(false) {
                todo.push((i, (x + dx, y + dy)));
            }
        }
    }
    let mut basin_sizes = basins.iter().map(|v| v.len()).collect::<Vec<_>>();
    basin_sizes.sort_unstable();
    println!(
        "Part 2: {}",
        basin_sizes.iter().rev().take(3).product::<usize>()
    );
}
