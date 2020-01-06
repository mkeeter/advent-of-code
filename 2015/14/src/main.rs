use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let input = std::io::stdin().lock()
        .lines()
        .map(|line| {
            let v = line.unwrap().split(' ')
                .filter_map(|i| usize::from_str(i).ok())
                .collect::<Vec<usize>>();
            (v[0], v[1], v[2])
        })
        .collect::<Vec<(usize, usize, usize)>>();

    let mut distances = vec![0; input.len()];
    let mut scores = vec![0; input.len()];
    for t in 0..2503 {
        for (i, (speed, fly, rest)) in input.iter().enumerate() {
            if t % (fly + rest) < *fly {
                distances[i] += speed;
            }
        }
        let best = distances.iter().max().unwrap();
        for (i, d) in distances.iter().enumerate() {
            if d == best {
                scores[i] += 1;
            }
        }
    }
    println!("Part 1: {}", distances.iter().max().unwrap());
    println!("Part 2: {}", scores.iter().max().unwrap());
}
