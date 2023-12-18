#[derive(Debug)]
struct Dig {
    direction: char,
    distance: i64,
}

fn parse(s: &str) -> (Dig, Dig) {
    let mut iter = s.split_whitespace();
    let dir = iter.next().unwrap();
    assert_eq!(dir.len(), 1);
    let direction = dir.chars().next().unwrap();
    let distance = iter.next().unwrap().parse::<i64>().unwrap();

    let p1 = Dig {
        direction,
        distance,
    };

    let color = iter.next().unwrap();
    let color = color.strip_prefix("(#").unwrap();
    let color = color.strip_suffix(')').unwrap();

    let direction = match color.chars().last().unwrap() {
        '0' => 'R',
        '1' => 'D',
        '2' => 'L',
        '3' => 'U',
        c => panic!("invalid color char {c}"),
    };
    let distance = i64::from_str_radix(color.split_at(5).0, 16).unwrap();
    let p2 = Dig {
        direction,
        distance,
    };
    (p1, p2)
}

fn run(digs: &[Dig]) -> u64 {
    let mut pos = (0, 0);
    let mut sum = 0;

    for d in digs {
        let start = pos;
        let (dx, dy) = match d.direction {
            'U' => (0, -1),
            'D' => (0, 1),
            'L' => (-1, 0),
            'R' => (1, 0),
            c => panic!("invalid direction {c}"),
        };
        pos.0 += dx * d.distance;
        pos.1 += dy * d.distance;
        sum += (start.0 * pos.1 - start.1 * pos.0) + d.distance;
    }

    sum.unsigned_abs() / 2 + 1 // why +1?  it's a mystery
}

pub fn solve(s: &str) -> (String, String) {
    let mut p1_digs = vec![];
    let mut p2_digs = vec![];
    for line in s.lines() {
        let (a, b) = parse(line);
        p1_digs.push(a);
        p2_digs.push(b);
    }

    let p1 = run(&p1_digs);
    let p2 = run(&p2_digs);

    (p1.to_string(), p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_digs() {
        let s = indoc::indoc! {"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        "};
        let (p1, p2) = solve(s);
        assert_eq!(p1, "62");
        assert_eq!(p2, "952408144115");
    }
}
