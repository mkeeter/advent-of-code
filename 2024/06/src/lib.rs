use std::collections::HashSet;
use util::Grid;

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
    let mut seen = HashSet::new();
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

    // haha Rust go brrrrrrr
    let mut count = 0;
    for by in 0..g.height() {
        for bx in 0..g.width() {
            let (mut x, mut y) = start;
            let (mut dx, mut dy) = (0i64, -1i64);
            let mut seen = HashSet::new();
            let obstructed = loop {
                if !seen.insert((x, y, dx, dy)) {
                    break true;
                }
                match g.get(x + dx, y + dy) {
                    _ if x + dx == bx as i64 && y + dy == by as i64 => {
                        (dx, dy) = (-dy, dx);
                    }
                    Some(&b'#') => {
                        (dx, dy) = (-dy, dx);
                    }
                    Some(_) => {
                        x += dx;
                        y += dy;
                    }
                    None => break false,
                }
            };
            count += obstructed as usize;
        }
    }

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
