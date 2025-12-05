use std::collections::BTreeMap;

pub fn solve(s: &str) -> (u64, u64) {
    let mut iter = s.split("\n\n");
    let mut ranges = vec![];
    for line in iter.next().unwrap().lines() {
        let mut iter = line.split('-');
        let a: u64 = iter.next().unwrap().parse().unwrap();
        let b: u64 = iter.next().unwrap().parse().unwrap();
        assert!(iter.next().is_none());
        ranges.push(a..=b);
    }

    let mut ids = vec![];
    for line in iter.next().unwrap().lines() {
        ids.push(line.parse::<u64>().unwrap());
    }
    assert!(iter.next().is_none());

    let mut item_count = 0;
    'outer: for id in &ids {
        for r in &ranges {
            if r.contains(id) {
                item_count += 1;
                continue 'outer;
            }
        }
    }

    let mut range_tree: BTreeMap<u64, i64> = BTreeMap::new();
    for r in ranges {
        *range_tree.entry(*r.start()).or_default() += 1;
        *range_tree.entry(*r.end()).or_default() -= 1;
    }
    let mut start_pos = 0;
    let mut depth = 0;
    let mut sum = 0;
    for (v, delta) in range_tree {
        if depth == 0 {
            start_pos = v;
        }
        depth += delta;
        if depth == 0 {
            sum += v - start_pos + 1;
        }
    }
    assert_eq!(depth, 0);
    (item_count, sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let s = indoc::indoc! {"
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        "};
        let (a, b) = solve(s);
        assert_eq!(a, 3);
        assert_eq!(b, 14);
    }
}
