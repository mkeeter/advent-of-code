use std::io::Read;
use std::collections::HashMap;
use vm::Vm;

fn optimize(commands: Vec<String>, c: char) -> (Vec<String>, Vec<String>) {
    let mut best_score = 0;
    let mut best_start = 0;
    let mut best_len = 0;
    for k in 1..commands.len() {
        for i in 0..(commands.len() - k) {
            if commands[i..i+k].iter()
                .map(|i| i.len() + 1)
                .sum::<usize>() - 1 > 20
            {
                continue;
            }
            if commands[i..i+k].iter()
                .flat_map(|s| s.chars())
                .any(|c| !char::is_numeric(c) && c != 'L' && c != 'R')
            {
                continue;
            }

            let mut score = k - 1;
            for j in (i + k)..(commands.len() - k) {
                if commands[i..i+k] == commands[j..j+k] {
                    score += k - 1;
                }
            }
            if score > best_score {
                best_score = score;
                best_start = i;
                best_len = k;
            }
        }
    }

    let mut new_commands = Vec::new();
    let mut i = 0;
    let s = &commands[best_start..(best_start + best_len)];
    while i < commands.len() {
        if i + best_len <= commands.len() && s == &commands[i..(i+best_len)] {
            new_commands.push(c.to_string());
            i += best_len;
        } else {
            new_commands.push(commands[i].clone());
            i += 1;
        }
    }
    (new_commands, s.to_vec())
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

    let alignment = tiles.iter()
        .filter(|(_k, v)| **v == '#')
        .filter(|((x, y), _v)| *tiles.get(&(x + 1, *y)).unwrap_or(&'.') == '#')
        .filter(|((x, y), _v)| *tiles.get(&(x - 1, *y)).unwrap_or(&'.') == '#')
        .filter(|((x, y), _v)| *tiles.get(&(*x, y + 1)).unwrap_or(&'.') == '#')
        .filter(|((x, y), _v)| *tiles.get(&(*x, y - 1)).unwrap_or(&'.') == '#')
        .map(|((x, y), _v)| x * y)
        .sum::<i32>();

    println!("Part 1: {}", alignment);

    let xmax = tiles.keys().map(|p| p.0).max().unwrap();
    let ymax = tiles.keys().map(|p| p.1).max().unwrap();

    // Get bot location and details
    let bot = tiles.iter().filter(|(k, v)| **v != '.' && **v != '#').next().unwrap();
    let mut pos: (i32, i32) = *bot.0;
    let mut dir = match *bot.1 {
        '^' => ( 0, -1),
        'v' => ( 0,  1),
        '>' => ( 1,  0),
        '<' => (-1,  0),
        _ => panic!("Invalid bot character {}", bot.1),
    };

    let tile = |pos: &(i32, i32)| -> char {
        *tiles.get(pos).unwrap_or(&'.')
    };

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

        /*
        for y in 0..=ymax {
            for x in 0..=xmax {
                if (x, y) == pos {
                    print!("{}", match dir {
                        ( 0, -1) => '^',
                        ( 0,  1) => 'v',
                        ( 1,  0) => '>',
                        (-1,  0) => '<',
                        _ => 'X',
                    });
                } else {
                    print!("{}", tile(&(x, y)));
                }
            }
            print!("\n");
        }
        */
    }

    let (commands, a) = optimize(commands, 'A');
    let (commands, b) = optimize(commands, 'B');
    let (commands, c) = optimize(commands, 'C');
    println!("commands: {:?}", commands);
    println!("A: {:?}", a);
    println!("B: {:?}", b);
    println!("C: {:?}", c);

    let mut vm = Vm::from_str(&input);
    vm.poke(0, 2);
    /*
    for a in "A\n".chars() {
        vm.input(a as i64);
    }
    for a in "R,4,R,5\n".chars() {
        vm.input(a as i64);
    }
    for a in "L\n".chars() {
        vm.input(a as i64);
    }
    for a in "L\n".chars() {
        vm.input(a as i64);
    }
    for y in "y\n".chars() {
        vm.input(y as i64);
    }

    let mut x = 0;
    let mut y = 0;
    let mut i = 0;
    loop { // Solve the halting problem
        if let Some(o) = vm.step() {
            let c = o as u8 as char;
            if c == '\n' {
                x = 0;
                y += 1;
                if y == ymax {
                    println!("\n\n\n");
                    y = 0;
                    i += 1;
                    if i == 10 {
                        break;
                    }
                }
            } else {
                tiles.insert((x, y), c);
                x += 1;
            }
            print!("{}", c);
        }
    }
    */
}
