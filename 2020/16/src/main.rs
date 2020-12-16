use std::io::Read;

use lazy_static::lazy_static;
use regex::Regex;

struct Rule<'a> {
    name: &'a str,
    bounds: [(u64, u64); 2],
}
impl<'a> Rule<'a> {
    fn parse(s: &str) -> Rule {
        lazy_static! {
            static ref RULE: Regex = Regex::new(
                r#"^(.*): (\d+)-(\d+) or (\d+)-(\d+)$"#)
                .unwrap();
        }
        let cap = RULE.captures(s).unwrap();
        let name = cap.get(1).unwrap().as_str();
        let min1 = cap.get(2).unwrap().as_str().parse().unwrap();
        let max1 = cap.get(3).unwrap().as_str().parse().unwrap();
        let min2 = cap.get(4).unwrap().as_str().parse().unwrap();
        let max2 = cap.get(5).unwrap().as_str().parse().unwrap();

        Rule { name, bounds: [(min1, max1), (min2, max2)] }
    }

    fn check(&self, i: u64) -> bool {
        (i >= self.bounds[0].0 && i <= self.bounds[0].1) ||
        (i >= self.bounds[1].0 && i <= self.bounds[1].1)
    }
}


fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split("\n\n");
    let rules = iter.next().unwrap().lines()
        .map(|s| Rule::parse(s))
        .collect::<Vec<_>>();

    let my_ticket = iter.next().unwrap().lines().skip(1)
        .next().unwrap()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<u64>>();

    let others = iter.next().unwrap().lines().skip(1)
        .map(|s| s.split(',')
            .map(|i| i.parse().unwrap())
            .collect::<Vec<u64>>())
        .collect::<Vec<_>>();

    let mut err_rate = 0;
    let out: Vec<_> = others.into_iter().filter(|o| {
        let mut valid = true;
        for k in o.iter() {
            if !rules.iter().any(|r| r.check(*k)) {
                err_rate += k;
                valid = false;
            }
        }
        valid
    }).collect();

    println!("Part 1: {}", err_rate);

    // This is an array of which rules are possible given the inputs,
    // indexed as possible[rule][item]
    let mut possible = vec![vec![true; out[0].len()]; rules.len()];

    for o in out.iter() {
        println!("Testing {:?}", o);
        for (i, k) in o.iter().enumerate() {
            for (j, r) in rules.iter().enumerate() {
                possible[j][i] &= r.check(*k);
            }
        }
    }
    // Now that we've built the matrix, loop through and assign rules
    let mut decoded = Vec::new();
    loop {
        let mut t = None;
        for (j, p) in possible.iter().enumerate() {
            if p.iter().filter(|b| **b).count() == 1 {
                let i = p.iter().enumerate().find(|b| *b.1).unwrap().0;
                t = Some((j, i))
            }
        }
        match t {
            Some(t) => {
                decoded.push(t);
                for r in possible.iter_mut() {
                    r[t.1] = false;
                }
            },
            _ => break,
        }
    }

    let mut out = 1;
    for (r,k) in decoded {
        if rules[r].name.starts_with("departure") {
            out *= my_ticket[k];
        }
    }
    println!("Part 2: {}", out);
}
