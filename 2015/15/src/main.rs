use std::cmp::max;
use std::io::BufRead;
use std::str::FromStr;

type Recipe = [i32; 5];

fn optimize<F>(remaining: usize, recipe: Recipe, ingredients: &[Recipe], pred: &F) -> Option<i32>
where
    F: Fn(&Recipe) -> bool,
{
    if remaining == 0 {
        if pred(&recipe) {
            return Some(recipe.iter().take(4).fold(1, |acc, i| acc * max(*i, 0)));
        } else {
            return None;
        }
    }
    let add = |tbs| {
        let mut recipe = recipe;
        for (i, j) in ingredients[0].iter().enumerate() {
            recipe[i] += j * tbs;
        }
        recipe
    };
    // If we only have one ingredient left, then use it all
    if ingredients.len() == 1 {
        optimize(0, add(remaining as i32), &ingredients[1..], pred)
    // Otherwise, try all possible combinations
    } else {
        (0..remaining)
            .filter_map(|tbs| optimize(remaining - tbs, add(tbs as i32), &ingredients[1..], pred))
            .max()
    }
}

fn main() {
    let ingredients = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let mut out = [0; 5];
            line.unwrap()
                .replace(',', " ")
                .split(' ')
                .filter_map(|word| i32::from_str(word).ok())
                .enumerate()
                .for_each(|(i, v)| out[i] = v);
            out
        })
        .collect::<Vec<Recipe>>();

    let part1 = optimize(100, [0; 5], &ingredients, &|_| true);
    println!("Part 1: {}", part1.unwrap());

    let part2 = optimize(100, [0; 5], &ingredients, &|r| r[4] == 500);
    println!("Part 2: {}", part2.unwrap());
}
