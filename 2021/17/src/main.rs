use parse_display::{Display, FromStr};
use std::io::Read;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("target area: x={xmin}..{xmax}, y={ymin}..{ymax}")]
struct Target {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
}

fn sim(mut vx: i64, mut vy: i64, target: &Target) -> Option<i64> {
    let (mut x, mut y) = (0, 0);
    let mut ymax = 0;
    while y >= target.ymin {
        if x >= target.xmin && x <= target.xmax && y >= target.ymin && y <= target.ymax {
            return Some(ymax);
        }
        x += vx;
        y += vy;
        ymax = ymax.max(y);
        vx -= vx.signum();
        vy -= 1
    }
    None
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let target: Target = input.trim().parse().unwrap();

    assert!(target.xmin > 0);
    let vy_max = target.ymax.abs().max(target.ymin.abs());
    let out = &(0..=target.xmax)
        .flat_map(|vx| (-vy_max..=vy_max).map(move |vy| (vx, vy)))
        .filter_map(|(vx, vy)| sim(vx, vy, &target))
        .fold((0, 0), |(ymax, count), y| (ymax.max(y), count + 1));
    println!("Part 1: {}", out.0);
    println!("Part 2: {}", out.1);
}
