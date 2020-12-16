use std::io::Read;

use lazy_static::lazy_static;
use regex::Regex;

struct Rule<'a> {
    name: &'a str,
    bounds: [(u32, u32); 2],
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

    fn check(&self, i: u32) -> bool {
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
            if !rules.iter().any(|r| r.check(*i)) {
                err_rate += i;
                valid = false;
            }
        }
        if valid {
            out.push(o);
        }
    }

    println!("{}", err_rate);
}
