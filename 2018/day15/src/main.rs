use std::io::{self, Read};
use std::iter;
use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Race {
    Elf,
    Goblin
}

#[derive(PartialEq, Copy, Clone)]
struct Creature {
    race: Race,
    hp: usize,
}

#[derive(PartialEq, Copy, Clone)]
enum Tile {
    Creature(Creature),
    Floor,
    Wall,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            'E' => Tile::Creature(Creature { race: Race::Elf, hp: 200 }),
            'G' => Tile::Creature(Creature { race: Race::Goblin, hp: 200 }),
            '#' => Tile::Wall,
            '.' => Tile::Floor,
             _  => unimplemented!(),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Tile::Creature(Creature { race: Race::Elf, hp: _ }) => 'E',
            Tile::Creature(Creature { race: Race::Goblin, hp: _ }) => 'G',
            Tile::Floor => '.',
            Tile::Wall => '#',
        }
    }

    fn is_creature(&self) -> bool {
        self.to_creature().is_some()
    }

    fn to_creature(&self) -> Option<&Creature> {
        match self {
            Tile::Creature(c) => Some(c),
            Tile::Floor | Tile::Wall => None,
        }
    }

    fn to_mut_creature(&mut self) -> Option<&mut Creature> {
        match self {
            Tile::Creature(c) => Some(c),
            Tile::Floor | Tile::Wall => None,
        }
    }
}

struct State(Vec<Vec<Tile>>, /* Game board */
             usize           /* Elf power */);

enum StepResult {
    Continue,
    Finished,
    Killed(usize, usize),
}

enum FightResult {
    NoFight,
    Fought,
    Killed(usize, usize),
}

impl State {
    fn from_string(buffer: &String, elf_power: usize) -> State {
        State(buffer
                .lines()
                .map(|line| line
                     .chars()
                     .map(Tile::from_char)
                     .collect())
                .collect(),
            elf_power)
    }

    fn find(&self, p: impl Fn(&Tile) -> bool) -> Vec<(usize, usize)> {
        let mut out = self.0.iter()
            .enumerate()
            .flat_map(|(y, row)| iter::repeat(y).zip(row.iter().enumerate()))
            .filter(|(_, (_, tile))| p(*tile))
            .map(|(y, (x, _))| (y, x))
            .collect::<Vec<(usize, usize)>>();
        out.sort();
        out.dedup();
        return out;
    }

    fn print(&self) {
        for line in self.0.iter() {
            line.iter().map(|t| print!("{}", Tile::to_char(t))).for_each(drop);
            print!("  ");
            line.iter().filter_map(|t| t.to_creature())
                .map(|c| print!("{}, ", c.hp)).for_each(drop);
            print!("\n");
        }
    }

    /*  This is the core game loop updater.  It returns true on termination. */
    fn step(&mut self) -> bool {
        let mut killed = HashSet::new();
        for (cy, cx) in self.find(Tile::is_creature) {
            if killed.contains(&(cy, cx)) {
                continue;
            }
            match self.walk_monster(cy, cx) {
                StepResult::Finished => return true,
                StepResult::Killed(y, x) => drop(killed.insert((y, x))),
                StepResult::Continue => (),
            }
        }
        return false;
    }

    /*  Builds a new grid of the same size and the given type + value */
    fn grid<T: Clone>(&self, t: T) -> Vec<Vec<T>> {
        vec![vec![t; self.0[0].len()]; self.0.len()]
    }

    fn neighbors(y: usize, x: usize) -> impl Iterator<Item=(usize, usize)>
    {
        [(0, 1), (0, -1), (1, 0), (-1, 0)].iter()
            .map(move |(dy, dx)| ((y as i32 + dy) as usize,
                                  (x as i32 + dx) as usize))
    }

    fn try_attack(&mut self, y: usize, x: usize) -> FightResult {
        // First, check to see if we have someone to fight,
        // sorting by HP then by positions.
        let race = self.0[y][x].to_creature().unwrap().race;
        let fightable = State::neighbors(y, x)
            .map(|(y, x)| (self.0[y][x].to_creature(), y, x))
            .filter(|(c, _, _)| c.map(|c| c.race != race).unwrap_or(false))
            .map(|(c, y, x)| (c.unwrap().hp, y, x))
            .min();

        if let Some((_, y, x)) = fightable {
            // Hacks to help the elves
            let attack_power = match race {
                Race::Elf => self.1,
                Race::Goblin => 3,
            };

            let c = self.0[y][x].to_mut_creature().unwrap();
            c.hp = c.hp.saturating_sub(attack_power);
            if c.hp == 0 { // DEAD
                self.0[y][x] = Tile::Floor;
                return FightResult::Killed(y, x);
            }
            return FightResult::Fought;
        }
        return FightResult::NoFight;
    }

