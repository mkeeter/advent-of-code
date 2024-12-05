use std::collections::{HashMap, HashSet};

fn check(row: &[u8], illegal: &HashMap<u8, HashSet<u8>>) -> bool {
    if row.is_empty() {
        true
    } else if row[1..]
        .iter()
        .any(|b| illegal.get(&row[0]).map(|v| v.contains(b)).unwrap_or(false))
    {
        false
    } else {
        check(&row[1..], illegal)
    }
}

fn sort(row: &mut [u8], illegal: &HashMap<u8, HashSet<u8>>) -> u8 {
    // Sort enough of the list to find the middle item
    for n in 0..row.len() / 2 + 1 {
        let i = (0..row.len() - n)
            .position(|i| {
                (0..row.len() - n).all(|j| {
                    illegal
                        .get(&row[i])
                        .map(|v| !v.contains(&row[j]))
                        .unwrap_or(true)
                })
            })
            .unwrap();
        row.swap(row.len() - n - 1, i);
    }
    row[row.len() / 2]
}

pub fn solve(s: &str) -> (u64, u64) {
    let mut illegal: HashMap<u8, HashSet<u8>> = HashMap::new();
    let mut runs: Vec<Vec<u8>> = vec![];
    for line in s.lines() {
        if line.contains('|') {
            let mut iter = line.split('|');
            let a = iter.next().and_then(|s| s.parse::<u8>().ok()).unwrap();
            let b = iter.next().and_then(|s| s.parse::<u8>().ok()).unwrap();
            illegal.entry(b).or_default().insert(a);
        } else if line.contains(',') {
            runs.push(
                line.split(',').map(|i| i.parse::<u8>().unwrap()).collect(),
            );
        }
    }

    let mut sum = 0u64;
    let mut sorted = 0u64;
    for r in runs.iter_mut() {
        if check(r, &illegal) {
            sum += u64::from(r[r.len() / 2]);
        } else {
            sorted += u64::from(sort(r, &illegal));
        }
    }
    (sum, sorted)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const EXAMPLE: &str = indoc::indoc! {"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
        "};
        assert_eq!(solve(EXAMPLE), (143, 123));
    }
}
