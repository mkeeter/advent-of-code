use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut iter = stdin.lock().lines();
    let t: i64 = iter.next().unwrap().unwrap().parse().unwrap();

    let buses = iter.next().unwrap().unwrap();

    let p1 = buses.split(",").filter_map(|s| s.parse().ok())
        .map(|i: i64| ((i - t).rem_euclid(i), i))
        .min()
        .unwrap();
    println!("Part 1: {}", p1.0 * p1.1);

    let p2 = buses.split(",").enumerate()
        .filter_map(|(i, s)| s.parse().ok().map(|b| (i, b)))
        .map(|(i, b): (usize, i64)| (b, (b - (i as i64)).rem_euclid(b)))
        .collect::<Vec<(i64, i64)>>();
    println!("{:?}", p2);

    let mut d = 1;
    let mut t = 0;
    let mut i = 0;
    let mut m: Option<i64> = None;
    loop {
        if t % p2[i].0 == p2[i].1 {
            if let Some(n) = m {
                i += 1;
                d = t - n;
                m = None;
            } else {
                m = Some(t);
                if i == p2.len() - 1 {
                    break;
                }
            }
        }
        t += d;
    }
    println!("Part 2: {}", t);
}
