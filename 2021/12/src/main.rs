use std::collections::HashMap;
use std::convert::TryInto;
use std::io::BufRead;

const START: u16 = 0;
const END: u16 = 1;

fn search(next: u16, path_seen: u16, links: &[u16], small_mask: u16, allow_revisit: bool) -> usize {
    let next_mask = links[next as usize];
    (0..16)
        .filter(|b| (next_mask & (1 << b)) != 0)
        .map(|next| {
            match next {
                START => return 0,
                END => return 1,
                _ => (),
            }
            let allow_revisit = if (small_mask & path_seen & (1 << next)) != 0 {
                if !allow_revisit {
                    return 0;
                } else {
                    false
                }
            } else {
                allow_revisit
            };
            let out = search(
                next,
                path_seen | (1 << next),
                links,
                small_mask,
                allow_revisit,
            );
            out
        })
        .sum()
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
                    small_mask |= 1u16 << out;
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

    println!("Part 1: {}", search(START, 0, &links, small_mask, false));
    println!("Part 2: {}", search(START, 0, &links, small_mask, true));
}
