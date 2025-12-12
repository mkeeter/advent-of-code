#[derive(Debug)]
struct Region {
    width: u64,
    height: u64,
    counts: Vec<usize>,
}

pub fn solve(s: &str) -> (usize, &'static str) {
    let iter = s.split("\n\n");
    let mut pieces = vec![];
    let mut regions = vec![];
    for chunk in iter {
        let mut iter = chunk.lines();
        let line = iter.next().unwrap();
        if line.ends_with(':') {
            let mut piece = Vec::new();
            for (y, line) in iter.enumerate() {
                for (x, c) in line.chars().enumerate() {
                    match c {
                        '#' => piece.push((x, y)),
                        '.' => (),
                        c => panic!("invalid piece character '{c}'"),
                    }
                }
            }
            pieces.push(piece)
        } else {
            for line in std::iter::once(line).chain(iter) {
                let mut iter = line.split_whitespace();
                let size = iter.next().unwrap();
                let size = size.strip_suffix(':').unwrap();
                let mut size_iter = size.split('x');
                let width = size_iter.next().unwrap().parse().unwrap();
                let height = size_iter.next().unwrap().parse().unwrap();
                let counts = iter.map(|i| i.parse().unwrap()).collect();
                regions.push(Region {
                    width,
                    height,
                    counts,
                });
            }
        }
    }

    let mut fits = 0;
    for r in regions {
        let tiles: usize = r
            .counts
            .iter()
            .enumerate()
            .map(|(i, n)| n * pieces[i].len())
            .sum();
        if tiles as u64 <= r.width * r.height {
            fits += 1;
        }
    }
    (fits, "⭐️")
}
