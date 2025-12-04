use crate::{Grid, GridSet};

pub fn solve(s: &str) -> (usize, usize) {
    let g = Grid::new(s);

    let mut removed = GridSet::new(&g);
    let mut changed = true;
    let mut count = None;
    while std::mem::take(&mut changed) {
        for x in 0..g.width() {
            for y in 0..g.height() {
                if g[(x, y)] == b'@' && !removed.contains(x, y) {
                    let mut n = 0;
                    for dx in [-1, 0, 1] {
                        for dy in [-1, 0, 1] {
                            // Special case for the first iteration: we ignore
                            // the `removed` data to get an accurate p1 count
                            if g[(x + dx, y + dy)] == b'@'
                                && (count.is_none()
                                    || !removed.contains(x + dx, y + dy))
                            {
                                n += 1 // this counts itself; subtract 1 later
                            }
                        }
                    }
                    if n <= 4 {
                        removed.insert(x, y);
                        changed = true;
                    }
                }
            }
        }
        if count.is_none() {
            count = Some(removed.len());
        }
    }
    (count.unwrap_or(0), removed.len())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let s = indoc::indoc! {"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        "};
        let (a, b) = solve(s);
        assert_eq!(a, 13);
        assert_eq!(b, 43);
    }
}
