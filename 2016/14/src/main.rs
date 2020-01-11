use std::collections::{VecDeque, HashSet};
use std::io::Read;

fn run<T>(itr: T) -> usize
    where T: Iterator<Item=md5::Digest>
{
    let mut itr = itr
        .map(|i| format!("{:x}", i).chars().collect::<Vec<_>>())
        .enumerate()
        .map(|(n, s)| {
             let triples = s.windows(3)
                 .filter(|w| w.iter().all(|&c| c == w[0]))
                 .map(|w| w[0])
                 .next();
             let quintics = s.windows(5)
                 .filter(|w| w.iter().all(|&c| c == w[0]))
                 .map(|w| w[0])
                 .collect::<HashSet<char>>();
             (n, triples, quintics)
        });

    let mut q = VecDeque::new();
    for _i in 0..=1000 {
        q.push_back(itr.next().unwrap());
    }

    let mut found = 0;
    while let Some((n, t, _quintics)) = q.pop_front() {
        if let Some(t) = t {
            for i in 0..1000 {
                if q.get(i).unwrap().2.contains(&t) {
                    found += 1;
                    if found == 64 {
                        return n;
                    }
                    break;
                }
            }
        }
        q.push_back(itr.next().unwrap());
    }
    unreachable!();
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();

    let hashes = (0..)
        .map(|i| input.to_string() + &i.to_string())
        .map(md5::compute);

    println!("Part 1: {}", run(hashes));
}
