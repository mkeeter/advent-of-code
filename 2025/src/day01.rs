pub fn solve(s: &str) -> (usize, usize) {
    let mut pos = 50i64;
    let mut count = 0usize;
    let mut any_count = 0usize;
    for line in s.lines() {
        let v: i64 = line.trim_start_matches(['L', 'R']).parse().unwrap();
        let sign = match line.as_bytes()[0] {
            b'L' => -1i64,
            b'R' => 1i64,
            _ => panic!("invalid line {line}"),
        };
        for _ in 0..v {
            pos += sign;
            pos = pos.rem_euclid(100);
            if pos == 0 {
                any_count += 1;
            }
        }

        if pos == 0 {
            count += 1;
        }
    }

    (count, any_count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let s = indoc::indoc! {"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "};
        let (a, b) = solve(s);
        assert_eq!(a, 3);
        assert_eq!(b, 6);
    }
}
