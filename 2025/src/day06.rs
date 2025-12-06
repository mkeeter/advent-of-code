use crate::Grid;

type Monoid = (fn(i64, i64) -> i64, i64);
const ADD_MONOID: Monoid = (|a, b| a + b, 0);
const MUL_MONOID: Monoid = (|a, b| a * b, 1);

pub fn solve(s: &str) -> (i64, i64) {
    let mut iter = s.lines();
    let ops = iter
        .next_back()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>();
    let rows = iter
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut part1 = 0;
    for (i, op) in ops.iter().enumerate() {
        let (op, v): (fn(i64, i64) -> i64, i64) = match *op {
            "+" => ADD_MONOID,
            "*" => MUL_MONOID,
            _ => panic!("unknown op {op}"),
        };
        part1 += rows.iter().map(|row| row[i]).fold(v, op);
    }

    let g = Grid::new(s);
    let mut part2 = 0;
    for x in 0..g.width() {
        let (op, mut v) = match g[(x, g.height() - 1)] {
            b'+' => ADD_MONOID,
            b'*' => MUL_MONOID,
            b' ' => continue,
            c => panic!("unknown op character '{c}'"),
        };
        for dx in 0.. {
            let mut value = None;
            for y in 0..g.height() - 1 {
                let c = g[(x + dx, y)];
                if c.is_ascii_digit() {
                    let c = i64::from(c - b'0');
                    value = Some(value.unwrap_or_default() * 10 + c);
                }
            }
            if let Some(value) = value {
                v = op(v, value);
            } else {
                break;
            }
        }
        part2 += v;
    }
    (part1, part2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        // Trailing spaces are load-bearing here!
        let s = indoc::indoc! {"
            123 328  51 64 
             45 64  387 23 
              6 98  215 314
            *   +   *   +  
        "};
        let (a, b) = solve(s);
        assert_eq!(a, 4277556);
        assert_eq!(b, 3263827);
    }
}
