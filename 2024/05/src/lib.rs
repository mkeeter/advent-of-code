use util::{get_integers, BitSet};

struct PairSet(BitSet);

impl PairSet {
    fn new() -> Self {
        Self(BitSet::new(256 * 256))
    }
    fn get(&self, a: u8, b: u8) -> bool {
        self.0.get(a as usize * 256 + b as usize)
    }
    fn set(&mut self, a: u8, b: u8) {
        self.0.set(a as usize * 256 + b as usize)
    }
}

fn check(row: &[u8], illegal: &PairSet) -> bool {
    if row.is_empty() {
        true
    } else if row[1..].iter().any(|&b| illegal.get(row[0], b)) {
        false
    } else {
        check(&row[1..], illegal)
    }
}

fn sort(row: &mut [u8], illegal: &PairSet) -> u8 {
    sort_recurse(row, illegal, row.len() / 2);
    row[row.len() / 2]
}

fn sort_recurse(row: &mut [u8], illegal: &PairSet, stop: usize) {
    if row.len() > stop {
        let n = row.len();
        // Find an item that would be valid if placed at the end
        let i = (0..n)
            .position(|i| (0..n).all(|j| !illegal.get(row[i], row[j])))
            .unwrap();
        row.swap(n - 1, i);
        sort_recurse(&mut row[..n - 1], illegal, stop);
    }
}

pub fn solve(s: &str) -> (u64, u64) {
    let mut rules = true;
    let mut illegal = PairSet::new();
    let mut runs: Vec<Vec<u8>> = vec![];
    for line in s.lines() {
        if line.is_empty() {
            rules = false;
        } else if rules {
            let mut iter = get_integers::<u8>(line);
            let a = iter.next().unwrap();
            let b = iter.next().unwrap();
            illegal.set(b, a);
        } else {
            runs.push(get_integers::<u8>(line).collect());
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
