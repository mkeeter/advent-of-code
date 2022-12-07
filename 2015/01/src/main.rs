use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let itr = || {
        input.chars().map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
    };

    let p1: i32 = itr().sum();
    println!("Part 1: {}", p1);

    let p2: usize = itr()
        .scan(0, |state, i| {
            *state += i;
            Some(*state)
        })
        .enumerate()
        .find(|(_i, floor)| *floor < 0)
        .unwrap()
        .0
        + 1;
    println!("Part 2: {}", p2);
}
