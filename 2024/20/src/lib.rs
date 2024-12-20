use std::collections::{hash_map::Entry, BTreeMap, HashMap, VecDeque};
use util::{Dir, Grid};

struct Map {
    distance_to_start: HashMap<(i64, i64), u64>,
    distance_to_end: BTreeMap<i64, BTreeMap<i64, u64>>,
    shortest_path: u64,
}

impl Map {
    fn new(s: &str) -> Self {
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

        // Make a map of distances from a particular point
        let flood = |pos: (i64, i64)| {
            let mut todo = VecDeque::new();
            let mut dist = HashMap::new();
            todo.push_back((0, pos.0, pos.1));
            while let Some((t, x, y)) = todo.pop_front() {
                match dist.entry((x, y)) {
                    Entry::Vacant(v) => v.insert(t),
                    Entry::Occupied(..) => continue,
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
        let de = flood(end);
        let mut distance_to_end: BTreeMap<i64, BTreeMap<i64, u64>> =
            BTreeMap::new();
        for ((x, y), v) in de {
            distance_to_end.entry(x).or_default().insert(y, v);
        }
        Self {
            shortest_path: distance_to_start[&end],
            distance_to_start,
            distance_to_end,
        }
    }

    fn run(&self, cheat_length: i64) -> HashMap<u64, usize> {
        let mut skip_count: HashMap<u64, usize> = HashMap::new();
        for ((sx, sy), sd) in &self.distance_to_start {
            for ((ex, ey), ed) in self
                .distance_to_end
                .range(*sx - cheat_length..=*sx + cheat_length)
                .flat_map(|(ex, cols)| {
                    cols.range(*sy - cheat_length..=*sy + cheat_length)
                        .map(move |(ey, ed)| ((ex, ey), ed))
                })
            {
                let d = ex.abs_diff(*sx) + ey.abs_diff(*sy);
                if d <= cheat_length as u64 {
                    let path_len = d + *sd + *ed;
                    if path_len < self.shortest_path {
                        let delta = self.shortest_path - path_len;
                        *skip_count.entry(delta).or_default() += 1;
                    }
                }
            }
        }
        skip_count
    }
}

pub fn solve(s: &str) -> (usize, usize) {
    let m = Map::new(s);
    let run = |i| {
        m.run(i)
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
        let m = Map::new(EXAMPLE);

        let s = m.run(2);
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

        let s = m.run(20);
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
