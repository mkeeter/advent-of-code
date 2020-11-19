use std::str::FromStr;
use std::collections::{HashMap, HashSet};

fn trace(s : &str) -> HashMap<(i32, i32), usize> {
    let mut pos = (0, 0);
    let mut out = HashMap::new();
    let mut steps = 0;
    for cmd in s.split(',') {
        let (dx, dy) = match cmd.chars().next().unwrap() {
            'L' => (-1,  0),
            'R' => ( 1,  0),
            'U' => ( 0,  1),
            'D' => ( 0, -1),
            _ => panic!("Invalid direction"),
        };
        let count = i32::from_str(&cmd[1..]).expect("Could not parse int");
        for _ in 0..count {
            pos = (pos.0 + dx, pos.1 + dy);
            steps += 1;
            out.insert(pos, steps);
        }
    }
    out
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut a = String::new();
    let mut b = String::new();
    std::io::stdin().read_line(&mut a)?;
    std::io::stdin().read_line(&mut b)?;

    let a = trace(a.trim());
    let b = trace(b.trim());

    let crossings = a.keys()
        .collect::<HashSet<_>>()
        .intersection(&b.keys().collect())
        .cloned()
        .collect::<Vec<_>>();

    let closest = crossings.iter()
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .expect("No crossings?");
    println!("Part 1: {}", closest);

    let fastest = crossings.iter()
        .map(|k| a[k] + b[k])
        .min()
        .expect("No crossings?");
    println!("Part 2: {}", fastest);

    Ok(())
}
