use std::io::Read;
use std::str::FromStr;

fn check1(i: &u32) -> bool {
    let mut prev = i % 10;
    let mut num = i / 10;
    let mut has_double = false;
    for _ in 0..5 {
        let next = num % 10;
        if next > prev {
            return false;
        } else if next == prev {
            has_double = true;
        }
        prev = next;
        num /= 10;
    }
    has_double
}

fn check2(i: &u32) -> bool {
    let mut prev = i % 10;
    let mut num = i / 10;
    let mut count = 0;
    for _ in 0..5 {
        let next = num % 10;
        if next == prev {
            count += 1;
        } else {
            if count == 1 {
                return true;
            }
            count = 0;
        }
        prev = next;
        num /= 10;
    }
    count == 1
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let data = buffer.trim()
        .split('-')
        .map(|i| u32::from_str(i))
        .map(|r| r.expect("Could not parse int"))
        .collect::<Vec<_>>();

    let min = data[0];
    let max = data[1];
    println!("Part 1: {}", (min..=max).filter(check1).count());
    println!("Part 2: {}", (min..=max).filter(check1).filter(check2).count());
}
