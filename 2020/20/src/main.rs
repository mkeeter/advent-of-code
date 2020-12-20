use std::io::Read;
use std::collections::HashMap;

use regex::Regex;
use itertools::Itertools;

// All possible rotation + flip matrices, as a 4x4 + shift
const MATRICES: [[i32; 6]; 8] = [
    [ 1,  0,  0,  1,    0, 0],
    [ 1,  0,  0, -1,    0, 1],
    [-1,  0,  0,  1,    1, 0],
    [-1,  0,  0, -1,    1, 1],
    [ 0,  1,  1,  0,    0, 0],
    [ 0,  1, -1,  0,    0, 1],
    [ 0, -1,  1,  0,    1, 0],
    [ 0, -1, -1,  0,    1, 1],
];

// Ordering of edges: top, right, bottom, left
const EDGES: [(i32, i32); 4] = [
    ( 0, -1),
    ( 1,  0),
    ( 0,  1),
    (-1,  0),
];

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct Tile {
    id: u64,
    image: Vec<Vec<bool>>,
    edges: [[u16; 4]; 8],
}

impl Tile {
    fn new<'a, I>(id: u64, lines: I) -> Tile
        where I: Iterator<Item=&'a str>
    {
        let image = lines
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect::<Vec<Vec<bool>>>();

        let mut edges = [[0; 4]; 8];
        for i in 0..8 {
            let img = Self::project(&image, i);
            edges[i as usize] = Self::edges(&img);
        }

        Tile { id, image, edges }
    }

    fn _print(img: &[Vec<bool>]) {
        for row in img {
            for x in row {
                print!("{}", if *x { '⬛' } else { '⬜' });
            }
            println!();
        }
    }

    fn unpack<I: Iterator<Item=bool>>(iter: I) -> u16 {
        iter.fold(0, |a, b| (a << 1) | (b as u16))
    }

    fn edges(img: &[Vec<bool>]) -> [u16; 4] {
        // In the same order as EDGES above: top, right, bottom, left
        [Self::unpack(img[0].iter().copied()),
         Self::unpack(img.iter().map(|row| row[row.len() - 1])),
         Self::unpack(img[img.len() - 1].iter().copied()),
         Self::unpack(img.iter().map(|row| row[0]))]
    }

    fn project(img: &[Vec<bool>], orientation: u8) -> Vec<Vec<bool>> {
        let mat = MATRICES[orientation as usize];
        let mut out = img.to_owned();
        for y in 0..img.len() {
            let row = &img[y];
            for x in 0..row.len() {
                let x = x as i32;
                let y = y as i32;

                let x_ = x*mat[0] + y*mat[1] + (row.len() as i32 - 1)*mat[4];
                let y_ = x*mat[2] + y*mat[3] + (img.len() as i32 - 1)*mat[5];

                out[y_ as usize][x_ as usize] = row[x as usize];
            }
        }
        out
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
struct State<'a> {
    tiles: &'a [Tile],

    // Tiles which are placed have a bound orientation here
    bind: Vec<Option<u8>>,

    // The grid contains placed tiles as indexes into bind or tiles
    grid: HashMap<(i32, i32), usize>,

    // Open tiles, along with a list of edge constraints to satisfy
    constraints: HashMap<(i32, i32), [Option<u16>; 4]>,

    // Number of remaining tiles to place
    unplaced: usize,
}

impl<'a> State<'a> {
    fn new(tiles: &'a [Tile]) -> Self {
        let mut out = State {
            tiles,
            bind: vec![None; tiles.len()],
            grid: HashMap::new(),
            constraints: HashMap::new(),
            unplaced: tiles.len(),
        };
        // Lock tile 0 in place
        out.place((0, 0), 0, 0);
        out
    }

