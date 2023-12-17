use rayon::prelude::*;
use std::collections::HashMap;

fn recurse(
    row: Row<u8>,
    target: Row<usize>,
    seen: &mut HashMap<u32, usize>,
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

impl<'a, T: Copy + Clone + std::fmt::Debug> Row<'a, T> {
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

    #[must_use]
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

    #[must_use]
    fn map_first<F: FnMut(T) -> T>(&self, mut f: F) -> Self {
        if let Some(t) = &self.first {
            Self {
                first: Some(f(*t)),
                data: self.data,
            }
        } else {
            Self {
                first: Some(f(self.data[0])),
                data: &self.data[1..],
            }
        }
    }

    #[must_use]
    fn swap_first(&self, t: T) -> Self {
        self.map_first(|_| t)
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.first.iter().chain(self.data.iter())
    }
}

fn recurse_inner(
    mut row: Row<u8>,
    mut target: Row<usize>,
    seen: &mut HashMap<u32, usize>,
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
    } else if row.is_empty() {
        // The target is not empty, or we already have too many tiles
        0
    } else if leading_size == 0 {
        // Ambiguous case: we could either recurse with '.' or '#'
        let score_a = recurse(row.swap_first(b'#'), target, seen);
        let score_b = recurse(row.skip_first(), target, seen);
        score_a + score_b
    } else {
        match target.first().unwrap().cmp(&leading_size) {
            std::cmp::Ordering::Equal => {
                // Unambiguous case: if the run before the ambiguous `?` was
                // exactly our target length, then we have to replace it with
                // '.', which we can do by trimming the block.
                recurse(row.skip_first(), target.skip_first(), seen)
            }
            std::cmp::Ordering::Less => {
                // Our initial run of tiles is too long already
                0
            }
            std::cmp::Ordering::Greater => {
                // We have some trailing values, but not enough to reach the
                // target size.  We must put a '#' here and recurse.
                recurse(
                    row.swap_first(b'#'),
                    target.map_first(|t| t - leading_size),
                    seen,
                )
            }
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
    for (row, run) in rows.iter_mut().zip(runs.iter_mut()) {
        out += recurse(Row::new(row), Row::new(run), &mut HashMap::new());
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
            recurse(Row::new(row), Row::new(run), &mut HashMap::new())
        })
        .sum::<usize>();
    let p2 = out;

    (p1.to_string(), p2.to_string())
}
