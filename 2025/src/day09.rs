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
        hlines
            .get(&y)
            .is_some_and(|rs| rs.iter().any(|r| r.contains(&x)))
            || hlines
                .range(0..=y)
                .filter(|(_y, rs)| rs.iter().any(|r| r.contains(&x)))
                .count()
                % 2
                == 1
            || hlines
                .range(y..)
                .filter(|(_y, rs)| rs.iter().any(|r| r.contains(&x)))
                .count()
                % 2
                == 1
    };
    'outer: for (size, (xmin, ymin), (xmax, ymax)) in rects.into_iter().rev() {
        println!("{size}");
        assert!(xmin <= xmax);
        assert!(ymin <= ymax);
        for x in [xmin, xmax] {
            for y in [ymin, ymax] {
                if !inside(x, y) {
                    continue 'outer;
                }
            }
        }
        for x in xmin..=xmax {
            for y in [ymin, ymax] {
                if !inside(x, y) {
                    continue 'outer;
                }
            }
        }
        for x in [xmin, xmax] {
            for y in ymin..=ymax {
                if !inside(x, y) {
                    continue 'outer;
                }
            }
        }
        if !inside((xmin + xmax) / 2, (ymin + ymax) / 2) {
            continue 'outer;
        }
        for x in xmin..=xmax {
            for y in ymin..=ymax {
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
