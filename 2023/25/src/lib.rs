use rand::Rng;
use std::{
    collections::BTreeMap,
    sync::atomic::{AtomicBool, Ordering},
    thread::available_parallelism,
};

#[derive(Clone, Default, Debug)]
struct Graph<'a> {
    nodes: BTreeMap<&'a str, usize>,
    edges: BTreeMap<&'a str, BTreeMap<&'a str, usize>>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Self::default()
    }
    fn insert_edge(&mut self, src: &'a str, dst: &'a str) {
        self.nodes.insert(src, 1);
        self.nodes.insert(dst, 1);
        self.edges.entry(src).or_default().insert(dst, 1);
        self.edges.entry(dst).or_default().insert(src, 1);
    }
    fn random_edge(&self) -> (&'a str, &'a str) {
        let num_edges = self.edges.values().map(|e| e.len()).sum();
        let mut i = rand::thread_rng().gen_range(0..num_edges);
        for (src, edges) in &self.edges {
            if i < edges.len() {
                let (dst, _) = edges.iter().nth(i).unwrap();
                return (src, dst);
            } else {
                i -= edges.len();
            }
        }
        unreachable!();
    }
    fn remove_edge(&mut self, edge: (&'a str, &'a str)) {
        let (src, dst) = edge;
        // We'll merge dst into src, since we have to pick a polarity
        *self.nodes.get_mut(src).unwrap() += self.nodes.remove(dst).unwrap();
        let e = self.edges.remove(dst).unwrap();
        for (neighbor_name, n) in &e {
            let neighbor = self.edges.get_mut(neighbor_name).unwrap();
            let m = neighbor.remove(dst).unwrap();
            assert_eq!(*n, m);
            *neighbor.entry(src).or_default() += n;
            *self
                .edges
                .get_mut(src)
                .unwrap()
                .entry(neighbor_name)
                .or_default() += n;
        }
        self.edges.get_mut(src).unwrap().remove(src).unwrap();
    }
}

fn contract(mut g: Graph, n: usize) -> Graph {
    while g.nodes.len() > n {
        let e = g.random_edge();
        g.remove_edge(e);
    }
    g
}

fn recurse(g: Graph) -> Option<usize> {
    let n = ((g.nodes.len() as f64) / 2f64.sqrt()).round() as usize + 1;
    if n < 6 {
        let ga = contract(g.clone(), 2);
        let gb = contract(g.clone(), 2);

        for g in [ga, gb] {
            let mut iter = g.nodes.keys();
            let src = iter.next().unwrap();
            let dst = iter.next().unwrap();
            if g.edges[src][dst] == 3 {
                return Some(g.nodes[src] * g.nodes[dst]);
            }
        }
        None
    } else {
        let g = contract(g, n);
        recurse(g)
    }
}

fn search(g: &Graph, done: &AtomicBool) -> Option<usize> {
    while !done.load(Ordering::Acquire) {
        if let Some(out) = recurse(g.clone()) {
            done.store(true, Ordering::Release);
            return Some(out);
        }
    }
    None
}

pub fn solve(s: &str) -> (String, String) {
    let mut graph = Graph::new();
    for line in s.lines() {
        let mut iter = line.split(": ");
        let src = iter.next().unwrap();
        for dst in iter.next().unwrap().split_whitespace() {
            graph.insert_edge(src, dst);
        }
    }

    let done = std::sync::atomic::AtomicBool::new(false);
    let p1 = std::thread::scope(|s| {
        let mut h = vec![];
        for _ in 0..available_parallelism().unwrap().into() {
            h.push(s.spawn(|| search(&graph, &done)));
        }
        let mut sol = None;
        for h in h {
            if let Some(v) = h.join().unwrap() {
                sol = Some(v);
            }
        }
        sol.unwrap()
    });
    (p1.to_string(), "⭐".to_owned())
}
