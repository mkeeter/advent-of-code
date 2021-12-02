use parse_display::{Display, FromStr};
use std::io::BufRead;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display(style = "snake_case")]
enum Direction {
    Forward,
    Up,
    Down,
}
impl Direction {
    fn dx(&self) -> i64 {
        match self {
            Self::Forward => 1,
            _ => 0,
        }
    }
    fn dy(&self) -> i64 {
        match self {
            Self::Up => -1,
            Self::Down => 1,
            _ => 0,
        }
    }
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{dir} {count}")]
struct Command {
    dir: Direction,
    count: i64,
}

impl Command {
    fn dx(&self) -> i64 {
        self.count * self.dir.dx()
    }
    fn dy(&self) -> i64 {
        self.count * self.dir.dy()
    }
}

fn main() {
    let cmd = std::io::stdin()
        .lock()
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect::<Vec<Command>>();
    let end = cmd
        .iter()
        .fold((0, 0), |(x, y), cmd| (x + cmd.dx(), y + cmd.dy()));
    println!("Part 1: {}", end.0 * end.1);

    let end = cmd.iter().fold((0, 0, 0), |(x, y, aim), cmd| {
        (x + cmd.dx(), y + cmd.dx() * aim, aim + cmd.dy())
    });
    println!("Part 2: {} {} {}", end.0, end.1, end.0 * end.1);
}
