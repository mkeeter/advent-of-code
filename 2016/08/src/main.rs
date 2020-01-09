use std::collections::HashSet;
use std::io::BufRead;
use std::str::FromStr;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

fn main() {
    let mut screen = HashSet::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let words = line.split(' ').collect::<Vec<_>>();
        match words[0] {
            "rect" => {
                let xy = words[1].split('x')
                    .filter_map(|w| usize::from_str(w).ok())
                    .collect::<Vec<usize>>();
                for x in 0..xy[0] {
                    for y in 0..xy[1] {
                        screen.insert((x, y));
                    }
                }
            },
            "rotate" => {
                let rc = words[2].split('=')
                    .filter_map(|w| usize::from_str(w).ok())
                    .next()
                    .unwrap();
                let amount = usize::from_str(words[4]).unwrap();
                match words[1] {
                    "column" => {
                        let mut next = screen.iter()
                            .copied()
                            .filter(|(x, _y)| *x != rc)
                            .collect::<HashSet<_>>();
                        for y in 0..HEIGHT {
                            if screen.contains(&(rc, y)) {
                                next.insert((rc, (y + amount) % HEIGHT));
                            }
                        }
                        screen = next;
                    },
                    "row" => {
                        let mut next = screen.iter()
                            .copied()
                            .filter(|(_x, y)| *y != rc)
                            .collect::<HashSet<_>>();
                        for x in 0..WIDTH {
                            if screen.contains(&(x, rc)) {
                                next.insert(((x + amount) % WIDTH, rc));
                            }
                        }
                        screen = next;
                    }
                    _ => panic!("Invalid rotation {}", words[1]),
                }
            }
            _ => panic!("Invalid command {}", words[0]),
        }
    }
    println!("Part 1: {}", screen.len());
    println!("Part 2:");
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", if screen.contains(&(x, y)) { '█' } else { ' ' });
        }
        println!();
    }
}
