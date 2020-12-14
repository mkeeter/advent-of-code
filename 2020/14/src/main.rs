use std::collections::HashMap;
use std::io::BufRead;
use regex::Regex;

type Memory = HashMap<u64, u64>;
fn run<F, G, T>(lines: &[String], decode: F, write: G) -> u64
    where F: Fn(&str) -> T,
          G: Fn(u64, u64, &T, &mut Memory),
{
    let imem = Regex::new(r#"^mem\[([0-9]+)\] = ([0-9]+)$"#).unwrap();
    let imask = Regex::new(r#"^mask = ([01X]+)$"#).unwrap();

    let mut mem = HashMap::new();

    let mut mask = None;
    for line in lines.iter() {
        if let Some(c) = imask.captures(&line) {
            mask = Some(decode(c.get(1).unwrap().as_str()));
        } else if let Some(c) = imem.captures(&line) {
            let addr = c.get(1).unwrap().as_str().parse().unwrap();
            let val = c.get(2).unwrap().as_str().parse().unwrap();
            write(val, addr, mask.as_ref().unwrap(), &mut mem);
        } else {
            panic!("Failed to parse: {}", line);
        }
    }
    mem.iter().map(|(_k, v)| *v).sum()
}

////////////////////////////////////////////////////////////////////////////////

fn decode_set_clear(s: &str) -> (u64, u64) {
    let mut set = 0;
    let mut clear = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            '1' => set |= 1 << (35 - i),
            '0' => clear |= 1 << (35 - i),
            'X' => continue,
            _ => panic!("Invalid mask char: {}", c),
        }
    }
    (set, clear)
}

fn write_set_clear(val: u64, addr: u64, (set, clear): &(u64, u64),
                   mem: &mut Memory) {
    mem.insert(addr, (val | set) & (!clear));
}

////////////////////////////////////////////////////////////////////////////////

fn decode_multi(s: &str) -> Vec<(usize, char)> {
    s.chars().enumerate().collect()
}

fn write_multi<D: ?Sized>(val: u64, addr: u64, mask: &D, mem: &mut Memory)
    where D: AsRef<[(usize, char)]>
{
    let mask = mask.as_ref();
    match mask.get(0) {
        None => { mem.insert(addr, val); },
        Some((i, c)) => match c {
            '0' => write_multi(val, addr, &mask[1..], mem),
            '1' => write_multi(val, addr | (1 << (35 - i)), &mask[1..], mem),
            'X' => {
                write_multi(val, addr | (1 << (35 - i)), &mask[1..], mem);
                write_multi(val, addr & (!(1 << (35 - i))), &mask[1..], mem);
            },
            _ => panic!("Invalid mask char: {}", c),
        }
    };
}

////////////////////////////////////////////////////////////////////////////////

fn main() {
    let stdin = std::io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|s| s.unwrap()).collect();

    println!("Part 1: {}", run(&lines, decode_set_clear, write_set_clear));
    println!("Part 2: {}", run(&lines, decode_multi, write_multi));
}
