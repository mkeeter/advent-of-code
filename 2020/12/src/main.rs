use std::io::BufRead;

fn run<F>(input: &Vec<(char, i64)>, mut dir: (i64, i64), f: F) -> i64
    where F: Fn(char, (i64, i64), (i64, i64)) -> ((i64, i64), (i64, i64))
{
    let mut pos: (i64, i64) = (0, 0);
    for (cmd, count) in input.iter() {
        let n = match cmd {
            'N'|'S'|'E'|'W'|'F' => *count,
            'R'|'L' => count / 90,
            _ => panic!("Invalid command {}", cmd),
        };
        for _i in 0..n {
            let next = f(*cmd, pos, dir);
            pos = next.0;
            dir = next.1;
        }
    }
    pos.0.abs() + pos.1.abs()
}

fn main() {
    let input: Vec<(char, i64)> = std::io::stdin().lock().lines()
        .map(|line| {
            let line = line.unwrap();
            let (cmd, count) = line.split_at(1);
            (cmd.chars().next().unwrap(), count.parse().unwrap())
        })
        .collect();

    let p1 = |cmd, pos: (i64, i64), dir: (i64, i64)| match cmd {
        'N' => ((pos.0, pos.1 + 1), dir),
        'S' => ((pos.0, pos.1 - 1), dir),
        'E' => ((pos.0 + 1, pos.1), dir),
        'W' => ((pos.0 - 1, pos.1), dir),
        'F' => ((pos.0 + dir.0, pos.1 + dir.1), dir),
        'L' => (pos, (-dir.1,  dir.0)),
        'R' => (pos, ( dir.1, -dir.0)),
        _ => panic!("Invalid command {}", cmd),
    };
    println!("Part 1: {}", run(&input, (1, 0), p1));

    let p2 = |cmd, pos: (i64, i64), dir: (i64, i64)| match cmd {
        'N' => (pos, (dir.0, dir.1 + 1)),
        'S' => (pos, (dir.0, dir.1 - 1)),
        'E' => (pos, (dir.0 + 1, dir.1)),
        'W' => (pos, (dir.0 - 1, dir.1)),
        'F' => ((pos.0 + dir.0, pos.1 + dir.1), dir),
        'L' => (pos, (-dir.1,  dir.0)),
        'R' => (pos, ( dir.1, -dir.0)),
        _ => panic!("Invalid command {}", cmd),
    };
    println!("Part 2: {}", run(&input, (10, 1), p2));
}
