use std::cmp::Reverse;
use std::collections::HashMap;
use std::io::BufRead;

use priority_queue::PriorityQueue;

use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
enum Item {
    Generator(u8),
    Microchip(u8),
}

use Item::*;

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, Hash, Eq, Default, PartialEq)]
struct Floor {
    // Bitfield masks (8 items max)
    microchips: u8,
    generators: u8,
}

impl Floor {
    fn insert(&mut self, item: Item) {
        match item {
            Generator(i) => self.generators |= i,
            Microchip(i) => self.microchips |= i,
        }
    }
    fn remove(&mut self, item: Item) {
        match item {
            Generator(i) => self.generators &= !i,
            Microchip(i) => self.microchips &= !i,
        }
    }
    fn count_items(self) -> u32 {
        self.generators.count_ones() + self.microchips.count_ones()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
struct State {
    floors: [Floor; 4],
    elevator: u8,
}

impl State {
    fn valid(&self) -> bool {
        self.floors.iter().all(|f|
            // Either this floor only contains microchips
            f.generators == 0 ||
            // Or every microchip has a matching generator
            f.microchips & !f.generators == 0)
    }

    fn heuristic(&self) -> u32 {
        self.floors.iter()
            .enumerate()
            .map(|(i, f)| {
                // In the best case, we carry two items per trip,
                // and need to go up-and-back (other than the
                // final trip, which only goes up).
                let num_trips = f.count_items() / 2;
                i as u32 * (num_trips * 2).saturating_sub(1)
            })
            .sum()
    }

    fn neighbors(&self) -> Vec<State> {
        let mut out = Vec::new();
        let elevator = self.elevator as usize;
        let floor = self.floors[elevator];
        for i in (1..=2).rev() {
            let microchips = (0..8)
                .map(|i| 1 << i)
                .filter(|i| floor.microchips & i != 0)
                .map(Microchip);
            let generators = (0..8)
                .map(|i| 1 << i)
                .filter(|i| floor.generators & i != 0)
                .map(Generator);

            for c in microchips.chain(generators).combinations(i) {
                if elevator < 3 {
                    let mut next = *self;
                    for j in c.iter() {
                        next.floors[elevator + 1].insert(*j);
                        next.floors[elevator].remove(*j);
                    }
                    next.elevator += 1;
                    if next.valid() {
                        out.push(next);
                    }
                }
                if elevator > 0 {
                    let mut next = *self;
                    for j in c.iter() {
                        next.floors[elevator - 1].insert(*j);
                        next.floors[elevator].remove(*j);
                    }
                    next.elevator -= 1;
                    if next.valid() {
                        out.push(next);
                    }
                }
            }
        }
        out
    }
}

fn run(start: State) -> u32 {
    let mut open_set = PriorityQueue::new();

    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();

    g_score.insert(start.clone(), 0);
    f_score.insert(start.clone(), start.heuristic());

    open_set.push((start, 0), Reverse(start.heuristic()));
    while let Some(((state, steps), _p)) = open_set.pop() {
        // Check our termination condition
        if state.floors[0..=2].iter()
            .all(|f| f.count_items() == 0)
        {
            return g_score[&state];
        }

        for n in state.neighbors().into_iter() {
            let g_score_tentative = steps + 1;
            if g_score_tentative < *g_score.get(&n).unwrap_or(&std::u32::MAX) {
                g_score.insert(n, g_score_tentative);

                let f = g_score_tentative + n.heuristic();
                f_score.insert(n, f);

                open_set.push((n, steps + 1), Reverse(f));
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
        *elements.entry(s.to_string()).or_insert(1 << i)
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

    println!("Part 1: {}", run(state));

    state.floors[0].insert(Generator(get_element("elerium")));
    state.floors[0].insert(Microchip(get_element("elerium")));
    state.floors[0].insert(Generator(get_element("dilithium")));
    state.floors[0].insert(Microchip(get_element("dilithium")));
    println!("Part 2: {}", run(state));
}