    fn walk_monster(&mut self, y: usize, x: usize) -> StepResult {
        // See if we can attack before moving.
        match self.try_attack(y, x) {
            FightResult::Fought => return StepResult::Continue,
            FightResult::Killed(y, x) => return StepResult::Killed(y, x),
            FightResult::NoFight => (),
        }

        // Otherwise, pick out enemies and move towards them
        let race = self.0[y][x].to_creature().unwrap().race;
        let enemies = self.find(|tile| tile
                                .to_creature()
                                .map(|c| c.race != race)
                                .unwrap_or(false));
        if enemies.is_empty() {
            return StepResult::Finished;
        }

        let distance = self.flood(y, x);
        let reachable = enemies.into_iter()
            .flat_map(|(y, x)| State::neighbors(y, x))
            .filter(|(y, x)| self.0[*y][*x] == Tile::Floor)
            .filter_map(|(y, x)| distance[y][x].map(|d| (y, x, d)))
            .collect::<Vec<(usize, usize, usize)>>();
        if reachable.is_empty() {
            return StepResult::Continue;
        }

        let min_distance = *reachable.iter()
            .map(|(_, _, d)| d)
            .min()
            .unwrap();

        // This is where we're trying to move to.
        let target = reachable.into_iter()
            .filter(|(_, _, d)| *d == min_distance)
            .min()
            .map(|(y, x, _)| (y, x))
            .unwrap();

        // Backtrack along the path, finding how to walk
        let mut path = self.grid(false);
        path[target.0][target.1] = true;
        {
            let mut todo = VecDeque::new();
            todo.push_back(target);
            while let Some((y, x)) = todo.pop_front() {
                let d = distance[y][x].unwrap();
                for (y, x) in State::neighbors(y, x) {
                    if let Some(nd) = distance[y][x] {
                        if d == nd + 1 && !path[y][x] {
                            path[y][x] = true;
                            todo.push_back((y, x));
                        }
                    }
                }
            }
        }
        let next_pos = State::neighbors(y, x)
            .filter(|(y, x)| path[*y][*x])
            .min().unwrap();
        self.0[next_pos.0][next_pos.1] = self.0[y][x].clone();
        self.0[y][x] = Tile::Floor;

        // Try to attack from the new position
        match self.try_attack(next_pos.0, next_pos.1) {
            FightResult::Killed(y, x) => return StepResult::Killed(y, x),
            _ => return StepResult::Continue,
        }
    }

    fn creatures(&self) -> Vec<Creature> {
        self.find(|t| t.is_creature()).iter()
            .map(|(y, x)| self.0[*y][*x].to_creature().unwrap())
            .cloned()
            .collect()
    }

    /*
     *  Executes a flood-fill from the given point, returning a
     *  grid of the same size with distance to that point or None
     */
    fn flood(&self, y: usize, x: usize) -> Vec<Vec<Option<usize>>> {
        let mut todo = VecDeque::new();
        todo.push_back((y, x));
        let mut out = self.grid(None);
        out[y][x] = Some(0);

        while let Some((y, x)) = todo.pop_front() {
            let d = out[y][x].unwrap();

            for (y, x) in State::neighbors(y, x) {
                if out[y][x].is_none() && self.0[y][x] == Tile::Floor {
                    out[y][x] = Some(d + 1);
                    todo.push_back((y, x));
                }
            }
        }
        out
    }
}

fn main()
{
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    State::from_string(&buffer, 0).print();

    for elf_power in 3.. {
        let mut state = State::from_string(&buffer, elf_power);
        let initial_elf_count = state.creatures().into_iter()
            .filter(|c| c.race == Race::Elf)
            .count();

        let mut time = 0;
        while !state.step() {
            time += 1;
        }
        let hp = state.creatures().into_iter()
            .map(|c| c.hp)
            .sum::<usize>();

        let elf_count = state.creatures().into_iter()
            .filter(|c| c.race == Race::Elf)
            .count();

        println!("At elf power {}, ended at time {} with hp {}, outcome: {}",
                 elf_power, time, hp, time * hp);
        if initial_elf_count == elf_count {
            println!("  FLAWLESS ELF VICTORY!");
            state.print();
            break;
        }
    }
}
