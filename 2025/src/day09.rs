use std::collections::{BTreeMap, BTreeSet};

pub fn solve(s: &str) -> (u64, u64) {
    let pts = s
        .lines()
        .map(|line| {
            let mut iter = line.split(',');
            let x = iter.next().unwrap().parse().unwrap();
            let y = iter.next().unwrap().parse().unwrap();
            assert!(iter.next().is_none());
            (x, y)
        })
        .collect::<Vec<(u64, u64)>>();
    let rects = pts
        .iter()
        .enumerate()
        .flat_map(|(i, (ax, ay))| {
            pts[0..i].iter().map(move |(bx, by)| {
                (
                    (ax.abs_diff(*bx) + 1) * (ay.abs_diff(*by) + 1),
                    ((*ax).min(*bx), (*ay).min(*by)),
                    ((*ax).max(*bx), (*ay).max(*by)),
                )
            })
        })
        .collect::<BTreeSet<_>>();
    let part1 = rects.last().unwrap().0;

    let mut hlines: BTreeMap<u64, Vec<std::ops::RangeInclusive<u64>>> =
        BTreeMap::new();
    for ((ax, ay), (bx, by)) in pts
        .iter()
        .zip(pts.iter().skip(1).chain(std::iter::once(&pts[0])))
    {
        if ay == by {
            hlines
                .entry(*ay)
                .or_default()
                .push((*ax).min(*bx)..=(*ax).max(*bx));
        }
    }
    let inside = |x, y| {
        let mut count = 0;
        let mut start_count = 0;
        let mut end_count = 0;
        for (v, rs) in hlines.range(0..=y) {
            for r in rs {
                if r.contains(&x) && y == *v {
                    return true;
                } else if *r.start() == x {
                    start_count += 1;
                } else if *r.end() == x {
                    end_count += 1;
                } else if r.contains(&x) {
                    count += 1;
                }
            }
        }
        while start_count > 0 && end_count > 0 {
            start_count -= 1;
            end_count -= 1;
            count += 1;
        }
        count % 2 == 1 || start_count % 2 == 1 || end_count % 2 == 1
    };
    'outer: for (size, (xmin, ymin), (xmax, ymax)) in rects.into_iter().rev() {
        assert!(xmin <= xmax);
        assert!(ymin <= ymax);
        // We need to prove that all four lines are inside, along with a point
        // in the center.  We'll start with the corners and center, because
        // those allow us to quickly weed out rectangles.
        for x in [xmin, xmax] {
            for y in [ymin, ymax] {
                if !inside(x, y) {
                    continue 'outer;
                }
            }
        }
        if !inside((xmin + xmax) / 2, (ymin + ymax) / 2) {
            continue 'outer;
        }
        // Then exhaustively check the edges
        for x in xmin..=xmax {
            for y in [ymax, ymin] {
                if !inside(x, y) {
                    continue 'outer;
                }
            }
        }
        for y in ymin..=ymax {
            for x in [xmin, xmax] {
                if !inside(x, y) {
                    continue 'outer;
                }
            }
        }
        return (part1, size);
    }
    panic!("no containing rectangle found")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        // Trailing spaces are load-bearing here!
        let s = indoc::indoc! {"
            7,1
            11,1
            11,7
            9,7
            9,5
            2,5
            2,3
            7,3
        "};
        let (a, b) = solve(s);
        assert_eq!(a, 50);
        assert_eq!(b, 24);
    }
}
