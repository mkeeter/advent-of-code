use std::collections::HashSet;

fn part1(constraints: &Vec<(char, HashSet<char>)>) {
    let step = |state: &String| {
        let seen = state.chars().collect::<HashSet<char>>();
        constraints
            .iter()
            .filter(|(a, _)| !seen.contains(a))
            .filter(|(_, b)| b.difference(&seen).count() == 0)
            .map(|(a, _)| a.clone())
            .min()
    };

    let mut state = "".to_string();
    while let Some(c) = step(&state) {
        state.push(c);
    }
    println!("Part 1: {:?}", state);
}

fn part2(constraints: &Vec<(char, HashSet<char>)>, workers: usize, delay: usize) {
    let available = |state: &String| {
        let seen = state.chars().collect::<HashSet<char>>();
        let mut out: Vec<char> = constraints
            .iter()
            .filter(|(a, _)| !seen.contains(a))
            .filter(|(_, b)| b.difference(&seen).count() == 0)
            .map(|(a, _)| a.clone())
            .collect();
        out.sort();
        out.reverse();
        out
    };

    let mut time = 0;
    let mut state = "".to_string();
    let mut tasks = vec![None; workers];
    loop {
        // Update in-progress tasks
        for v in tasks.iter_mut() {
            if let Some((target, t)) = v {
                *t -= 1;
                if *t == 0 {
                    state.push(*target);
                    *v = None;
                }
            }
        }

        // Record which tasks are being worked on
        let working_on = tasks
            .iter()
            .filter_map(|t| *t)
            .map(|(target, _)| target)
            .collect::<HashSet<char>>();

        // Find available targets
        let mut targets = available(&state)
            .into_iter()
            .filter(|t| !working_on.contains(t))
            .collect::<Vec<char>>();

        if targets.len() == 0 && working_on.len() == 0 {
            break;
        }

        // Start new tasks
        for v in tasks.iter_mut() {
            if v.is_none() && !targets.is_empty() {
                let c = *targets.last().unwrap();
                targets.pop();
                *v = Some((c, delay + (c as usize - 'A' as usize) + 1));
            }
        }
        time += 1;
    }
    println!("Part 2: got time {}", time);
}

fn main() {
    let constraints = include_str!("../input")
        .lines()
        .map(|p| p.split(' '))
        .map(|itr| itr.skip(1))
        .map(|mut itr| (itr.next().unwrap(), itr.skip(5).next().unwrap()))
        .map(|(a, b)| (a.chars().next().unwrap(), b.chars().next().unwrap()))
        .collect::<Vec<(char, char)>>();

    let prereqs = constraints
        .iter()
        .map(|p| p.0.clone())
        .collect::<HashSet<char>>();
    let tasks = constraints
        .iter()
        .map(|p| p.1.clone())
        .collect::<HashSet<char>>();
    let constraints = tasks
        .union(&prereqs)
        .map(|t| {
            (
                *t,
                constraints
                    .iter()
                    .filter(|c| c.1 == *t)
                    .map(|c| c.0)
                    .collect::<HashSet<char>>(),
            )
        })
        .collect::<Vec<(char, HashSet<char>)>>();

    part1(&constraints);
    part2(&constraints, 5, 60);
}
