use std::collections::HashMap;

fn check<'a>(s: &'a str, can_make: &mut HashMap<&'a str, bool>) -> bool {
    if let Some(v) = can_make.get(s) {
        return *v;
    } else if s.len() == 1 {
        return false;
    }

    for i in 1..s.len() {
        let (a, b) = s.split_at(i);
        println!("checking {a:?}, {b:?}");
        if check(a, can_make) && check(b, can_make) {
            can_make.insert(s, true);
            return true;
        }
    }
    can_make.insert(s, false);
    false
}

pub fn solve(s: &str) -> (usize, usize) {
    let mut iter = s.lines();
    let mut can_make = iter
        .next()
        .unwrap()
        .split(", ")
        .map(|s| (s, true))
        .collect::<HashMap<_, _>>();
    can_make.insert("", true); // we can always make the empty string

    iter.next().unwrap(); // skip the blank link
    let count = iter
        .filter(|t| {
            println!();
            check(t, &mut can_make)
        })
        .count();

    (count, 0)
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
