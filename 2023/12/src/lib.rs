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

    // Early exit: if the target is empty but the row still contains tiles
    if target.is_empty() {
        return Ok(!row.contains(&b'#'));
    }
    // Early exit: if the row is empty (and the target is non-empty, above)
    if row.is_empty() {
        return Ok(false);
    }

    // Semi-early exit: if the max run is larger than any of our target runs
    let mut max_run = 0;
    let mut start = None;
    for (i, c) in row.iter().enumerate() {
        if *c == b'#' {
            if start.is_none() {
                start = Some(i);
            }
        } else if let Some(s) = start {
            let run = i - s;
            if run > max_run {
                max_run = run;
            }
            start = None;
        }
    }
    if max_run > *target.iter().max().unwrap() {
        return Ok(false);
    }

    let min_total = row.iter().filter(|c| **c == b'#').count();
    if min_total > target.iter().sum() {
        return Ok(false);
    }

    let mut min_runs = 0;
    let mut running = false;
    for c in &*row {
        if *c == b'.' {
            running = false;
        } else if *c == b'#' && !running {
            min_runs += 1;
            running = true;
        }
    }
    if min_runs > target.len() {
        return Ok(false);
    }

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

fn recurse(
    mut row: &mut [u8],
    mut target: &mut [usize],
    indent: usize,
) -> usize {
    /*
    for _ in 0..indent * 2 {
        print!(" ");
    }
    println!("{} {target:?}", std::str::from_utf8(row).unwrap());
    */
    let leading_size = loop {
        // Trim the end of the row
        while row.first() == Some(&b'.') {
            row = &mut row[1..];
        }
        // Trim all '#' from the end of the row
        let mut leading_size = 0;
        while row.first() == Some(&b'#') {
            leading_size += 1;
            row = &mut row[1..];
        }
        // We've found either a gap or an ambiguity, check the first target
        if row.first() == Some(&b'.') || (row.is_empty() && !target.is_empty())
        {
            if leading_size != target.first().cloned().unwrap_or(0) {
                return 0;
            }
            target = &mut target[1..];
        } else {
            break leading_size;
        }
    };
    /*
    for _ in 0..indent * 2 {
        print!(" ");
    }
    println!(
        "{} {target:?} {leading_size}",
        std::str::from_utf8(row).unwrap()
    );
    */

    if target.is_empty() {
        if leading_size > 0 || row.iter().any(|c| *c == b'#') {
            return 0;
        } else {
            return 1;
        }
    }

    if row.is_empty() {
        assert!(!target.is_empty());
        return 0;
    }
    assert_eq!(row[0], b'?');

    if target[0] == leading_size {
        // Unambiguous case: if the run before the ambiguous `?` was exactly our
        // target length, then we have to replace it with '.', which we can do
        // by trimming the block.
        recurse(&mut row[1..], &mut target[1..], indent + 1)
    } else if leading_size > target[0] {
        0
    } else if leading_size == 0 {
        row[0] = b'#';
        let score_a = recurse(row, target, indent + 1);
        row[0] = b'.';
        let score_b = recurse(row, target, indent + 1);
        row[0] = b'?';
        score_a + score_b
    } else {
        // We have some trailing values, but not enough to reach the target
        // size.  We must put a '#' here and recurse.
        row[0] = b'#';
        target[0] -= leading_size;
        let score = recurse(row, target, indent + 1);
        row[0] = b'?';
        target[0] += leading_size;
        score
    }
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

struct Chunk(Vec<u8>);

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for c in &self.0 {
            write!(f, "{}", *c as char)?;
        }
        Ok(())
    }
}

impl Chunk {
    fn can_split(&self) -> bool {
        for (i, &c) in self.0.iter().enumerate() {
            if c == b'?' && (i > 0 && i < self.0.len() - 1) {
                return true;
            }
        }
        false
    }
    fn split_point(&self) -> Option<usize> {
        let midpoint = self.0.len() / 2;
        self.0
            .iter()
            .enumerate()
            .filter(|(_i, k)| **k == b'?')
            .max_by_key(|(i, _k)| i.abs_diff(midpoint))
            .map(|(i, _k)| i)
    }

    fn split(&self) -> (Self, Self) {
        let i = self.split_point().unwrap();
        (
            Chunk(self.0[0..i].to_vec()),
            Chunk(self.0[i + 1..].to_vec()),
        )
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
    for (row, run) in rows.iter_mut().zip(runs.iter_mut()) {
        let r = recurse(row, run, 0);
        println!("{r}");
        out += r;
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
            let start = std::time::Instant::now();
            let c = counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            println!("{c}: {} {run:?}", std::str::from_utf8(row).unwrap());
            let n = count(row, run);
            println!("{c} / {total} done in {:?}", start.elapsed());
            n
        })
        .sum::<usize>();
    let p2 = out;

    (p1.to_string(), p2.to_string())
}