    fn place(&mut self, (x, y): (i32, i32), tile: usize, orientation: u8) {
        assert!(self.bind[tile].is_none());
        self.bind[tile] = Some(orientation);

        assert!(!self.grid.contains_key(&(x, y)));
        self.grid.insert((x, y), tile);

        assert!(self.unplaced > 0);
        self.unplaced -= 1;

        for (e, (dx, dy)) in EDGES.iter().enumerate() {
            let x_ = x + dx;
            let y_ = y + dy;
            let m = self.tiles[tile].edges[orientation as usize][e as usize];

            // Confirm that the placement is valid
            if let Some(t) = self.grid.get(&(x_, y_)) {
                let o = self.bind[*t].unwrap() as usize;
                assert!(m == self.tiles[*t].edges[o][(e + 2) % 4])
            } else {
                let s = self.constraints.entry((x_, y_))
                    .or_default()
                    [(e + 2) % 4].replace(m);
                assert!(s == None);
            }
        }

        self.constraints.remove(&(x, y));
    }

    fn run(self) -> Option<Self> {
        if self.unplaced == 0 {
            return Some(self);
        }
        for (&(x, y), cs) in self.constraints.iter() {
            // Check against every possible tile, which is inefficient
            // but fast enough in practice!
            for (t, tile) in self.tiles.iter().enumerate() {
                if self.bind[t].is_some() {
                    continue;
                }
                for o in 0..8 {
                    if tile.edges[o].iter().zip(cs)
                        .all(|(m, n)| n.unwrap_or(*m) == *m)
                    {
                        // We've found a tile to place!
                        let mut next = self.clone();
                        next.place((x, y), t, o as u8);
                        if let Some(done) = next.run() {
                            return Some(done);
                        }
                    }
                }
            }
        }
        None
    }
}

////////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut tiles = Vec::new();
    let re = Regex::new(r#"^Tile (\d+):$"#).unwrap();

    for t in input.split("\n\n") {
        let mut iter = t.lines();

        // Read ID from first line
        let id_line = iter.next();
        if id_line == None {
            continue;
        }
        let id: u64 = re.captures(id_line.unwrap()).unwrap()
            .get(1).unwrap()
            .as_str().parse().unwrap();

        tiles.push(Tile::new(id, iter));
    }

    // Solve the system
    let sol = State::new(&tiles).run().unwrap();

    let xm = sol.grid.iter().map(|p| p.0.0).minmax().into_option().unwrap();
    let ym = sol.grid.iter().map(|p| p.0.1).minmax().into_option().unwrap();
    assert!(xm.1 - xm.0 == ym.1 - ym.0); // check for square image

    let mut out = 1;
    for x in [xm.0, xm.1].iter() {
        for y in [ym.0, ym.1].iter() {
            out *= tiles[*sol.grid.get(&(*x, *y)).unwrap()].id;
        }
    }
    println!("Part 1: {}", out);

    ////////////////////////////////////////////////////////////////////////////
    let size = tiles[0].image.len() - 2; // assume square tiles

    let image_size_px = size * (xm.1 - xm.0 + 1) as usize;
    let mut img = vec![vec![false; image_size_px]; image_size_px];

    for gx in xm.0..=xm.1 {
        for gy in ym.0..=ym.1 {
            let t = *sol.grid.get(&(gx, gy)).unwrap();
            let tile = Tile::project(&tiles[t].image, sol.bind[t].unwrap());
            let x = (gx - xm.0) as usize * size;
            let y = (gy - ym.0) as usize * size;
            for sx in 0..size {
                for sy in 0..size {
                    img[y + sy][x + sx] = tile[sy + 1][sx + 1];
                }
            }
        }
    }
    const MONSTER: &str = "
                  #
#    ##    ##    ###
 #  #  #  #  #  #   ";
    let monster_img: Vec<(usize, usize)> = MONSTER.lines()
        .skip(1)
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate()
            .filter(|(_, c)| *c == '#')
            .map(move |(x, _)| (x, y)))
        .collect();

    for o in 0..8 {
        let mut monster_count = 0;
        let flipped = Tile::project(&img, o);
        for x in 0..image_size_px {
            for y in 0..image_size_px {
                let found = monster_img.iter().all(|(dx, dy)| {
                    let x = x + dx;
                    let y = y + dy;
                    x < image_size_px && y < image_size_px && flipped[y][x]
                });
                if found {
                    monster_count += monster_img.len();
                }
            }
        }
        if monster_count > 0 {
            let num_hash = img.iter()
                .flat_map(|r| r.iter())
                .filter(|c| **c)
                .count();
            println!("Part 2: {}", num_hash - monster_count);
        }
    }
}
