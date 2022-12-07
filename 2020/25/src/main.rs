use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.lines();
    let a: u64 = iter.next().unwrap().parse().unwrap();
    let b: u64 = iter.next().unwrap().parse().unwrap();

    let f = |s: u64| std::iter::successors(Some(1), move |t| Some(t * s % 20201227));

    let n = f(7).enumerate().find(|(_i, t)| *t == a).unwrap().0;

    println!("Part 1: {}", f(b).nth(n).unwrap());
    println!("Part 2: ⭐️");
}
