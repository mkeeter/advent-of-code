use anyhow::{anyhow, bail, Error, Result};
use std::{cmp::Ordering, io::BufRead};

#[derive(Clone, Debug, Eq, PartialEq)]
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

    fn compare(&self, right: &Self) -> Option<bool> {
        match (self, right) {
            (Packet::Integer(a), Packet::Integer(b)) => match a.cmp(b) {
                Ordering::Less => Some(true),
                Ordering::Greater => Some(false),
                Ordering::Equal => None,
            },
            (Packet::Integer(a), Packet::List(..)) => {
                Packet::List(vec![Packet::Integer(*a)]).compare(right)
            }
            (Packet::List(..), Packet::Integer(b)) => {
                self.compare(&Packet::List(vec![Packet::Integer(*b)]))
            }
            (Packet::List(a), Packet::List(b)) => {
                for i in 0.. {
                    let a = a.get(i);
                    let b = b.get(i);
                    match (a, b) {
                        // Lists are the same length; no result
                        (None, None) => return None,
                        // Left list terminated first
                        (None, Some(..)) => return Some(true),
                        // Right list terminated first
                        (Some(..), None) => return Some(false),
                        // Recurse into comparison
                        (Some(a), Some(b)) => {
                            if let Some(v) = a.compare(b) {
                                return Some(v);
                            }
                        }
                    }
                }
                None
            }
        }
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
        .filter(|(_i, (a, b))| a.compare(b).unwrap())
        .map(|i| i.0)
        .sum::<usize>();
    println!("Part 1: {sum}");

    let mut out = out
        .into_iter()
        .flat_map(|i| [i.0, i.1].into_iter())
        .collect::<Vec<Packet>>();
    let marker1: Packet = "[[2]]".parse()?;
    let marker2: Packet = "[[6]]".parse()?;
    out.push(marker1.clone());
    out.push(marker2.clone());
    out.sort_unstable_by(|a, b| match a.compare(b) {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        None => Ordering::Equal,
    });

    let index1 = (1..)
        .zip(out.iter())
        .find(|(_i, p)| **p == marker1)
        .unwrap()
        .0;
    let index2 = (1..)
        .zip(out.iter())
        .find(|(_i, p)| **p == marker2)
        .unwrap()
        .0;
    println!("Part 2: {}", index1 * index2);

    Ok(())
}
