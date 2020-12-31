use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let sizes = std::io::stdin().lock().lines()
        .map(|line| {
            let mut v = line.unwrap()
                 .split('x')
                 .map(|s| usize::from_str(s).unwrap())
                 .collect::<Vec<_>>();
            v.sort_unstable();
            (v[0], v[1], v[2]) })
        .collect::<Vec<_>>();

    let paper = sizes.iter()
        .map(|(a, b, c)| 2*(a*b + b*c + a*c) + a*b)
        .sum::<usize>();
    println!("Part 1: {}", paper);

    let ribbon = sizes.iter()
        .map(|(a, b, c)| 2*(a + b) + a*b*c)
        .sum::<usize>();
    println!("Part 2: {}", ribbon);
}
