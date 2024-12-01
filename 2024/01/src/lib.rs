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

    let distance: usize = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    let mut score = 0;
    let mut i = 0;
    let mut j = 0;
    while j < list2.len() {
        let v = list2[j];
        while i < list1.len() && list1[i] < v {
            i += 1;
        }
        let mut count = 0;
        while i < list1.len() && list1[i] == v {
            i += 1;
            count += v;
        }
        while j < list2.len() && list2[j] == v {
            j += 1;
            score += count;
        }
    }

    (distance.to_string(), score.to_string())
}
