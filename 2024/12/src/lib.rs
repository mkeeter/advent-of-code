use util::{Grid, GridSet};

fn check_region(x: i64, y: i64, g: &Grid, seen: &mut GridSet) -> u64 {
    if seen.contains(x, y) {
        return 0;
    }

    let c = g[(x, y)];
    let mut todo = vec![(x, y)];
    let mut edge_count = 0;
    let mut cell_count = 0;
    while let Some((x, y)) = todo.pop() {
        assert_eq!(g[(x, y)], c);
        if !seen.insert(x, y) {
            continue;
        }
        cell_count += 1;
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (nx, ny) = (x + dx, y + dy);
            if g[(nx, ny)] == c {
                if !seen.contains(nx, ny) {
                    todo.push((nx, ny));
                }
            } else {
                edge_count += 1;
            }
        }
    }
    edge_count * cell_count
}

pub fn solve(s: &str) -> (u64, u64) {
    let g = Grid::new(s);
    let mut seen = GridSet::new(&g);

    let mut score = 0;
    for y in 0..g.height() {
        for x in 0..g.width() {
            score += check_region(x, y, &g, &mut seen);
        }
    }
    (score, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const EXAMPLE: &str = indoc::indoc! {"
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE
        "};
        assert_eq!(solve(EXAMPLE), (1930, 0));
    }
}
