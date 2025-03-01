use util::get_integers;

pub fn solve(s: &str) -> (usize, usize) {
    let mut list1 = vec![];
    let mut list2 = vec![];
    let mut iter = get_integers(s);
    let mut max_item = 0usize;
    while let Some(a) = iter.next() {
        list1.push(a);
        let b = iter.next().unwrap();
        max_item = max_item.max(b);
        list2.push(b);
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

    (distance, score)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let s = indoc::indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "};
        let (a, b) = solve(s);
        assert_eq!(a, 11);
        assert_eq!(b, 31);
    }
}
