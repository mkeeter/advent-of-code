use std::collections::{BTreeSet, HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Name([char; 2]);

impl Name {
    fn starts_with(&self, c: char) -> bool {
        self.0[0] == c
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0[0], self.0[1])
    }
}

#[derive(Default)]
struct Graph {
    edges: HashSet<(Name, Name)>,
    nodes: BTreeSet<Name>,
}

impl Graph {
    fn insert(&mut self, a: Name, b: Name) {
        self.edges
            .insert((std::cmp::min(a, b), std::cmp::max(a, b)));
        self.nodes.insert(a);
        self.nodes.insert(b);
    }

    fn contains_edge(&self, a: Name, b: Name) -> bool {
        self.edges
            .contains(&(std::cmp::min(a, b), std::cmp::max(a, b)))
    }
}

pub fn solve(s: &str) -> (usize, String) {
    let mut graph = Graph::default();
    for line in s.lines() {
        let mut iter = line.chars();
        let a = Name([iter.next().unwrap(), iter.next().unwrap()]);
        assert_eq!(iter.next().unwrap(), '-');
        let b = Name([iter.next().unwrap(), iter.next().unwrap()]);
        graph.insert(a, b);
    }

    let mut cliques = graph
        .edges
        .iter()
        .map(|(a, b)| vec![a, b])
        .collect::<Vec<_>>();
    let mut count = 0;
    for size in 3.. {
        let mut next = vec![];
        for c in &cliques {
            let last = *c.last().unwrap();
            for n in graph.nodes.range(*last..).skip(1) {
                if c.iter().all(|p| graph.contains_edge(**p, *n)) {
                    let mut c = c.clone();
                    c.push(n);
                    next.push(c);
                }
            }
        }
        if next.is_empty() {
            break;
        }
        cliques = next;
        if size == 3 {
            count = cliques
                .iter()
                .filter(|a| a.iter().any(|n| n.starts_with('t')))
                .count();
        }
    }
    assert_eq!(cliques.len(), 1);

    let c = cliques.pop().unwrap();
    assert!(c.is_sorted());
    let mut password = String::new();
    for (i, name) in c.iter().enumerate() {
        password += &format!("{}{name}", if i == 0 { "" } else { "," });
    }

    (count, password)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const EXAMPLE_1: &str = indoc::indoc! {"
            kh-tc
            qp-kh
            de-cg
            ka-co
            yn-aq
            qp-ub
            cg-tb
            vc-aq
            tb-ka
            wh-tc
            yn-cg
            kh-ub
            ta-co
            de-co
            tc-td
            tb-wq
            wh-td
            ta-ka
            td-qp
            aq-cg
            wq-ub
            ub-vc
            de-ta
            wq-aq
            wq-vc
            wh-yn
            ka-de
            kh-ta
            co-tc
            wh-qp
            tb-vc
            td-yn
        "};
        assert_eq!(solve(EXAMPLE_1).0, 7);

        const EXAMPLE_2: &str = indoc::indoc! {"
            ka-co
            ta-co
            de-co
            ta-ka
            de-ta
            ka-de
        "};
        assert_eq!(solve(EXAMPLE_2).1, "co,de,ka,ta");
    }
}
