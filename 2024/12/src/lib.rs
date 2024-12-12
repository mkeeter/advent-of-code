use util::{Grid, GridSet};

fn check_region(x: i64, y: i64, g: &Grid, seen: &mut GridSet) -> (u64, u64) {
    if seen.contains(x, y) {
        return (0, 0);
    }

    let c = g[(x, y)];
    let mut todo = vec![(x, y)];
    let mut edge_count = 0;
    let mut cell_count = 0;
    let mut side_count = 0;
    while let Some((x, y)) = todo.pop() {
        assert_eq!(g[(x, y)], c);
        if !seen.insert(x, y) {
            continue;
        }
        cell_count += 1;
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let (nx, ny) = (x + dx, y + dy);
            if g[(nx, ny)] == c {
                if !seen.contains(nx, ny) {
                    todo.push((nx, ny));
                }
            } else {
                edge_count += 1;
            }
        }
        let mut corners = 0;
        for (dx, dy) in [(-1, -1), (-1, 1), (1, 1), (1, -1)] {
            let convex_corner = g[(x + dx, y)] != c && g[(x, y + dy)] != c;
            let concave_corner = g[(x + dx, y + dy)] != c
                && (g[(x + dx, y)] == c && g[(x, y + dy)] == c);
            corners += (convex_corner || concave_corner) as u64;
        }
        side_count += corners;
    }
    (edge_count * cell_count, side_count * cell_count)
}

pub fn solve(s: &str) -> (u64, u64) {
    let g = Grid::new(s);
    let mut seen = GridSet::new(&g);

    let mut score = 0;
    let mut discount = 0;
    for y in 0..g.height() {
        for x in 0..g.width() {
            let (ds, dd) = check_region(x, y, &g, &mut seen);
            score += ds;
            discount += dd;
        }
    }
    (score, discount)
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
        assert_eq!(solve(EXAMPLE), (1930, 1206));

        const SMALL: &str = indoc::indoc! {"
            AAAA
            BBCD
            BBCC
            EEEC
        "};
        assert_eq!(solve(SMALL), (140, 80));

        const MEDIUM: &str = indoc::indoc! {"
            EEEEE
            EXXXX
            EEEEE
            EXXXX
            EEEEE
        "};
        assert_eq!(solve(MEDIUM).1, 236);

        const LARGE: &str = indoc::indoc! {"
            AAAAAA
            AAABBA
            AAABBA
            ABBAAA
            ABBAAA
            AAAAAA
        "};
        assert_eq!(solve(LARGE).1, 368);
    }
}
