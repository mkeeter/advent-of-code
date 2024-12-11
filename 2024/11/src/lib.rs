use std::collections::HashMap;
use util::get_integers;

struct StepCache<const N: usize>([HashMap<u64, u64>; N]);
impl<const N: usize> StepCache<N> {
    fn new() -> Self {
        Self(std::array::from_fn(|_| HashMap::new()))
    }
    fn get(&self, i: u64, steps: usize) -> Option<&u64> {
        steps
            .checked_sub(1)
            .and_then(|s| self.0.get(s))
            .and_then(|v| v.get(&i))
    }
    fn insert(&mut self, i: u64, steps: usize, v: u64) {
        steps
            .checked_sub(1)
            .and_then(|s| self.0.get_mut(s))
            .and_then(|h| h.insert(i, v));
    }
}

fn recurse<const N: usize>(
    i: u64,
    steps: usize,
    cache: &mut StepCache<N>,
) -> u64 {
    if steps == 0 {
        1
    } else if i == 0 {
        recurse(1, steps - 1, cache)
    } else {
        let d = i.ilog10() + 1;
        if d % 2 == 0 {
            if let Some(v) = cache.get(i, steps) {
                *v
            } else {
                let scale = 10u64.pow(d / 2);
                let v = recurse(i % scale, steps - 1, cache)
                    + recurse(i / scale, steps - 1, cache);
                cache.insert(i, steps, v);
                v
            }
        } else {
            recurse(i * 2024, steps - 1, cache)
        }
    }
}

pub fn solve(s: &str) -> (u64, u64) {
    let mut cache = StepCache::<75>::new();
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
