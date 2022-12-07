use std::io::Read;

fn step(v: Vec<u8>) -> Vec<u8> {
    let mut count = 1;
    let mut start = v[0];
    let mut next = Vec::new();
    for &c in &v[1..] {
        if c == start {
            count += 1;
        } else {
            next.push(count);
            next.push(start);
            start = c;
            count = 1;
        }
    }
    next.push(count);
    next.push(start);
    next
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut v = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();

    for _i in 0..40 {
        v = step(v);
    }
    println!("Part 1: {}", v.len());

    for _i in 0..10 {
        v = step(v);
    }
    println!("Part 2: {}", v.len());
}
