pub fn solve(s: &str) -> (String, String) {
    let mut empty_rows = vec![];
    let mut empty_cols = vec![];
    let mut galaxies = vec![];
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((x, y));
                for (p, empty) in [(x, &mut empty_cols), (y, &mut empty_rows)] {
                    if p >= empty.len() {
                        empty.resize(p + 1, 1);
                    }
                    empty[p] = 0;
                }
            }
        }
    }
    let make_cumsum = |vs: &[usize], scale: usize| -> Vec<usize> {
        let mut out: Vec<usize> = vec![];
        for v in vs {
            let prev = out.last().unwrap_or(&0);
            out.push(prev + v * scale);
        }
        out
    };
    let run = |scale| {
        let row_sum = make_cumsum(&empty_rows, scale);
        let col_sum = make_cumsum(&empty_cols, scale);

        let mut out = 0;
        for (i, (ax, ay)) in galaxies.iter().cloned().enumerate() {
            for (bx, by) in galaxies.iter().cloned().skip(i + 1) {
                let dx = bx.abs_diff(ax) + col_sum[ax].abs_diff(col_sum[bx]);
                let dy = by.abs_diff(ay) + row_sum[ay].abs_diff(row_sum[by]);
                out += dx + dy;
            }
        }
        out
    };

    let p1 = run(1);
    let p2 = run(1_000_000 - 1);
    (p1.to_string(), p2.to_string())
}
