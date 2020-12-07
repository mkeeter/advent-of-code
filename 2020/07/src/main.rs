use std::io::Read;
use std::str::FromStr;
use std::collections::HashMap;

fn parse(s: &str) -> (&str, HashMap<&str, usize>) {
    let mut itr = s.split(" bags contain ");
    let key = itr.next().unwrap();
    let val = itr.next().unwrap().split(", ")
        .filter_map(|sub| {
            if sub == "no other bags." {
                return None;
            }
            let mut itr = sub.splitn(2, ' ');
            let count = usize::from_str(itr.next().unwrap()).unwrap();
            let rest = itr.next().unwrap();
            let bag = rest.rsplitn(2, ' ').nth(1).unwrap();
            Some((bag, count))
        })
        .collect();
    (key, val)
}

fn search1<'a>(key: &'a str, graph: &'a HashMap<&str, HashMap<&str, usize>>,
               cache: &mut HashMap<&'a str, bool>) -> bool {
    if let Some(v) = cache.get(key) {
        *v
    } else {
        let out = graph.get(key).unwrap().iter()
            .any(|(next, _)| search1(next, graph, cache));
        cache.insert(key, out);
        out
    }
}

fn search2<'a>(key: &'a str, graph: &'a HashMap<&str, HashMap<&str, usize>>,
               cache: &mut HashMap<&'a str, usize>) -> usize {
    if let Some(v) = cache.get(key) {
        *v
    } else {
        let out = 1 + graph.get(key).unwrap().iter()
            .map(|(next, count)| count * search2(next, graph, cache))
            .sum::<usize>();
        cache.insert(key, out);
        out
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let graph = input.lines()
        .map(parse)
        .collect::<HashMap<_, _>>();

    let mut cache: HashMap<&str, bool> = HashMap::new();
    cache.insert("shiny gold", true);

    let p1 = graph.iter().map(|(k, _v)| search1(k, &graph, &mut cache))
        .filter(|i| *i)
        .count();
    println!("Part 1: {}", p1 - 1); // shiny gold is in the cache

    let mut cache: HashMap<&str, usize> = HashMap::new();
    println!("Part 2: {}", search2("shiny gold", &graph, &mut cache) - 1);
}
