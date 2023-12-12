use rayon::prelude::*;
use std::collections::BTreeMap;

fn recurse(
    row: &mut [u8],
    target: &mut [usize],
    seen: &mut BTreeMap<Vec<u8>, usize>,
) -> usize {
    let mut key = Vec::with_capacity(row.len() + target.len());
    key.extend(row.iter().cloned());
    key.extend(target.iter().map(|v| {
        let v: u8 = (*v).try_into().unwrap();
        v
    }));
    if let Some(v) = seen.get(&key) {
        return *v;
    }
    let r = recurse_inner(row, target, seen);
    seen.insert(key, r);
    r
}

fn recurse_inner(
    mut row: &mut [u8],
    mut target: &mut [usize],
    seen: &mut BTreeMap<Vec<u8>, usize>,
) -> usize {
    if row.len() < target.len() {
        return 0;
    }
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
        // We've found a gap (or the end of the line)
        if matches!(row.first(), Some(&b'.') | None) {
            if target.is_empty() {
                return (leading_size == 0) as usize;
            } else if leading_size != target[0] {
                return 0;
            }
            target = &mut target[1..];
        } else {
            break leading_size;
        }
    };
    assert!(row.is_empty() || row[0] == b'?');

    if target.is_empty() {
        // If the target is empty, then we better not have any springs
        ((leading_size == 0) && row.iter().all(|c| *c != b'#')) as usize
    } else if row.is_empty() {
        // The target is not empty, so we have failed
        return 0;
    } else if target[0] == leading_size {
        // Unambiguous case: if the run before the ambiguous `?` was exactly our
        // target length, then we have to replace it with '.', which we can do
        // by trimming the block.
        recurse(&mut row[1..], &mut target[1..], seen)
    } else if leading_size > target[0] {
        0
    } else if leading_size == 0 {
        // Ambiguous case: we could either recurse with '.' or '#'
        row[0] = b'#';
        let score_a = recurse(row, target, seen);
        let score_b = recurse(&mut row[1..], target, seen);
        row[0] = b'?';
        score_a + score_b
    } else {
        // We have some trailing values, but not enough to reach the target
        // size.  We must put a '#' here and recurse.
        row[0] = b'#';
        target[0] -= leading_size;
        let score = recurse(row, target, seen);
        row[0] = b'?';
        target[0] += leading_size;
        score
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
        out += recurse(row, run, &mut BTreeMap::new());
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
    let mut runs: Vec<Vec<usize>> = runs
        .into_iter()
        .map(|v| std::iter::repeat(v).take(5).flatten().collect())
        .collect();

    let out = rows
        .iter_mut()
        .zip(runs.iter_mut())
        .par_bridge()
        .map(|(row, run)| recurse(row, run, &mut BTreeMap::new()))
        .sum::<usize>();
    let p2 = out;

    (p1.to_string(), p2.to_string())
}
