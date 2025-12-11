use std::collections::{HashMap, HashSet};

fn get_edges(s: &str) -> HashMap<&str, Vec<&str>> {
    let mut edges = HashMap::new();
    for line in s.lines() {
        let mut iter = line.split(": ");
        let start = iter.next().unwrap();
        let line = iter.next().unwrap();
        assert!(iter.next().is_none());
        let ends = line.split_whitespace().collect::<Vec<_>>();
        let prev = edges.insert(start, ends);
        assert!(prev.is_none(), "duplicate start node '{start}'");
    }
    edges
}

fn part1<'a, 'b>(edges: &'b HashMap<&'a str, Vec<&'a str>>) -> u64 {
    search(
        "you",
        "out",
        &edges,
        &mut Default::default(),
        &mut Default::default(),
    )
}

fn part2<'a, 'b>(edges: &'b HashMap<&'a str, Vec<&'a str>>) -> u64 {
    let mut required = ["dac", "fft"].into_iter().collect();
    search("svr", "out", &edges, &mut required, &mut Default::default())
}

pub fn solve(s: &str) -> (u64, u64) {
    let edges = get_edges(s);
    (part1(&edges), part2(&edges))
}

fn search<'a, 'b>(
    start: &'a str,
    end: &'a str,
    edges: &'b HashMap<&'a str, Vec<&'a str>>,
    required: &'b mut HashSet<&'a str>,
    cache: &'b mut HashMap<Vec<&'a str>, u64>,
) -> u64 {
    if start == end {
        if required.is_empty() { 1 } else { 0 }
    } else {
        let mut key = required.iter().cloned().collect::<Vec<_>>();
        key.sort();
        key.push(start);
        key.push(end);
        if let Some(v) = cache.get(&key) {
            *v
        } else {
            let prev = required.remove(start);
            let out = edges[start]
                .iter()
                .map(|start| search(start, end, edges, required, cache))
                .sum();
            cache.insert(key, out);
            if prev {
                required.insert(start);
            }
            out
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let s = indoc::indoc! {"
            aaa: you hhh
            you: bbb ccc
            bbb: ddd eee
            ccc: ddd eee fff
            ddd: ggg
            eee: out
            fff: out
            ggg: out
            hhh: ccc fff iii
            iii: out
        "};
        let edges = get_edges(s);
        assert_eq!(part1(&edges), 5);
    }

    #[test]
    fn test_part2() {
        let s = indoc::indoc! {"
            svr: aaa bbb
            aaa: fft
            fft: ccc
            bbb: tty
            tty: ccc
            ccc: ddd eee
            ddd: hub
            hub: fff
            eee: dac
            dac: fff
            fff: ggg hhh
            ggg: out
            hhh: out
        "};
        let edges = get_edges(s);
        assert_eq!(part2(&edges), 2);
    }
}
