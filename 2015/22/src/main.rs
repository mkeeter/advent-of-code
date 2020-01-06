use std::cmp::{max, Ordering};
use std::io::BufRead;
use std::str::FromStr;
use std::collections::BinaryHeap;

#[derive(Copy, Clone)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn mana(self) -> i32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
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

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    // The max-heap returns the state with the lowest spent mana
    fn cmp(&self, other: &Self) -> Ordering {
        other.spent_mana.cmp(&self.spent_mana)
    }
}

macro_rules! check_boss_hp {
    ($next:ident) => {
        if $next.boss_hp <= 0 {
            return Some($next);
        }
    }
}
macro_rules! effect {
    ($timer:expr, $value:expr) => {
        if $timer > 0 {
            return None;
        } else {
            $timer = $value;
        }
    }
}

impl State {
    fn step(&self, spell: Spell) -> Option<Self> {
        let mut next = self.clone();

        //////////////////////////////////////////////////////////////////////
        // Your turn!
        next.apply_effects();
        check_boss_hp!(next);

        let spell_mana = spell.mana();
        if spell_mana > next.your_mana {
            return None;
        }
        next.your_mana -= spell_mana;
        next.spent_mana += spell_mana;

        match spell {
            Spell::MagicMissile => next.boss_hp -= 4,
            Spell::Drain => {
                next.boss_hp -= 2;
                next.your_hp += 2;
            },
            Spell::Shield => effect!(next.shield_timer, 6),
            Spell::Poison => effect!(next.poison_timer, 6),
            Spell::Recharge => effect!(next.recharge_timer, 5),
        }
        check_boss_hp!(next);

        //////////////////////////////////////////////////////////////////////
        // Boss's turn
        next.apply_effects();
        check_boss_hp!(next);

        let your_armor = if self.shield_timer > 0 { 7 } else { 0 };
        next.your_hp -= max(1, next.boss_damage - your_armor);

        Some(next).filter(|n| n.your_hp > 0)
    }

    fn apply_effects(&mut self) {
        if self.poison_timer > 0 {
            self.boss_hp -= 3;
            self.poison_timer -= 1;
        }
        if self.shield_timer > 0 {
            self.shield_timer -= 1;
        }
        if self.recharge_timer > 0 {
            self.your_mana += 101;
            self.recharge_timer -= 1;
        }
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

    let start = State {
        your_hp: 50,
        your_mana: 500,
        spent_mana: 0,

        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,

        boss_hp, boss_damage};

    let spells = [Spell::MagicMissile,
                  Spell::Drain,
                  Spell::Shield,
                  Spell::Poison,
                  Spell::Recharge];

    let mut todo = BinaryHeap::new();
    todo.push(start.clone());
    while let Some(state) = todo.pop() {
        if state.boss_hp <= 0 {
            println!("Part 1: {}", state.spent_mana);
            break;
        }
        spells.iter()
            .filter_map(|s| state.step(*s))
            .for_each(|n| todo.push(n));
    }

    let mut todo = BinaryHeap::new();
    todo.push(start);
    while let Some(mut state) = todo.pop() {
        if state.boss_hp <= 0 {
            println!("Part 2: {}", state.spent_mana);
            break;
        }
        state.your_hp -= 1;
        if state.your_hp <= 0 {
            continue;
        }
        spells.iter()
            .filter_map(|s| state.step(*s))
            .for_each(|n| todo.push(n));
    }
}
