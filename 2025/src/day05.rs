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

    let mut range_tree: BTreeMap<u64, u64> = BTreeMap::new();
    for r in ranges {
        let mut prev = range_tree.range(..r.end());
        println!("inserting {r:?}");
        match prev.next_back() {
            None => {
                println!("  no match");
                // empty tree
                range_tree.insert(*r.start(), *r.end());
            }
            Some((&start, &end)) => {
                println!("  got {start}, {end}");
                if end < *r.start() {
                    println!("    non-overlapping");
                    // non-overlapping intervals
                    range_tree.insert(*r.start(), *r.end());
                } else {
                    range_tree.remove(&start);
                    println!("    overlapping");
                    // overlapping intervals, grow the range
                    range_tree
                        .insert((*r.start()).min(start), (*r.end()).max(end));
                }
            }
        }
    }
    for r in &range_tree {
        println!("{r:?}");
    }
    let all_valid = range_tree.iter().map(|(start, end)| end - start + 1).sum();

    (item_count, all_valid)
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
