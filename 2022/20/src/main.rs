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
        let v = nodes[i].value;

        // The 0 node does not move
        if v == 0 {
            continue;
        }

        // Cut node `i` out of the list
        let prev = nodes[i].prev;
        let next = nodes[i].next;
        nodes[prev].next = next;
        nodes[next].prev = prev;
        assert!(next != prev);

        let mut j = i;
        for _ in 0..v.abs() {
            if v < 0 {
                j = nodes[j].prev;
            } else {
                j = nodes[j].next;
            }
        }

        // We are going to insert node `i` after node `j`, so take an extra step
        // back if we're iterating backwards through the list.
        if v < 0 {
            j = nodes[j].prev;
        }

        // Insert `i` after `j`
        let n = nodes[j].next;
        assert!(i != j);
        assert!(i != n);
        assert!(j != n);
        nodes[j].next = i;
        nodes[n].prev = i;
        nodes[i].prev = j;
        nodes[i].next = n;

        // Sanity checking of list invariants
        for (i, n) in nodes.iter().enumerate() {
            assert_eq!(i, nodes[n.prev].next);
            assert_eq!(i, nodes[n.next].prev);
        }
        let mut j = i;
        for _ in 0..nodes.len() {
            j = nodes[j].next;
        }
        assert_eq!(j, i);

        /*
        let mut j = i;
        loop {
            print!("{} ", nodes[j].value);
            j = nodes[j].next;
            if j == i {
                break;
            }
        }
        println!();
        */
    }
    nodes
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
    for (i, n) in nodes.iter().enumerate() {
        assert_eq!(i, nodes[n.prev].next);
        assert_eq!(i, nodes[n.next].prev);
    }

    let mixed = mix(nodes.clone());
    let (mut i, _) = mixed
        .iter()
        .enumerate()
        .find(|(_, n)| n.value == 0)
        .unwrap();
    let mut sum = 0;
    for _ in 0..3 {
        for _ in 0..1000 {
            i = mixed[i].next;
        }
        println!("sum += {}", mixed[i].value);
        sum += mixed[i].value;
    }
    println!("Part 1: {sum}");
    // 17042 is too high
    Ok(())
}
