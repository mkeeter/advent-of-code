fn run(time: &[i64], dist: &[i64]) -> i64 {
    let mut out = 1;
    for (t, d) in time.iter().zip(dist.iter()) {
        // x * (t - x) = d
        // -x^2 + tx = d
        // -x^2 + tx - d = 0
        let det = t.pow(2) - 4 * d;
        if det >= 0 {
            let d = (det as f64).sqrt();
            let t = *t as f64;
            // We have to win, not just tie the best score, so shrink our range
            // by one if the results happen to be perfect integers.
            let pad = ((d / 2.0).fract() == 0.0) as i64;
            let lo = ((t - d) / 2.0).ceil() as i64 + pad;
            let hi = ((t + d) / 2.0).floor() as i64 - pad;
            out *= (hi - lo) + 1;
        }
    }
    out
}

pub fn solve(s: &str) -> (String, String) {
    let lines = s.lines().collect::<Vec<&str>>();
    let f = |i: usize, v| {
        lines[i]
            .strip_prefix(v)
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    };

    let time = f(0, "Time:");
    let dist = f(1, "Distance:");
    let p1 = run(&time, &dist);

    let f = |i: usize, v| {
        lines[i]
            .replace(' ', "")
            .strip_prefix(v)
            .unwrap()
            .parse::<i64>()
            .unwrap()
    };
    let time = f(0, "Time:");
    let dist = f(1, "Distance:");
    let p2 = run(&[time], &[dist]);

    (p1.to_string(), p2.to_string())
}
