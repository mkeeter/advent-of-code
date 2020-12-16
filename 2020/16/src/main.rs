use std::io::Read;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

struct Rule {
    name: String,
    bounds: [(u64, u64); 2],
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        lazy_static! {
            static ref RULE: Regex = Regex::new(
                r#"^(.*): (\d+)-(\d+) or (\d+)-(\d+)$"#)
                .unwrap();
        }
        let cap = RULE.captures(s).ok_or(())?;
        let name = cap.get(1).ok_or(())?.as_str().to_owned();
        let min1 = cap.get(2).ok_or(())?.as_str().parse().map_err(|_| ())?;
        let max1 = cap.get(3).ok_or(())?.as_str().parse().map_err(|_| ())?;
        let min2 = cap.get(4).ok_or(())?.as_str().parse().map_err(|_| ())?;
        let max2 = cap.get(5).ok_or(())?.as_str().parse().map_err(|_| ())?;

        Ok(Rule { name, bounds: [(min1, max1), (min2, max2)] })
    }
}

impl Rule {
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
        .map(|s| s.parse().unwrap())
        .collect::<Vec<Rule>>();

    let my_ticket = iter.next().unwrap().lines().nth(1).unwrap()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<u64>>();

    let tickets = iter.next().unwrap().lines().skip(1)
        .map(|s| s.split(',')
            .map(|i| i.parse().unwrap())
            .collect::<Vec<u64>>())
        .collect::<Vec<_>>();

    let mut err_rate = 0;
    let tickets: Vec<_> = tickets.into_iter().filter(|ticket| {
        let err: u64 = ticket.iter()
            .filter(|k| !rules.iter().any(|r| r.check(**k)))
            .sum();
        err_rate += err;
        err == 0
    }).collect();
    println!("Part 1: {}", err_rate);

    // This is an array of which rules are possible given the inputs,
    // indexed as possible[rule][item]
    let n = tickets[0].len();
    let mut possible: Vec<Vec<bool>> = rules.iter().map(
            |rule| (0..n).map(
                |j| tickets.iter().all(|ticket| rule.check(ticket[j]))
        ).collect()
    ).collect();

    // Now that we've built the matrix, loop through and assign rules.  At
    // any given time, at least one rule must be available to assign, so
    // we assign it then clear its "possible" flag in all other rows.
    let step = || (0..possible.len()).find_map(|j| {
        let mut iter = possible[j].iter().enumerate().filter(|b| *b.1);
        if let Some((i, _)) = iter.next() {
            if iter.next() == None {
                possible.iter_mut().for_each(|p| p[i] = false);
                if rules[j].name.starts_with("departure") {
                    return Some(my_ticket[i]);
                } else {
                    return Some(1);
                }
            }
        }
        None
    });
    println!("Part 2: {}", std::iter::from_fn(step).product::<u64>());
}
