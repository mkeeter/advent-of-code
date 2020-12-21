use std::io::Read;
use std::collections::{HashMap, HashSet};

fn parse_line(s: &str) -> (HashSet<&str>, HashSet<&str>) {
    let mut iter = s.split(" (contains ");
    let ingredients = iter.next().unwrap()
        .split(' ')
        .collect();

    let allergens = iter.next().unwrap();
    let allergens = allergens[0..allergens.len() - 1]
        .split(", ")
        .collect();

    (ingredients, allergens)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    // Map from an allergen name to a list of ingredients that *may* contain it
    let mut allergens: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut ingredient_count: HashMap<&str, usize> = HashMap::new();

    for line in input.lines() {
        let (ings, algs) = parse_line(line);
        for i in ings.iter() {
            *ingredient_count.entry(*i).or_insert(0) += 1;
        }
        for a in algs.iter() {
            let e = allergens.entry(a).or_insert(ings.clone());
            *e = e.intersection(&ings).map(|i| *i).collect();
        }
    }

    // Map from ingredient to matching allergen
    let mut bound: HashMap<&str, &str> = HashMap::new();

    while let Some(allergen) = allergens.iter()
            .find(|(_a, i)| i.len() == 1)
            .map(|(a, _i)| *a)
    {
        let ings: HashSet<&str> = allergens.remove(allergen).unwrap();
        let ing: &str = ings.into_iter().next().unwrap();
        for (_a, i) in allergens.iter_mut() {
            i.remove(&ing);
        }
        bound.insert(ing, allergen);
    }

    let banned: HashSet<&str> = bound.iter().map(|(k, _v)| *k).collect();
    let out = ingredient_count.iter()
        .filter(|(i, _n)| !banned.contains(*i))
        .map(|(_i, n)| n)
        .sum::<usize>();
    println!("{}", out);

    let mut ingredients: Vec<&str> = ingredient_count.into_iter()
        .map(|(i, _n)| i)
        .filter(|i| bound.contains_key(i))
        .collect();
    ingredients.sort_by_key(|i| bound.get(i).unwrap());
    println!("Part 2: {}", ingredients.join(","));
}
