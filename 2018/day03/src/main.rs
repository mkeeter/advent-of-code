use std::str::FromStr;

#[macro_use] extern crate nom;
use nom::types::CompleteByteSlice;

#[macro_use] extern crate itertools;

#[derive(Clone, Debug)]
struct Patch {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize
}

named!(coord<CompleteByteSlice, usize>,
       map_res!(recognize!(nom::digit), |s:CompleteByteSlice|
                usize::from_str(std::str::from_utf8(s.0).unwrap())));
named!(parse_line<CompleteByteSlice, Patch>,
    do_parse!(tag!("#") >>
              id: coord >>
              tag!(" @ ") >>
              x: coord >>
              tag!(",") >>
              y: coord >>
              tag!(": ") >>
              w: coord >>
              tag!("x") >>
              h: coord >>
              (Patch { id, x, y, w, h})));

named!(parse_lines<CompleteByteSlice, Vec<Patch>>,
       many0!(do_parse!( p: parse_line >> tag!("\n") >> (p))));

fn main() {
    let input = include_bytes!("../input");
    let patches: Vec<Patch> = parse_lines(CompleteByteSlice(input)).unwrap().1;

    const SIZE: usize = 1000;
    let mut grid = [[0; SIZE]; SIZE];
    for patch in patches.iter() {
        for (i, j) in iproduct!(0..patch.w, 0..patch.h) {
            grid[patch.x + i][patch.y + j] += 1;
        }
    }

    let doubled = iproduct!(0..SIZE, 0..SIZE)
        .filter(|(x, y)| grid[*x][*y] >= 2).count();
    println!("Number of double-claimed tiles: {}", doubled);

    for patch in patches.iter() {
        if iproduct!(0..patch.w, 0..patch.h)
            .all(|(i, j)| grid[patch.x + i][patch.y + j] == 1)
        {
            println!("Found target with id #{}", patch.id);
        }
    }
}
