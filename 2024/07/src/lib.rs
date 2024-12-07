use rayon::prelude::*;
use util::get_integers;

#[derive(Debug)]
struct Equation {
    total: u64,
    values: Vec<u64>,
}

impl Equation {
    fn is_valid(&self) -> bool {
        Self::recurse(self.total, self.values[0], &self.values[1..])
    }
    fn is_valid_any(&self) -> bool {
        Self::recurse_any(self.total, self.values[0], &self.values[1..])
    }
    fn recurse(total: u64, accum: u64, slice: &[u64]) -> bool {
        if slice.is_empty() {
            total == accum
        } else {
            Self::recurse(total, accum * slice[0], &slice[1..])
                || Self::recurse(total, accum + slice[0], &slice[1..])
        }
    }
    fn recurse_any(total: u64, accum: u64, slice: &[u64]) -> bool {
        if slice.is_empty() {
            total == accum
        } else if accum > total {
            false
        } else {
            Self::recurse_any(total, accum * slice[0], &slice[1..])
                || Self::recurse_any(total, accum + slice[0], &slice[1..])
                || Self::recurse_any(
                    total,
                    accum * 10u64.pow(slice[0].ilog10() + 1) + slice[0],
                    &slice[1..],
                )
        }
    }
}

pub fn solve(s: &str) -> (u64, u64) {
    let mut eqns = vec![];
    for line in s.lines() {
        let mut iter = get_integers::<u64>(line);
        let total = iter.next().unwrap();
        let values = iter.collect::<Vec<_>>();
        eqns.push(Equation { total, values })
    }
    let valid: u64 = eqns
        .par_iter()
        .filter(|e| e.is_valid())
        .map(|e| e.total)
        .sum();
    let any_valid: u64 = eqns
        .par_iter()
        .filter(|e| e.is_valid_any())
        .map(|e| e.total)
        .sum();

    (valid, any_valid)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        const EXAMPLE: &str = indoc::indoc! {"
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
        "};
        assert_eq!(solve(EXAMPLE), (3749, 11387));
    }
}
