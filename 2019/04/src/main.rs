use std::io::Read;
use std::cmp::Ordering;
use std::str::FromStr;
use std::iter::successors;

fn check(i: u32) -> (bool, bool) {
    let mut prev = 10;
    let mut has_any_double = false;
    let mut has_double = false;
    let mut count = 0;
    let itr = successors(Some(i), |&i| if i > 0 { Some(i/10) }
                                       else     { None });
    for next in itr.map(|d| d % 10) {
         match next.cmp(&prev) {
            Ordering::Greater => return (false, false),
            Ordering::Less => {
                has_double      |= count == 1;
                has_any_double  |= count  > 0;
                count = 0;
            },
            Ordering::Equal => count += 1,
        }
        prev = next;
    }
    (has_any_double || count > 0, has_double || count == 1)
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
    println!("Part 1: {}", (min..=max).filter(|i| check(*i).0).count());
    println!("Part 2: {}", (min..=max).filter(|i| check(*i).1).count());
}
