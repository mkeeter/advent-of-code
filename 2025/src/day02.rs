use rayon::prelude::*;

pub fn solve(s: &str) -> (usize, usize) {
    let pairs = s
        .trim()
        .split(',')
        .map(|pair| {
            let mut iter = pair.split('-').map(|s| s.parse().unwrap());
            let a: usize = iter.next().unwrap();
            let b: usize = iter.next().unwrap();
            assert!(iter.next().is_none());
            (a, b)
        })
        .collect::<Vec<_>>();

    let part1 = pairs
        .par_iter()
        .flat_map(|(a, b)| *a..=*b)
        .filter(|v| is_symmetrical(*v))
        .sum();
    let part2 = pairs
        .par_iter()
        .flat_map(|(a, b)| *a..=*b)
        .filter(|v| is_repeated(*v))
        .sum();
    (part1, part2)
}

fn is_symmetrical(v: usize) -> bool {
    let len = v.ilog10() + 1;
    if len.is_multiple_of(2) {
        let r = 10usize.pow(len / 2);
        v % r == v / r
    } else {
        false
    }
}

fn is_repeated(v: usize) -> bool {
    let len = v.ilog10() + 1;
    for group_size in 1..len {
        if !len.is_multiple_of(group_size) {
            continue;
        }
        let r = 10usize.pow(group_size);
        let mut v = v;
        let base = v % r;
        let mut matched = true;
        for _i in 1..len / group_size {
            v /= r;
            matched &= (v % r) == base;
        }
        if matched {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let s = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let (a, b) = solve(s);
        assert_eq!(a, 1227775554);
        assert_eq!(b, 4174379265);
    }

    #[test]
    fn repeat_check() {
        for i in [
            11, 22, 99, 111, 999, 1010, 1188511885, 22222, 446446, 38593859,
            565656, 824824824, 2121212121,
        ] {
            assert!(is_repeated(i), "{i} should be repeated");
        }
    }

    #[test]
    fn non_repeat_check() {
        assert!(!is_repeated(96952600))
    }
}
