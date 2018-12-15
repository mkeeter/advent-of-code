#[macro_use] extern crate itertools;

fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn part1(pts: &Vec<(i32, i32)>, center: &(i32, i32)) {
    let closest = |target: (i32, i32)| -> Option<usize> {
        pts.iter()
            .map(|&pt| distance(pt, target))
            .enumerate()
            .try_fold((0, std::i32::MAX), |best, current| {
                if best.1 == current.1 { Err(()) }
                else if current.1 < best.1 { Ok(current) }
                else { Ok(best) }
            })
            .ok()
            .map(|p| p.0)
    };


    let run = |size| {
        let mut count = Vec::new();
        count.resize(pts.len(), 0);
        iproduct!((center.0 - size)..=(center.0 + size),
                  (center.1 - size)..=(center.1 + size))
            .filter_map(closest)
            .map(|c| count[c] += 1)
            .collect::<()>();
        count
    };

    let a = run(1000);
    let b = run(2000);

    println!("{:?}", a.iter().zip(b.iter())
             .filter(|(a, b)| a == b)
             .map(|p| p.0).max().unwrap());
}

fn part2(pts: &Vec<(i32, i32)>, center: &(i32, i32)) {
    let count = |(x, y): (i32, i32)| -> i32 {
        pts.iter()
            .map(|(px, py)| (px - x).abs() + (py - y).abs())
            .sum::<i32>()
    };

    let limit = 10000;
    let size = 4000;
    let answer = iproduct!(
            (center.0 - size)..=(center.0 + size),
            (center.1 - size)..=(center.1 + size))
        .map(count)
        .filter(|p| *p < limit)
        .count();
    println!("Got answer {}", answer);
}

fn main() {
    let s = include_str!("../input");
    let pts: Vec<(i32, i32)> = s.lines()
        .map(|line| line.split(", "))
        .map(|mut v| (v.next().unwrap().parse().unwrap(),
                      v.next().unwrap().parse().unwrap()))
        .collect();

    let center = {
        let xmin = pts.iter().map(|p| p.0).min().unwrap();
        let xmax = pts.iter().map(|p| p.0).max().unwrap();
        let ymin = pts.iter().map(|p| p.1).min().unwrap();
        let ymax = pts.iter().map(|p| p.1).max().unwrap();
        ((xmin + xmax) / 2, (ymin + ymax) / 2)
    };

    part1(&pts, &center);
    part2(&pts, &center);
}

