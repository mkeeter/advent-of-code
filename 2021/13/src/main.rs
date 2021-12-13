use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let mut dots: Vec<[u32; 2]> = vec![];
    let mut folds: Vec<(usize, u32)> = vec![];
    for line in std::io::stdin().lock().lines().map(|line| line.unwrap()) {
        if let Some(fold) = line.strip_prefix("fold along ") {
            let mut iter = fold.split('=');
            folds.push((
                match iter.next().unwrap() {
                    "x" => 0,
                    "y" => 1,
                    _ => panic!("Invalid fold axis"),
                },
                iter.next().unwrap().parse().unwrap(),
            ));
        } else if !line.is_empty() {
            let mut iter = line.split(',');
            dots.push([
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            ]);
        }
    }

    let mut apply = |axis: usize, pos: u32| -> usize {
        for d in dots.iter_mut().filter(|d| d[axis] > pos) {
            d[axis] = pos - (d[axis] - pos);
        }
        dots.sort_unstable();
        dots.dedup();
        dots.len()
    };

    println!("Part 1: {}", apply(folds[0].0, folds[0].1));
    for (axis, pos) in &folds[1..] {
        apply(*axis, *pos);
    }

    let xmin = dots.iter().map(|d| d[0]).min().unwrap();
    let xmax = dots.iter().map(|d| d[0]).max().unwrap();
    let ymin = dots.iter().map(|d| d[1]).min().unwrap();
    let ymax = dots.iter().map(|d| d[1]).max().unwrap();
    let dots = dots.into_iter().collect::<HashSet<_>>();

    println!("Part 2:");
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if dots.contains(&[x, y]) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
