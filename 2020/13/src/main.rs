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

    let p2 = buses.iter().fold((0, 1), |(t, step), bus| {
        let mut itr = (t..).step_by(step).filter(|t| t % bus.0 == bus.1);
        let t = itr.next().unwrap();
        let step = (itr.next().unwrap() - t) as usize;
        (t, step)
    });
    println!("Part 2: {}", p2.0);
}
