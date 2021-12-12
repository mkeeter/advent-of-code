use std::collections::{HashMap, HashSet};
use std::io::Read;

type Path<'a> = Vec<&'a str>;

fn search<'a>(
    path: Vec<&'a str>,
    links: &HashMap<&'a str, Path<'a>>,
    seen: &mut HashSet<Path<'a>>,
    allow_revisit: bool,
) -> usize {
    if !seen.insert(path.clone()) {
        return 0;
    }
    if *path.last().unwrap() == "end" {
        return 1;
    }
    let mut out = 0;
    for next in links[path.last().unwrap()].iter() {
        if *next == "start" {
            continue;
        }
        let is_small = next.chars().all(|c| c.is_lowercase());
        // something something accidentally quadratic
        let prev_seen = is_small && path.iter().any(|p| p == next);
        let allow_revisit = if prev_seen {
            if !allow_revisit {
                continue;
            } else {
                false
            }
        } else {
            allow_revisit
        };
        let mut path = path.clone();
        path.push(next);
        out += search(path, links, seen, allow_revisit);
    }
    out
}

fn main() {
    let mut links: HashMap<&str, Vec<&str>> = HashMap::new();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    input.lines().for_each(|line| {
        let mut iter = line.split('-');
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();

        links.entry(a).or_default().push(b);
        links.entry(b).or_default().push(a);
    });

    let mut seen: HashSet<Vec<&str>> = HashSet::new();
    println!("Part 1: {}", search(vec!["start"], &links, &mut seen, false));

    let mut seen: HashSet<Vec<&str>> = HashSet::new();
    println!("Part 2: {}", search(vec!["start"], &links, &mut seen, true));
}
