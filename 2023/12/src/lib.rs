use rayon::prelude::*;

fn recurse(mut row: &mut [u8], mut target: &mut [usize]) -> usize {
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
        recurse(&mut row[1..], &mut target[1..])
    } else if leading_size > target[0] {
        0
    } else if leading_size == 0 {
        row[0] = b'#';
        let score_a = recurse(row, target);
        row[0] = b'.';
        let score_b = recurse(row, target);
        row[0] = b'?';
        score_a + score_b
    } else {
        // We have some trailing values, but not enough to reach the target
        // size.  We must put a '#' here and recurse.
        row[0] = b'#';
        target[0] -= leading_size;
        let score = recurse(row, target);
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
        let r = recurse(row, run);
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
    let mut runs: Vec<Vec<usize>> = runs
        .into_iter()
        .map(|v| std::iter::repeat(v).take(5).flatten().collect())
        .collect();

    let counter = std::sync::atomic::AtomicUsize::new(0);
    let total = rows.len();
    let out = rows
        .iter_mut()
        .zip(runs.iter_mut())
        .par_bridge()
        .map(|(row, run)| {
            let start = std::time::Instant::now();
            let c = counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            println!("{c}: {} {run:?}", std::str::from_utf8(row).unwrap());
            let n = recurse(row, run);
            println!("{c} / {total} done in {:?}", start.elapsed());
            n
        })
        .sum::<usize>();
    let p2 = out;

    (p1.to_string(), p2.to_string())
}
