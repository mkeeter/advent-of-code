use std::collections::{BTreeSet, HashMap, HashSet};
use util::Grid;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn cw(&self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }
    fn ccw(&self) -> Self {
        match self {
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
            Dir::N => Dir::W,
        }
    }
    fn dx(&self) -> i64 {
        match self {
            Dir::E => 1,
            Dir::W => -1,
            Dir::S | Dir::N => 0,
        }
    }
    fn dy(&self) -> i64 {
        match self {
            Dir::E | Dir::W => 0,
            Dir::S => 1,
            Dir::N => -1,
        }
    }
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::N => 'N',
                Dir::E => 'E',
                Dir::S => 'S',
                Dir::W => 'N',
            }
        )
    }
}

pub fn solve(s: &str) -> (u64, u64) {
    let g = Grid::new(s);
    let mut start = None;
    let mut end = None;
    for y in 0..g.height() {
        for x in 0..g.width() {
            match g[(x, y)] {
                b'E' => {
                    assert!(end.is_none());
                    end = Some((x, y))
                }
                b'S' => {
                    assert!(start.is_none());
                    start = Some((x, y))
                }
                _ => (),
            }
        }
    }

    let mut todo = BTreeSet::new();
    let (x, y) = start.unwrap();
    todo.insert((0, x, y, Dir::E));
    let end = end.unwrap();

    let mut out = None;
    let mut seen = HashSet::new();
    let mut prev: HashMap<_, HashSet<_>> = HashMap::new();
    while let Some((score, x, y, d)) = todo.pop_first() {
        let mut push = |t| {
            prev.entry(t).or_default().insert((score, x, y, d));
            todo.insert(t);
        };
        if !seen.insert((x, y, d)) {
            continue;
        }
        if (x, y) == end {
            out = Some((score, d));
            break;
        }
        let (nx, ny) = (x + d.dx(), y + d.dy());
        if g[(nx, ny)] != b'#' {
            push((score + 1, nx, ny, d));
        }
        push((score + 1000, x, y, d.cw()));
        push((score + 1000, x, y, d.ccw()));
    }

    let (best_score, best_d) = out.unwrap();
    let mut todo = vec![(best_score, end.0, end.1, best_d)];
    let mut seen = HashSet::new();
    while let Some(t) = todo.pop() {
        seen.insert((t.1, t.2));
        todo.extend(prev.get(&t).iter().flat_map(|i| i.iter().cloned()));
    }

    (best_score, seen.len() as u64)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const SMALL: &str = indoc::indoc! {"
            ###############
            #.......#....E#
            #.#.###.#.###.#
            #.....#.#...#.#
            #.###.#####.#.#
            #.#.#.......#.#
            #.#.#####.###.#
            #...........#.#
            ###.#.#####.#.#
            #...#.....#.#.#
            #.#.#.###.#.#.#
            #.....#...#.#.#
            #.###.#.#.#.#.#
            #S..#.....#...#
            ###############
        "};
        assert_eq!(solve(SMALL), (7036, 45));

        const LARGE: &str = indoc::indoc! {"
            #################
            #...#...#...#..E#
            #.#.#.#.#.#.#.#.#
            #.#.#.#...#...#.#
            #.#.#.#.###.#.#.#
            #...#.#.#.....#.#
            #.#.#.#.#.#####.#
            #.#...#.#.#.....#
            #.#.#####.#.###.#
            #.#.#.......#...#
            #.#.###.#####.###
            #.#.#...#.....#.#
            #.#.#.#####.###.#
            #.#.#.........#.#
            #.#.#.#########.#
            #S#.............#
            #################
        "};
        assert_eq!(solve(LARGE), (11048, 64));
    }
}
