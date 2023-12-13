use rayon::prelude::*;
use std::collections::BTreeMap;

fn recurse(
    row: Row<u8>,
    target: Row<usize>,
    seen: &mut BTreeMap<u32, usize>,
) -> usize {
    // Build a single `u32` key by byte-packing the relevant values
    assert!(row.first != Some(0xFF));
    assert!(target.first != Some(0xFF));
    let key: [u8; 4] = [
        row.first.unwrap_or(0xFF),
        row.data.len().try_into().unwrap(),
        target.first.unwrap_or(0xFF).try_into().unwrap(),
        target.data.len().try_into().unwrap(),
    ];
    let key = u32::from_le_bytes(key);
    if let Some(v) = seen.get(&key) {
        return *v;
    }
    let r = recurse_inner(row, target, seen);
    seen.insert(key, r);
    r
}

#[derive(Copy, Clone, Debug)]
struct Row<'a, T> {
    first: Option<T>,
    data: &'a [T],
}

impl<'a, T: Clone> Row<'a, T> {
    fn new(data: &'a [T]) -> Self {
        Self { first: None, data }
    }
    fn len(&self) -> usize {
        self.data.len() + self.first.is_some() as usize
    }
    fn is_empty(&self) -> bool {
        self.first.is_none() && self.data.is_empty()
    }
    fn first(&self) -> Option<&T> {
        self.first.as_ref().or_else(|| self.data.first())
    }

    fn skip_first(&self) -> Self {
        Self {
            first: None,
            data: if self.first.is_some() {
                self.data
            } else {
                &self.data[1..]
            },
        }
    }
    fn pop_first(&mut self) -> Option<T> {
        std::mem::take(&mut self.first).or_else(|| {
            if let Some(out) = self.data.get(0).cloned() {
                self.data = &self.data[1..];
                Some(out)
            } else {
                None
            }
        })
    }

    fn push_first(&mut self, t: T) {
        assert!(self.first.is_none());
        self.first = Some(t);
    }

    fn swap_first(&mut self, t: T) -> T {
        let out = self.pop_first().unwrap();
        self.push_first(t);
        out
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.first.iter().chain(self.data.iter())
    }
}

fn recurse_inner(
    mut row: Row<u8>,
    mut target: Row<usize>,
    seen: &mut BTreeMap<u32, usize>,
) -> usize {
    if row.len() < target.len() {
        return 0;
    }
    let leading_size = loop {
        // Trim the end of the row
        while row.first() == Some(&b'.') {
            row = row.skip_first();
        }
        // Trim all '#' from the end of the row
        let mut leading_size = 0;
        while row.first() == Some(&b'#') {
            leading_size += 1;
            row = row.skip_first();
        }
        // We've found a gap (or the end of the line)
        if matches!(row.first(), Some(&b'.') | None) {
            if target.is_empty() {
                return (leading_size == 0) as usize;
            } else if leading_size != *target.first().unwrap() {
                return 0;
            }
            target = target.skip_first();
        } else {
            break leading_size;
        }
    };
    assert!(row.is_empty() || *row.first().unwrap() == b'?');

    if target.is_empty() {
        // If the target is empty, then we better not have any springs
        ((leading_size == 0) && row.iter().all(|c| *c != b'#')) as usize
    } else if row.is_empty() || leading_size > *target.first().unwrap() {
        // The target is not empty, or we already have too many tiles
        return 0;
    } else if *target.first().unwrap() == leading_size {
        // Unambiguous case: if the run before the ambiguous `?` was exactly our
        // target length, then we have to replace it with '.', which we can do
        // by trimming the block.
        recurse(row.skip_first(), target.skip_first(), seen)
    } else if leading_size == 0 {
        // Ambiguous case: we could either recurse with '.' or '#'
        row.swap_first(b'#');
        let score_a = recurse(row, target, seen);
        let score_b = recurse(row.skip_first(), target, seen);
        score_a + score_b
    } else {
        // We have some trailing values, but not enough to reach the target
        // size.  We must put a '#' here and recurse.
        let t = target.pop_first().unwrap();
        target.push_first(t - leading_size);
        row.swap_first(b'#');
        recurse(row, target, seen)
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
        out += recurse(Row::new(row), Row::new(run), &mut BTreeMap::new());
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
        .map(|(row, run)| {
            recurse(Row::new(row), Row::new(run), &mut BTreeMap::new())
        })
        .sum::<usize>();
    let p2 = out;

    (p1.to_string(), p2.to_string())
}
