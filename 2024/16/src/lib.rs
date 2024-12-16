use std::collections::{BTreeSet, HashMap};
use util::{Dir, Grid, GridSet, TupleSet};

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
    let mut seen = TupleSet::new((g.width(), g.height(), 4usize));
    let mut prev: HashMap<_, smallvec::SmallVec<[_; 4]>> = HashMap::new();
    while let Some((score, x, y, d)) = todo.pop_first() {
        if (x, y) == end {
            out = Some((score, d));
            break;
        } else if !seen.insert((x, y, d.index())) {
            continue;
        }

        let mut push = |t| {
            prev.entry(t).or_default().push((score, x, y, d));
            todo.insert(t);
        };
        let (nx, ny) = (x + d.x(), y + d.y());
        if g[(nx, ny)] != b'#' {
            push((score + 1, nx, ny, d));
        }
        for d in [d.left(), d.right()] {
            // Only turn if there's not a wall right there
            if g[(x + d.x(), y + d.y())] != b'#' {
                push((score + 1000, x, y, d));
            }
        }
    }

    let (best_score, best_d) = out.unwrap();
    let mut todo = vec![(best_score, end.0, end.1, best_d)];
    let mut seen = TupleSet::new((g.width(), g.height(), 4));
    while let Some(t) = todo.pop() {
        if seen.insert((t.1, t.2, t.3.index())) {
            todo.extend(prev.get(&t).iter().flat_map(|i| i.iter().cloned()));
        }
    }
    let mut tiles = GridSet::new(&g);
    for (x, y, _d) in seen.iter() {
        tiles.insert(x, y);
    }

    (best_score, tiles.len() as u64)
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
