use std::collections::HashSet;
use std::io::BufRead;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Item {
    Generator(String),
    Microchip(String),
}
use Item::*;

impl Item {
    fn is_microchip(&self) -> bool {
        match self {
            Generator(_) => false,
            Microchip(_) => true,
        }
    }
    fn element(&self) -> &str {
        match self {
            Generator(s) => s,
            Microchip(s) => s,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct State {
    floors: [HashSet<Item>; 4],
    elevator: u8,
}

impl State {
    fn valid(&self) -> bool {
        self.floors.iter().all(|f|
            // Either this floor only contains microchips
            f.iter().all(|i| i.is_microchip()) ||
            // Or every microchip has a matching generator
            f.iter().filter(|i| i.is_microchip())
                .all(|i| f.contains(&Generator(i.element().to_string()))))
    }
}

fn main() {
    let mut state: State = Default::default();

    for (floor, line) in std::io::stdin().lock().lines().enumerate() {
        let line = line.unwrap();
        let words = line.split(' ').collect::<Vec<_>>();
        for ws in words.windows(2) {
            if ws[1].starts_with("microchip") {
                let element = ws[0].replace("-compatible", "");
                state.floors[floor].insert(Item::Microchip(element));
            } else if ws[1].starts_with("generator") {
                let element = ws[0].to_string();
                state.floors[floor].insert(Item::Generator(element));
            }
        }
    }
    assert!(state.valid());
    println!("{:?}", state);
    println!("Hello, world!");
}
