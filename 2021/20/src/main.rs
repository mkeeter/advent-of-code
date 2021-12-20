use std::collections::HashSet;
use std::io::BufRead;

const REGION: [(i64, i64); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];
const BACKGROUND: (i64, i64) = (i64::MAX, i64::MAX);

fn enhance(input: &HashSet<(i64, i64)>, alg: &[bool]) -> HashSet<(i64, i64)> {
    let xmin = input.iter().map(|p| p.0).min().unwrap();
    let xmax = input
        .iter()
        .map(|p| p.0)
        .filter(|p| *p != i64::MAX)
        .max()
        .unwrap();
    let ymin = input.iter().map(|p| p.1).min().unwrap();
    let ymax = input
        .iter()
        .map(|p| p.1)
        .filter(|p| *p != i64::MAX)
        .max()
        .unwrap();

    let mut out = HashSet::new();
    for x in (xmin - 2)..=(xmax + 2) {
        for y in (ymin - 2)..=(ymax + 2) {
            let mut index = 0;
            for &(dx, dy) in &REGION {
                let x = x + dx;
                let y = y + dy;
                let filled = if x < xmin || x > xmax || y < ymin || y > ymax {
                    input.contains(&BACKGROUND)
                } else {
                    input.contains(&(x, y))
                };
                index = (index << 1) | filled as usize;
            }
            if alg[index] {
                out.insert((x, y));
            }
        }
    }
    if (input.contains(&BACKGROUND) && alg[0b111111111]) || (!input.contains(&BACKGROUND) && alg[0])
    {
        out.insert(BACKGROUND);
    }
    out
}

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let alg = lines[0].bytes().map(|b| b == b'#').collect::<Vec<bool>>();

    let mut img = lines[2..]
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter(|(_, b)| *b == b'#')
                .map(move |(x, _)| (x as i64, y as i64))
        })
        .collect::<HashSet<(i64, i64)>>();

    for _ in 0..2 {
        img = enhance(&img, &alg);
    }
    println!("Part 1: {}", img.len());
    for _ in 0..48 {
        img = enhance(&img, &alg);
    }
    println!("Part 2: {}", img.len());
}
