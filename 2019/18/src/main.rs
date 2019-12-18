use std::io::BufRead;
use std::collections::HashMap;

type Map = HashMap<(i32, i32), char>;
type Cache = HashMap<(i32, i32, u32), Option<u32>>;

fn step(x: i32, y: i32, keys: u32, target: u32,
         map: &Map, cache: &mut Cache) -> Option<u32>
{
    [(0, 1), (0, -1), (1, 0), (-1, 0)].iter()
        .filter_map(|(dx, dy)| solve(x + dx, y + dy, keys, target, map, cache))
        .map(|i| i + 1)
        .min()
}

fn solve(x: i32, y: i32, keys: u32, target: u32,
         map: &Map, cache: &mut Cache) -> Option<u32>
{
    if keys == target {
        println!("Success!");
        cache.insert((x, y, keys), Some(0));
        return Some(0)
    }

    if let Some(c) = cache.get(&(x, y, keys)) {
        return *c;
    }

    let c = *map.get(&(x, y)).unwrap_or(&'#');
    cache.insert((x, y, keys), None);
    let result = if c == '#' {
        None
    } else if char::is_lowercase(c) {
        let key = 1 << ((c as u8) - ('a' as u8)) as u32;
        if (keys & key) == 0 {
            solve(x, y, keys | key, target, map, cache)
        } else {
            step(x, y, keys, target, map, cache)
        }
    } else if char::is_uppercase(c) {
        let door = 1 << ((c as u8) - ('A' as u8)) as u32;
        if (keys & door) == 0 {
            None
        } else {
            step(x, y, keys, target, map, cache)
        }
    } else {
        println!("{} {} {}", x, y, keys);
        assert!(c == '.' || c == '@');
        step(x, y, keys, target, map, cache)
    };

    cache.insert((x, y, keys), result);
    result
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
    println!("{:?}", solve(start.0, start.1, 0, target, &tiles, &mut cache));
    println!("Hello, world!");
    println!("{}", target);

    // 4864 is too high
}
