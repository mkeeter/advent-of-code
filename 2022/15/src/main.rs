use anyhow::{bail, Result};
use parse_display::{Display, FromStr};
use smallvec::SmallVec;
use std::io::BufRead;

#[derive(Copy, Clone, Debug, FromStr, Display)]
#[display("Sensor at x={sx}, y={sy}: closest beacon is at x={bx}, y={by}")]
struct Reading {
    sx: i64,
    sy: i64,
    bx: i64,
    by: i64,
}

fn process_row(
    readings: &[Reading],
    intervals: &mut Vec<(i64, i64)>,
    row: i64,
) -> SmallVec<[(i64, i64); 2]> {
    intervals.clear();
    for r in readings {
        let manhattan = (r.bx - r.sx).abs() + (r.by - r.sy).abs();
        let dy = (r.sy - row).abs();
        let dx = manhattan - dy;
        if dx >= 0 {
            intervals.push((r.sx - dx, r.sx + dx))
        }
    }
    intervals.sort();
    let mut i = 0;
    let mut merged = SmallVec::new();
    while let Some(mut a) = intervals.get(i).cloned() {
        i += 1;
        while let Some(b) = intervals.get(i) {
            // If these intervals overlap, then merge b into a
            if b.0 >= a.0 && b.0 <= a.1 {
                a.1 = a.1.max(b.1);
                i += 1;
            } else {
                break;
            }
        }
        merged.push(a);
    }
    merged
}

fn main() -> Result<()> {
    let readings = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse())
        .collect::<Result<Vec<Reading>, _>>()?;

    if readings.is_empty() {
        bail!("Empty input");
    }

    // Picking parameters based on example vs actual
    let row = if readings[0].sx == 2 { 10 } else { 2000000 };

    // Reuse this allocation everywhere
    let mut intervals = Vec::with_capacity(readings.len());

    let merged = process_row(&readings, &mut intervals, row);
    let sum: i64 = merged.iter().map(|(start, end)| (end - start)).sum();
    println!("Part 1: {sum}");

    for y in 0..row * 2 {
        let merged = process_row(&readings, &mut intervals, y);
        if merged.len() == 2 {
            let x = merged[0].1 + 1;
            println!("Part 2: {}", x * 4000000 + y);
            break;
        }
    }

    Ok(())
}
