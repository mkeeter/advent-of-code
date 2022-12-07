use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let mut blacklist = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut itr = line.split('-').filter_map(|i| i64::from_str(i).ok());
            (itr.next().unwrap(), itr.next().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();
    blacklist.sort();

    let allowed = |i: i64| -> bool { blacklist.iter().all(|&(min, max)| i < min || i > max) };
    println!("Part 1: {}", (0..).find(|i| allowed(*i)).unwrap());

    // Sort the blacklist in descending order by size
    blacklist.sort_by_key(|b| b.1 - b.0);
    blacklist.reverse();
    for i in 0..blacklist.len() {
        for j in (i + 1)..blacklist.len() {
            // If j is completely contained in i, replace with empty interval
            if blacklist[j].0 >= blacklist[i].0 && blacklist[j].1 <= blacklist[i].1 {
                blacklist[j] = (0, -1);
            }
            // If j is partially above i, move it to be totally above
            else if blacklist[j].0 >= blacklist[i].0 && blacklist[j].0 <= blacklist[i].1 {
                let v = blacklist[i].1 + 1;
                blacklist[j].0 = v;
            }
            // If j is partially below i, move it to be totally below
            else if blacklist[j].1 >= blacklist[i].0 && blacklist[j].1 <= blacklist[i].1 {
                let v = blacklist[i].0 - 1;
                blacklist[j].1 = v;
            }
        }
    }
    let blocked = blacklist.iter().map(|i| i.1 - i.0 + 1).sum::<i64>();
    let free = (4294967295 + 1) - blocked;
    println!("Part 2: {}", free);
}
