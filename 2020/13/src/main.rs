use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut iter = stdin.lock().lines();
    let t: i64 = iter.next().unwrap().unwrap().parse().unwrap();

    let buses = iter.next().unwrap().unwrap()
        .split(",")
        .enumerate()
        .filter_map(|(i, s)| s.parse().ok().map(|b| (i as i64, b)))
        .map(|(i, b): (i64, i64)| (b, (b - i).rem_euclid(b)))
        .collect::<Vec<_>>();

    let p1 = buses.iter()
        .map(|i| ((i.0 - t).rem_euclid(i.0), i.0))
        .min()
        .unwrap();
    println!("Part 1: {}", p1.0 * p1.1);

    let mut d = 1;
    let mut t = 0;
    let mut i = 0;
    let mut m: Option<i64> = None;
    loop {
        if t % buses[i].0 == buses[i].1 {
            if let Some(n) = m {
                i += 1;
                d = t - n;
                m = None;
            } else {
                m = Some(t);
                if i == buses.len() - 1 {
                    break;
                }
            }
        }
        t += d;
    }
    println!("Part 2: {}", t);
}
