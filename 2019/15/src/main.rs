use std::io::Read;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use vm::Vm;

struct Robot {
    pos: (i64, i64),
    history: Vec<i64>,
    vm: Vm,
}

fn _draw(seen: &HashMap<(i64, i64), i64>) {
    let xmin = seen.keys().map(|i| i.0).min().unwrap();
    let xmax = seen.keys().map(|i| i.0).max().unwrap();
    let ymin = seen.keys().map(|i| i.1).min().unwrap();
    let ymax = seen.keys().map(|i| i.1).max().unwrap();
    for y in (ymin..=ymax).rev() {
        for x in xmin..=xmax {
            match seen.get(&(x, y)).unwrap_or(&0) {
                0 => print!("██"),
                1 => print!("  "),
                2 => print!("░░"),
                _ => panic!("Invalid tile"),
            }
        }
        println!();
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let vm = Vm::from_str(&input).unwrap();

    let mut todo = Vec::new();
    todo.push(Robot { pos: (0, 0), history: vec![], vm });

    let mut seen = HashMap::new();
    while let Some(bot) = todo.pop()
    {
        for (cmd, delta) in [(1, ( 0,  1)),
                             (2, ( 0, -1)),
                             (3, (-1,  0)),
                             (4, ( 1,  0))].iter()
        {
            let pos = (bot.pos.0 + delta.0, bot.pos.1 + delta.1);
            if !seen.contains_key(&pos) {
                let mut next = Robot {
                    pos,
                    history: bot.history.clone(),
                    vm: bot.vm.clone(),
                };
                next.history.push(*cmd);
                next.vm.input(*cmd);
                if let Some(o) = next.vm.run_until() {
                    seen.insert(pos, o);
                    match o {
                        0 => (),
                        1 => todo.push(next),
                        2 => println!("Part 1: {}", next.history.len()),
                        _ => panic!("Invalid output"),
                    };
                }
            }
        }
    }

    let source = *seen.iter().find(|i| *i.1 == 2).unwrap().0;
    let mut next: HashSet<(i64, i64)> = HashSet::new();
    next.insert(source);

    let mut minutes = 0;
    while !next.is_empty() {
        next = next.iter()
            .flat_map(|i| [(0, 1), (0, -1), (-1, 0), (1, 0)]
                    .iter()
                    .map(move |d| (i.0 + d.0, i.1 + d.1)))
            .filter(|j| seen.get(j).unwrap_or(&0) == &1)
            .collect::<HashSet<(i64, i64)>>();
        for n in next.iter() {
            seen.insert(*n, 2);
        }
        minutes += 1;
    }
    println!("Part 2: {}", minutes - 1);
}
