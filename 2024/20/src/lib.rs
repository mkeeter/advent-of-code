use std::collections::{HashMap, VecDeque};
use util::{Dir, Grid};

pub fn solve_with(s: &str, cheat_length: u64) -> HashMap<u64, usize> {
    let g = Grid::new(s);
    let mut start = None;
    let mut end = None;
    for y in 0..g.height() {
        for x in 0..g.width() {
            match g[(x, y)] {
                b'S' => start = Some((x, y)),
                b'E' => end = Some((x, y)),
                b'#' | b'.' => (),
                c => panic!("invalid grid character '{c}'"),
            }
        }
    }
    let start = start.unwrap();
    let end = end.unwrap();

    let flood = |pos: (i64, i64)| {
        // Make a map of distance-to-the-end
        let mut todo = VecDeque::new();
        let mut dist = HashMap::new();
        todo.push_back((0, pos.0, pos.1));
        while let Some((t, x, y)) = todo.pop_front() {
            match dist.entry((x, y)) {
                std::collections::hash_map::Entry::Vacant(v) => v.insert(t),
                std::collections::hash_map::Entry::Occupied(..) => continue,
            };
            for d in Dir::iter() {
                let x = x + d.x();
                let y = y + d.y();
                if g[(x, y)] != b'#' {
                    todo.push_back((t + 1, x, y));
                }
            }
        }
        dist
    };
    let distance_to_start = flood(start);
    let distance_to_end = flood(end);
    let shortest_path = distance_to_end[&start];

    let mut skip_count: HashMap<u64, usize> = HashMap::new();
    for ((sx, sy), sd) in &distance_to_start {
        for ((ex, ey), ed) in &distance_to_end {
            let d = ex.abs_diff(*sx) + ey.abs_diff(*sy);
            if d <= cheat_length {
                let path_len = d + sd + ed;
                if path_len < shortest_path {
                    let delta = shortest_path - path_len;
                    *skip_count.entry(delta).or_default() += 1;
                }
            }
        }
    }
    skip_count
}

pub fn solve(s: &str) -> (usize, usize) {
    let run = |i| {
        solve_with(s, i)
            .iter()
            .filter(|(saved, _count)| **saved >= 100)
            .map(|(_saved, count)| *count)
            .sum()
    };

    (run(2), run(20))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const EXAMPLE: &str = indoc::indoc! {"
            ###############
            #...#...#.....#
            #.#.#.#.#.###.#
            #S#...#.#.#...#
            #######.#.#.###
            #######.#.#...#
            #######.#.###.#
            ###..E#...#...#
            ###.#######.###
            #...###...#...#
            #.#####.#.###.#
            #.#...#.#.#...#
            #.#.#.#.#.#.###
            #...#...#...###
            ###############
        "};
        let s = solve_with(EXAMPLE, 2);
        assert_eq!(s[&2], 14);
        assert_eq!(s[&4], 14);
        assert_eq!(s[&6], 2);
        assert_eq!(s[&8], 4);
        assert_eq!(s[&10], 2);
        assert_eq!(s[&12], 3);
        assert_eq!(s[&20], 1);
        assert_eq!(s[&36], 1);
        assert_eq!(s[&38], 1);
        assert_eq!(s[&40], 1);
        assert_eq!(s[&64], 1);

        let s = solve_with(EXAMPLE, 20);
        assert_eq!(s[&50], 32);
        assert_eq!(s[&52], 31);
        assert_eq!(s[&54], 29);
        assert_eq!(s[&56], 39);
        assert_eq!(s[&58], 25);
        assert_eq!(s[&60], 23);
        assert_eq!(s[&62], 20);
        assert_eq!(s[&64], 19);
        assert_eq!(s[&66], 12);
        assert_eq!(s[&68], 14);
        assert_eq!(s[&70], 12);
        assert_eq!(s[&72], 22);
        assert_eq!(s[&74], 4);
        assert_eq!(s[&76], 3);
    }
}
