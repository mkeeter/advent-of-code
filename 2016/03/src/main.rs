use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let sides = line
                .unwrap()
                .split(' ')
                .filter_map(|i| i32::from_str(i).ok())
                .collect::<Vec<_>>();
            (sides[0], sides[1], sides[2])
        })
        .collect::<Vec<(i32, i32, i32)>>();

    let tri = |a: i32, b: i32, c: i32| a + b > c && a + c > b && b + c > a;
    let valid = input.iter().filter(|(a, b, c)| tri(*a, *b, *c)).count();

    // 865 is too high
    println!("Part 1: {}", valid);

    let valid = input
        .chunks(3)
        .map(|v| {
            tri(v[0].0, v[1].0, v[2].0) as u32
                + tri(v[0].1, v[1].1, v[2].1) as u32
                + tri(v[0].2, v[1].2, v[2].2) as u32
        })
        .sum::<u32>();
    println!("Part 2: {}", valid);
}
