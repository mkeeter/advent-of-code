extern crate nalgebra as na;
use na::{Matrix2, Vector2};
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
                '>' => Vector2::new(1, 0),
                '<' => Vector2::new(-1, 0),
                '^' => Vector2::new(0, -1),
                'v' => Vector2::new(0, 1),
                _ => panic!("invalid cart character"),
            },
            turn: 0,
            id: 0,
        }
    }
}

fn main() {
    let mut tracks = HashMap::new();
    let mut carts = Vec::new();

    include_str!("../input")
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let x = x as i32;
                    let y = y as i32;
                    match c {
                        '\\' | '/' | '+' => drop(tracks.insert((x, y), c)),
                        '>' | '<' | '^' | 'v' => carts.push(Cart::new(x, y, c)),
                        _ => (),
                    };
                })
                .for_each(drop)
        })
        .for_each(drop);

    for (i, c) in carts.iter_mut().enumerate() {
        c.id = i;
    }

    while carts.len() > 1 {
        carts.sort_by_key(|cart| (cart.pt.y, cart.pt.x));

        // Store initial cart positions
        let mut positions = HashMap::new();
        for c in carts.iter() {
            positions.insert(c.pt, c.id);
        }

        let mut crashed = HashSet::new();
        for c in carts.iter_mut() {
            // Move the cart by one, storing whether it crashed
            positions.remove(&c.pt);
            c.pt += c.dir;
            if let Some(j) = positions.get(&c.pt) {
                println!("Carts {} and {} crashed at {},{}", c.id, *j, c.pt.x, c.pt.y);
                crashed.insert(c.id);
                crashed.insert(*j);
            }
            positions.insert(c.pt, c.id);

            // Rotate carts!
            if let Some(d) = tracks.get(&(c.pt.x, c.pt.y)) {
                c.dir = match *d {
                    '/' => Matrix2::new(0, -1, -1, 0),
                    '\\' => Matrix2::new(0, 1, 1, 0),
                    '+' => {
                        c.turn = (c.turn + 1) % 3;
                        match c.turn % 3 {
                            1 => Matrix2::new(0, 1, -1, 0),
                            2 => Matrix2::new(1, 0, 0, 1),
                            0 => Matrix2::new(0, -1, 1, 0),
                            _ => panic!("oh no"),
                        }
                    }
                    _ => panic!("oh no"),
                } * c.dir;
            }
        }

        carts = carts
            .into_iter()
            .filter(|c| !crashed.contains(&c.id))
            .collect();
    }
    println!("last cart is at {}", carts[0].pt);
}
