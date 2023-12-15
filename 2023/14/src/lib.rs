use std::collections::{btree_map::Entry, BTreeMap};

trait Grid {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get_raw(&self, x: usize, y: usize) -> &u8;
    fn get_raw_mut(&mut self, x: usize, y: usize) -> &mut u8;

    fn remove(&mut self, x: usize, y: usize) -> Option<u8> {
        self.get_mut(x, y).map(|c| std::mem::replace(c, b'.'))
    }
    fn insert(&mut self, x: usize, y: usize, c: u8) -> Option<u8> {
        assert_ne!(c, b'.');
        let c = std::mem::replace(self.get_raw_mut(x, y), c);
        Some(c).filter(|c| *c != b'.')
    }
    fn get(&self, x: usize, y: usize) -> Option<&u8> {
        Some(self.get_raw(x, y)).filter(|c| **c != b'.')
    }
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut u8> {
        Some(self.get_raw_mut(x, y)).filter(|c| **c != b'.')
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Base {
    data: Vec<u8>,
    width: usize,
    count: usize,
}

impl Base {
    fn key(&self) -> Vec<u16> {
        let mut out = Vec::with_capacity(self.count);
        for (i, c) in self.data.iter().enumerate() {
            if *c == b'O' {
                let x: u8 = (i % self.width).try_into().unwrap();
                let y: u8 = (i / self.width).try_into().unwrap();
                out.push(u16::from_le_bytes([x, y]));
            }
        }
        out
    }
}

impl std::fmt::Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.data.chunks_exact(self.width) {
            for c in row {
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid for Base {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        let h = self.data.len();
        assert_eq!(h % self.width, 0);
        h / self.width
    }
    fn get_raw(&self, x: usize, y: usize) -> &u8 {
        &self.data[x + y * self.width]
    }
    fn get_raw_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        &mut self.data[x + y * self.width]
    }
}

struct FlipNorthSouth<'a, G>(&'a mut G);
impl<G: Grid> Grid for FlipNorthSouth<'_, G> {
    fn width(&self) -> usize {
        self.0.width()
    }
    fn height(&self) -> usize {
        self.0.height()
    }
    fn get_raw(&self, x: usize, y: usize) -> &u8 {
        self.0.get_raw(x, self.width() - y - 1)
    }
    fn get_raw_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        self.0.get_raw_mut(x, self.width() - y - 1)
    }
}

struct FlipNorthWest<'a, G>(&'a mut G);
impl<G: Grid> Grid for FlipNorthWest<'_, G> {
    fn width(&self) -> usize {
        self.0.height()
    }
    fn height(&self) -> usize {
        self.0.width()
    }
    fn get_raw(&self, x: usize, y: usize) -> &u8 {
        self.0.get_raw(y, x)
    }
    fn get_raw_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        self.0.get_raw_mut(y, x)
    }
}

fn roll<G: Grid>(grid: &mut G) {
    let mut wavefront = vec![0; grid.width()];
    for y in 0..grid.height() {
        for (x, w) in wavefront.iter_mut().enumerate() {
            if let Some(c) = grid.get(x, y).cloned() {
                match c {
                    b'O' => {
                        grid.remove(x, y).unwrap();
                        grid.insert(x, *w, c);
                        *w += 1;
                    }
                    b'#' => {
                        *w = y + 1;
                    }
                    c => panic!("invalid character {c}"),
                }
            }
        }
    }
}

fn find_load(grid: &Base) -> usize {
    let mut load = 0;
    for y in 0..grid.height() {
        for x in 0..grid.height() {
            if grid.get(x, y) == Some(&b'O') {
                load += grid.height() - y;
            }
        }
    }
    load
}

fn part1(mut grid: Base) -> usize {
    roll(&mut grid);
    find_load(&grid)
}

fn part2(mut grid: Base) -> usize {
    let mut seen = BTreeMap::new();
    let mut c = 0;
    const N: usize = 1_000_000_000;
    while c < N {
        roll(&mut grid); // North
        roll(&mut FlipNorthWest(&mut grid)); // West
        roll(&mut FlipNorthSouth(&mut grid)); // South
        roll(&mut FlipNorthSouth(&mut FlipNorthWest(&mut grid))); // East
        c += 1;
        match seen.entry(grid.key()) {
            Entry::Vacant(e) => {
                e.insert(c);
            }
            Entry::Occupied(o) => {
                let gap = c - o.get();
                c += gap * ((N - c) / gap);
            }
        }
    }
    find_load(&grid)
}

pub fn solve(s: &str) -> (String, String) {
    let mut data = vec![];
    let mut width = 0;
    for line in s.lines() {
        if width > 0 {
            assert_eq!(width, line.len());
        } else {
            width = line.len();
        }
        data.extend(line.bytes());
    }

    let count = data.iter().filter(|c| **c == b'O').count();
    let grid = Base { data, width, count };

    let p1 = part1(grid.clone());
    let p2 = part2(grid);

    (p1.to_string(), p2.to_string())
}
