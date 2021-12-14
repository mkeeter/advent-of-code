use std::collections::HashMap;
use std::io::BufRead;

/// Pairs of chars are packed into a 10-bit value
fn count(
    pair: u16,
    rounds: u8,
    rules: &[u8],
    cache: &mut HashMap<(u16, u8), [usize; 26]>,
) -> [usize; 26] {
    if rounds == 0 {
        let mut out = [0; 26];
        out[(pair >> 5) as usize] += 1;
        return out;
    } else if let Some(score) = cache.get(&(pair, rounds)) {
        return *score;
    }

    let a = (pair & 0b1111100000) | (rules[pair as usize] as u16);
    let b = (pair & 0b0000011111) | ((rules[pair as usize] as u16) << 5);
    let mut out = count(a, rounds - 1, rules, cache);
    for (i, v) in count(b, rounds - 1, rules, cache).iter().enumerate() {
        out[i] += v;
    }
    cache.insert((pair, rounds), out);
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
        let mut input = iter.next().unwrap().bytes();
        let a = (input.next().unwrap() - b'A') as u16;
        let b = (input.next().unwrap() - b'A') as u16;
        let pair = (a << 5) | b;

        let next = iter.next().unwrap().bytes().next().unwrap() - b'A';
        rules[pair as usize] = next;
    }

    let input = template
        .bytes()
        .zip(template.bytes().skip(1))
        .map(|(a, b)| (((a - b'A') as u16) << 5) | ((b - b'A') as u16))
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();
    for (i, &n) in [10, 40].iter().enumerate() {
        let mut out = [0; 26];
        for pair in &input {
            for (i, v) in count(*pair, n, &rules, &mut cache).iter().enumerate() {
                out[i] += v;
            }
        }
        out[(template.bytes().last().unwrap() - b'A') as usize] += 1;

        println!(
            "Part {}: {}",
            i + 1,
            out.iter().max().unwrap() - out.iter().filter(|v| **v > 0).min().unwrap()
        );
    }
}
