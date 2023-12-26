use indexmap::IndexMap;
use rand::Rng;
use std::{
    collections::HashMap,
    sync::atomic::{AtomicBool, Ordering},
    thread::available_parallelism,
};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Key(u16);

#[derive(Clone, Default, Debug)]
struct Graph {
    nodes: IndexMap<Key, usize>,
    edges: IndexMap<Key, IndexMap<Key, usize>>,
}

impl Graph {
    fn new() -> Self {
        Self::default()
    }
    fn insert_edge(&mut self, src: Key, dst: Key) {
        self.nodes.insert(src, 1);
        self.nodes.insert(dst, 1);
        self.edges.entry(src).or_default().insert(dst, 1);
        self.edges.entry(dst).or_default().insert(src, 1);
    }
    fn random_edge(&self) -> (Key, Key) {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..self.edges.len());
        let (src, e) = self.edges.get_index(i).unwrap();
        let j = rng.gen_range(0..e.len());
        let (dst, _) = e.get_index(j).unwrap();

        (*src, *dst) // not uniform, but good enough!
    }
    fn remove_edge(&mut self, edge: (Key, Key)) {
        let (src, dst) = edge;
        // We'll merge dst into src, since we have to pick a polarity
        *self.nodes.get_mut(&src).unwrap() += self.nodes.remove(&dst).unwrap();
        let e = self.edges.remove(&dst).unwrap();
        for (neighbor_name, n) in &e {
            let neighbor = self.edges.get_mut(neighbor_name).unwrap();
            let m = neighbor.remove(&dst).unwrap();
            assert_eq!(*n, m);
            *neighbor.entry(src).or_default() += n;
        }
        let src_conns = self.edges.get_mut(&src).unwrap();
        for (neighbor_name, n) in &e {
            *src_conns.entry(*neighbor_name).or_default() += n;
        }
        src_conns.remove(&src).unwrap();
    }
}

fn contract(mut g: Graph, n: usize) -> Graph {
    while g.nodes.len() > n {
        let e = g.random_edge();
        g.remove_edge(e);
    }
    g
}

fn recurse(g: &Graph) -> Option<usize> {
    if g.nodes.len() <= 6 {
        for _ in 0..2 {
            let g = contract(g.clone(), 2);
            let mut iter = g.nodes.keys();
            let src = iter.next().unwrap();
            let dst = iter.next().unwrap();
            if g.edges[src][dst] == 3 {
                return Some(g.nodes[src] * g.nodes[dst]);
            }
        }
        return None;
    }

    let n = g.nodes.len() / 4;
    let g = contract(g.clone(), n);
    for _ in 0..2 {
        if let Some(out) = recurse(&g) {
            return Some(out);
        }
    }
    None
}

fn search(g: &Graph, done: &AtomicBool) -> Option<usize> {
    while !done.load(Ordering::Acquire) {
        if let Some(out) = recurse(g) {
            done.store(true, Ordering::Release);
            return Some(out);
        }
    }
    None
}

fn get_name<'a>(names: &mut HashMap<&'a str, u16>, s: &'a str) -> Key {
    let n = names.len().try_into().unwrap();
    Key(*names.entry(s).or_insert(n))
}

pub fn solve(s: &str) -> (String, String) {
    let mut graph = Graph::new();
    let mut names = HashMap::new();
    for line in s.lines() {
        let mut iter = line.split(": ");
        let src = get_name(&mut names, iter.next().unwrap());
        for dst in iter.next().unwrap().split_whitespace() {
            let dst = get_name(&mut names, dst);
            graph.insert_edge(src, dst);
        }
    }

    let done = AtomicBool::new(false);
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
