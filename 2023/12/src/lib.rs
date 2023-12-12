use std::collections::VecDeque;

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

/*
#[derive(Copy, Clone)]
enum Tile {
    Spring,
    Unknown,
    // Empty tiles are implicit between chunks
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let c = match self {
            Self::Spring => '#',
            Self::Unknown => '?',
        };
        write!(f, "{c}")
    }
}

struct Chunk(Vec<Tile>);
impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for c in &self.0 {
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

struct Row(Vec<Chunk>);
impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for (i, c) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, "..")?;
            }
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

fn make_chunks(row: &str, target: &[usize]) {
    let chunks = row
        .split('.')
        .filter(|c| !c.is_empty())
        .collect::<VecDeque<&str>>();

    // Peel off unambiguous chunks from the front
    if chunks.len() == target.len() {
        // we're done, perfect match
    } else if chunks.len() > target.len() {
        // too many chunks, I hope some of them are all-ambiguous
        for c in chunks {
            // phew
            if c.chars().all(|c| c == '?') {}
        }
    }

    chunks[0].contains('?');
    todo!()
}
*/

pub fn solve(s: &str) -> (String, String) {
    let mut rows: Vec<Vec<u8>> = vec![];
    let mut runs: Vec<Vec<usize>> = vec![];
    for line in s.split('\n').filter(|line| !line.is_empty()) {
        let mut iter = line.split_ascii_whitespace();
        let row = iter.next().unwrap();
        rows.push(
            row.chars()
                .map(|c| {
                    assert!(c.is_ascii());
                    c as u8
                })
                .collect(),
        );
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

    let mut out = 0;
    for (row, run) in rows.iter_mut().zip(runs.iter()) {
        let n = count(row, run);
        println!("{n}");
        out += n;
    }
    let p2 = out;

    (p1.to_string(), p2.to_string())
}
