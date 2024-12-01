pub fn solve(s: &str) -> (String, String) {
    let mut list1 = vec![];
    let mut list2 = vec![];
    let mut max_item = 0usize;
    for line in s.lines() {
        let mut iter = line.split_ascii_whitespace();
        let a = iter.next().and_then(|s| s.parse::<usize>().ok()).unwrap();
        let b = iter.next().and_then(|s| s.parse::<usize>().ok()).unwrap();
        list1.push(a);
        list2.push(b);
        max_item = max_item.max(b);
    }
    assert_eq!(list1.len(), list2.len());
    list1.sort_unstable();
    list2.sort_unstable();

    let mut count = vec![0u16; max_item + 1];
    let distance: usize = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    for b in list2 {
        count[b] += 1;
    }
    let score: usize = list1.into_iter().map(|a| a * count[a] as usize).sum();

    (distance.to_string(), score.to_string())
}
