const WIDTH: usize = 5;
const HEIGHT: usize = 7;

pub fn solve(s: &str) -> (usize, String) {
    let mut iter = std::iter::once("").chain(s.lines());
    let mut locks = vec![];
    let mut keys = vec![];
    while let Some(..) = iter.next() {
        let mut min = [u8::MAX; WIDTH];
        let mut max = [0u8; WIDTH];
        for y in 0..HEIGHT {
            let line = iter.next().unwrap();
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    min[x] = min[x].min(y as u8);
                    max[x] = max[x].max(y as u8);
                }
            }
        }
        if min.iter().all(|i| *i == 0) {
            locks.push(max);
        } else if max.iter().all(|i| *i == HEIGHT as u8 - 1) {
            keys.push(min.map(|i| HEIGHT as u8 - 1 - i));
        } else {
            panic!("found neither lock nor key");
        }
    }
    let mut valid = 0;
    for lock in &locks {
        for key in &keys {
            if lock
                .iter()
                .zip(key.iter())
                .all(|(a, b)| *a + *b < HEIGHT as u8 - 1)
            {
                valid += 1;
            }
        }
    }

    (valid, "⭐".to_owned())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const EXAMPLE_1: &str = indoc::indoc! {"
            #####
            .####
            .####
            .####
            .#.#.
            .#...
            .....

            #####
            ##.##
            .#.##
            ...##
            ...#.
            ...#.
            .....

            .....
            #....
            #....
            #...#
            #.#.#
            #.###
            #####

            .....
            .....
            #.#..
            ###..
            ###.#
            ###.#
            #####

            .....
            .....
            .....
            #....
            #.#..
            #.#.#
            #####
        "};
        assert_eq!(solve(EXAMPLE_1).0, 3);
    }
}
