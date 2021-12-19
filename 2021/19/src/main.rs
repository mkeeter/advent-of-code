use std::collections::HashSet;
use std::io::BufRead;

const X: i8 = 1;
const Y: i8 = 2;
const Z: i8 = 3;
const ROTATIONS: [Rotation; 24] = [
    [X, Y, Z],
    [X, Z, -Y],
    [X, -Y, -Z],
    [X, -Z, Y],
    [Y, -X, Z],
    [Y, Z, X],
    [Y, X, -Z],
    [Y, -Z, -X],
    [-X, -Y, Z],
    [-X, Z, Y],
    [-X, Y, -Z],
    [-X, -Z, -Y],
    [-Y, X, Z],
    [-Y, Z, -X],
    [-Y, -X, -Z],
    [-Y, -Z, X],
    [Z, Y, -X],
    [Z, -X, -Y],
    [Z, -Y, X],
    [Z, X, Y],
    [-Z, Y, X],
    [-Z, X, -Y],
    [-Z, -Y, -X],
    [-Z, -X, Y],
];

type Rotation = [i8; 3];
type Position = [i64; 3];

#[derive(Copy, Clone, Debug)]
struct Alignment {
    rot: Rotation,
    offset: Position,
}

fn rotate(pos: Position, rot: Rotation) -> Position {
    [
        pos[rot[0].abs() as usize - 1] * rot[0].signum() as i64,
        pos[rot[1].abs() as usize - 1] * rot[1].signum() as i64,
        pos[rot[2].abs() as usize - 1] * rot[2].signum() as i64,
    ]
}

fn transform(pos: Position, t: Alignment) -> Position {
    let a = rotate(pos, t.rot);
    [a[0] + t.offset[0], a[1] + t.offset[1], a[2] + t.offset[2]]
}

fn align(base: &HashSet<Position>, other: &HashSet<Position>) -> Option<Alignment> {
    for &rot in &ROTATIONS {
        for &b in base {
            for &o in other {
                let t = rotate(o, rot);
                let offset = [b[0] - t[0], b[1] - t[1], b[2] - t[2]];
                let alignment = Alignment { rot, offset };
                assert_eq!(transform(o, alignment), b);
                let mut matches = 0;
                for &o in other {
                    let o = transform(o, alignment);
                    if base.contains(&o) {
                        matches += 1;
                    }
                }
                if matches >= 12 {
                    return Some(alignment);
                }
            }
        }
    }
    None
}

fn main() {
    let mut data = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.starts_with("---") {
            data.push(HashSet::new());
        } else if !line.is_empty() {
            let mut iter = line.split(',').map(|s| s.parse::<i64>().unwrap());
            let mut d = [0; 3];
            d[0] = iter.next().unwrap();
            d[1] = iter.next().unwrap();
            d[2] = iter.next().unwrap();
            assert!(iter.next().is_none());
            data.last_mut().unwrap().insert(d);
        }
    }
    println!("{:?}", data);

    let mut checked = vec![vec![false; data.len()]; data.len()];
    let mut alignments: Vec<Option<(Alignment, usize)>> = vec![None; data.len()];
    alignments[0] = Some((
        Alignment {
            rot: [X, Y, Z],
            offset: [0, 0, 0],
        },
        0,
    ));

    while !alignments.iter().all(|a| a.is_some()) {
        for j in 0..data.len() {
            if alignments[j].is_some() {
                continue;
            }
            for i in 0..data.len() {
                if i == j || alignments[i].is_none() || checked[i][j] {
                    continue;
                }
                checked[i][j] = true;
                if let Some(a) = align(&data[i], &data[j]) {
                    alignments[j] = Some((a, i));
                }
            }
        }
    }
    let mut beacons = HashSet::new();
    let mut scanner_positions = vec![];
    for mut i in 0..data.len() {
        let mut points = data[i].iter().cloned().collect::<Vec<_>>();
        let mut scanner_pos = [0, 0, 0];
        while i != 0 {
            let a = alignments[i].unwrap();
            for p in points.iter_mut() {
                *p = transform(*p, a.0);
            }
            scanner_pos = transform(scanner_pos, a.0);
            i = a.1;
        }
        for p in points.into_iter() {
            beacons.insert(p);
        }
        scanner_positions.push(scanner_pos);
    }
    println!("Part 1: {}", beacons.len());

    let d = scanner_positions
        .iter()
        .flat_map(|a| {
            scanner_positions
                .iter()
                .map(move |b| (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs())
        })
        .max()
        .unwrap();
    println!("Part 2: {}", d);
}
