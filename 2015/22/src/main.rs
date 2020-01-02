use std::io::BufRead;
use std::str::FromStr;

enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    your_hp: i32,
    your_mana: i32,
    spent_mana: i32,

    // Effects
    shield_timer: i32,
    poison_timer: i32,
    recharge_timer: i32,

    boss_hp: i32,
    boss_damage: i32,
}

impl State {
    fn step(&mut self, spell: Spell) {
    }
}

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let boss_hp = input.iter()
        .filter_map(|d| i32::from_str(&d.replace("Hit Points: ", "")).ok())
        .next()
        .unwrap();
    let boss_damage = input.iter()
        .filter_map(|d| i32::from_str(&d.replace("Damage: ", "")).ok())
        .next()
        .unwrap();
    println!("Hello, world!");
}
