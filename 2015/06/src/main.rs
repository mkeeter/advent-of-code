use std::io::BufRead;
use std::str::FromStr;

enum Cmd { On, Off, Toggle }

fn main() {
    let mut grid = vec![vec![false; 1000]; 1000];
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let cmd = if line.starts_with("turn on") {
            Cmd::On
        } else if line.starts_with("turn off") {
            Cmd::Off
        } else if line.starts_with("toggle") {
            Cmd::Toggle
        } else {
            panic!("Invalid line {}", line);
        };
        let nums = line.replace(",", " ")
                    .split(" ")
                    .filter_map(|s| usize::from_str(s).ok())
                    .collect::<Vec<_>>();

        for x in nums[0]..=nums[2] {
            for y in nums[1]..=nums[3] {
                let prev = grid[x][y];
                grid[x][y] = match cmd {
                    Cmd::On => true,
                    Cmd::Off => false,
                    Cmd::Toggle => !prev,
                };
            }
        }
    }

    // 378838 is too low
    let lit = grid.iter()
        .flat_map(|g| g.iter())
        .filter(|b| **b)
        .count();
    println!("Part 1: {}", lit);
}
