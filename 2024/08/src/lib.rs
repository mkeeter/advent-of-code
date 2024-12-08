use std::collections::{HashMap, HashSet};
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
    let mut antinodes = HashSet::new();
    for pos in antennas.values() {
        for (a, b) in pos
            .iter()
            .enumerate()
            .flat_map(|(i, a)| pos[i + 1..].iter().map(move |b| (a, b)))
        {
            let dx = b.0 - a.0;
            let dy = b.1 - a.1;
            antinodes.insert((a.0 - dx, a.1 - dy));
            antinodes.insert((b.0 + dx, b.1 + dy));
        }
    }
    let antinode_count = antinodes
        .iter()
        .filter(|(x, y)| {
            *x >= 0 && *x < g.width() && *y >= 0 && *y < g.height()
        })
        .count();

    let mut many_antinodes = HashSet::new();
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
                    many_antinodes.insert((x, y));
                    x += dx * sign;
                    y += dy * sign;
                }
            }
        }
    }
    (antinode_count, many_antinodes.len())
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
