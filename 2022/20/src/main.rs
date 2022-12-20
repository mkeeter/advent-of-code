use anyhow::{bail, Result};
use std::io::BufRead;

#[derive(Copy, Clone, Debug)]
struct Node {
    value: i64,
    prev: usize,
    next: usize,
}

fn mix(mut nodes: Vec<Node>) -> Vec<Node> {
    for i in 0..nodes.len() {
        let v = nodes[i].value.rem_euclid((nodes.len() - 1) as i64);

        // The 0 node does not move
        if v == 0 {
            continue;
        }

        let mut j = i;
        for _ in 0..v.abs() {
            j = nodes[j].next;
        }

        // Cut node `i` out of the list
        let prev = nodes[i].prev;
        let next = nodes[i].next;
        nodes[prev].next = next;
        nodes[next].prev = prev;

        // Insert `i` after `j`
        let n = nodes[j].next;
        nodes[j].next = i;
        nodes[n].prev = i;
        nodes[i].prev = j;
        nodes[i].next = n;
    }
    nodes
}

fn score(nodes: &[Node]) -> i64 {
    let (mut i, _) = nodes
        .iter()
        .enumerate()
        .find(|(_, n)| n.value == 0)
        .unwrap();
    let mut sum = 0;
    for _ in 0..3 {
        for _ in 0..1000 {
            i = nodes[i].next;
        }
        sum += nodes[i].value;
    }
    sum
}

fn main() -> Result<()> {
    let mut nodes = std::io::stdin()
        .lock()
        .lines()
        .enumerate()
        .map(|(i, line)| -> Result<Node> {
            Ok(Node {
                value: line.unwrap().parse()?,
                prev: i.saturating_sub(1),
                next: i + 1,
            })
        })
        .collect::<Result<Vec<Node>, _>>()?;

    if nodes.is_empty() {
        bail!("Empty input?");
    }

    // Stitch together the list
    nodes[0].prev = nodes.len() - 1;
    nodes.last_mut().unwrap().next = 0;

    let mixed = mix(nodes.clone());
    println!("Part 1: {}", score(&mixed));

    let mut mixed = nodes
        .into_iter()
        .map(|n| Node {
            value: n.value * 811589153,
            ..n
        })
        .collect::<Vec<Node>>();
    for _ in 0..10 {
        mixed = mix(mixed);
    }
    println!("Part 2: {}", score(&mixed));

    Ok(())
}
