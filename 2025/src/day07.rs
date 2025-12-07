use crate::Grid;
use std::collections::HashMap;

pub fn solve(s: &str) -> (usize, usize) {
    let g = Grid::new(s);
    let mut beams = (0..g.width())
        .filter(|x| g[(*x, 0)] == b'S')
        .collect::<Vec<_>>();
    assert_eq!(beams.len(), 1);
    let start_pos = beams[0];

    let mut next = Vec::with_capacity(usize::try_from(g.width()).unwrap());
    let mut split_count = 0;
    for y in 1..g.height() {
        next.clear();
        let mut insert = |x| {
            if next.last() != Some(&x) {
                next.push(x);
            }
        };
        for &x in &beams {
            if g[(x, y)] == b'^' {
                insert(x - 1);
                insert(x + 1);
                split_count += 1;
            } else {
                insert(x);
            }
        }
        std::mem::swap(&mut beams, &mut next);
    }

    let part2 = recurse(&g, start_pos, 0, &mut Default::default());
    (split_count, part2)
}

fn recurse(
    g: &Grid,
    x: i64,
    y: i64,
    cache: &mut HashMap<(i64, i64), usize>,
) -> usize {
    if let Some(v) = cache.get(&(x, y)) {
        *v
    } else {
        assert!(g[(x, y)] == b'.' || g[(x, y)] == b'S');
        let y = y + 1;
        let v = if y < g.height() {
            if g[(x, y)] == b'^' {
                recurse(g, x - 1, y, cache) + recurse(g, x + 1, y, cache)
            } else {
                recurse(g, x, y, cache)
            }
        } else {
            1
        };
        cache.insert((x, y), v);
        v
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        // Trailing spaces are load-bearing here!
        let s = indoc::indoc! {"
            .......S.......
            ...............
            .......^.......
            ...............
            ......^.^......
            ...............
            .....^.^.^.....
            ...............
            ....^.^...^....
            ...............
            ...^.^...^.^...
            ...............
            ..^...^.....^..
            ...............
            .^.^.^.^.^...^.
            ...............
        "};
        let (a, b) = solve(s);
        assert_eq!(a, 21);
        assert_eq!(b, 40);
    }
}
