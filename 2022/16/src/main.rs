use anyhow::{anyhow, bail, Error, Result};
use bimap::BiBTreeMap;
use parse_display::FromStr;
use std::{
    collections::{btree_map::Entry, BTreeMap},
    io::{BufRead, Write},
};

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, FromStr)]
#[from_str(regex = "Valve (?P<name>[A-Z]+) has flow rate=(?P<rate>[0-9]+); \
                    tunnels? leads? to valves? (?P<out>.*)")]
struct Input {
    name: String,
    rate: u64,
    out: String,
}

/// Single room in the map, with outlets as indexes into a `Vec<Room>`
struct Room {
    flow_rate: u64,
    out: Vec<usize>,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct State<const N: usize> {
    people: [usize; N],
    valves: u64,
}

impl<const N: usize> State<N> {
    fn canonicalize(&mut self) {
        self.people.sort()
    }
}

enum Action {
    Open(usize),
    MoveTo(usize),
}

////////////////////////////////////////////////////////////////////////////////

fn run<const N: usize>(
    rooms: &[Room],
    start_time: usize,
    end_time: usize,
    start_room: usize,
) -> u64 {
    let mut todo = vec![];
    todo.push((
        State {
            people: [start_room; N],
            valves: 0,
        },
        0,
    ));

    let flow_rate = |state: &State<N>| -> u64 {
        rooms
            .iter()
            .enumerate()
            .map(|(i, room)| {
                if state.valves & (1 << i) != 0 {
                    room.flow_rate
                } else {
                    0
                }
            })
            .sum()
    };

    // Breadth-first search
    let mut seen = BTreeMap::new();
    for time in start_time..end_time {
        if time != start_time {
            print!("\r");
        }
        print!("{time} / 30  ");
        std::io::stdout().lock().flush().unwrap();

        // Elephants and humans are interchangeable
        todo.iter_mut().for_each(|(state, _)| state.canonicalize());

        // Deduplicate any identical states that have released strictly less
        // pressure than the best case for this state.
        todo.sort_by_key(|(state, n)| (*state, std::cmp::Reverse(*n)));
        todo.dedup_by_key(|(state, _)| *state);

        // Release any state which has already been seen with a better score
        todo.retain(|(state, released)| match seen.entry(*state) {
            Entry::Vacant(v) => {
                v.insert(*released);
                true
            }
            Entry::Occupied(mut o) => {
                if o.get() >= released {
                    false
                } else {
                    o.insert(*released);
                    true
                }
            }
        });

        // Sort by the number of set bits on a per-room basis, so we can
        // deduplicate further.
        todo.sort_by_key(|(s, _)| (s.people, s.valves.count_ones()));

        let mut next = vec![];
        for (i, (s, rs)) in todo.iter().enumerate() {
            let mut j = i + 1;
            let mut good = true;
            while let Some((t, rt)) = todo.get(j) {
                if s.people != t.people {
                    break;
                }
                if s.valves & t.valves == s.valves && rt >= rs {
                    good = false;
                    break;
                }
                j += 1;
            }
            if good {
                next.push((*s, *rs));
            }
        }
        todo = std::mem::take(&mut next);

        // Figure out the lower bound on best score (which is "stand still and
        // continue to accumulate"), then filter any positions that couldn't
        // possibly hit it.
        let minimum_best_score = todo
            .iter()
            .map(|(s, rs)| {
                let flow_rate = flow_rate(s);
                rs + flow_rate * (end_time - time) as u64
            })
            .max()
            .unwrap_or(0);
        todo.retain(|(s, mut r)| {
            let mut todo: Vec<u64> = rooms
                .iter()
                .enumerate()
                .filter(|(i, r)| s.valves & (1 << i) == 0 && r.flow_rate > 0)
                .map(|(_i, r)| r.flow_rate)
                .collect();
            let mut flow_rate = flow_rate(s);
            todo.sort_unstable();
            for _t in time..end_time {
                r += flow_rate;
                for _ in 0..N {
                    flow_rate += todo.pop().unwrap_or(0);
                }
            }
            r >= minimum_best_score
        });

        // Okay, we've done enough filtering.  Time to run the next step of the
        // simulation and build up `next`
        for (state, mut released) in todo.into_iter() {
            // Update pressure based on open valves
            released += flow_rate(&state);

            // Every person (elephants count as people) can take a few different
            // actions to modify the current state.
            let mut new_states = vec![state];
            for (i, &r) in state.people.iter().enumerate() {
                let mut actions = vec![];
                // If we can open this valve, then do so!
                if state.valves & (1 << r) == 0 && rooms[r].flow_rate > 0 {
                    actions.push(Action::Open(r));
                }
                // Try moving to every room that's attached to this one.
                for &o in &rooms[r].out {
                    actions.push(Action::MoveTo(o));
                }
                // Apply these actions to our state and accumulate new
                // states as we go.
                let mut new_new_states = vec![];
                for s in new_states.into_iter() {
                    for a in actions.iter() {
                        let mut s = s;
                        match a {
                            Action::Open(o) => s.valves |= 1 << o,
                            Action::MoveTo(t) => s.people[i] = *t,
                        }
                        new_new_states.push(s);
                    }
                }
                new_states = new_new_states;
            }
            next.extend(new_states.into_iter().map(|s| (s, released)));
        }
        todo = next;
    }

    print!("\r");
    todo.iter().map(|i| i.1).max().unwrap_or(0)
}

fn main() -> Result<()> {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse())
        .collect::<Result<Vec<Input>, _>>()?;

    let name_to_index: BiBTreeMap<usize, String> = input
        .iter()
        .enumerate()
        .map(|(i, n)| (i, n.name.to_owned()))
        .collect();

    let rooms = input
        .iter()
        .map(|i| {
            let out = i
                .out
                .split(", ")
                .map(|v| {
                    name_to_index
                        .get_by_right(v)
                        .ok_or_else(|| anyhow!("No such room {v}"))
                        .cloned()
                })
                .collect::<Result<Vec<usize>, _>>()?;
            Ok::<_, Error>(Room {
                flow_rate: i.rate,
                out,
            })
        })
        .collect::<Result<Vec<Room>, _>>()?;

    if rooms.len() > 64 {
        bail!("Cannot pack {} rooms into a bitmask", rooms.len());
    }

    let start_room = *name_to_index
        .get_by_right("AA")
        .ok_or_else(|| anyhow!("Could not find 'AA'"))?;

    println!("Part 1: {}", run::<1>(&rooms, 0, 30, start_room));
    println!("Part 2: {}", run::<2>(&rooms, 4, 30, start_room));

    Ok(())
}
