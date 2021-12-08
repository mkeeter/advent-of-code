use std::collections::{HashMap, HashSet};
use std::io::BufRead;

fn solve(input: &HashSet<u8>, output: &[u8]) -> u32 {
    let count = |i| input.iter().find(|word| word.count_ones() == i).unwrap();
    let wires1 = *count(2);
    let wires7 = *count(3);
    let wires4 = *count(4);
    let wires8 = *count(7);

    let seg_a = wires7 ^ wires1;

    let mut seg_counts = [0; 7];
    input
        .iter()
        .flat_map(|num| (0..7).filter(move |i| num & (1 << i) != 0))
        .for_each(|i| seg_counts[i] += 1);

    let seg_b = 1 << seg_counts.iter().position(|&n| n == 6).unwrap();
    let seg_e = 1 << seg_counts.iter().position(|&n| n == 4).unwrap();
    let seg_f = 1 << seg_counts.iter().position(|&n| n == 9).unwrap();
    let seg_c = 1
        << seg_counts
            .iter()
            .enumerate()
            .find(|(i, n)| **n == 8 && (1 << i) != seg_a)
            .unwrap()
            .0;

    let wires0 = *input
        .iter()
        .find(|word| word.count_ones() == 6 && (**word & seg_c) != 0 && (**word & seg_e) != 0)
        .unwrap();
    let wires6 = *input
        .iter()
        .find(|word| word.count_ones() == 6 && (**word & seg_c) == 0)
        .unwrap();
    let seg_d = wires0 ^ wires6 ^ seg_c;

    let seg_g = 1
        << seg_counts
            .iter()
            .enumerate()
            .find(|(i, n)| **n == 7 && (1 << i) != seg_d)
            .unwrap()
            .0;

    let wires2 = seg_a | seg_c | seg_d | seg_e | seg_g;
    let wires3 = seg_a | seg_c | seg_d | seg_f | seg_g;
    let wires5 = seg_a | seg_b | seg_d | seg_f | seg_g;
    let wires9 = seg_a | seg_b | seg_c | seg_d | seg_f | seg_g;

    let mut decode = HashMap::new();
    for (num, wires) in [
        wires0, wires1, wires2, wires3, wires4, wires5, wires6, wires7, wires8, wires9,
    ]
    .iter()
    .enumerate()
    {
        decode.insert(*wires, num as u32);
    }

    output.iter().fold(0, |o, d| o * 10 + decode[d])
}

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut iter = line.split('|').map(|chunk| {
                chunk.split_whitespace().map(|word| {
                    word.chars()
                        .map(|c| 1 << (c as u32 - 'a' as u32))
                        .fold(0, |a, b| a | b) as u8
                })
            });
            let inputs: HashSet<u8> = iter.next().unwrap().collect();
            let outputs: Vec<u8> = iter.next().unwrap().collect();
            assert!(inputs.len() == 10);
            assert!(iter.next().is_none());
            assert!(outputs.len() == 4);
            (inputs, outputs)
        })
        .collect::<Vec<_>>();

    let p1 = lines
        .iter()
        .map(|line| {
            line.1
                .iter()
                .filter(|word| matches!(word.count_ones(), 2 | 4 | 3 | 7))
                .count()
        })
        .sum::<usize>();
    println!("Part 1: {}", p1);

    let p2: u32 = lines.iter().map(|line| solve(&line.0, &line.1)).sum();
    println!("Part 2: {}", p2);
}
