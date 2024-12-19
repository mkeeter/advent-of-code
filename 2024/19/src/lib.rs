use std::collections::{HashMap, HashSet};

fn check<'a>(
    s: &'a str,
    count: &mut HashMap<&'a str, u64>,
    roots: &HashSet<&'a str>,
) -> u64 {
    if let Some(v) = count.get(s) {
        return *v;
    }

    let n = roots.contains(s) as u64
        + (1..s.len())
            .map(|i| {
                let (a, b) = s.split_at(i);
                if roots.contains(a) {
                    check(b, count, roots)
                } else {
                    0
                }
            })
            .sum::<u64>();
    count.insert(s, n);
    n
}

pub fn solve(s: &str) -> (u64, u64) {
    let mut iter = s.lines();
    let roots = iter.next().unwrap().split(", ").collect::<HashSet<_, _>>();
    let mut count = HashMap::new();

    let mut can_make = 0;
    iter.next().unwrap(); // skip the blank link
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
