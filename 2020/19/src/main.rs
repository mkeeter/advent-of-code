use std::io::Read;
use std::str::FromStr;

use smallvec::{smallvec, SmallVec};

#[derive(Debug)]
enum Rule {
    Char(char),
    Rule(usize),
    Alt(Vec<Rule>),
    Chain(Vec<Rule>),
}

impl Rule {
    fn check(&self, s: &str, rules: &[Rule]) -> bool {
        self.check_(s, &rules).into_iter().any(|v| v.is_empty())
    }

    // Returns possible postfixes based on matching on s
    fn check_<'a>(&self, s: &'a str, rules: &[Rule]) -> SmallVec<[&'a str; 1]> {
        match self {
            Rule::Char(c) => {
                if s.starts_with(*c) {
                    smallvec![&s[1..]]
                } else {
                    smallvec![]
                }
            }
            Rule::Rule(i) => rules[*i].check_(s, rules),
            Rule::Alt(alt) => alt
                .iter()
                .flat_map(|a| a.check_(s, rules).into_iter())
                .collect(),
            Rule::Chain(cs) => cs.iter().fold(smallvec![s], |out, r| {
                out.into_iter()
                    .flat_map(|a| r.check_(a, rules).into_iter())
                    .collect()
            }),
        }
    }
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut iter = s.split(": ");
        iter.next().unwrap(); // Skip number, since we sorted first

        // Grab the text of the rule
        let r = iter.next().unwrap();

        // Detect a character because it's always wrapped in quotes
        let mut chars = r.chars();
        if chars.next().unwrap() == '"' {
            return Ok(Rule::Char(chars.next().unwrap()));
        }

        let alts = r
            .split(" | ")
            .map(|sub| {
                let rs = sub
                    .split(' ')
                    .map(|i| Rule::Rule(i.parse().unwrap()))
                    .collect::<Vec<Rule>>();
                if rs.len() == 1 {
                    rs.into_iter().next().unwrap()
                } else {
                    Rule::Chain(rs)
                }
            })
            .collect::<Vec<Rule>>();

        if alts.len() == 1 {
            Ok(alts.into_iter().next().unwrap())
        } else {
            Ok(Rule::Alt(alts))
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split("\n\n");
    let mut rules: Vec<&str> = iter.next().unwrap().lines().collect();
    rules.sort_by_key(|line| line.split(':').next().unwrap().parse::<usize>().unwrap());

    let rules: Vec<Rule> = rules.into_iter().map(|r| r.parse().unwrap()).collect();

    let lines: Vec<_> = iter.next().unwrap().lines().collect();
    let matched = lines
        .iter()
        .filter(|line| rules[0].check(line, &rules))
        .count();
    println!("Part 1: {}", matched);

    let mut rules = rules;
    rules[8] = Rule::Alt(vec![
        Rule::Rule(42),
        Rule::Chain(vec![Rule::Rule(42), Rule::Rule(8)]),
    ]);
    rules[11] = Rule::Alt(vec![
        Rule::Chain(vec![Rule::Rule(42), Rule::Rule(31)]),
        Rule::Chain(vec![Rule::Rule(42), Rule::Rule(11), Rule::Rule(31)]),
    ]);
    let matched = lines
        .iter()
        .filter(|line| rules[0].check(line, &rules))
        .count();
    println!("Part 2: {}", matched);
}
