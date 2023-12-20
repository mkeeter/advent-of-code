use std::collections::{HashMap, VecDeque};
use util::{FlatMap, FlatSet};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Gate {
    Broadcast,
    FlipFlop,
    Conjunction,
}

#[derive(Clone, Debug)]
struct Node {
    gate: Gate,
    out: smallvec::SmallVec<[u8; 8]>,
}

#[derive(Debug)]
struct State<'a> {
    nodes: &'a FlatMap<Node>,
    todo: VecDeque<(u8, u8, bool)>,
    conj_state: FlatMap<FlatMap<bool>>,
    flip_state: FlatMap<bool>,
}

impl<'a> State<'a> {
    fn new(nodes: &'a FlatMap<Node>) -> Self {
        // Gate activation: the gate is active at 0 and inactive at > 0
        let mut conj_state: FlatMap<FlatMap<bool>> = nodes
            .iter()
            .filter(|(_k, v)| v.gate == Gate::Conjunction)
            .map(|(k, _v)| (k, FlatMap::new()))
            .collect();
        for (k, n) in nodes.iter() {
            for t in &n.out {
                if let Some(c) = conj_state.get_mut(*t) {
                    c.insert(k, false);
                }
            }
        }

        let flip_state: FlatMap<bool> = nodes
            .iter()
            .filter(|(_k, v)| v.gate == Gate::FlipFlop)
            .map(|(k, _v)| (k, false))
            .collect();

        Self {
            nodes,
            todo: VecDeque::new(),
            conj_state,
            flip_state,
        }
    }

    fn step(&mut self) {
        let Some((name, prev, pulse)) = self.todo.pop_front() else {
            panic!("empty");
        };
        let Some(node) = self.nodes.get(name) else {
            // Output node
            return;
        };
        match node.gate {
            Gate::Broadcast => {
                self.todo.extend(node.out.iter().map(|n| (*n, name, pulse)))
            }
            Gate::Conjunction => {
                let state = self.conj_state.get_mut(name).unwrap();
                *state.get_mut(prev).unwrap() = pulse;
                let out = !state.values().all(|v| *v);
                self.todo.extend(node.out.iter().map(|n| (*n, name, out)));
            }
            Gate::FlipFlop => {
                if !pulse {
                    let state = self.flip_state.get_mut(name).unwrap();
                    *state = !*state;
                    self.todo
                        .extend(node.out.iter().map(|n| (*n, name, *state)));
                }
            }
        }
    }
}

pub fn solve(s: &str) -> (String, String) {
    let mut raw_nodes = HashMap::new();
    let mut has_rx = false;
    for line in s.lines() {
        let mut iter = line.split(" -> ");
        let name = iter.next().unwrap();
        let (gate, name) = if let Some(name) = name.strip_prefix('&') {
            (Gate::Conjunction, name)
        } else if let Some(name) = name.strip_prefix('%') {
            (Gate::FlipFlop, name)
        } else if name == "broadcaster" {
            (Gate::Broadcast, name)
        } else {
            panic!("invalid gate name {name}");
        };
        let out = iter.next().unwrap().split(", ").collect::<Vec<&str>>();
        has_rx |= out.contains(&"rx");
        raw_nodes.insert(name, (gate, out));
    }

    let node_names: HashMap<&str, u8> = raw_nodes
        .keys()
        .enumerate()
        .map(|(i, n)| (*n, i.try_into().unwrap()))
        .collect();

    let nodes: FlatMap<_> = raw_nodes
        .into_values()
        .enumerate()
        .map(|(i, (gate, out))| {
            let node = Node {
                gate,
                out: out
                    .iter()
                    .map(|v| node_names.get(v).cloned().unwrap_or(u8::MAX))
                    .collect(),
            };
            (i.try_into().unwrap(), node)
        })
        .collect();
    let broadcast = node_names["broadcaster"];

    let mut state = State::new(&nodes);

    let mut high_pulses = 0;
    let mut low_pulses = 0;
    for _ in 0..1000 {
        state.todo.push_front((broadcast, u8::MAX, false));
        while let Some((_, _, pulse)) = state.todo.front() {
            if *pulse {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }
            state.step();
        }
    }
    let p1 = (low_pulses * high_pulses).to_string();

    // Check for an rx output node, bailing out early otherwise
    if !has_rx {
        return (p1, "n/a".to_string());
    };

    // The graph structure has a single accumulator feeding into `rx`, with the
    // accumulator being fed by 4 intermediate clusters.
    let (acc, _) = nodes
        .iter()
        .find(|(_k, n)| n.out.as_slice() == [u8::MAX])
        .unwrap();
    let subs = nodes
        .iter()
        .filter(|(_k, n)| n.out.as_slice() == [acc])
        .map(|(k, _)| k)
        .collect::<FlatSet>();
    let subsub = nodes
        .iter()
        .filter(|(_k, n)| n.out.iter().any(|s| subs.contains(s)))
        .map(|(k, _)| k)
        .collect::<FlatSet>();

    // Helper function to find the period of a particular subgraph, which begins
    // at `root` and terminates at one of `subsub`.
    let find_subgraph_period = |root| {
        let n = &nodes[root];

        // Output of this cluster
        let out = n.out.iter().find(|s| subsub.contains(s)).unwrap();

        // Find all nodes in this cluster
        let mut todo = vec![root];
        let mut cluster = [*out].into_iter().collect::<FlatSet>();
        while let Some(next) = todo.pop() {
            if !cluster.insert(next) {
                continue;
            }
            todo.extend(nodes[next].out.iter().cloned());
        }

        // Build a subgraph that contains only those nodes
        let subgraph = nodes
            .iter()
            .filter(|(k, _v)| cluster.contains(k))
            .map(|(k, v)| (k, v.clone()))
            .collect::<FlatMap<Node>>();

        // Run that subgraph to find its period
        let mut state = State::new(&subgraph);
        let mut prev: Option<usize> = None;
        for i in 1.. {
            let mut found = false;
            state.todo.push_front((root, u8::MAX, false));
            while let Some((_dst, src, pulse)) = state.todo.front() {
                if src == out && !pulse && !found {
                    found = true;
                    if let Some(p) = prev {
                        assert_eq!(i - p, p);
                        return i - p;
                    }
                    prev = Some(i);
                }
                state.step();
            }
        }
        unreachable!();
    };

    let lcm = std::thread::scope(|s| {
        let mut threads = vec![];
        for &root in &nodes[broadcast].out {
            let h = s.spawn(move || find_subgraph_period(root));
            threads.push(h);
        }
        threads.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let p2 = util::lcm(lcm);

    (p1, p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pulses() {
        let example = indoc::indoc! {"
            broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a
        "};
        let (p1, p2) = solve(example);
        assert_eq!(p1, "32000000");
        assert_eq!(p2, "n/a");

        let example = indoc::indoc! {"
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output
        "};
        let (p1, p2) = solve(example);
        assert_eq!(p1, "11687500");
        assert_eq!(p2, "n/a");
    }
}
