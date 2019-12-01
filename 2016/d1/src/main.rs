use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut x : i32 = 0;
    let mut y : i32 = 0;
    let mut dir : i32 = 0;
    let vecs = [[0, 1], [-1, 0], [0, -1], [1, 0]];

    let mut visited = HashSet::new();
    visited.insert((0,0));

    for cmd in buffer.split(',')
    {
        let trimmed = cmd.trim();
        dir = match trimmed.as_bytes()[0] as char {
            'R' => if dir == 0 { 3 } else { dir - 1 },
            'L' => if dir == 3 { 0 } else { dir + 1 },
            _ => panic!("OMG"),
        };

        let steps = trimmed[1..].parse::<usize>().unwrap();
        for _ in 0..steps
        {
            x += vecs[dir as usize][0];
            y += vecs[dir as usize][1];

            let pt = (x, y);

            println!("Visited {} {} again", x, y);
            if visited.contains(&pt)
            {
                println!("Visited {} {} again, {} blocks!", x, y,
                         x.abs() + y.abs());
                return;
            }
            visited.insert(pt);
        }
    }
    println!("x: {}\ty: {}\tblocks: {}", x, y, x.abs() + y.abs());
}
