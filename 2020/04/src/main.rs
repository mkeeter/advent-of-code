use std::io::Read;
use std::collections::HashSet;

use regex::Regex;
use lazy_static::lazy_static;

fn check(c: &str) -> bool {
    lazy_static! {
        static ref HCL: Regex = Regex::new("^#[a-f0-9]{6}$").unwrap();
        static ref ECL: HashSet<&'static str> = "amb blu brn gry grn hzl oth"
            .split(' ').collect();
        static ref PID: Regex = Regex::new("^[0-9]{9}$").unwrap();
        static ref HGT: Regex = Regex::new("^([0-9]+)(cm|in)$").unwrap();
    }

    let mut itr = c.splitn(2, ':');
    let key = itr.next().unwrap();
    let val = itr.next().unwrap();

    match key {
        "byr" => val.parse().map_or(false, |v: i32| (1920..=2002).contains(&v)),
        "iyr" => val.parse().map_or(false, |v: i32| (2010..=2020).contains(&v)),
        "eyr" => val.parse().map_or(false, |v: i32| (2020..=2030).contains(&v)),
        "hgt" => HGT.captures(val).map_or(false, |c| {
            let h: i32 = c.get(1).unwrap().as_str().parse().unwrap();
            let unit = c.get(2).unwrap().as_str();
            match unit {
                "in" => (59..=76).contains(&h),
                "cm" => (150..=193).contains(&h),
                _ => panic!("Invalid height unit {}", unit),
            }
        }),
        "hcl" => HCL.is_match(val),
        "ecl" => ECL.contains(val),
        "pid" => PID.is_match(val),
        "cid" => true,
        _ => panic!("unknown key {}", key),
    }
}

fn valid1(s: &str) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|i| s.contains(i))
}

fn valid2(s: &str) -> bool {
    valid1(s) && s.split_whitespace().all(check)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let lines: Vec<&str> = input.split("\n\n").collect();
    let p1 = lines.iter().filter(|s| valid1(*s)).count();
    println!("Part 1: {}", p1);

    let p2 = lines.iter().filter(|s| valid2(*s)).count();
    println!("Part 2: {}", p2);
}
