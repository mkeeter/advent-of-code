use rayon::prelude::*;
use util::{Grid, TupleSet};

pub fn solve(s: &str) -> (usize, usize) {
    let g = Grid::new(s);
    let mut pos = None;
    for y in 0..g.height() {
        for x in 0..g.width() {
            if g[(x, y)] == b'^' {
                pos = Some((x, y));
                break;
            }
        }
    }
    let Some(start) = pos else {
        panic!("could not find guard's starting position");
    };
    assert_eq!(g.width(), g.height());
    let mut seen = TupleSet::new((g.width(), g.height()));
    {
        let (mut x, mut y) = start;
        let (mut dx, mut dy) = (0i64, -1i64);
        loop {
            seen.insert((x, y));
            match g.get(x + dx, y + dy) {
                Some(&b'#') => {
                    (dx, dy) = (-dy, dx);
                }
                Some(_) => {
                    x += dx;
                    y += dy;
                }
                None => break,
            }
        }
    }

    // haha Rust go brrrrrrr
    let count = (0..g.height())
        .into_par_iter()
        .flat_map(|by| (0..g.width()).into_par_iter().map(move |bx| (bx, by)))
        .filter(|&(bx, by)| {
            if !seen.contains((bx, by)) {
                return false;
            }
            let (mut x, mut y) = start;
            let (mut dx, mut dy) = (0i64, -1i64);
            let mut angle = 0;
            let mut seen = TupleSet::new((g.width(), g.height(), 4));
            loop {
                let rot = (x + dx == bx && y + dy == by)
                    || match g.get(x + dx, y + dy) {
                        Some(&b'#') => true,
                        Some(_) => false,
                        None => return false,
                    };
                if rot {
                    if !seen.insert((x, y, angle)) {
                        break true;
                    }
                    (dx, dy) = (-dy, dx);
                    angle = (angle + 1) % 4;
                } else {
                    (x, y) = (x + dx, y + dy);
                }
            }
        })
        .count();

    (seen.len(), count)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        const EXAMPLE: &str = indoc::indoc! {"
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
        "};
        assert_eq!(solve(EXAMPLE), (41, 6));
    }
}
