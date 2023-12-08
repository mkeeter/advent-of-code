use anyhow::{bail, Result};
use std::{collections::BTreeSet, io::BufRead, str::FromStr};

use parse_display::FromStr;

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
        self.0 % 36 == 0 // 'A'
    }
    fn is_end(&self) -> bool {
        self.0 % 36 == 25 // 'Z'
    }
}

impl std::str::FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            bail!("Invalid length (expected 3, got {})", s.len());
        }
        let mut out = 0u16;
        for c in s.chars() {
            let i = match c {
                'A'..='Z' => c as u8 - b'A',
                '0'..='9' => c as u8 - b'9',
                _ => bail!("Invalid character {c}"),
            };
            out = out * 36 + i as u16;
        }
        Ok(Self(out))
    }
}

fn main() -> Result<()> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<String>, _>>()?;

    let mut map = vec![];
    let mut nodes = vec![];
    for line in &lines[2..] {
        let i = line.parse::<Input>().unwrap();
        let n = i.start.0 as usize;
        if n >= map.len() {
            map.resize(n + 1, (Node(u16::MAX), Node(u16::MAX)));
        }
        map[n] = (i.left, i.right);
        nodes.push(i.start);
    }

    let mut pos = Node::from_str("AAA").unwrap();
    let dest = Node::from_str("ZZZ").unwrap();
    for (i, lr) in std::iter::repeat(&lines[0])
        .flat_map(|c| c.chars())
        .enumerate()
    {
        if pos == dest {
            println!("Part 1: {i}");
            break;
        }
        match lr {
            'L' => pos = map[pos.0 as usize].0,
            'R' => pos = map[pos.0 as usize].1,
            c => panic!("invalid direction '{c}'"),
        }
    }

    let starts: Vec<_> = nodes.iter().filter(|n| n.is_start()).collect();
    let mut periods = Vec::with_capacity(starts.len());
    for start in starts {
        let mut pos = *start;
        let mut seen = None;
        for (i, lr) in std::iter::repeat(&lines[0])
            .flat_map(|c| c.chars())
            .enumerate()
        {
            if pos.is_end() {
                if let Some((prev_end, prev_steps)) = seen {
                    assert_eq!(prev_end, pos, "multi-end loop detected");
                    assert_eq!(
                        prev_steps % lines[0].len(),
                        i % lines[0].len(),
                        "offset detected"
                    );
                    periods.push((prev_steps, i - prev_steps));
                    break;
                } else {
                    seen = Some((pos, i));
                }
            }
            match lr {
                'L' => pos = map[pos.0 as usize].0,
                'R' => pos = map[pos.0 as usize].1,
                c => panic!("invalid direction '{c}'"),
            }
        }
    }

    for (off, per) in &periods {
        assert_eq!(off, per, "oh no, the input isn't deliberately easy");
    }

    let mut periods = periods.into_iter().map(|p| p.0).collect::<Vec<usize>>();
    while periods.len() > 1 {
        periods.sort();
        let mut a = periods[0];
        let mut b = periods[1];
        while a != b {
            if a < b {
                a += (b - a).div_ceil(periods[0]) * periods[0];
            } else {
                b += (a - b).div_ceil(periods[1]) * periods[1];
            }
        }
        periods = periods[2..]
            .iter()
            .cloned()
            .chain(std::iter::once(a))
            .collect();
    }
    println!("Part 2: {}", periods[0]);
    Ok(())
}
