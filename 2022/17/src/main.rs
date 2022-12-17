use anyhow::{bail, Result};
use std::{
    collections::{btree_map::Entry, BTreeMap, BTreeSet},
    io::Read,
};

const PIECES: [&[(i64, i64)]; 5] = [
    &[(0, 0), (1, 0), (2, 0), (3, 0)],         // -
    &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // +
    &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // ⅃
    &[(0, 0), (0, 1), (0, 2), (0, 3)],         // |
    &[(0, 0), (0, 1), (1, 0), (1, 1)],         // ■
];

/// Flood fill to find reachable points, then discard any point that's
/// not currently reachable to keep the grid size small.
fn simplify_grid(grid: &mut BTreeSet<(i64, i64)>) {
    let ymax = grid.iter().map(|g| g.1).max().unwrap_or(0);
    let mut reachable = BTreeSet::new();
    reachable.insert((0, ymax + 1));
    let mut seen = BTreeSet::new();
    let mut todo = vec![(0, ymax + 1)];
    while let Some(t) = todo.pop() {
        if !seen.insert(t) {
            continue;
        }
        for (dx, dy) in [(0, -1), (1, 0), (-1, 0)] {
            let x = t.0 + dx;
            let y = t.1 + dy;
            reachable.insert((x, y));
            if (0..7).contains(&x) && y >= 0 && !grid.contains(&(x, y)) {
                todo.push((x, y));
            }
        }
    }
    grid.retain(|p| reachable.contains(p));
}

fn run(input: &str, num_pieces: usize) -> Result<i64> {
    let mut grid: BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut cycle_detector = BTreeMap::new();

    let mut gas_iter = input.chars().enumerate().cycle().peekable();
    let mut piece_iter = PIECES.iter().enumerate().cycle();

    let mut y_offset = 0;

    let mut count = 0;
    while count < num_pieces {
        simplify_grid(&mut grid);

        // Normalize Y coordinates, shifting the grid down
        let ymin = grid.iter().map(|g| g.1).min().unwrap_or(0);
        y_offset += ymin;
        grid = grid.into_iter().map(|p| (p.0, p.1 - ymin)).collect();

        // Pick out where we are in the gas and piece cycle
        let (piece_index, piece) = piece_iter.next().unwrap();
        let gas_index = gas_iter.peek().unwrap().0;

        // Check whether we've seen this point before
        let key = (grid.clone(), gas_index, piece_index);
        let height = grid.iter().map(|g| g.1).max().unwrap_or(0) + y_offset;
        match cycle_detector.entry(key) {
            Entry::Occupied(o) => {
                let (prev_count, prev_height) = o.get();
                let dc = count - prev_count;
                let dy = height - prev_height;

                let n = (num_pieces - count - 1) / dc;
                count += dc * n;
                y_offset += dy * n as i64;
            }
            Entry::Vacant(v) => {
                v.insert((count, height));
            }
        }

        // Drop the rock
        let mut x = 2;
        let mut y = grid.iter().map(|g| g.1).max().unwrap_or(-1) + 4;
        loop {
            // Apply the gas operation
            let dx = match gas_iter.next().unwrap().1 {
                '<' => -1,
                '>' => 1,
                c => bail!("Invalid gas direction '{c}'"),
            };
            if piece.iter().all(|(px, py)| {
                let x = px + x + dx;
                let y = py + y;
                (0..7).contains(&x) && !grid.contains(&(x, y))
            }) {
                x += dx;
            }

            // Piece falls
            let dy = -1;
            if piece.iter().all(|(px, py)| {
                let x = px + x;
                let y = py + y + dy;
                y >= 0 && !grid.contains(&(x, y))
            }) {
                y += dy;
            } else {
                grid.extend(piece.iter().map(|(px, py)| (x + px, y + py)));
                break;
            }
        }
        count += 1;
    }

    Ok(grid.iter().map(|g| g.1).max().unwrap_or(0) + 1 + y_offset)
}

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = input.trim();

    println!("Part 1: {}", run(input, 2022)?);
    println!("Part 2: {}", run(input, 1000000000000)?);
    Ok(())
}
