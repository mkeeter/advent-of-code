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
    room: usize,
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

    let mut todo = vec![];
    todo.push((
        State {
            room: *name_to_index.get_by_right("AA").unwrap(),
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

            // If we can open this valve, then do so!
            if state.valves & (1 << state.room) == 0
                && rooms[state.room].flow_rate > 0
            {
                next.push((
                    State {
                        valves: state.valves | (1 << state.room),
                        room: state.room,
                    },
                    released,
                ));
            }
            for &o in &rooms[state.room].out {
                next.push((
                    State {
                        room: o,
                        valves: state.valves,
                    },
                    released,
                ));
            }
        }
        std::mem::swap(&mut todo, &mut next);
    }
    println!("Part 1: {}", todo.iter().map(|i| i.1).max().unwrap_or(0));

    println!("Hello, world!");
    Ok(())
}
