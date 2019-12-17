use std::io::Read;
use std::collections::HashMap;
use vm::Vm;

fn optimize(commands: Vec<String>, c: i32) -> Option<Vec<Vec<String>>> {
    if c == 0 {
        let len = commands.iter()
            .map(|i| i.len() + 1)
            .sum::<usize>() - 1;
        if len <= 20 {
            return Some(vec![commands])
        } else {
            return None;
        }
    }

    for k in (2..commands.len()).rev() {
        for i in 0..(commands.len() - k) {
            let s = &commands[i..i+k];
            if s.iter()
                .map(|i| i.len() + 1)
                .sum::<usize>() - 1 > 20
            {
                continue;
            }
            if s.iter()
                .flat_map(|s| s.chars())
                .any(|c| !char::is_numeric(c) && c != 'L' && c != 'R')
            {
                continue;
            }

            // Build a reduced command tape that uses these commands
            let mut new_commands = Vec::new();
            let mut j = 0;
            let sub = ('A' as u8 + (c - 1) as u8) as char;
            while j < commands.len() {
                if j + k <= commands.len() && s == &commands[j..(j+k)] {
                    new_commands.push(sub.to_string());
                    j += k;
                } else {
                    new_commands.push(commands[j].clone());
                    j += 1;
                }
            }

            if let Some(vs) = optimize(new_commands, c - 1) {
                let mut vs = vs.clone();
                vs.push(s.to_vec());
                return Some(vs);
            }
        }
    }
    None
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut vm = Vm::from_str(&input);

    let mut tiles = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut i = 0;
    while i < 10000 { // Solve the halting problem
        if let Some(o) = vm.step() {
            i = 0;
            let c = o as u8 as char;
            if c == '\n' {
                x = 0;
                y += 1;
            } else {
                tiles.insert((x, y), c);
                x += 1;
            }
            print!("{}", c);
        } else {
            i += 1;
        }
    }

    let tile = |pos: &(i32, i32)| -> char {
        *tiles.get(pos).unwrap_or(&'.')
    };

    let alignment = tiles.iter()
        .filter(|((x, y), v)| **v == '#' &&
                              tile(&(x + 1, *y)) == '#' &&
                              tile(&(x - 1, *y)) == '#' &&
                              tile(&(*x, y + 1)) == '#' &&
                              tile(&(*x, y - 1)) == '#')
        .map(|((x, y), _v)| x * y)
        .sum::<i32>();

    println!("Part 1: {}", alignment);

    // Get bot location and orientation
    let bot = tiles.iter()
        .filter(|(_k, v)| **v != '.' && **v != '#')
        .next()
        .unwrap();
    let mut pos: (i32, i32) = *bot.0;
    let mut dir = match *bot.1 {
        '^' => ( 0, -1),
        'v' => ( 0,  1),
        '>' => ( 1,  0),
        '<' => (-1,  0),
        _ => panic!("Invalid bot character {}", bot.1),
    };

    // Figure out how to walk through the maze
    let mut commands = Vec::new();
    let mut distance = 0;
    loop {
        let n = (pos.0 + dir.0, pos.1 + dir.1);
        if tile(&n) != '#' {
            if distance != 0 {
                commands.push(distance.to_string());
                distance = 0;
            }

            let left  = ( dir.1, -dir.0);
            let right = (-dir.1,  dir.0);
            if tile(&(pos.0 + left.0, pos.1 + left.1)) == '#' {
                commands.push("L".to_string());
                dir = left;
            } else if tile(&(pos.0 + right.0, pos.1 + right.1)) == '#' {
                commands.push("R".to_string());
                dir = right;
            } else {
                break;
            }
        }
        distance += 1;
        pos.0 += dir.0;
        pos.1 += dir.1;
    }

    let mut vm = Vm::from_str(&input);
    vm.poke(0, 2);

    let cmds = optimize(commands, 3).unwrap();
    for cmd in cmds {
        for (i,word) in cmd.iter().enumerate() {
            for c in word.chars() {
                vm.input(c as i64);
            }
            if i == cmd.len() - 1 {
                vm.input('\n' as i64);
            } else {
                vm.input(',' as i64);
            }
        }
    }
    vm.input('n' as i64);
    vm.input('\n' as i64);

    while let Some(o) = vm.run_until() {
        if o > 255 {
            println!("Part 2: {}", o);
        } else {
            print!("{}", o as u8 as char);
        }
    }
}
