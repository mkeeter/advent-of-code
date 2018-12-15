extern crate nalgebra as na;
use na::{Vector2, Matrix2};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Cart {
    pt: Vector2<i32>,
    dir: Vector2<i32>,
    turn: u32,
    id: usize,
}

impl Cart {
    fn new(x: i32, y: i32, c: char) -> Cart {
        Cart {
            pt: Vector2::new(x as i32, y as i32),
            dir: match c {
                '>' => Vector2::new( 1,  0),
                '<' => Vector2::new(-1,  0),
                '^' => Vector2::new( 0, -1),
                'v' => Vector2::new( 0,  1),
                 _  => panic!("invalid cart character"),
            },
            turn: 0,
            id: 0,
        }
    }
}

fn main() {
    let mut tracks = HashMap::new();
    let mut carts = Vec::new();

    include_str!("../dummy")
        .lines()
        .enumerate()
        .map(|(y, line)| line
             .chars()
             .enumerate()
             .map(|(x, c)| {
                 let x = x as i32;
                 let y = y as i32;
                 match c {
                     '\\' | '/' | '+' => { tracks.insert((x, y), c); () },
                     '>' | '<' | '^' | 'v' => carts.push(Cart::new(x, y, c)),
                     _ => (),
                 };
             }).for_each(drop))
        .for_each(drop);

    for (i, c) in carts.iter_mut().enumerate() {
        c.id = i;
    }

    println!("carts: {:?}", carts);
    println!("tracks: {:?}", tracks);
    println!("Hello, world!");

    let mut crashed = false;
    while !crashed {
        carts.sort_by_key(|cart| (cart.pt.y, cart.pt.x));
        for c in carts.iter() {
            print!("{},{}  ", c.pt.x, c.pt.y);
        }
        print!("\n");

        let mut positions = HashSet::new();
        for c in carts.iter() {
            positions.insert(c.pt);
        }

        for c in carts.iter_mut() {
            positions.remove(&c.pt);
            c.pt += c.dir;
            if positions.contains(&c.pt) {
                println!("Crash at {:?}", c.pt);
                crashed = true;
                break;
            }
            positions.insert(c.pt);

            if let Some(d) = tracks.get(&(c.pt.x, c.pt.y)) {
                c.dir = match *d {
                    '/' => Matrix2::new(0, -1, -1, 0),
                    '\\' => Matrix2::new(0, 1, 1, 0),
                    '+' => {
                        let out = {
                            match c.turn % 3 {
                                0 => Matrix2::new(0,  1, -1, 0),
                                1 => Matrix2::new(1,  0,  0, 1),
                                2 => Matrix2::new(0, -1,  1, 0),
                                _ => panic!("oh no"),
                            }
                        };
                        c.turn = (c.turn + 1) % 3;
                        out
                    },
                    _ => panic!("oh no"),
                } * c.dir;
            }
        }
    }
    // 66,77 is wrong
}
