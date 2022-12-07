#[macro_use]
extern crate nom;
use nom::types::CompleteStr;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Debug)]
enum Flavor {
    Cold,
    Bludgeoning,
    Radiation,
    Slashing,
    Fire,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Team {
    Immune,
    Infection,
}

#[derive(Debug, Clone)]
struct Army {
    team: Team,
    units: usize,
    hp: usize,
    damage: usize,
    initiative: usize,
    damage_type: Flavor,
    weak: HashSet<Flavor>,
    immune: HashSet<Flavor>,
}

impl Army {
    fn effective_power(&self) -> usize {
        self.units * self.damage
    }

    fn damage_from(&self, other: &Army) -> usize {
        if self.immune.contains(&other.damage_type) {
            0
        } else {
            let d = other.units * other.damage;
            if self.weak.contains(&other.damage_type) {
                2 * d
            } else {
                d
            }
        }
    }
}

named!(usize_<CompleteStr, usize>,
       map_res!(recognize!(nom::digit), |s:CompleteStr|
                usize::from_str(s.0)));

named!(properties<CompleteStr, (HashSet<Flavor>, HashSet<Flavor>)>,
map!(
opt!(do_parse!(
    tag_s!("(") >>
    props: many0!(do_parse!(opt!(tag_s!("; ")) >> p: property >> (p))) >>
    tag_s!(") ") >>
    (props))),
|props| {
    let mut weak = HashSet::new();
    let mut immune = HashSet::new();
    for (p, f) in props.unwrap_or(Vec::new()).iter() {
        let h = match p {
            'i' => &mut immune,
            'w' => &mut weak,
             _  => unreachable!(),
        };
        for f in f {
            h.insert(f.clone());
        }
    }
    (weak, immune)
}));

named!(property<CompleteStr, (char, Vec<Flavor>)>,
       do_parse!(
           t: alt!(value!('i', tag_s!("immune to ")) |
                   value!('w', tag_s!("weak to "))) >>
           p: many0!(do_parse!(opt!(tag_s!(", ")) >> prop: flavor >> (prop))) >>
           (t, p)));

named!(flavor<CompleteStr, Flavor>,
       alt!(value!(Flavor::Cold, tag_s!("cold")) |
            value!(Flavor::Bludgeoning, tag_s!("bludgeoning")) |
            value!(Flavor::Radiation, tag_s!("radiation")) |
            value!(Flavor::Slashing, tag_s!("slashing")) |
            value!(Flavor::Fire, tag_s!("fire"))));

named_args!(parse_line(team: Team)<CompleteStr, Army>,
do_parse!(count: usize_ >>
          tag_s!(" units each with ") >>
          hp: usize_ >>
          tag_s!(" hit points ") >>
          properties: properties >>
          tag_s!("with an attack that does ") >>
          damage: usize_ >>
          tag_s!(" ") >>
          damage_type: flavor >>
          tag_s!(" damage at initiative ") >>
          initiative: usize_ >>
          (Army {
                team: team,
                units: count,
                hp: hp,
                damage: damage,
                damage_type: damage_type,
                initiative: initiative,
                weak: properties.0,
                immune: properties.1,
          })));

fn run(armies: &Vec<Army>, boost: usize) -> Vec<Army> {
    let mut armies: Vec<Army> = armies.iter().cloned().collect();
    for a in armies.iter_mut() {
        if a.team == Team::Immune {
            a.damage += boost;
        }
    }

    loop {
        let mut order = (0..armies.len()).collect::<Vec<usize>>();

        // Target selection phase
        order.sort_by_key(|&i| (armies[i].effective_power(), armies[i].initiative));
        let mut attacks = HashMap::new();
        let mut targeted = HashSet::new();
        for i in order.iter().rev() {
            let a = &armies[*i];
            armies
                .iter()
                .enumerate()
                .filter(|(j, b)| b.team != a.team && !targeted.contains(j))
                .map(|(j, b)| (j, b, b.damage_from(a)))
                .filter(|(_, _, d)| *d > 0)
                .max_by_key(|(_, b, d)| (*d, b.effective_power(), b.initiative))
                .map(|(j, _, _)| {
                    attacks.insert(*i, j);
                    targeted.insert(j);
                });
        }

        // Attack phase
        order.sort_by_key(|&i| armies[i].initiative);

        let mut any_kills = false;
        for &i in order.iter().rev().filter(|i| attacks.contains_key(i)) {
            let j = *attacks.get(&i).unwrap();

            let attacker = &armies[i];
            let target = &armies[j];

            let damage = target.damage_from(attacker);
            let kills = min(damage / target.hp, target.units);
            any_kills |= kills > 0;

            armies[j].units -= kills;
        }
        if !any_kills {
            break;
        }
        armies = armies.into_iter().filter(|a| a.units != 0).collect();
    }

    armies
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut current_team = None;
    let mut armies = Vec::new();
    for line in buffer.lines() {
        if line == "Immune System:" {
            current_team = Some(Team::Immune);
        } else if line == "Infection:" {
            current_team = Some(Team::Infection);
        } else if line.len() > 0 {
            armies.push(
                parse_line(CompleteStr(line), current_team.unwrap())
                    .ok()
                    .unwrap()
                    .1,
            );
        }
    }

    let part1 = run(&armies, 0);
    let units: usize = part1.iter().map(|a| a.units).sum();
    println!("Part 1: {}", units);

    for boost in 0.. {
        let part2 = run(&armies, boost);
        if part2.iter().all(|a| a.team == Team::Immune) {
            let units: usize = part2.iter().map(|a| a.units).sum();
            println!("Part 2: {}", units);
            break;
        }
    }
}
