use std::io::BufRead;

fn main() {
    let input: Vec<(char, i64)> = std::io::stdin().lock().lines()
        .map(|line| {
            let line = line.unwrap();
            let (cmd, count) = line.split_at(1);
            (cmd.chars().next().unwrap(), count.parse().unwrap())
        })
        .collect();

    let mut pos: (i64, i64) = (0, 0);
    let mut dir = (1, 0);
    for (cmd, count) in input.iter() {
        let n = match cmd {
            'N'|'S'|'E'|'W'|'F' => *count,
            'R'|'L' => count / 90,
            _ => panic!("Invalid command {}", cmd),
        };
        for _i in 0..n {
            match cmd {
                'N' => pos.1 += 1,
                'S' => pos.1 -= 1,
                'E' => pos.0 += 1,
                'W' => pos.0 -= 1,
                'F' => { pos.0 += dir.0; pos.1 += dir.1 },
                'L' => dir = (-dir.1,  dir.0),
                'R' => dir = ( dir.1, -dir.0),
                _ => panic!("Invalid command {}", cmd),
            }
        }
    }
    println!("Part 1: {}", pos.0.abs() + pos.1.abs());

    let mut pos: (i64, i64) = (0, 0);
    let mut way = (10, 1);
    for (cmd, count) in input.iter() {
        let n = match cmd {
            'N'|'S'|'E'|'W'|'F' => *count,
            'R'|'L' => count / 90,
            _ => panic!("Invalid command {}", cmd),
        };
        for _i in 0..n {
            match cmd {
                'N' => way.1 += 1,
                'S' => way.1 -= 1,
                'E' => way.0 += 1,
                'W' => way.0 -= 1,
                'F' => { pos.0 += way.0; pos.1 += way.1 },
                'L' => way = (-way.1,  way.0),
                'R' => way = ( way.1, -way.0),
                _ => panic!("Invalid command {}", cmd),
            }
        }
    }
    println!("Part 2: {}", pos.0.abs() + pos.1.abs());
}
