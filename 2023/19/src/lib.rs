use anyhow::bail;
use parse_display::FromStr;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, FromStr, Eq, PartialEq)]
#[display("{{x={x},m={m},a={a},s={s}}}")]
struct GenericPart<T> {
    x: T,
    m: T,
    a: T,
    s: T,
}

impl<T: Clone> GenericPart<T> {
    fn new(t: T) -> Self {
        Self {
            x: t.clone(),
            m: t.clone(),
            a: t.clone(),
            s: t.clone(),
        }
    }
}

type Part = GenericPart<u64>;
type RangePart = GenericPart<std::ops::Range<u64>>;

impl RangePart {
    fn empty() -> Self {
        Self::new(0..0)
    }
    fn is_empty(&self) -> bool {
        self.x.is_empty()
            && self.m.is_empty()
            && self.a.is_empty()
            && self.s.is_empty()
    }
}

impl<T> std::ops::Index<char> for GenericPart<T> {
    type Output = T;
    fn index(&self, c: char) -> &Self::Output {
        match c {
            'x' => &self.x,
            'm' => &self.m,
            'a' => &self.a,
            's' => &self.s,
            c => panic!("{c} is not part of the christmas spirit"),
        }
    }
}

impl<T> std::ops::IndexMut<char> for GenericPart<T> {
    fn index_mut(&mut self, c: char) -> &mut Self::Output {
        match c {
            'x' => &mut self.x,
            'm' => &mut self.m,
            'a' => &mut self.a,
            's' => &mut self.s,
            c => panic!("{c} is not part of the christmas spirit"),
        }
    }
}

#[derive(Debug)]
enum Condition {
    LessThan(char, u64),
    GreaterThan(char, u64),
    Always,
}

impl Condition {
    fn check(&self, part: &Part) -> bool {
        match self {
            Condition::LessThan(c, v) => part[*c] < *v,
            Condition::GreaterThan(c, v) => part[*c] > *v,
            Condition::Always => true,
        }
    }
    /// Returns a tuple of `(accepted parts, rejected parts)`
    fn check_range(&self, part: &RangePart) -> (RangePart, RangePart) {
        match self {
            Condition::LessThan(c, v) => {
                let p = &part[*c];
                let mut a = part.clone();
                let mut b = part.clone();
                a[*c] = p.start..*v;
                b[*c] = *v..p.end;
                (a, b)
            }
            Condition::GreaterThan(c, v) => {
                let p = &part[*c];
                let mut a = part.clone();
                let mut b = part.clone();
                a[*c] = *v + 1..p.end;
                b[*c] = p.start..*v + 1;
                (a, b)
            }
            Condition::Always => (part.clone(), RangePart::empty()),
        }
    }
}

impl std::str::FromStr for Condition {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('<') {
            let mut iter = s.split('<');
            let cond = iter.next().unwrap();
            assert_eq!(cond.len(), 1);

            let count = iter.next().unwrap();
            let i = count.parse().unwrap();
            Ok(Condition::LessThan(cond.chars().next().unwrap(), i))
        } else if s.contains('>') {
            let mut iter = s.split('>');
            let cond = iter.next().unwrap();
            assert_eq!(cond.len(), 1);

            let count = iter.next().unwrap();
            let i = count.parse().unwrap();
            Ok(Condition::GreaterThan(cond.chars().next().unwrap(), i))
        } else {
            bail!("invalid condition: {s}");
        }
    }
}

#[derive(Debug)]
struct Rule {
    cond: Condition,
    dest: String,
}

impl Rule {
    fn check(&self, part: &Part) -> Option<&str> {
        if self.cond.check(part) {
            Some(self.dest.as_str())
        } else {
            None
        }
    }
}

impl std::str::FromStr for Rule {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(':') {
            let mut iter = s.split(':');
            let cond = iter.next().unwrap().parse().unwrap();
            let dest = iter.next().unwrap().to_owned();
            Ok(Rule { cond, dest })
        } else {
            Ok(Rule {
                cond: Condition::Always,
                dest: s.to_owned(),
            })
        }
    }
}

#[derive(Debug)]
struct Workflow(Vec<Rule>);

impl Workflow {
    fn next(&self, part: &Part) -> &str {
        for r in &self.0 {
            if let Some(n) = r.check(part) {
                return n;
            }
        }
        panic!("part {part:?} is not accepted by workflow {self:?}");
    }
}

pub fn solve(s: &str) -> (String, String) {
    let mut workflows = HashMap::new();
    let mut iter = s.lines();
    for line in &mut iter {
        if line.is_empty() {
            break;
        } else {
            let mut iter = line.split('{');
            let label = iter.next().unwrap();
            let wf = iter.next().unwrap().strip_suffix('}').unwrap();
            let rules = wf.split(',').map(|r| r.parse().unwrap()).collect();
            workflows.insert(label, Workflow(rules));
        }
    }
    let mut parts: Vec<Part> = vec![];
    for line in &mut iter {
        parts.push(line.parse().unwrap());
    }

    let mut accepted = vec![];
    for p in &parts {
        let mut w = "in";
        loop {
            w = workflows[&w].next(p);
            match w {
                "A" => {
                    accepted.push(*p);
                    break;
                }
                "R" => break,
                _ => (),
            }
        }
    }

    let p1 = accepted
        .iter()
        .flat_map(|a| "xmas".chars().map(|c| a[c]))
        .sum::<u64>();

    let mut accepted = vec![];
    let mut parts = vec![("in", RangePart::new(1..4001))];
    while let Some((w, mut p)) = parts.pop() {
        for rule in workflows[&w].0.iter() {
            let (acc, rej) = rule.cond.check_range(&p);
            if !acc.is_empty() {
                match rule.dest.as_str() {
                    "A" => accepted.push(acc),
                    "R" => (),
                    d => parts.push((d, acc)),
                }
            }
            p = rej;
            if p.is_empty() {
                break;
            }
        }
        assert!(p.is_empty());
    }
    let p2 = accepted
        .iter()
        .map(|a| {
            "xmas"
                .chars()
                .map(|c| a[c].clone().count())
                .product::<usize>()
        })
        .sum::<usize>();
    (p1.to_string(), p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let s = indoc::indoc! {"
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}
        "};
        let (p1, p2) = solve(s);
        assert_eq!(p1, "19114");
        assert_eq!(p2, "167409079868000");
    }
}
