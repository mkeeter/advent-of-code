use std::collections::HashMap;
use std::str::FromStr;
use std::io::BufRead;
use smallvec::{SmallVec, smallvec};

type Path = SmallVec<[u8; 8]>;

fn paths(s: Path, todo: &Path) -> Vec<Path> {
    if todo.is_empty() {
        return vec![s];
    } else {
        todo.iter()
            .flat_map(|t| {
                let mut next_path = s.clone();
                next_path.push(*t);
                let next_todo = todo.iter()
                    .filter(|i| *i != t)
                    .map(|i| *i)
                    .collect();
                paths(next_path, &next_todo) })
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

    let mut edges = HashMap::new();
    for line in std::io::stdin().lock().lines() {
        let words = line.unwrap()
            .split(" ")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let a = planet(&words[0]);
        let b = planet(&words[2]);
        let dist = usize::from_str(words.last().unwrap()).unwrap();
        edges.entry(a)
            .or_insert(HashMap::new())
            .insert(b, dist);
        edges.entry(b)
            .or_insert(HashMap::new())
            .insert(a, dist);
    }

    let start = planets.values().map(|i| *i).collect::<Path>();
    let ps = paths(smallvec![], &start);
    let best: usize = ps.iter()
        .map(|path| path[..].windows(2)
             .map(|w| {
                 edges.get(&w[0]).unwrap().get(&w[1]).unwrap()
             })
            .sum())
        .min()
        .unwrap();
    println!("{:?}", best);

    let worst: usize = ps.iter()
        .map(|path| path[..].windows(2)
             .map(|w| {
                 edges.get(&w[0]).unwrap().get(&w[1]).unwrap()
             })
            .sum())
        .max()
        .unwrap();
    println!("{:?}", worst);
}
