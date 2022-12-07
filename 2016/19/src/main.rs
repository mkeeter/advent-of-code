use std::collections::VecDeque;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let count = usize::from_str(input.trim()).unwrap();

    let mut elves = VecDeque::new();
    for i in 0..count {
        elves.push_back((i + 1, 1));
    }
    while elves.len() > 1 {
        let mut robber = elves.pop_front().unwrap();
        let victim = elves.pop_front().unwrap();
        robber.1 += victim.1;
        elves.push_back(robber);
    }
    let winner = elves.pop_front().unwrap().0;
    println!("Part 1: {}", winner);

    // Use a pair of queues to represent each half of the list
    let mut robbers = VecDeque::new();
    let mut victims = VecDeque::new();
    for i in 0..count {
        if i < count / 2 {
            &mut robbers
        } else {
            &mut victims
        }
        .push_back((i + 1, 1));
    }
    for i in 1..count {
        let robber = robbers.pop_front().unwrap();
        let victim = victims.pop_front().unwrap();
        victims.push_back((robber.0, robber.1 + victim.1));
        if i % 2 == 1 {
            robbers.push_back(victims.pop_front().unwrap());
        }
    }
    println!("Part 2: {}", victims.pop_back().unwrap().0);
}
