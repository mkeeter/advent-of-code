use util::Grid;

pub fn solve(s: &str) -> (usize, usize) {
    let g = Grid::new(s);
    let mut found = 0;
    for y in 0..g.height() {
        for x in 0..g.width() {
            for dx in [-1i64, 0, 1] {
                for dy in [-1i64, 0, 1] {
                    found += b"XMAS".iter().enumerate().all(|(i, c)| {
                        let p = (x + i as i64 * dx, y + i as i64 * dy);
                        g[p] == *c
                    }) as usize;
                }
            }
        }
    }

    let mut diag = 0;
    for y in 0..g.height() {
        for x in 0..g.width() {
            if g[(x, y)] != b'A' {
                continue;
            }
            let find = |dx: i64, dy: i64| {
                let a = g[(x + dx, y + dy)];
                let b = g[(x - dx, y - dy)];

                (a == b'M' && b == b'S') || (a == b'S' && b == b'M')
            };
            diag += (find(-1, -1) && find(-1, 1)) as usize;
        }
    }
    (found, diag)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let a = indoc::indoc! {"
            ..X...
            .SAMX.
            .A..A.
            XMAS.S
            .X....
        "};
        assert_eq!(solve(a).0, 4);

        let b = indoc::indoc! {"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        "};
        assert_eq!(solve(b), (18, 9));
    }
}
