use bimap::BiHashMap;
use util::{BitSet, TupleSet};

////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////

struct Graph {
    edges: TupleSet<(usize, usize)>,
    count: usize,
}

impl Graph {
    fn new(count: usize) -> Self {
        Self {
            edges: TupleSet::new((count, count)),
            count,
        }
    }

    fn insert(&mut self, a: usize, b: usize) {
        self.edges.insert((a.min(b), b.max(a)));
    }

    fn contains_edge(&self, a: usize, b: usize) -> bool {
        self.edges.contains((a.min(b), b.max(a)))
    }

    /// Returns the largest clique starting at the given node
    fn max_clique(&self) -> Vec<usize> {
        let mut seen = BitSet::new(self.count);
        let mut best = vec![];
        for n in 0..self.count {
            if !seen.get(n) {
                let mut clique = vec![];
                self.recurse(&mut vec![n], &mut clique);
                for c in &clique {
                    seen.insert(*c);
                }
                if clique.len() > best.len() {
                    best = clique;
                }
            }
        }
        best
    }

    fn recurse(&self, clique: &mut Vec<usize>, best: &mut Vec<usize>) {
        if clique.len() > best.len() {
            *best = clique.clone();
        }
        let last = *clique.last().unwrap();
        for n in (last + 1)..self.count {
            if clique.iter().all(|p| self.contains_edge(*p, n)) {
                clique.push(n);
                self.recurse(clique, best);
                clique.pop();
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn solve(s: &str) -> (usize, String) {
    let mut name_to_index = BiHashMap::new();
    let mut edges = Vec::new();
    for line in s.lines() {
        let mut iter = line.chars();
        let a = Name([iter.next().unwrap(), iter.next().unwrap()]);
        assert_eq!(iter.next().unwrap(), '-');
        let b = Name([iter.next().unwrap(), iter.next().unwrap()]);

        for n in [a, b] {
            let i = name_to_index.len();
            let _ = name_to_index.insert_no_overwrite(n, i);
        }
        edges.push((
            *name_to_index.get_by_left(&a).unwrap(),
            *name_to_index.get_by_left(&b).unwrap(),
        ));
    }
    let mut has_t = BitSet::new(name_to_index.len());
    for (n, i) in &name_to_index {
        if n.starts_with('t') {
            has_t.insert(*i);
        }
    }

    let mut graph = Graph::new(name_to_index.len());
    for (a, b) in edges {
        graph.insert(a, b);
    }

    let mut count = 0;
    for (a, b) in graph.edges.iter() {
        for n in b + 1..graph.count {
            if graph.contains_edge(a, n)
                && graph.contains_edge(b, n)
                && [a, b, n].iter().any(|c| has_t.get(*c))
            {
                count += 1;
            }
        }
    }

    let mut clique = graph
        .max_clique()
        .into_iter()
        .map(|i| *name_to_index.get_by_right(&i).unwrap())
        .collect::<Vec<Name>>();
    clique.sort();
    let mut password = String::new();
    for (i, name) in clique.iter().enumerate() {
        password += &format!("{}{name}", if i == 0 { "" } else { "," },);
    }
    (count, password)
}

////////////////////////////////////////////////////////////////////////////////

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
