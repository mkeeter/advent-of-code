use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let mut counters = vec![HashMap::new(); 8];
    for line in std::io::stdin().lock().lines() {
        for (i, c) in line.unwrap().chars().enumerate() {
            *counters[i].entry(c).or_insert(0) += 1;
        }
    }

    print!("Part 1: ");
    for c in counters.iter() {
        let m = c.iter().max_by_key(|(_k, v)| *v).unwrap().0;
        print!("{}", m);
    }
    println!();

    print!("Part 2: ");
    for c in counters.iter() {
        let m = c.iter().min_by_key(|(_k, v)| *v).unwrap().0;
        print!("{}", m);
    }
    println!();
}
