use rayon::prelude::*;

/// Checks a single row
///
/// Returns `Ok(true)` for a valid row, `Ok(false)` for an invalid row, and
/// `Err(i)` for an ambiguous row (where `i` is the first ambiguous tile)
fn check<'a>(
    mut row: &'a mut [u8],
    mut target: &'a [usize],
) -> Result<bool, (&'a mut [u8], &'a [usize], usize)> {
    let mut i = 0; // index into row
    let mut run = 0;
    while i < row.len() {
        if run > 0 && run > target[0] {
            return Ok(false);
        }
        match row[i] {
            b'?' => return Err((row, target, i)),
            b'.' => {
                if run > 0 {
                    if run != target[0] {
                        return Ok(false);
                    }
                    run = 0;
                    target = &target[1..];
                }
                row = &mut row[i + 1..];
                i = 0;
            }
            b'#' => {
                if target.is_empty() {
                    return Ok(false);
                }
                run += 1;
                i += 1;
            }
            c => panic!("invalid character {c}"),
        }
    }
    Ok(target.is_empty() || (target.len() == 1 && run == target[0]))
}

fn count(row: &mut [u8], target: &[usize]) -> usize {
    match check(row, target) {
        Ok(true) => 1,
        Ok(false) => 0,
        Err((row, target, i)) => {
            assert_eq!(row[i], b'?');
            row[i] = b'#';
            let score_a = count(row, target);
            row[i] = b'.';
            let score_b = count(row, target);
            row[i] = b'?';
            score_a + score_b
        }
    }
}

pub fn solve(s: &str) -> (String, String) {
    let mut rows: Vec<Vec<u8>> = vec![];
    let mut runs: Vec<Vec<usize>> = vec![];
    for line in s.split('\n').filter(|line| !line.is_empty()) {
        let mut iter = line.split_ascii_whitespace();
        let row = iter.next().unwrap();
        let mut out = vec![];
        let mut empty = false;
        for c in row.chars() {
            assert!(c.is_ascii());
            if c == '.' {
                if !empty {
                    out.push(c as u8);
                    empty = true;
                }
            } else {
                out.push(c as u8);
                empty = false;
            }
        }
        rows.push(out);
        let rs = iter.next().unwrap();
        runs.push(rs.split(',').map(|s| s.parse::<usize>().unwrap()).collect());
    }
    let mut out = 0;
    for (row, run) in rows.iter_mut().zip(runs.iter()) {
        out += count(row, run);
    }
    let p1 = out;

    let mut rows: Vec<Vec<u8>> = rows
        .into_iter()
        .map(|mut v| {
            v.push(b'?');
            let mut out: Vec<u8> =
                std::iter::repeat(v).take(5).flatten().collect();
            out.pop().unwrap();
            out
        })
        .collect();
    let runs: Vec<Vec<usize>> = runs
        .into_iter()
        .map(|v| std::iter::repeat(v).take(5).flatten().collect())
        .collect();

    let counter = std::sync::atomic::AtomicUsize::new(0);
    let total = rows.len();
    let out = rows
        .iter_mut()
        .zip(runs.iter())
        .par_bridge()
        .map(|(row, run)| {
            let n = count(row, run);
            let c = counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            println!("{} / {}", c + 1, total);
            n
        })
        .sum::<usize>();
    let p2 = out;

    (p1.to_string(), p2.to_string())
}
