struct Grid<'a> {
    bytes: &'a [u8],
    width: usize,
    height: usize,
    empty: u8,
}

impl std::ops::Index<(i64, i64)> for Grid<'_> {
    type Output = u8;
    fn index(&self, index: (i64, i64)) -> &Self::Output {
        let (x, y) = index;
        if x < 0
            || y < 0
            || x as usize >= self.width
            || y as usize >= self.height
        {
            &self.empty
        } else {
            &self.bytes[(x as usize) + (y as usize) * (self.width + 1)]
        }
    }
}

impl<'a> Grid<'a> {
    fn new(s: &'a str) -> Self {
        assert!(s.is_ascii());
        let mut width = None;
        let mut height = 0;
        for row in s.lines() {
            let w = row.len();
            let prev = *width.get_or_insert(w);
            assert_eq!(prev, w);
            height += 1;
        }
        Self {
            bytes: s.as_bytes(),
            width: width.unwrap_or(0),
            height,
            empty: b'.',
        }
    }
}

pub fn solve(s: &str) -> (usize, usize) {
    let g = Grid::new(s);
    let mut found = 0;
    for y in 0..g.height as i64 {
        for x in 0..g.width as i64 {
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
    for y in 0..g.height as i64 {
        for x in 0..g.width as i64 {
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
