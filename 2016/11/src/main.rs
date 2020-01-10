use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, HashMap};
use std::io::BufRead;
use std::hash::{Hash, Hasher};

use itertools::Itertools;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Item {
    Generator(u8),
    Microchip(u8),
}
use Item::*;

impl Item {
    fn is_microchip(&self) -> bool {
        match self {
            Generator(_) => false,
            Microchip(_) => true,
        }
    }
    fn element(&self) -> u8 {
        *match self {
            Generator(s) => s,
            Microchip(s) => s,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct State {
    floors: [HashSet<Item>; 4],
    elevator: usize,
}

impl State {
    fn valid(&self) -> bool {
        self.floors.iter().all(|f|
            // Either this floor only contains microchips
            f.iter().all(|i| i.is_microchip()) ||
            // Or every microchip has a matching generator
            f.iter().filter(|i| i.is_microchip())
                .all(|i| f.contains(&Generator(i.element()))))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let weight = |s: &State| (0..4)
            .map(|i| (i + 1) * s.floors[i].len())
            .sum::<usize>();
        weight(self).cmp(&weight(other))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut v: [Vec<&Item>; 4] = Default::default();
        for i in 0..4 {
            for j in self.floors[i].iter() {
                v[i].push(j);
            }
            v[i].sort();
        }
        (v, self.elevator).hash(state);
    }
}

fn run(state: State) -> i32 {
    let mut todo = BinaryHeap::new();
    let mut seen = HashSet::new();
    todo.push((0, state));
    while let Some((steps, state)) = todo.pop() {
        // De-duplicate states which we've already seen
        if !state.valid() {
            continue;
        }

        if seen.contains(&state) {
            continue;
        }
        seen.insert(state.clone());

        // Check our termination condition
        if state.floors[0..=2].iter().all(|f| f.is_empty()) {
            return -steps;
        }

        for i in 1..=2 {
            for c in state.floors[state.elevator].iter().combinations(i) {
                if state.elevator > 0 {
                    let mut next = state.clone();
                    for j in c.iter() {
                        next.floors[state.elevator - 1].insert(**j);
                        next.floors[state.elevator].remove(j);
                    }
                    next.elevator -= 1;
                    todo.push((steps - 1, next));
                }
                if state.elevator < 3 {
                    let mut next = state.clone();
                    for j in c.iter() {
                        next.floors[state.elevator].remove(j);
                        next.floors[state.elevator + 1].insert(**j);
                    }
                    next.elevator += 1;
                    todo.push((steps - 1, next));
                }
            }
        }
    }
    unreachable!();
}

fn main() {
    let mut state: State = Default::default();
    let mut elements = HashMap::new();
    let mut get_element = |s: &str| -> u8 {
        let i = elements.len() as u8;
        *elements.entry(s.to_string()).or_insert(i)
    };

    for (floor, line) in std::io::stdin().lock().lines().enumerate() {
        let line = line.unwrap();
        let words = line.split(' ').collect::<Vec<_>>();
        for ws in words.windows(2) {
            if ws[1].starts_with("microchip") {
                let element = get_element(&ws[0].replace("-compatible", ""));
                state.floors[floor].insert(Item::Microchip(element));
            } else if ws[1].starts_with("generator") {
                let element = get_element(ws[0]);
                state.floors[floor].insert(Item::Generator(element));
            }
        }
    }
    assert!(state.valid());

    println!("Part 1: {}", run(state.clone()));

    state.floors[0].insert(Generator(get_element("elerium")));
    state.floors[0].insert(Microchip(get_element("elerium")));
    state.floors[0].insert(Generator(get_element("dilithium")));
    state.floors[0].insert(Microchip(get_element("dilithium")));
    println!("Part 2: {}", run(state.clone()));
}
