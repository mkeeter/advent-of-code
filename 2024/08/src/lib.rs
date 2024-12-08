use std::collections::HashMap;
use util::{BitSet, Grid};

pub fn solve(s: &str) -> (usize, usize) {
    let g = Grid::new(s);
    let mut antennas: HashMap<u8, Vec<(i64, i64)>> = HashMap::new();
    for y in 0..g.height() {
        for x in 0..g.width() {
            let c = g[(x, y)];
            if c != b'.' {
                antennas.entry(c).or_default().push((x, y));
            }
        }
    }
    let mut antinodes = BitSet::new((g.width() * g.height()) as usize);
    for pos in antennas.values() {
        for (a, b) in pos
            .iter()
            .enumerate()
            .flat_map(|(i, a)| pos[i + 1..].iter().map(move |b| (a, b)))
        {
            let dx = b.0 - a.0;
            let dy = b.1 - a.1;
            for sign in [-1, 2] {
                let x = a.0 + sign * dx;
                let y = a.1 + sign * dy;
                if x >= 0 && x < g.width() && y >= 0 && y < g.height() {
                    antinodes.insert((x + y * g.width()) as usize);
                }
            }
        }
    }
    let antinode_count = antinodes.len();

    for pos in antennas.values() {
        for (a, b) in pos
            .iter()
            .enumerate()
            .flat_map(|(i, a)| pos[i + 1..].iter().map(move |b| (a, b)))
        {
            let dx = b.0 - a.0;
            let dy = b.1 - a.1;
            for sign in [1, -1] {
                let (mut x, mut y) = a;
                while x >= 0 && x < g.width() && y >= 0 && y < g.height() {
                    antinodes.insert((x + y * g.width()) as usize);
                    x += dx * sign;
                    y += dy * sign;
                }
            }
        }
    }
    let many_antinode_count = antinodes.len();

    (antinode_count, many_antinode_count)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        const EXAMPLE: &str = indoc::indoc! {"
            ............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............
        "};
        assert_eq!(solve(EXAMPLE), (14, 34));
    }
}
