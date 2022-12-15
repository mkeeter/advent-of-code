use anyhow::{anyhow, bail, Result};
use parse_display::{Display, FromStr};
use rayon::prelude::*;
use smallvec::SmallVec;
use std::{cell::RefCell, io::BufRead};

#[derive(Copy, Clone, Debug, FromStr, Display)]
#[display("Sensor at x={sx}, y={sy}: closest beacon is at x={bx}, y={by}")]
struct Reading {
    sx: i64,
    sy: i64,
    bx: i64,
    by: i64,
}

type Interval = (i64, i64);

fn process_row(readings: &[Reading], row: i64) -> SmallVec<[(i64, i64); 2]> {
    thread_local!(static SCRATCH: RefCell<Vec<Interval>> = RefCell::default());
    SCRATCH.with(|intervals| {
        let mut intervals = intervals.borrow_mut();
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
                if b.0 >= a.0 && b.0 <= a.1 + 1 {
                    a.1 = a.1.max(b.1);
                    i += 1;
                } else {
                    break;
                }
            }
            merged.push(a);
        }
        merged
    })
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
    let row = if readings[0].sx == 2 { 10 } else { 2_000_000 };

    let sum: i64 = process_row(&readings, row)
        .into_iter()
        .map(|(start, end)| (end - start))
        .sum();
    println!("Part 1: {sum}");

    let freq = (0..row * 2)
        .into_par_iter()
        .find_map_any(|y| {
            let merged = process_row(&readings, y);
            if merged.len() == 2 {
                let x = merged[0].1 + 1;
                Some(x * 4000000 + y)
            } else {
                None
            }
        })
        .ok_or_else(|| anyhow!("Could not find row"))?;
    println!("Part 2: {freq}");

    Ok(())
}
