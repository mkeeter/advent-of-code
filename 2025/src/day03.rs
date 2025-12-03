fn get_joltage(s: &str) -> u64 {
    let n = s.len();
    let (i, tens) = s.as_bytes()[..n - 1]
        .iter()
        .enumerate()
        .max_by_key(|(i, v)| (*v, std::cmp::Reverse(*i)))
        .unwrap();
    let ones = s.as_bytes()[i + 1..].iter().max().unwrap();
    u64::from(tens - b'0') * 10 + u64::from(ones - b'0')
}

pub fn solve(s: &str) -> (u64, u64) {
    let mut out = 0;
    for line in s.lines() {
        let j = get_joltage(line);
        println!("{line} => {j}");
        out += get_joltage(line);
    }
    (out, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_joltage() {
        assert_eq!(get_joltage("987654321111111"), 98);
        assert_eq!(get_joltage("811111111111119"), 89);
        assert_eq!(get_joltage("234234234234278"), 78);
        assert_eq!(get_joltage("818181911112111"), 92);
    }

    #[test]
    fn joltage_sum() {
        let s = indoc::indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        "};
        let (a, b) = solve(s);
        assert_eq!(a, 357);
        assert_eq!(b, 3121910778619);
    }
}
