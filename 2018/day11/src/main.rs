#[macro_use] extern crate itertools;
use std::env;

fn power(x: i64, y: i64, serial: i64) -> i64 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial;
    power_level *= rack_id;
    power_level = (power_level / 100) % 10;
    power_level -= 5;
    return power_level;
}

fn main() {
    const SIZE: usize = 300;
    let mut grid = [[0; SIZE]; SIZE];
    let serial_number = std::env::args().nth(1)
        .expect("Must provide serial number as argument").parse()
        .expect("Serial number must be parseable as an integer");

    for x in 0..SIZE {
        for y in 0..SIZE {
            grid[x][y] = power(x as i64, y as i64, serial_number);
        }
    }
    for x in 0..SIZE {
        for y in 1..SIZE {
            grid[x][y] += grid[x][y - 1];
        }
    }
    for x in 1..SIZE {
        for y in 0..SIZE {
            grid[x][y] += grid[x - 1][y];
        }
    }

    let score = |x: usize, y: usize, r: usize| {
        grid[x + r][y + r] + grid[x][y] -
        grid[x][y + r] - grid[x + r][y]
    };

    let best_score = |r: usize| {
        iproduct!(0..(SIZE - r), 0..(SIZE - r))
            .map(|(x, y)| (score(x, y, r), (x + 1, y + 1, r)))
            .max_by_key(|p| p.0)
            .unwrap()
    };

    println!("{:?}", best_score(3));
    println!("{:?}", (1..SIZE).map(|r| best_score(r)).max_by_key(|p| p.0).unwrap());
}
