use smallvec::{smallvec, SmallVec};
use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

type Path = SmallVec<[u8; 8]>;

fn paths(s: Path, todo: &Path) -> Vec<Path> {
    if todo.is_empty() {
        return vec![s];
    } else {
        todo.iter()
            .flat_map(|t| {
                let mut next_path = s.clone();
                next_path.push(*t);
                let next_todo = todo.iter().filter(|i| *i != t).copied().collect();
                paths(next_path, &next_todo)
            })
            .collect()
    }
}

fn main() {
    let mut planets = HashMap::new();
    let mut planet = |name: &str| -> u8 {
        if let Some(p) = planets.get(name) {
            *p
        } else {
            let i = planets.len() as u8;
            planets.insert(name.to_owned(), i);
            i
        }
    };

    let mut edges: HashMap<u8, HashMap<u8, usize>> = HashMap::new();
    for line in std::io::stdin().lock().lines() {
        let words = line
            .unwrap()
            .split(' ')
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let a = planet(&words[0]);
        let b = planet(&words[2]);
        let dist = usize::from_str(words.last().unwrap()).unwrap();
        edges.entry(a).or_default().insert(b, dist);
        edges.entry(b).or_default().insert(a, dist);
    }

    let start = planets.values().copied().collect::<Path>();
    let ps = paths(smallvec![], &start);
    let best: usize = ps
        .iter()
        .map(|path| {
            path[..]
                .windows(2)
                .map(|w| edges.get(&w[0]).unwrap().get(&w[1]).unwrap())
                .sum()
        })
        .min()
        .unwrap();
    println!("{:?}", best);

    let worst: usize = ps
        .iter()
        .map(|path| {
            path[..]
                .windows(2)
                .map(|w| edges.get(&w[0]).unwrap().get(&w[1]).unwrap())
                .sum()
        })
        .max()
        .unwrap();
    println!("{:?}", worst);
}
