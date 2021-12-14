use std::collections::HashMap;
use std::io::BufRead;

/// Pairs of chars are packed into a 10-bit value
fn count(
    start: u16,
    num: usize,
    rules: &[u8],
    cache: &mut HashMap<(u16, usize), HashMap<u16, usize>>,
) -> HashMap<u16, usize> {
    if let Some(score) = cache.get(&(start, num)) {
        return score.clone();
    }
    if num == 0 {
        let mut out = HashMap::new();
        *out.entry(start & 0x3e0).or_insert(0) += 1;
        return out;
    }

    let a = (start & 0x3e0) | (rules[start as usize] as u16);
    let b = (start & 0x01F) | ((rules[start as usize] as u16) << 5);
    let mut out = count(a, num - 1, rules, cache);
    for (k, v) in count(b, num - 1, rules, cache).into_iter() {
        *out.entry(k).or_default() += v;
    }
    cache.insert((start, num), out.clone());
    out
}

fn main() {
    let stdin = std::io::stdin();
    let mut iter = stdin.lock().lines().map(|line| line.unwrap());

    let template = iter.next().unwrap();
    iter.next().unwrap(); // Skip blank link

    let mut rules = vec![0; 1024];
    for line in iter {
        let mut iter = line.split(" -> ");
        let mut input = iter.next().unwrap().chars();
        let a = (input.next().unwrap() as u32 - 'A' as u32) as u16;
        let b = (input.next().unwrap() as u32 - 'A' as u32) as u16;
        let pair = (a << 5) | b;

        let next = (iter.next().unwrap().chars().next().unwrap() as u32 - 'A' as u32) as u8;
        rules[pair as usize] = next;
    }

    let input = template.bytes().zip(template.bytes().skip(1)).map(|(a, b)| {
        (((a - b'A') as u16) << 5) | ((b - b'A') as u16)
    }).collect::<Vec<_>>();

    let mut pairs: HashMap<u16, usize> = HashMap::new();
    let mut cache = HashMap::new();
    for pair in input {
        for (k, v) in count(pair, 40, &rules, &mut cache).into_iter() {
            *pairs.entry(k).or_default() += v;
        }
    }
    let mut out = [0; 26];
    for (k, v) in pairs.into_iter() {
        out[(k >> 5) as usize] += v;
    }
    out[(template.bytes().last().unwrap() - b'A') as usize] += 1;

    println!("{}", out.iter().max().unwrap() - out.iter().filter(|v| **v > 0).min().unwrap());
}
