use util::get_integers;

pub fn score(a: i64, b: i64, c: i64, d: i64, x: i64, y: i64) -> usize {
    let det = a * d - b * c;
    assert!(det != 0);
    let nx = d * x - b * y;
    let ny = a * y - c * x;
    if nx % det == 0 && nx / det >= 0 && ny % det == 0 && ny % det >= 0 {
        usize::try_from((nx / det) * 3 + ny / det).unwrap()
    } else {
        0
    }
}

pub fn solve(s: &str) -> (usize, usize) {
    let mut iter = std::iter::once("\n").chain(s.lines());
    let mut p1 = 0usize;
    let mut p2 = 0usize;
    while iter.next().is_some() {
        let points = std::array::from_fn(|_i| {
            let a = iter.next().unwrap();
            let mut i = get_integers(a);
            let x = i.next().unwrap();
            let y = i.next().unwrap();
            assert!(i.next().is_none());
            (x, y)
        });
        let [(a, c), (b, d), (x, y)] = points;
        p1 += score(a, b, c, d, x, y);
        p2 += score(a, b, c, d, x + 10000000000000, y + 10000000000000);
    }
    (p1, p2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const EXAMPLE: &str = indoc::indoc! {"
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400

            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176

            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450

            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279
        "};
        assert_eq!(solve(EXAMPLE).0, 480);
    }
}
