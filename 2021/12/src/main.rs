use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::io::BufRead;

use smallvec::SmallVec;

const START: u16 = 0;
const END: u16 = 1;

type Path = SmallVec<[u16; 16]>;

fn search(
    path: &mut Path,
    path_seen: u16,
    links: &[u16],
    small_mask: u16,
    seen: &mut HashSet<Path>,
    allow_revisit: bool,
) -> usize {
    if !seen.insert(path.clone()) {
        return 0;
    }
    let next_mask = links[*path.last().unwrap() as usize];
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
            path.push(next);
            let out = search(
                path,
                path_seen | (1 << next),
                links,
                small_mask,
                seen,
                allow_revisit,
            );
            path.pop();
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

    let mut seen: HashSet<Path> = HashSet::new();
    let mut path = Path::new();
    path.push(START);
    println!(
        "Part 1: {}",
        search(&mut path, 0, &links, small_mask, &mut seen, false)
    );

    let mut seen: HashSet<Path> = HashSet::new();
    println!(
        "Part 2: {}",
        search(&mut path, 0, &links, small_mask, &mut seen, true)
    );
}
