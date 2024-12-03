pub fn solve(s: &str) -> (u64, u64) {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let p1 = re
        .captures_iter(s)
        .map(|m| {
            let a = m[1].parse::<u64>().unwrap();
            let b = m[2].parse::<u64>().unwrap();
            a * b
        })
        .sum();

    let re =
        regex::Regex::new(r"(do\(\)|mul\((\d+),(\d+)\)|don't\(\))").unwrap();
    let (p2, _) =
        re.captures_iter(s)
            .fold((0, true), |(sum, enabled), m| match &m[0] {
                "do()" => (sum, true),
                "don't()" => (sum, false),
                _ => {
                    let d = if enabled {
                        let a = m[2].parse::<u64>().unwrap();
                        let b = m[3].parse::<u64>().unwrap();
                        a * b
                    } else {
                        0
                    };
                    (sum + d, enabled)
                }
            });
    (p1, p2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const EX1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let (a, _b) = solve(EX1);
        assert_eq!(a, 161);

        const EX2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let (_a, b) = solve(EX2);
        assert_eq!(b, 48);
    }
}
