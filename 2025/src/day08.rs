use std::collections::{BTreeMap, BTreeSet, HashMap};

fn parse(s: &str) -> Vec<[u64; 3]> {
    s.lines()
        .map(|line| {
            let mut iter = line.split(',');
            let pt = [(); 3].map(|()| iter.next().unwrap().parse().unwrap());
            assert!(iter.next().is_none());
            pt
        })
        .collect()
}

pub fn solve(s: &str) -> (u64, u64) {
    let pts = parse(s);
    let p1 = part1(&pts, 1000);
    (p1, 0)
}

pub fn part1(pts: &[[u64; 3]], connections: usize) -> u64 {
    let mut distances = BTreeSet::new();
    for (j, a) in pts.iter().enumerate() {
        for (i, b) in pts[0..j].iter().enumerate() {
            let d: u64 =
                a.iter().zip(b).map(|(a, b)| a.abs_diff(*b).pow(2)).sum();
            distances.insert((d, i, j));
        }
    }
    let cs = distances
        .into_iter()
        .take(connections)
        .map(|(_d, i, j)| (i, j))
        .collect::<BTreeSet<_>>();
    let mut ids = (0..pts.len()).collect::<Vec<_>>();
    let mut changed = true;
    while std::mem::take(&mut changed) {
        for &(lo, hi) in &cs {
            if ids[hi] != ids[lo] {
                changed = true;
                if ids[hi] < ids[lo] {
                    ids[lo] = ids[hi];
                } else {
                    ids[hi] = ids[lo];
                }
            }
        }
    }
    let mut cliques = HashMap::<usize, u64>::new();
    for i in ids {
        *cliques.entry(i).or_default() += 1;
    }
    let mut sorted = cliques.into_values().collect::<Vec<_>>();
    sorted.sort();
    sorted[sorted.len() - 3..].iter().product()
    // 10304 is too low
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        // Trailing spaces are load-bearing here!
        let s = indoc::indoc! {"
            162,817,812
            57,618,57
            906,360,560
            592,479,940
            352,342,300
            466,668,158
            542,29,236
            431,825,988
            739,650,466
            52,470,668
            216,146,977
            819,987,18
            117,168,530
            805,96,715
            346,949,466
            970,615,88
            941,993,340
            862,61,35
            984,92,344
            425,690,689
        "};
        let pts = parse(s);
        assert_eq!(part1(&pts, 10), 40);
    }
}
