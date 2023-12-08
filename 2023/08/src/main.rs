use anyhow::{bail, Result};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::BufRead,
};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq)]
#[display("{start} = ({left}, {right})")]
struct Input {
    start: Node,
    left: Node,
    right: Node,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Node([u8; 3]);

impl Node {
    fn is_start(&self) -> bool {
        self.0[2] == b'A'
    }
    fn is_end(&self) -> bool {
        self.0[2] == b'Z'
    }
}
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.0[0] as char, self.0[1] as char, self.0[2] as char
        )
    }
}

impl std::str::FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            bail!("Invalid length (expected 3, got {})", s.len());
        }
        let mut out = [0u8; 3];
        for (i, c) in s.chars().enumerate() {
            out[i] = c.try_into().unwrap();
        }
        Ok(Self(out))
    }
}

fn main() -> Result<()> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<String>, _>>()?;

    let mut map = BTreeMap::new();
    for line in &lines[2..] {
        let i = line.parse::<Input>().unwrap();
        map.insert(i.start, (i.left, i.right));
    }

    let mut pos = "AAA".parse::<Node>().unwrap();
    let dest = "ZZZ".parse::<Node>().unwrap();
    for (i, lr) in std::iter::repeat(&lines[0])
        .flat_map(|c| c.chars())
        .enumerate()
    {
        if pos == dest {
            println!("Part 1: {i}");
            break;
        }
        match lr {
            'L' => pos = map[&pos].0,
            'R' => pos = map[&pos].1,
            c => panic!("invalid direction '{c}'"),
        }
    }

    let starts: Vec<_> = map.keys().filter(|n| n.is_start()).collect();
    let ends: BTreeSet<_> = map.keys().filter(|n| n.is_end()).collect();
    let mut periods = vec![];
    for start in starts {
        let mut pos = *start;
        let mut seen = None;
        for (i, lr) in std::iter::repeat(&lines[0])
            .flat_map(|c| c.chars())
            .enumerate()
        {
            if ends.contains(&pos) {
                if let Some((prev_end, prev_steps)) = seen {
                    assert_eq!(prev_end, pos, "multi-end loop detected");
                    periods.push((prev_steps, i - prev_steps));
                    break;
                } else {
                    seen = Some((pos, i));
                }
            }
            match lr {
                'L' => pos = map[&pos].0,
                'R' => pos = map[&pos].1,
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
                a += periods[0]
            } else {
                b += periods[1]
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
