use std::collections::HashMap;
use std::convert::TryInto;
use std::io::BufRead;

const START: u16 = 0;
const END: u16 = 1;

struct Graph<'a> {
    links: &'a [u16],
    small_mask: u16,
}

impl<'a> Graph<'a> {
    fn search(&self, next: u16, visited: u16, allow_revisit: bool) -> usize {
        let next_mask = self.links[next as usize];
        ((START + 1)..self.links.len() as u16)
            .filter(|b| (next_mask & (1 << b)) != 0)
            .map(|next| {
                if next == END {
                    return 1;
                }
                let revisiting = (self.small_mask & visited & (1 << next)) != 0;
                if revisiting && !allow_revisit {
                    return 0;
                }
                self.search(next, visited | (1 << next), allow_revisit && !revisiting)
            })
            .sum()
    }
}

fn main() {
    let mut rooms: HashMap<String, u16> = HashMap::new();
    rooms.insert("start".to_string(), START);
    rooms.insert("end".to_string(), END);

    let mut small_mask: u16 = 0;
    let mut room_id = |name: String| -> u16 {
        match rooms.get(&name) {
            Some(r) => *r,
            None => {
                let out = rooms.len().try_into().unwrap();
                if name.chars().all(|c| c.is_lowercase()) {
                    small_mask |= (1 << out) as u16;
                }
                rooms.insert(name, out);
                out
            }
        }
    };

    let mut links: Vec<u16> = vec![];
    std::io::stdin().lock().lines().for_each(|line| {
        let line = line.unwrap();
        let mut iter = line.split('-');
        let a = room_id(iter.next().unwrap().to_string());
        let b = room_id(iter.next().unwrap().to_string());

        links.resize(links.len().max(a.max(b) as usize + 1), 0);
        links[a as usize] |= 1 << b;
        links[b as usize] |= 1 << a;
    });

    let g = Graph {
        links: &links,
        small_mask,
    };

    println!("Part 1: {}", g.search(START, 0, false));
    println!("Part 2: {}", g.search(START, 0, true));
}
