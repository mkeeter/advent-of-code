use std::collections::HashSet;
use std::io::BufRead;
use std::str::FromStr;

use itertools::*;

fn main() {
    let weights = std::io::stdin()
        .lock()
        .lines()
        .map(|line| usize::from_str(&line.unwrap()).unwrap())
        .collect::<Vec<usize>>();

    let qe = |k: &Vec<usize>| k.iter().map(|j| weights[*j]).product::<usize>();
    let wt = |k: &Vec<usize>| k.iter().map(|j| weights[*j]).sum::<usize>();

    let target_weight = weights.iter().sum::<usize>() / 3;
    'part1: for i in 1..weights.len() {
        let mut v = (0..weights.len())
            .combinations(i)
            .filter(|v| wt(v) == target_weight)
            .collect::<Vec<Vec<usize>>>();

        v.sort_by_key(qe);
        for v in v.iter() {
            let q = qe(v);
            let vs = v.iter().copied().collect::<HashSet<usize>>();
            for i in 1..weights.len() {
                if (0..weights.len())
                    .filter(|w| !vs.contains(w))
                    .combinations(i)
                    .any(|v| wt(&v) == target_weight)
                {
                    println!("Part 1: {}", q);
                    break 'part1;
                }
            }
        }
    }

    let target_weight = weights.iter().sum::<usize>() / 4;
    'part2: for i in 1..weights.len() {
        let mut v = (0..weights.len())
            .combinations(i)
            .filter(|v| wt(v) == target_weight)
            .collect::<Vec<Vec<usize>>>();
        v.sort_by_key(qe);

        for v in v.iter() {
            let q = qe(v);
            let vs = v.iter().copied().collect::<HashSet<usize>>();
            for i in 1..weights.len() {
                let v = (0..weights.len())
                    .filter(|w| !vs.contains(w))
                    .combinations(i)
                    .filter(|v| wt(&v) == target_weight)
                    .collect::<Vec<Vec<usize>>>();

                for v in v.iter() {
                    let mut vs = vs.clone();
                    for w in v {
                        vs.insert(*w);
                    }

                    for i in 1..weights.len() {
                        if (0..weights.len())
                            .filter(|w| !vs.contains(w))
                            .combinations(i)
                            .any(|v| wt(&v) == target_weight)
                        {
                            println!("Part 2: {}", q);
                            break 'part2;
                        }
                    }
                }
            }
        }
    }
}
