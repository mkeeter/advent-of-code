extern crate regex;

use regex::Regex;
use std::collections::HashSet;

type Pt = ((i64, i64), (i64, i64));

fn bounds(pts: &Vec<Pt>) -> ((i64, i64), (i64, i64)) {
    (
        (
            pts.iter().map(|p| (p.0).0).min().unwrap(),
            pts.iter().map(|p| (p.0).0).max().unwrap(),
        ),
        (
            pts.iter().map(|p| (p.0).1).min().unwrap(),
            pts.iter().map(|p| (p.0).1).max().unwrap(),
        ),
    )
}

fn score(pts: &Vec<Pt>) -> i64 {
    let ((xmin, xmax), (ymin, ymax)) = bounds(pts);
    (xmax - xmin) * (ymax - ymin)
}

fn print_pts(pts: &Vec<Pt>) {
    let ((xmin, xmax), (ymin, ymax)) = bounds(pts);

    let pts = pts.iter().map(|p| p.0).collect::<HashSet<(i64, i64)>>();
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if pts.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn main() {
    let re = Regex::new(r"[< ](-*\d+)").unwrap();

    let mut pts = include_str!("../input")
        .lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|i| str::parse::<i64>(&i[1]).unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|v| ((v[0], v[1]), (v[2], v[3])))
        .collect::<Vec<Pt>>();

    let mut prev_score = score(&pts);
    let mut this_score = score(&pts);
    let mut time = 0;
    loop {
        let next_pts = pts
            .iter()
            .map(|((x, y), (dx, dy))| ((x + dx, y + dy), (*dx, *dy)))
            .collect();
        let next_score = score(&next_pts);
        if this_score < prev_score && this_score < next_score {
            println!("Got local minima {} at time {}", this_score, time);
            print_pts(&pts);
            break;
        }
        pts = next_pts;
        prev_score = this_score;
        this_score = next_score;
        time += 1;
    }
}
