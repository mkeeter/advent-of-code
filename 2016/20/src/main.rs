use std::str::FromStr;
use std::io::BufRead;

fn main() {
    let mut blacklist = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut itr = line
                .split('-')
                .filter_map(|i| usize::from_str(i).ok());
            (itr.next().unwrap(), itr.next().unwrap())
        })
        .collect::<Vec<(usize, usize)>>();
    blacklist.sort();

    let allowed = |i: usize| -> bool {
        blacklist.iter().all(|&(min, max)| i < min || i > max)
    };

    println!("Part 1: {}", (0..).find(|i| allowed(*i)).unwrap());
    println!("Part 2: {}", (0..=4294967295).filter(|i| allowed(*i)).count());
}
