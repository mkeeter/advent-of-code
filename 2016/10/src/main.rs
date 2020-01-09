use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum Target {
    Bot(usize),
    Output(usize),
}

fn main() {
    let mut bots: HashMap<usize, HashSet<usize>> = HashMap::new();
    let cmds = std::io::stdin().lock().lines()
        .filter_map(|line| {
            let words =  line.unwrap()
                .split(' ')
                .map(|w| w.to_string())
                .collect::<Vec<String>>();
            let nums = words.iter()
                .filter_map(|i| usize::from_str(i).ok())
                .collect::<Vec<usize>>();
            if nums.len() == 2 {
                bots.entry(nums[1])
                    .or_default()
                    .insert(nums[0]);
                None
            } else {
                let min_target = if words[5] == "bot" {
                    Target::Bot(nums[1])
                } else {
                    Target::Output(nums[1])
                };
                let max_target = if words[10] == "bot" {
                    Target::Bot(nums[2])
                } else {
                    Target::Output(nums[2])
                };
                Some((nums[0], (min_target, max_target)))
            }
        })
        .collect::<HashMap<usize, (Target, Target)>>();

    let mut outputs = HashMap::new();
    loop {
        let active_bots = bots.iter()
            .filter(|b| b.1.len() >= 2)
            .map(|b| *b.0)
            .collect::<Vec<usize>>();
        for b in active_bots {
            assert!(bots[&b].len() == 2);
            let min_chip = *bots[&b].iter().min().unwrap();
            let max_chip = *bots[&b].iter().max().unwrap();

            if min_chip == 17 && max_chip == 61 {
                println!("Part 1: {}", b);
            }

            match cmds[&b].0 {
                Target::Bot(min_bot) => bots.entry(min_bot),
                Target::Output(min_out) => outputs.entry(min_out),
            }.or_default().insert(min_chip);

            match cmds[&b].1 {
                Target::Bot(max_bot) => bots.entry(max_bot),
                Target::Output(max_out) => outputs.entry(max_out),
            }.or_default().insert(max_chip);

            bots.remove(&b);
        }

        let o = (0..=2).map(|i| outputs.get(&i)).collect::<Vec<_>>();
        if o.iter().all(|i| i.is_some()) {
            let p: usize = o.iter()
                .flat_map(|i| i.unwrap().iter().next())
                .product();
            println!("Part 2: {}", p);
            break;
        }
    }
}
