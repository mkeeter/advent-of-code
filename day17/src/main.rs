use std::collections::HashMap;
use std::io::{self, Read};
use std::fmt;

use regex::Regex;

type Pt = (i64, i64);

struct State {
    grid: HashMap<Pt, char>,
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
}

impl State {
    fn from_str(s: &String) -> State {
        let mut out = HashMap::new();
        let re = Regex::new(r"(\d+)").unwrap();
        for line in s.lines() {
            let nums = re.captures_iter(line)
              .filter_map(|i| str::parse::<i64>(&i[1]).ok())
              .collect::<Vec<i64>>();
            assert!(nums.len() == 3);
            if line.chars().next().unwrap() == 'x' {
                for y in nums[1]..=nums[2] {
                    out.insert((nums[0], y), '#');
                }
            } else {
                for x in nums[1]..=nums[2] {
                    out.insert((x, nums[0]), '#');
                }
            }
        }
        out.insert((500, 0), '+');

        let pts = out.iter()
            .filter(|(_, c)| **c == '#' || **c == '+')
            .map(|(pt, _)| pt)
            .cloned()
            .collect::<Vec<Pt>>();

        State {
            grid: out,
            xmin: *pts.iter().map(|(x, _)| x).min().unwrap(),
            xmax: *pts.iter().map(|(x, _)| x).max().unwrap(),
            ymin: *pts.iter().map(|(_, y)| y).min().unwrap(),
            ymax: *pts.iter().map(|(_, y)| y).max().unwrap(),
        }
    }

    fn undrip(&mut self) {
        self.grid.retain(|_, v| *v != '|');
    }

    fn drip(&mut self, mut pt: Pt) -> bool {
        loop {
            pt.1 += 1;
            if pt.1 == self.ymax + 1 {
                return false;
            }
            match self.grid.get(&pt).cloned().unwrap_or('.') {
                '|' => return false,
                '#' | '~' => break,
                '.' => { self.grid.insert(pt, '|'); }
                 _  => unreachable!(),
            }
        }
        pt.1 -= 1; // Back off to the unoccupied space

        let mut capped = true;
        let mut out = false;
        for dx in [-1, 1].iter() {
            for i in 1.. {
                let x = pt.0 + (i * dx) as i64;

                // Try to spread horizontally
                match self.grid.get(&(x, pt.1)).cloned().unwrap_or('.') {
                    '#' => break,
                    '|' => { capped = false; break; },
                    '.' => { self.grid.insert((x, pt.1), '|'); }
                     _  => unreachable!(),
                }

                // Check whether we can fall vertically
                match self.grid.get(&(x, pt.1 + 1)).cloned().unwrap_or('.') {
                    '#' | '~' => continue,
                    '.' => { capped = false; out |= self.drip((x, pt.1)); break; },
                     _  => unreachable!(),
                }
            }
        }

        if capped {
            for dx in [-1, 1].iter() {
                self.grid.insert(pt, '~');
                for i in 1.. {
                    let x = pt.0 + (i * dx) as i64;
                    // Try to spread horizontally
                    match self.grid.get(&(x, pt.1)).cloned().unwrap_or('.') {
                        '#' => break,
                        '|' | '.' => { self.grid.insert((x, pt.1), '~'); }
                         _  => unreachable!(),
                    }
                }
            }
            return true;
        } else {
            return out;
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.ymin..=self.ymax {
            for x in self.xmin-1..=self.xmax+1 {
                write!(f, "{}", self.grid.get(&(x, y)).unwrap_or(&'.'))?;
            }
            if y != self.ymax {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut s = State::from_str(&buffer);

    while s.drip((500, 0)) {
        s.undrip();
    }

    println!("{} {}", s.ymin, s.ymax);
    let ymin = *s.grid.iter()
        .filter(|(_, v)| **v == '#')
        .map(|((_, y), _)| y)
        .min()
        .unwrap();

    println!("Flowing can reach {} tiles", s.grid.iter()
             .filter(|((_, y), v)| *y >= ymin && (**v == '~' || **v == '|'))
             .count());
    println!("At-rest water can reach {} tiles", s.grid.iter()
             .filter(|((_, y), v)| *y >= ymin && (**v == '~'))
             .count());
}
