use std::collections::BTreeSet;

pub fn solve(s: &str) -> (String, String) {
    let lines = s.lines().collect::<Vec<&str>>();

    let mut prev_index = 0;
    let mut scores = vec![];
    for line in lines {
        let mut iter = line.split(": ");
        let mut game = iter.next().unwrap().split_whitespace();
        assert_eq!(game.next().unwrap(), "Card");

        let index = game.next().unwrap().parse::<usize>().unwrap();
        assert_eq!(prev_index + 1, index);
        prev_index = index; // Checking that cards are in order

        let mut iter =
            iter.next().unwrap().split(" | ").map(|r| {
                r.split_whitespace().map(|v| v.parse::<usize>().unwrap())
            });
        let nums = iter.next().unwrap().collect::<Vec<usize>>();
        let winners = iter.next().unwrap().collect::<BTreeSet<usize>>();
        scores.push(nums.into_iter().filter(|n| winners.contains(n)).count());
    }

    // Per-card score
    let mut out = 0;
    for &s in scores.iter().filter(|s| **s > 0) {
        out += 1 << (s - 1);
    }
    let p1 = out;

    let mut count = vec![1; scores.len()];
    for (i, s) in scores.iter().enumerate() {
        for v in 0..*s {
            count[i + 1 + v] += count[i];
        }
    }
    let p2 = count.into_iter().sum::<usize>();
    (p1.to_string(), p2.to_string())
}
