use anyhow::{anyhow, bail, Error, Result};
use std::{cmp::Ordering, io::BufRead};

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Integer(i64),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(mut s: &str) -> Result<(Vec<Packet>, &str)> {
        let mut out = vec![];
        while !s.is_empty() {
            let (start, rest) = s.split_at(1);
            match start {
                "]" => break,
                "[" => {
                    let (list, rest) = Packet::parse(rest)?;
                    let (c, rest) = rest.split_at(1);
                    if c != "]" {
                        bail!("Missing closing ']' in {s}");
                    }
                    s = rest;
                    out.push(Packet::List(list));
                }
                "," => {
                    s = rest;
                    continue;
                }
                _ => {
                    let int_len = s
                        .chars()
                        .take_while(|c| char::is_digit(*c, 10))
                        .count();
                    let (int, rest) = s.split_at(int_len);
                    out.push(Packet::Integer(int.parse()?));
                    s = rest;
                }
            }
        }
        Ok((out, s))
    }
}

impl std::cmp::Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a.cmp(b),
            (Packet::Integer(a), Packet::List(..)) => {
                Packet::List(vec![Packet::Integer(*a)]).cmp(other)
            }
            (Packet::List(..), Packet::Integer(b)) => {
                self.cmp(&Packet::List(vec![Packet::Integer(*b)]))
            }
            (Packet::List(a), Packet::List(b)) => {
                for i in 0.. {
                    let a = a.get(i);
                    let b = b.get(i);
                    match (a, b) {
                        // Lists are the same length; no result
                        (None, None) => return Ordering::Equal,
                        // Left list terminated first
                        (None, Some(..)) => return Ordering::Less,
                        // Right list terminated first
                        (Some(..), None) => return Ordering::Greater,
                        // Recurse into comparison
                        (Some(a), Some(b)) => {
                            let v = a.cmp(b);
                            if v != Ordering::Equal {
                                return v;
                            }
                        }
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Packet::Integer(i) => write!(f, "{}", i)?,
            Packet::List(ps) => {
                write!(f, "[")?;
                for (i, v) in ps.iter().enumerate() {
                    write!(f, "{}", v)?;
                    if i != ps.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")?;
            }
        }
        Ok(())
    }
}

impl std::str::FromStr for Packet {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (out, rest) = Packet::parse(s)?;
        if out.len() != 1 {
            bail!("Multiple packets in one string!");
        }
        if !rest.is_empty() {
            bail!("Trailing data found in '{s}': '{rest}'");
        }
        Ok(out.into_iter().next().unwrap())
    }
}

fn main() -> Result<()> {
    let mut iter = std::io::stdin().lock().lines().map(Result::unwrap);
    let mut out: Vec<(Packet, Packet)> = vec![];
    while let Some(a) = iter.next() {
        let b = iter
            .next()
            .ok_or_else(|| anyhow!("Missing second packet"))?;
        out.push((a.parse()?, b.parse()?));
        iter.next(); // Skip newline
    }

    let sum = (1..)
        .zip(&out)
        .filter(|(_i, (a, b))| a < b)
        .map(|i| i.0)
        .sum::<usize>();
    println!("Part 1: {sum}");

    let mut out = out
        .into_iter()
        .flat_map(|i| [i.0, i.1].into_iter())
        .collect::<Vec<Packet>>();
    out.sort_unstable();

    // No need to insert the markers; we'll just find where they *would* go
    let a = out.binary_search(&"[[2]]".parse()?).err().unwrap();
    let b = out.binary_search(&"[[6]]".parse()?).err().unwrap();
    println!("Part 2: {}", (a + 1) * (b + 2));

    Ok(())
}
