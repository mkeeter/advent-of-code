use std::collections::{HashMap, HashSet};
use std::io::Read;

fn parse_line(s: &str) -> (HashSet<&str>, HashSet<&str>) {
    let mut iter = s.split(" (contains ");
    let ingredients = iter.next().unwrap().split(' ').collect();

    let allergens = iter
        .next()
        .unwrap()
        .strip_suffix(')')
        .unwrap()
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
            allergens
                .entry(a)
                .and_modify(|e| *e = (&*e) & (&ings))
                .or_insert_with(|| ings.clone());
        }
    }

    // Map from ingredient to matching allergen
    let mut bound: HashMap<&str, &str> = HashMap::new();

    while let Some((allergen, ingredient)) = allergens
        .iter()
        .find(|(_a, i)| i.len() == 1)
        .map(|(a, i)| (*a, i.iter().next().copied().unwrap()))
    {
        allergens.remove(allergen).unwrap();
        for (_a, i) in allergens.iter_mut() {
            i.remove(&ingredient);
        }
        bound.insert(ingredient, allergen);
    }

    let banned: HashSet<&str> = bound.keys().copied().collect();
    let out = ingredient_count
        .iter()
        .filter(|(i, _n)| !banned.contains(*i))
        .map(|(_i, n)| n)
        .sum::<usize>();
    println!("Part 1: {}", out);

    let mut ingredients: Vec<&str> = banned.into_iter().collect();
    ingredients.sort_by_key(|i| bound.get(i).unwrap());
    println!("Part 2: {}", ingredients.join(","));
}
