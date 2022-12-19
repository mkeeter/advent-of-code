use anyhow::Result;
use parse_display::{Display, FromStr};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::BufRead,
};

#[derive(Copy, Clone, FromStr, Display, Debug)]
#[display("Blueprint {id}: Each ore robot costs {ore_cost_ore} ore. \
          Each clay robot costs {clay_cost_ore} ore. \
          Each obsidian robot costs {obsidian_cost_ore} ore and {obsidian_cost_clay} clay. \
          Each geode robot costs {geode_cost_ore} ore and {geode_cost_obsidian} obsidian.")]
struct Blueprint {
    id: u64,
    ore_cost_ore: u64,
    clay_cost_ore: u64,
    obsidian_cost_ore: u64,
    obsidian_cost_clay: u64,
    geode_cost_ore: u64,
    geode_cost_obsidian: u64,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct State {
    ore: u64,
    clay: u64,
    obsidian: u64,
    geode: u64,

    ore_bots: u64,
    clay_bots: u64,
    obsidian_bots: u64,
    geode_bots: u64,
}

impl State {
    fn bot_key(&self) -> BotKey {
        BotKey {
            ore_bots: self.ore_bots,
            clay_bots: self.clay_bots,
            obsidian_bots: self.obsidian_bots,
            geode_bots: self.geode_bots,
        }
    }
    fn mineral_key(&self) -> MineralKey {
        MineralKey {
            ore: self.ore,
            clay: self.clay,
            obsidian: self.obsidian,
            geode: self.geode,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct BotKey {
    ore_bots: u64,
    clay_bots: u64,
    obsidian_bots: u64,
    geode_bots: u64,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct MineralKey {
    ore: u64,
    clay: u64,
    obsidian: u64,
    geode: u64,
}

fn run(blueprint: Blueprint, minutes: usize) -> u64 {
    let mut states = BTreeSet::new();
    states.insert(State {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
        ore_bots: 1,
        clay_bots: 0,
        obsidian_bots: 0,
        geode_bots: 0,
    });

    for minute in 1..=minutes {
        print!("{} -> ", states.len());
        let mut next = BTreeSet::new();

        // Deduplicate by finding states with the same number of bots and
        // strictly better ore counts.
        let mut per_bots: BTreeMap<_, BTreeSet<State>> = BTreeMap::new();
        for s in &states {
            per_bots.entry(s.bot_key()).or_default().insert(*s);
        }
        for s in &states {
            let mut better = true;
            for prev in per_bots.get(&s.bot_key()).unwrap().iter() {
                if prev.ore >= s.ore
                    && prev.clay >= s.clay
                    && prev.obsidian >= s.obsidian
                    && prev.geode >= s.geode
                    && prev != s
                {
                    better = false;
                    break;
                }
            }
            if better {
                next.insert(*s);
            }
        }
        states = std::mem::take(&mut next);
        print!("{} -> ", states.len());

        // Deduplicate by finding states with the same number of minerals and
        // strictly better bot counts.
        let mut next = BTreeSet::new();
        let mut per_minerals: BTreeMap<_, BTreeSet<State>> = BTreeMap::new();
        for s in &states {
            per_minerals.entry(s.mineral_key()).or_default().insert(*s);
        }
        for s in &states {
            let mut better = true;
            for prev in per_minerals.get(&s.mineral_key()).unwrap().iter() {
                if prev.ore_bots >= s.ore_bots
                    && prev.clay_bots >= s.clay_bots
                    && prev.obsidian_bots >= s.obsidian_bots
                    && prev.geode_bots >= s.geode_bots
                    && prev != s
                {
                    better = false;
                    break;
                }
            }
            if better {
                next.insert(*s);
            }
        }
        states = std::mem::take(&mut next);
        print!("{} -> ", states.len());

        // Filter by bounding on the minimum possible score
        let min_max_score = states
            .iter()
            .map(|s| (minutes - minute + 1) as u64 * s.geode_bots + s.geode)
            .max()
            .unwrap_or(0);
        states.retain(|s| {
            let mut s = *s;
            let mut max_score = s.geode;
            for _m in minute..=minutes {
                max_score += s.geode_bots;
                s.geode_bots += 1;
            }
            max_score >= min_max_score
        });
        println!("{}", states.len());

        // Do the actual recursion
        for s in states.into_iter() {
            let mut new_states = Vec::with_capacity(4);
            new_states.push(s);
            if s.ore >= blueprint.ore_cost_ore {
                new_states.push(State {
                    ore: s.ore - blueprint.ore_cost_ore,
                    ore_bots: s.ore_bots + 1,
                    ..s
                });
            }
            if s.ore >= blueprint.clay_cost_ore {
                new_states.push(State {
                    ore: s.ore - blueprint.clay_cost_ore,
                    clay_bots: s.clay_bots + 1,
                    ..s
                });
            }
            if s.ore >= blueprint.obsidian_cost_ore
                && s.clay >= blueprint.obsidian_cost_clay
            {
                new_states.push(State {
                    ore: s.ore - blueprint.obsidian_cost_ore,
                    clay: s.clay - blueprint.obsidian_cost_clay,
                    obsidian_bots: s.obsidian_bots + 1,
                    ..s
                });
            }
            if s.ore >= blueprint.geode_cost_ore
                && s.obsidian >= blueprint.geode_cost_obsidian
            {
                new_states.push(State {
                    ore: s.ore - blueprint.geode_cost_ore,
                    obsidian: s.obsidian - blueprint.geode_cost_obsidian,
                    geode_bots: s.geode_bots + 1,
                    ..s
                });
            }
            for n in &mut new_states {
                n.ore += s.ore_bots;
                n.clay += s.clay_bots;
                n.obsidian += s.obsidian_bots;
                n.geode += s.geode_bots;
            }
            next.extend(new_states.into_iter());
        }
        states = next;
    }
    states.iter().map(|k| k.geode).max().unwrap_or(0)
}

fn main() -> Result<()> {
    let blueprints = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse())
        .collect::<Result<Vec<Blueprint>, _>>()?;

    println!(
        "Part 1: {}",
        blueprints.iter().map(|b| run(*b, 24) * b.id).sum::<u64>()
    );

    println!(
        "Part 2: {}",
        blueprints
            .iter()
            .take(3)
            .map(|b| run(*b, 32))
            .product::<u64>()
    );

    Ok(())
}
