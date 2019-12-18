use std::io::BufRead;
use std::collections::{HashSet, HashMap};

type Map = HashMap<(i32, i32), char>;
type Cache = HashMap<(i32, i32, u32), u32>;

// Finds all available keys from the given position,
// return a map of their position to distance
fn available(x: i32, y: i32, keys: u32, map: &Map) -> HashMap<(i32, i32, u32), u32>
{
    let mut todo = vec![(x, y)];
    let mut next: Vec<(i32, i32)> = Vec::new();

    let mut step = 0;
    let mut found = HashMap::new();
    let mut seen = HashSet::new();
    while todo.len() > 0 {
        next.clear();
        for (tx, ty) in todo.iter() {
            if seen.contains(&(*tx, *ty)) {
                continue;
            }
            seen.insert((*tx, *ty));

            let c = *map.get(&(*tx, *ty)).unwrap_or(&'#');

            // Found a key :D
            if char::is_lowercase(c) {
                let key = 1 << ((c as u8) - ('a' as u8)) as u32;
                if keys & key == 0 {
                    found.insert((*tx, *ty, keys | key), step);
                    continue;
                }
            // Found a wall :(
            } else if c == '#' {
                continue;
            // Found a door :/
            } else if char::is_uppercase(c) {
                let door = 1 << ((c as u8) - ('A' as u8)) as u32;
                // We can't open the door :(
                if keys & door == 0 {
                    continue;
                }
            }
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
                next.push((tx + dx, ty + dy));
            }
        }

        std::mem::swap(&mut todo, &mut next);
        step += 1;
    }
    found
}

fn solve(x: i32, y: i32, keys: u32, target: u32,
         map: &Map, cache: &mut Cache) -> u32
{
    if keys == target {
        cache.insert((x, y, keys), 0);
        return 0;
    }

    if let Some(c) = cache.get(&(x, y, keys)) {
        return *c;
    }

    let r = available(x, y, keys, map)
        .iter()
        .map(|((px, py, keys), dist)|
             dist + solve(*px, *py, *keys, target, map, cache))
        .min()
        .unwrap();
    cache.insert((x, y, keys), r);
    r
}

fn main() {
    let mut tiles: Map = HashMap::new();

    let mut target = 0;
    let mut start = (0, 0);
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            tiles.insert((x as i32, y as i32), c);
            if char::is_lowercase(c) {
                let key = 1 << ((c as u8) - ('a' as u8)) as u32;
                target |= key;
            } else if c == '@' {
                start = (x as i32, y as i32);
            }
        }
    }

    let mut cache = Cache::new();
    available(start.0, start.1, 0, &tiles);
    println!("{:?}", solve(start.0, start.1, 0, target, &tiles, &mut cache));

    // 4864 is too high
}
