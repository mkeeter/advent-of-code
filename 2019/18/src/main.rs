use std::io::BufRead;
use std::collections::{HashSet, HashMap, VecDeque};
use smallvec::{SmallVec, smallvec};

////////////////////////////////////////////////////////////////////////////////

type Map = HashMap<(i32, i32), char>;
type Bots = SmallVec<[(i32, i32); 4]>;

#[derive(Clone, Hash, Eq, PartialEq)]
struct State {
    bots: Bots,
    keys: u32,
}

type Cache = HashMap<State, u32>;

struct Edge {
    target_x: i32,
    target_y: i32,
    required_keys: u32,
    new_key: u32,
    steps: u32,
}
type Edges = HashMap<(i32, i32), Vec<Edge>>;

////////////////////////////////////////////////////////////////////////////////

fn explore(x: i32, y: i32, map: &Map) -> Vec<Edge> {
    let mut todo = VecDeque::new();
    todo.push_back((x, y, 0, 0));

    let mut found = Vec::new();
    let mut seen = HashSet::new();
    while let Some((tx, ty, keys, step)) = todo.pop_front() {
        if !seen.insert((tx, ty)) {
            continue;
        }

        let c = *map.get(&(tx, ty)).unwrap_or(&'#');
        let mut door = 0;

        // Found a key :D
        if char::is_lowercase(c) && (tx != x || ty != y) {
            let key = 1 << ((c as u8) - b'a') as u32;
            found.push( Edge {
                target_x: tx,
                target_y: ty,
                required_keys: keys,
                new_key: key,
                steps: step });

        // Found a wall :(
        } else if c == '#' {
            continue;

        // Found a door :)
        } else if char::is_uppercase(c) {
            door = 1 << ((c as u8) - b'A') as u32;
        }

        // Take new steps
        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            todo.push_back((tx + dx, ty + dy, keys | door, step + 1));
        }
    }
    found
}

fn solve(state: State, target: u32, edges: &Edges, cache: &mut Cache) -> u32
{
    if state.keys == target {
        return 0;
    } else if let Some(c) = cache.get(&state) {
        return *c;
    }

    let r = state.bots.iter()
        .enumerate()
        .flat_map(|(i, b)| edges.get(&(b.0, b.1))
            .unwrap()
            .iter()
            .map(move |e| (i, e)))
        .filter(|(_i, e)|
            (e.required_keys & state.keys) == e.required_keys &&
            (e.new_key & state.keys) == 0)
        .map(|(i, e)| {
             let mut next = state.clone();
             next.bots[i].0 = e.target_x;
             next.bots[i].1 = e.target_y;
             next.keys |= e.new_key;
             e.steps + solve(next, target, edges, cache) })
        .min()
        .unwrap();

    cache.insert(state, r);
    r
}

fn build_graph(bots: &Bots, tiles: &Map) -> Edges {
    tiles.iter()
        .filter(|(_k, v)| char::is_lowercase(**v))
        .map(|(k, _v)| k)
        .chain(bots.iter())
        .map(|p| (*p, explore(p.0, p.1, &tiles)))
        .collect::<Edges>()
}

fn main() {
    let mut tiles: Map = HashMap::new();

    let mut target = 0;
    let mut start = (0, 0);
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            tiles.insert((x as i32, y as i32), c);
            if char::is_lowercase(c) {
                let key = 1 << ((c as u8) - b'a') as u32;
                target |= key;
            } else if c == '@' {
                start = (x as i32, y as i32);
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // Part 1
    let mut cache = Cache::new();
    let bots = smallvec![(start.0, start.1)];
    let edges = build_graph(&bots, &tiles);
    let state = State { bots, keys: 0 };
    println!("Part 1: {}", solve(state, target, &edges, &mut cache));

    ////////////////////////////////////////////////////////////////////////////
    // Part 2
    let mut cache = Cache::new();
    let mut tiles = tiles;

    // Modify the map to add a cross pattern at the bot's original location
    for (dx, dy) in &[(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
        tiles.insert((start.0 + dx, start.1 + dy), '#');
    }

    // We're now running four bots simultaneously
    let bots = [(1, 1), (1, -1), (-1, 1), (-1, -1)].iter()
        .map(|(dx, dy)| (start.0 + dx, start.1 + dy))
        .collect();

    let edges = build_graph(&bots, &tiles);
    let state = State { bots, keys: 0 };
    println!("Part 2: {}", solve(state, target, &edges, &mut cache));
}
