use std::io::Read;

use lazy_static::lazy_static;
use regex::Regex;

fn parse_rule(s: &str) -> (&str, [(u32, u32); 2]) {
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

    (name, [(min1, max1), (min2, max2)])
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split("\n\n");
    let rules = iter.next().unwrap().lines()
        .map(|s| parse_rule(s))
        .collect::<Vec<_>>();

    let my_ticket = iter.next().unwrap().lines().skip(1)
        .next().unwrap()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<u32>>();

    let others = iter.next().unwrap().lines().skip(1)
        .map(|s| s.split(',')
            .map(|i| i.parse().unwrap())
            .collect::<Vec<u32>>())
        .collect::<Vec<_>>();

    let mut out = Vec::new();
    let mut err_rate = 0;
    for o in others.into_iter() {
        let mut valid = true;
        for i in o.iter() {
            let mut matched = false;
            for r in rules.iter() {
                if (*i >= r.1[0].0 && *i <= r.1[0].1) ||
                   (*i >= r.1[1].0 && *i <= r.1[1].1)
                {
                    matched = true;
                    break;
                }
            }
            if !matched {
                err_rate += i;
                valid = false;
                break;
            }
        }
        if valid {
            out.push(o);
        }
    }

    println!("{}", err_rate);
}
