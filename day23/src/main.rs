use std::io::{self, Read};
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let re = Regex::new(r"(-*\d+)").unwrap();

    let pts = buffer
        .lines()
        .map(|line| re.captures_iter(line)
                .map(|i| str::parse::<i64>(&i[1]).unwrap())
                .collect::<Vec<i64>>())
        .map(|v| ((v[0], v[1], v[2]), v[3]))
        .collect::<Vec<((i64, i64, i64), i64)>>();

    let ((x, y, z), r) = pts.iter().max_by_key(|pt| pt.1).unwrap();
    let n = pts.iter()
        .filter(|(pt, _)| (pt.0 - x).abs() +
                          (pt.1 - y).abs() +
                          (pt.2 - z).abs() <= *r)
        .count();
    println!("Part 1: {}", n);

    let mut best_score = 0;
    let mut best_pts = HashSet::new();
    let mut check_score = |corner: (i64, i64, i64)| {
        let score = pts.iter()
            .filter(|(pt, r)| (corner.0 - pt.0).abs() +
                              (corner.1 - pt.1).abs() +
                              (corner.2 - pt.2).abs() <= *r)
            .count();
        if score > best_score {
            best_score = score;
            best_pts.clear();
        }
        if score >= best_score {
            best_pts.insert(corner.clone());
        }
    };

    for (i, ((x, y, z), r)) in pts.iter().enumerate() {
        println!("{} / {}", i, pts.len());
        let r = *r;
        for corner in [(r, 0, 0), (-r, 0, 0),
                       (0, r, 0), (0, -r, 0),
                       (0, 0, r), (0, 0, -r)].iter()
        {
            let corner = (x + corner.0, y + corner.1, z + corner.2);
            check_score(corner);
        }

        for (start, end, delta) in [(( r, 0, 0), (0,  r, 0), (-1,  1, 0)),
                                    (( r, 0, 0), (0, -r, 0), (-1, -1, 0)),
                                    ((-r, 0, 0), (0,  r, 0), ( 1,  1, 0)),
                                    ((-r, 0, 0), (0, -r, 0), ( 1, -1, 0)),

                                    ((0, 0,  r), (0,  r, 0), (0,  1, -1)),
                                    ((0, 0,  r), (0, -r, 0), (0, -1, -1)),
                                    ((0, 0, -r), (0,  r, 0), (0,  1,  1)),
                                    ((0, 0, -r), (0, -r, 0), (0, -1,  1)),

                                    ((0, 0,  r), ( r, 0, 0), ( 1, 0, -1)),
                                    ((0, 0,  r), (-r, 0, 0), (-1, 0, -1)),
                                    ((0, 0, -r), ( r, 0, 0), ( 1, 0,  1)),
                                    ((0, 0, -r), (-r, 0, 0), (-1, 0,  1))]
                                    .iter()
        {
            let mut pos = start.clone();
            while pos != *end {
                println!("{:?}, {:?}, {:?}", pos, end, delta);
                let corner = (x + pos.0, y + pos.1, z + pos.2);
                check_score(corner);

                pos.0 += delta.0;
                pos.1 += delta.1;
                pos.2 += delta.2;
            }
        }
    }

    //assert!(best_pts.len() == 1);
    let mut target = best_pts.iter().next().unwrap().clone();

    println!("walking from {:?}", target);
    let mut improved = true;
    while improved {
        improved = false;
        let offset = |i| match i % 3 {
            0 => -1,
            1 =>  0,
            2 =>  1,
            _ => unreachable!(),
        };

        for i in 0..27 {
            println!("{} {} {}", offset(i), offset(i/3), offset(i/9));
            let next = (target.0 + offset(i),
                        target.1 + offset(i/3),
                        target.2 + offset(i/9));
            println!("Checking {:?}", next);
            if next.0.abs() + next.1.abs() + next.2.abs() >=
               target.0.abs() + target.1.abs() + target.2.abs()
            {
                println!("  Not okay");
                continue;
            } else if pts.iter()
                .filter(|((x, y, z), r)| (next.0 - x).abs() +
                                         (next.1 - y).abs() +
                                         (next.2 - z).abs() <= *r)
                .count() == best_score
            {
                target = next;
                improved = true;
            } else {
                println!("  Not valid");
            }
        }
        if improved {
            println!("{:?}", target);
        }
    }
    println!("{:?}, {:?}", best_score, best_pts);

    for x in 42994500..42994515 {
        for y in 37742124..37742135 {
            for z in 48556960..48556970 {
                println!("{} {} {}, {}", x, y, z,
                        pts.iter().filter(|((xn, yn, zn), r)|
                                          (x - xn).abs() +
                                          (y - yn).abs() +
                                          (z - zn).abs() <= *r)
                        .count());
            }
        }
    }
    // 129293600 is too high
}
