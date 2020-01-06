use std::io::BufRead;
use std::str::FromStr;

enum Cmd { On, Off, Toggle }

fn main() {
    let input = std::io::stdin().lock()
        .lines()
        .map(|line| {
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
                        .split(' ')
                        .filter_map(|s| usize::from_str(s).ok())
                        .collect::<Vec<_>>();
            (cmd, nums)
        })
        .collect::<Vec<_>>();

    let mut grid = vec![vec![false; 1000]; 1000];
    for (cmd, nums) in input.iter() {
        for row in &mut grid[nums[0]..=nums[2]] {
            for g in &mut row[nums[1]..=nums[3]] {
                let prev = *g;
                *g = match cmd {
                    Cmd::On => true,
                    Cmd::Off => false,
                    Cmd::Toggle => !prev,
                };
            }
        }
    }

    let lit = grid.iter()
        .flat_map(|g| g.iter())
        .filter(|b| **b)
        .count();
    println!("Part 1: {}", lit);

    let mut grid = vec![vec![0; 1000]; 1000];
    for (cmd, nums) in input.iter() {
        for row in &mut grid[nums[0]..=nums[2]] {
            for g in &mut row[nums[1]..=nums[3]] {
                let prev = *g;
                *g += match cmd {
                    Cmd::On => 1,
                    Cmd::Off => -((prev > 0) as i32),
                    Cmd::Toggle => 2,
                };
            }
        }
    }

    let brightness = grid.iter()
        .flat_map(|g| g.iter())
        .sum::<i32>();
    println!("Part 2: {}", brightness);
}
