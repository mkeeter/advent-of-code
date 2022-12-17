use anyhow::{bail, Result};
use std::{collections::BTreeSet, io::Read};

const PIECES: [&[(i64, i64)]; 5] = [
    &[(0, 0), (1, 0), (2, 0), (3, 0)],         // -
    &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // +
    &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // L (reversed)
    &[(0, 0), (0, 1), (0, 2), (0, 3)],         // |
    &[(0, 0), (0, 1), (1, 0), (1, 1)],         // square
];

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = input.trim();

    let mut grid: BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut gas = input.chars().cycle();
    for piece in PIECES.iter().cycle().take(2022) {
        let mut x = 2;
        let mut y = grid.iter().map(|g| g.1).max().unwrap_or(-1) + 4;
        loop {
            // Apply the gas operation
            let dx = match gas.next().unwrap() {
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
    }

    let max = grid.iter().map(|g| g.1).max().unwrap_or(0) + 1;
    println!("Part 1: {max}");
    Ok(())
}
