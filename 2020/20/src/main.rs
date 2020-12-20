use std::io::Read;
use std::collections::HashMap;

use regex::Regex;

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
            let img = Tile::project(&image, i);
            edges[i as usize] = Tile::edges(&img);
        }

        Tile { id, image, edges }
    }

    fn print(img: &[Vec<bool>]) {
        for row in img {
            for x in row {
                print!("{}", if *x { '⬛' } else { '⬜' });
            }
            println!();
        }
    }

    fn to_u16<I: Iterator<Item=bool>>(iter: I) -> u16 {
        iter.fold(0, |a, b| (a << 1) | (b as u16))
    }

    fn edges(img: &[Vec<bool>]) -> [u16; 4] {
        // In the same order as EDGES above: top, right, bottom, left
        [Tile::to_u16(img[0].iter().copied()),
         Tile::to_u16(img.iter().map(|row| row[row.len() - 1])),
         Tile::to_u16(img[img.len() - 1].iter().copied()),
         Tile::to_u16(img.iter().map(|row| row[0]))]
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
// A set of tiles with an acceleration structure for edge lookups
#[derive(Debug)]
struct Tiles {
    tiles: Vec<Tile>,

    // Edge lookup table: u16 -> (tile, orientation, edge)
    edges: HashMap<u16, Vec<(usize, u8, u8)>>,
}

impl Tiles {
    fn new(tiles: Vec<Tile>) -> Tiles {
        let mut edges: HashMap<u16, Vec<_>> = HashMap::new();
        for (i,t) in tiles.iter().enumerate() {
            for orientation in 0..8 {
                for edge in 0..4 {
                    edges.entry(t.edges[orientation][edge])
                        .or_default()
                        .push((i, orientation as u8, edge as u8));
                }
            }
        }
        Tiles { tiles, edges }
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
    let tiles = Tiles::new(tiles);
}
