use rayon::prelude::*;
use util::{BitSet, Grid};

pub fn solve(s: &str) -> (usize, usize) {
    let g = Grid::new(s);
    let mut pos = None;
    for y in 0..g.height() {
        for x in 0..g.width() {
            if g[(x as i64, y as i64)] == b'^' {
                pos = Some((x as i64, y as i64));
                break;
            }
        }
    }
    let Some(start) = pos else {
        panic!("could not find guard's starting position");
    };
    let (mut x, mut y) = start;
    let (mut dx, mut dy) = (0i64, -1i64);
    let mut seen = BitSet::new(g.width() * g.height());
    loop {
        seen.set(x as usize + y as usize * g.height());
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

    // haha Rust go brrrrrrr
    let count = (0..g.height())
        .into_par_iter()
        .flat_map(|by| (0..g.width()).into_par_iter().map(move |bx| (bx, by)))
        .filter(|&(bx, by)| {
            if g[(bx as i64, by as i64)] == b'#' {
                return false;
            }
            let (mut x, mut y) = start;
            let (mut dx, mut dy) = (0i64, -1i64);
            let mut angle = 0;
            let mut seen = BitSet::new(g.width() * g.height() * 4);
            loop {
                let rot = (x + dx == bx as i64 && y + dy == by as i64)
                    || match g.get(x + dx, y + dy) {
                        Some(&b'#') => true,
                        Some(_) => false,
                        None => return false,
                    };
                if rot {
                    let i = angle + (x as usize + y as usize * g.width()) * 4;
                    if seen.get(i) {
                        break true;
                    }
                    seen.set(i);
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
