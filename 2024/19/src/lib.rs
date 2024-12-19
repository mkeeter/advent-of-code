use std::collections::{HashMap, HashSet};

fn check<'a>(
    s: &'a str,
    count: &mut HashMap<&'a str, u64>,
    roots: &HashSet<&'a str>,
) -> u64 {
    if let Some(v) = count.get(s) {
        return *v;
    }

    let n = (1..s.len())
        .map(|i| s.split_at(i))
        .filter(|(a, _b)| roots.contains(a))
        .map(|(_, b)| check(b, count, roots))
        .sum::<u64>()
        + roots.contains(s) as u64;
    count.insert(s, n);
    n
}

pub fn solve(s: &str) -> (u64, u64) {
    let mut iter = s.lines();
    let roots = iter.next().unwrap().split(", ").collect::<HashSet<_, _>>();
    iter.next().unwrap(); // skip the blank link
                          //
    let mut count = HashMap::new();
    let mut can_make = 0;
    let mut combinations = 0;
    for t in iter {
        let n = check(t, &mut count, &roots);
        if n > 0 {
            can_make += 1;
        }
        combinations += n;
    }

    (can_make, combinations)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const EXAMPLE: &str = indoc::indoc! {"
            r, wr, b, g, bwu, rb, gb, br

            brwrr
            bggr
            gbbr
            rrbgbr
            ubwu
            bwurrg
            brgr
            bbrgwb
        "};
        assert_eq!(solve(EXAMPLE), (6, 16));
    }
}
