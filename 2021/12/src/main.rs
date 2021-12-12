use std::collections::HashMap;
use std::convert::TryInto;
use std::io::BufRead;

type Room = u16;
const START: Room = 0;
const END: Room = 1;

fn search(next: Room, path_seen: Room, links: &[Room], small_mask: Room, allow_revisit: bool) -> usize {
    let next_mask = links[next as usize];
    ((START + 1)..(Room::BITS as Room))
        .filter(|b| (next_mask & (1 << b)) != 0)
        .map(|next| {
            if next == END {
                return 1;
            }
            let revisiting = (small_mask & path_seen & (1 << next)) != 0;
            if revisiting && !allow_revisit {
                return 0;
            }
            search(
                next,
                path_seen | (1 << next),
                links,
                small_mask,
                allow_revisit && !revisiting,
            )
        })
        .sum()
}

fn main() {
    let mut rooms: HashMap<String, Room> = HashMap::new();
    rooms.insert("start".to_string(), START);
    rooms.insert("end".to_string(), END);

    let mut small_mask: Room = 0;
    let mut room_id = |name: String| -> Room {
        match rooms.get(&name) {
            Some(r) => *r,
            None => {
                let out = rooms.len().try_into().unwrap();
                if name.chars().all(|c| c.is_lowercase()) {
                    small_mask |= (1 << out) as Room;
                }
                rooms.insert(name, out);
                out
            }
        }
    };

    let mut links: Vec<Room> = vec![];
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
