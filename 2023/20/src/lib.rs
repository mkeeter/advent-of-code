use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Gate {
    Broadcast,
    FlipFlop,
    Conjunction,
}

#[derive(Clone, Debug)]
struct Node<'a> {
    gate: Gate,
    out: Vec<&'a str>,
}

#[derive(Debug)]
struct State<'a> {
    nodes: &'a HashMap<&'a str, Node<'a>>,
    todo: VecDeque<(&'a str, &'a str, bool)>,
    conj_state: HashMap<&'a str, HashMap<&'a str, bool>>,
    flip_state: HashMap<&'a str, bool>,
}

impl<'a> State<'a> {
    fn new(nodes: &'a HashMap<&'a str, Node>) -> Self {
        // Gate activation: the gate is active at 0 and inactive at > 0
        let mut conj_state: HashMap<&str, HashMap<&str, bool>> = nodes
            .iter()
            .filter(|(_k, v)| v.gate == Gate::Conjunction)
            .map(|(k, _v)| (*k, HashMap::new()))
            .collect();
        for (k, n) in nodes.iter() {
            for t in &n.out {
                if let Some(c) = conj_state.get_mut(*t) {
                    c.insert(*k, false);
                }
            }
        }

        let flip_state: HashMap<&str, bool> = nodes
            .iter()
            .filter(|(_k, v)| v.gate == Gate::FlipFlop)
            .map(|(k, _v)| (*k, false))
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
    let mut nodes = HashMap::new();
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
        let out = iter.next().unwrap().split(", ").collect();
        nodes.insert(name, Node { gate, out });
    }

    let mut state = State::new(&nodes);

    let mut high_pulses = 0;
    let mut low_pulses = 0;
    for _ in 0..1000 {
        state.todo.push_front(("broadcaster", "", false));
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
    if !nodes.values().any(|v| v.out.iter().any(|c| *c == "rx")) {
        return (p1, "n/a".to_string());
    }

    // The graph structure has a single accumulator feeding into `rx`, with the
    // accumulator being fed by 4 intermediate clusters.
    let (&acc, _) = nodes.iter().find(|(_k, n)| n.out == ["rx"]).unwrap();
    let subs = nodes
        .iter()
        .filter(|(_k, n)| n.out == [acc])
        .map(|(k, _)| *k)
        .collect::<HashSet<_>>();
    let subsub = nodes
        .iter()
        .filter(|(_k, n)| n.out.iter().any(|s| subs.contains(*s)))
        .map(|(k, _)| *k)
        .collect::<HashSet<_>>();

    let mut lcm = vec![];
    for root in &nodes["broadcaster"].out {
        let n = &nodes[root];
        // Output of this cluster
        let out = n.out.iter().find(|s| subsub.contains(*s)).unwrap();
        let mut todo = vec![*root];
        let mut cluster = HashSet::new();
        cluster.insert(*out);
        while let Some(next) = todo.pop() {
            if !cluster.insert(next) {
                continue;
            }
            for n in &nodes[&next].out {
                todo.push(*n);
            }
        }
        let subgraph = nodes
            .iter()
            .filter(|(k, _v)| cluster.contains(*k))
            .map(|(k, v)| (*k, v.clone()))
            .collect::<HashMap<&str, Node>>();
        let mut state = State::new(&subgraph);
        let mut prev = None;
        for i in 1.. {
            let mut found = false;
            state.todo.push_front((root, "", false));
            while let Some((_dst, src, pulse)) = state.todo.front() {
                if src == out && !pulse {
                    found = true;
                }
                state.step();
            }
            if found {
                if let Some(p) = prev {
                    assert_eq!(i - p, p);
                    lcm.push(p);
                    break;
                }
                prev = Some(i);
            }
        }
    }
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
