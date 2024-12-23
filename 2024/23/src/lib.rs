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

    /// Returns the largest clique starting at the given node
    fn max_clique(&self) -> Vec<Name> {
        let mut seen = HashSet::new();
        let mut best = vec![];
        for n in &self.nodes {
            if !seen.contains(n) {
                let mut clique = vec![];
                self.recurse(&mut vec![*n], &mut clique);
                seen.extend(clique.iter().cloned());
                if clique.len() > best.len() {
                    best = clique;
                }
            }
        }
        best
    }

    fn recurse(&self, clique: &mut Vec<Name>, best: &mut Vec<Name>) {
        if clique.len() > best.len() {
            *best = clique.clone();
        }
        let last = *clique.last().unwrap();
        for n in self.nodes.range(last..).skip(1) {
            if clique.iter().all(|p| self.contains_edge(*p, *n)) {
                clique.push(*n);
                self.recurse(clique, best);
                clique.pop();
            }
        }
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

    let mut count = 0;
    for (a, b) in &graph.edges {
        for n in graph.nodes.range(*b..).skip(1) {
            if [a, b, n].iter().any(|c| c.starts_with('t'))
                && graph.contains_edge(*a, *n)
                && graph.contains_edge(*b, *n)
            {
                count += 1;
            }
        }
    }

    let clique = graph.max_clique();
    let mut password = String::new();
    for (i, name) in clique.iter().enumerate() {
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
