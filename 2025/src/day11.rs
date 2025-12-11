use std::collections::HashMap;

pub fn solve(s: &str) -> (u64, u64) {
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
    let part1 = search("you", "out", &edges, &mut Default::default());
    (part1, 0)
}

fn search<'a, 'b>(
    start: &'a str,
    end: &'a str,
    edges: &'b HashMap<&'a str, Vec<&'a str>>,
    cache: &'b mut HashMap<(&'a str, &'a str), u64>,
) -> u64 {
    if start == end {
        1
    } else if let Some(v) = cache.get(&(start, end)) {
        *v
    } else {
        let out = edges[start]
            .iter()
            .map(|start| search(start, end, edges, cache))
            .sum();
        cache.insert((start, end), out);
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
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
        let (a, b) = solve(s);
        assert_eq!(a, 5);
        assert_eq!(b, 0);
    }
}
