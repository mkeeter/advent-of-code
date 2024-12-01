use std::collections::HashMap;

pub fn solve(s: &str) -> (String, String) {
    let mut list1 = vec![];
    let mut list2 = vec![];
    for line in s.lines() {
        let mut iter = line.split_ascii_whitespace();
        let a = iter.next().and_then(|s| s.parse::<i64>().ok()).unwrap();
        let b = iter.next().and_then(|s| s.parse::<i64>().ok()).unwrap();
        list1.push(a);
        list2.push(b);
    }
    list1.sort_unstable();
    list2.sort_unstable();
    assert_eq!(list1.len(), list2.len());
    let distance: i64 = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (*a - *b).abs())
        .sum();

    let mut count: HashMap<i64, i64> = HashMap::new();
    for b in list2 {
        *count.entry(b).or_default() += 1;
    }
    let score: i64 = list1
        .into_iter()
        .map(|a| a * count.get(&a).cloned().unwrap_or(0))
        .sum();

    (distance.to_string(), score.to_string())
}
