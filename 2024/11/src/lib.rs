use std::collections::HashMap;
use util::get_integers;

pub fn recurse(
    i: u64,
    steps: u8,
    cache: &mut HashMap<(u64, u8), usize>,
) -> usize {
    if steps == 0 {
        1
    } else if i == 0 {
        recurse(1, steps - 1, cache)
    } else {
        let d = i.ilog10() + 1;
        if d % 2 == 0 {
            if let Some(v) = cache.get(&(i, steps)) {
                return *v;
            }
            let scale = 10u64.pow(d / 2);
            let v = recurse(i % scale, steps - 1, cache)
                + recurse(i / scale, steps - 1, cache);
            cache.insert((i, steps), v);
            v
        } else {
            recurse(i * 2024, steps - 1, cache)
        }
    }
}

pub fn solve(s: &str) -> (usize, usize) {
    let mut cache = HashMap::new();
    get_integers(s)
        .map(|i| (recurse(i, 25, &mut cache), recurse(i, 75, &mut cache)))
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        const EXAMPLE: &str = "125 17";
        assert_eq!(solve(EXAMPLE).0, 55312);
    }
}
