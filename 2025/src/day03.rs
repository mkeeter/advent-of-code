fn get_joltage(s: &str, digits: usize) -> u64 {
    recurse(s.as_bytes(), digits)
}

fn recurse(s: &[u8], digits: usize) -> u64 {
    let Some(digits) = digits.checked_sub(1) else {
        return 0;
    };

    // Find the largest number, preferring earlier values in the list
    let n = s.len();
    let (i, v) = s[..n - digits]
        .iter()
        .enumerate()
        .max_by_key(|(i, v)| (*v, std::cmp::Reverse(*i)))
        .unwrap();

    u64::from(v - b'0') * 10u64.pow(u32::try_from(digits).unwrap())
        + recurse(&s[i + 1..], digits)
}

pub fn solve(s: &str) -> (u64, u64) {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in s.lines() {
        part1 += get_joltage(line, 2);
        part2 += get_joltage(line, 12);
    }
    (part1, part2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_joltage() {
        assert_eq!(get_joltage("987654321111111", 2), 98);
        assert_eq!(get_joltage("811111111111119", 2), 89);
        assert_eq!(get_joltage("234234234234278", 2), 78);
        assert_eq!(get_joltage("818181911112111", 2), 92);

        assert_eq!(get_joltage("987654321111111", 12), 987654321111);
        assert_eq!(get_joltage("811111111111119", 12), 811111111119);
        assert_eq!(get_joltage("234234234234278", 12), 434234234278);
        assert_eq!(get_joltage("818181911112111", 12), 888911112111);
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
