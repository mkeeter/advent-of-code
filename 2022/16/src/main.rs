use anyhow::{anyhow, bail, Error, Result};
use bimap::BiBTreeMap;
use parse_display::{Display, FromStr};
use std::{
    collections::{BTreeMap, VecDeque},
    io::BufRead,
};

#[derive(Clone, Debug, FromStr, Display)]
#[display("Valve {name} has flow rate={rate}; {_t} {_l} to {_v} {out}")]
struct Input {
    name: String,
    rate: u64,
    out: String,
    _t: String,
    _l: String,
    _v: String,
}

struct Room {
    flow_rate: u64,
    out: Vec<usize>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct State {
    my_room: usize,
    el_room: usize,
    valves: u64,
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

    let all_on = rooms
        .iter()
        .enumerate()
        .filter(|(_i, r)| r.flow_rate > 0)
        .map(|(i, _r)| 1 << i)
        .reduce(|a, b| a | b)
        .ok_or_else(|| anyhow!("Empty input?"))?;

    let mut todo = vec![];
    let start_room = *name_to_index
        .get_by_right("AA")
        .ok_or_else(|| anyhow!("Could not find 'AA'"))?;
    todo.push((
        State {
            my_room: start_room,
            el_room: start_room,
            valves: 0,
        },
        0,
    ));

    // Breadth-first search
    let mut seen = BTreeMap::new();
    for _t in 0..30 {
        let mut next = vec![];
        while let Some((state, mut released)) = todo.pop() {
            if let Some(e) = seen.get_mut(&state) {
                if *e >= released {
                    continue;
                } else {
                    *e = released;
                }
            } else {
                seen.insert(state, released);
            }

            // Update pressure based on open valves
            for (i, r) in rooms.iter().enumerate() {
                if state.valves & (1 << i) != 0 {
                    released += r.flow_rate;
                }
            }

            if state.valves == all_on {
                next.push((state, released));
                continue;
            }

            // If we can open this valve, then do so!
            if state.valves & (1 << state.my_room) == 0
                && rooms[state.my_room].flow_rate > 0
            {
                next.push((
                    State {
                        valves: state.valves | (1 << state.my_room),
                        my_room: state.my_room,
                        el_room: state.el_room,
                    },
                    released,
                ));
            }
            for &o in &rooms[state.my_room].out {
                next.push((
                    State {
                        my_room: o,
                        el_room: state.el_room,
                        valves: state.valves,
                    },
                    released,
                ));
            }
        }
        std::mem::swap(&mut todo, &mut next);
    }
    println!("Part 1: {}", todo.iter().map(|i| i.1).max().unwrap_or(0));

    // Breadth-first search
    let mut todo = vec![];
    todo.push((
        State {
            my_room: start_room,
            el_room: start_room,
            valves: 0,
        },
        0,
    ));
    let mut seen = BTreeMap::new();
    for _t in 4..30 {
        println!("{_t}");
        let mut next = vec![];
        while let Some((state, mut released)) = todo.pop() {
            let canonical_state = State {
                my_room: state.my_room.min(state.el_room),
                el_room: state.my_room.max(state.el_room),
                valves: state.valves,
            };
            if let Some(e) = seen.get_mut(&canonical_state) {
                if *e >= released {
                    continue;
                } else {
                    *e = released;
                }
            } else {
                seen.insert(canonical_state, released);
            }

            // Update pressure based on open valves
            for (i, r) in rooms.iter().enumerate() {
                if state.valves & (1 << i) != 0 {
                    released += r.flow_rate;
                }
            }

            if state.valves == all_on {
                next.push((state, released));
                continue;
            }

            struct Action {
                open: u64,
                move_to: usize,
            }
            let mut actions =
                [state.my_room, state.el_room].into_iter().map(|room| {
                    let mut out = vec![];
                    // If we can open this valve, then do so!
                    if state.valves & (1 << room) == 0
                        && rooms[room].flow_rate > 0
                    {
                        out.push(Action {
                            open: 1 << room,
                            move_to: room,
                        });
                    }
                    for &o in &rooms[room].out {
                        out.push(Action {
                            open: 0,
                            move_to: o,
                        });
                    }
                    out
                });
            let my_actions = actions.next().unwrap();
            let el_actions = actions.next().unwrap();
            for a in &my_actions {
                for b in &el_actions {
                    next.push((
                        State {
                            my_room: a.move_to,
                            el_room: b.move_to,
                            valves: state.valves | a.open | b.open,
                        },
                        released,
                    ));
                }
            }
        }
        next.sort_by_key(|(state, released)| {
            (*state, std::cmp::Reverse(*released))
        });
        next.dedup_by_key(|(state, _released)| *state);
        std::mem::swap(&mut todo, &mut next);
    }
    println!("Part 2: {}", todo.iter().map(|i| i.1).max().unwrap_or(0));

    Ok(())
}
