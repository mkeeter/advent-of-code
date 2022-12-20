use anyhow::{bail, Result};
use std::collections::VecDeque;
use std::io::BufRead;

fn mix_vdq(mut nodes: VecDeque<(usize, i64)>, count: usize) -> i64 {
    for _ in 0..count {
        for i in 0..nodes.len() {
            let index = nodes.iter().position(|(j, _)| *j == i).unwrap();
            nodes.rotate_left(index);

            let n = nodes.pop_front().unwrap();
            nodes.rotate_left(n.1.rem_euclid(nodes.len() as i64) as usize);
            nodes.push_front(n);
        }
    }

    let index = nodes.iter().position(|(_, v)| *v == 0).unwrap();
    nodes.rotate_left(index);
    let mut sum = 0;
    for _ in 0..3 {
        nodes.rotate_left(1000 % nodes.len());
        sum += nodes[0].1;
    }
    sum
}

fn main() -> Result<()> {
    let nodes = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse())
        .collect::<Result<Vec<i64>, _>>()?;

    if nodes.is_empty() {
        bail!("Empty input?");
    }

    let v: VecDeque<_> =
        nodes.into_iter().enumerate().map(|(i, v)| (i, v)).collect();
    println!("Part 1: {}", mix_vdq(v.clone(), 1));

    let v = v.into_iter().map(|(i, v)| (i, v * 811589153)).collect();
    println!("Part 2: {}", mix_vdq(v, 10));

    Ok(())
}
