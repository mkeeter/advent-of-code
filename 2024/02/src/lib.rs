fn is_safe(row: &[i8]) -> bool {
    let sign = (row[1] - row[0]).signum();
    if sign == 0 {
        return false;
    }
    for (a, b) in row.iter().zip(row.iter().skip(1)) {
        let d = *b - *a;
        if d.signum() != sign || d.abs() > 3 {
            return false;
        }
    }
    true
}

fn any_safe(row: &[i8], scratch: &mut Vec<i8>) -> bool {
    for i in 0..row.len() {
        scratch.clear();
        for (j, v) in row.iter().enumerate() {
            if j != i {
                scratch.push(*v)
            }
        }
        if is_safe(scratch) {
            return true;
        }
    }
    false
}

pub fn solve(s: &str) -> (usize, usize) {
    let mut rows = vec![];
    for line in s.lines() {
        rows.push(
            line.split_ascii_whitespace()
                .map(|i| i.parse::<i8>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    let safe = rows.iter().filter(|row| is_safe(row)).count();
    let mut scratch = vec![];
    let modified = rows
        .iter()
        .filter(|row| any_safe(row, &mut scratch))
        .count();

    (safe, modified)
}
