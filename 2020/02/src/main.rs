use std::io::BufRead;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{a}-{b} {chr}: {pwd}")]
struct Line {
    a: usize,
    b: usize,
    chr: char,
    pwd: String,
}

fn main() {
    let sum = std::io::stdin().lock().lines().fold((0, 0),
        |sum, line| {
            let line = line.unwrap().parse::<Line>().unwrap();

            let p1 = {
                let n = line.pwd.chars().filter(|&c| c == line.chr).count();
                n >= line.a && n <= line.b
            };

            let p2 = (line.pwd.chars().nth(line.a - 1).unwrap() == line.chr) ^
                     (line.pwd.chars().nth(line.b - 1).unwrap() == line.chr);

            (sum.0 + p1 as usize, sum.1 + p2 as usize)
        }
    );

    println!("Part 1: {}", sum.0);
    println!("Part 2: {}", sum.1);
}
