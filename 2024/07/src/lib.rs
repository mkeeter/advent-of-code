use rayon::prelude::*;
use util::get_integers;

#[derive(Debug)]
struct Equation {
    total: u64,
    values: Vec<u64>,
}

impl Equation {
    fn is_valid(&self) -> bool {
        Self::recurse(self.total, self.values[0], &self.values[1..], false)
    }
    fn is_valid_concat(&self) -> bool {
        Self::recurse(self.total, self.values[0], &self.values[1..], true)
    }
    fn recurse(accum: u64, target: u64, slice: &[u64], concat: bool) -> bool {
        if let Some((v, next)) = slice.split_last() {
            [
                if accum > *v { Some(accum - *v) } else { None },
                if accum % *v == 0 {
                    Some(accum / *v)
                } else {
                    None
                },
                if concat {
                    let p = 10u64.pow(v.ilog10() + 1);
                    if accum % p == *v {
                        Some(accum / p)
                    } else {
                        None
                    }
                } else {
                    None
                },
            ]
            .iter()
            .flatten()
            .any(|i| Self::recurse(*i, target, next, concat))
        } else {
            accum == target
        }
    }
}

pub fn solve(s: &str) -> (u64, u64) {
    s.par_lines()
        .map(|line| {
            let mut iter = get_integers::<u64>(line);
            let total = iter.next().unwrap();
            let values = iter.collect::<Vec<_>>();
            let e = Equation { total, values };
            if e.is_valid() {
                (e.total, e.total)
            } else if e.is_valid_concat() {
                (0, e.total)
            } else {
                (0, 0)
            }
        })
        .reduce(|| (0, 0), |(a, b), (c, d)| (a + c, b + d))
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
