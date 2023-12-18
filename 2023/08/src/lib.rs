use std::str::FromStr;

use anyhow::{bail, Result};
use parse_display::FromStr;
use rayon::prelude::*;

#[derive(FromStr, PartialEq)]
#[display("{start} = ({left}, {right})")]
struct Input {
    start: Node,
    left: Node,
    right: Node,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Node(u16);

impl Node {
    fn is_start(&self) -> bool {
        self.0 & 0b11111 == 0 // 'A'
    }
    fn is_end(&self) -> bool {
        self.0 & 0b11111 == 25 // 'Z'
    }
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            bail!("Invalid length (expected 3, got {})", s.len());
        }
        let mut out = 0u16;
        for c in s.chars() {
            let i = match c {
                'A'..='Z' => c as u16 - b'A' as u16,
                '0'..='9' => c as u16 - b'9' as u16,
                _ => bail!("Invalid character {c}"),
            };
            out = (out << 5) + i;
        }
        Ok(Self(out))
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
}

pub fn solve(s: &str) -> (String, String) {
    let lines = s.lines().collect::<Vec<&str>>();

    let dirs = lines[0]
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            c => panic!("invalid direction '{c}'"),
        })
        .collect::<Vec<_>>();

    let mut map = vec![(Node(u16::MAX), Node(u16::MAX)); 36usize.pow(3)];
    let mut nodes = vec![];
    for line in &lines[2..] {
        let i = line.parse::<Input>().unwrap();
        let n = i.start.0 as usize;
        map[n] = (i.left, i.right);
        nodes.push(i.start);
    }

    let run = |start: Node| {
        std::iter::repeat(&dirs)
            .flat_map(|c| c.iter())
            .enumerate()
            .scan(start, |pos, (i, lr)| {
                match lr {
                    Direction::Left => *pos = map[pos.0 as usize].0,
                    Direction::Right => *pos = map[pos.0 as usize].1,
                }
                Some((i + 1, *pos))
            })
    };

    let dest = Node::from_str("ZZZ").unwrap();
    let mut p1 = 0;
    for (i, pos) in run(Node::from_str("AAA").unwrap()) {
        if pos == dest {
            p1 = i;
            break;
        }
    }

    let mut periods = nodes
        .par_iter()
        .filter(|n| n.is_start())
        .map(|start| {
            let mut seen = None;
            for (i, pos) in run(*start) {
                if pos.is_end() {
                    if let Some((prev_end, prev_steps)) = seen {
                        assert_eq!(prev_end, pos, "multi-end loop detected");
                        assert_eq!(
                            prev_steps % lines[0].len(),
                            i % lines[0].len(),
                            "offset detected"
                        );
                        assert_eq!(
                            prev_steps,
                            i - prev_steps,
                            "oh no, the input isn't deliberately easy"
                        );
                        return prev_steps;
                    } else {
                        seen = Some((pos, i));
                    }
                }
            }
            unreachable!()
        })
        .collect::<Vec<usize>>();

    // Hacky LCM calculation
    while periods.len() > 1 {
        let pa = periods.pop().unwrap();
        let pb = periods.pop().unwrap();
        let mut a = pa;
        let mut b = pb;
        while a != b {
            if a < b {
                a += (b - a).div_ceil(pa) * pa;
            } else {
                b += (a - b).div_ceil(pb) * pb;
            }
        }
        periods.push(a);
    }
    let p2 = periods[0];

    (p1.to_string(), p2.to_string())
}
