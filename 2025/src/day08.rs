use std::collections::{BTreeSet, HashMap};

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
    run(&pts, 1000)
}

pub fn run(pts: &[[u64; 3]], connections: usize) -> (u64, u64) {
    let mut distances = BTreeSet::new();
    for (j, a) in pts.iter().enumerate() {
        for (i, b) in pts[0..j].iter().enumerate() {
            let d: u64 =
                a.iter().zip(b).map(|(a, b)| a.abs_diff(*b).pow(2)).sum();
            distances.insert((d, i, j));
        }
    }
    let cs = distances
        .iter()
        .take(connections)
        .map(|(_d, i, j)| (*i, *j))
        .collect::<Vec<_>>();
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
    let part1 = sorted[sorted.len() - 3..].iter().product();

    // Dumb brute-force approach
    let cs = distances
        .into_iter()
        .map(|(_d, i, j)| (i, j))
        .collect::<Vec<_>>();
    let mut ids = (0..pts.len()).collect::<Vec<_>>();
    let mut cliques = std::iter::repeat_n(1, pts.len()).collect::<Vec<_>>();
    let mut live_count = pts.len();
    for i in 0..cs.len() {
        let mut changed = true;
        while std::mem::take(&mut changed) {
            for &(lo, hi) in &cs[0..i] {
                if ids[hi] != ids[lo] {
                    changed = true;
                    let (to, from) = if ids[hi] < ids[lo] {
                        (lo, hi)
                    } else {
                        (hi, lo)
                    };
                    cliques[ids[to]] -= 1;
                    if cliques[ids[to]] == 0 {
                        live_count -= 1;
                    }
                    ids[to] = ids[from];
                    cliques[ids[from]] += 1;
                }
            }
        }
        if live_count == 1 {
            let (a, b) = cs[i - 1];
            return (part1, pts[a][0] * pts[b][0]);
        }
    }
    unreachable!()
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
        let (a, b) = run(&pts, 10);
        assert_eq!(a, 40);
        assert_eq!(b, 25272);
    }
}
