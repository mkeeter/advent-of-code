use util::{BitSet, Grid};

fn find_trails(g: &Grid, x: i64, y: i64, seen: &mut BitSet) -> (usize, usize) {
    let mut todo = vec![(b'0', x, y)];
    seen.clear();
    let mut count = 0;
    let mut score = 0;
    while let Some((a, x, y)) = todo.pop() {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (x, y) = (x + dx, y + dy);
            let b = g[(x, y)];
            if b == a + 1 {
                if b == b'9' {
                    if seen.insert((x + y * g.width()) as usize) {
                        count += 1;
                    }
                    score += 1;
                } else {
                    todo.push((b, x, y));
                }
            }
        }
    }
    (count, score)
}

pub fn solve(s: &str) -> (usize, usize) {
    let g = Grid::new(s);
    let mut trail_count = 0;
    let mut trail_score = 0;
    let mut scratch = BitSet::new((g.width() * g.height()) as usize);
    for y in 0..g.width() {
        for x in 0..g.width() {
            if g[(x, y)] == b'0' {
                let (count, score) = find_trails(&g, x, y, &mut scratch);
                trail_count += count;
                trail_score += score;
            }
        }
    }
    (trail_count, trail_score)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        const EXAMPLE: &str = indoc::indoc! {"
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732
        "};
        assert_eq!(solve(EXAMPLE), (36, 81));
    }
}
