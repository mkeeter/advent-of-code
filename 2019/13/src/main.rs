use std::io::{Read, Write};
use vm::Vm;
use std::collections::HashMap;

fn run_until(vm: &mut Vm) -> Option<i64> {
    while vm.running() {
        assert!(!vm.needs_input());
        if let Some(i) = vm.step() {
            return Some(i);
        }
    }
    None
}

fn get_pixel(vm: &mut Vm) -> Option<(i64, i64, i64)> {
    if let Some(x) = run_until(vm) {
        Some((x, run_until(vm).unwrap(),
                 run_until(vm).unwrap()))
    } else {
        None
    }
}

fn run_until_with(vm: &mut Vm, input: i64) -> Option<i64> {
    while vm.running() {
        if vm.needs_input() {
            vm.input(input);
        }
        if let Some(i) = vm.step() {
            return Some(i);
        }
    }
    None
}

fn get_pixel_with(vm: &mut Vm, input: i64) -> Option<(i64, i64, i64)> {
    if let Some(x) = run_until_with(vm, input) {
        Some((x, run_until_with(vm, input).unwrap(),
                 run_until_with(vm, input).unwrap()))
    } else {
        None
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut vm = Vm::from_str(&input);

    let mut tiles = HashMap::new();
    while vm.running() {
        let x = run_until(&mut vm);
        let y = run_until(&mut vm);
        let c = run_until(&mut vm);
        if vm.running() {
            tiles.insert((x.unwrap(), y.unwrap()), c.unwrap());
        } else {
            break;
        }
    }
    // Save game bounds forl ater
    let xmin = tiles.keys().map(|p| p.0).min().unwrap();
    let xmax = tiles.keys().map(|p| p.0).max().unwrap();
    let ymin = tiles.keys().map(|p| p.1).min().unwrap();
    let ymax = tiles.keys().map(|p| p.1).max().unwrap();
    println!("Part 1: {}", tiles.values().filter(|v| **v == 2).count());

    let mut vm = Vm::from_str(&input);
    vm.poke(0, 2);

    // Build the initial game state
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut input = 0;
    while tiles.values().filter(|v| **v == 2).count() > 0 {
        while let Some((x, y, c)) = get_pixel_with(&mut vm, input) {
            let redraw = tiles.get(&(x, y)) != Some(&c);
            tiles.insert((x, y), c);

            if c == 4 {
                ball_x = x;
            } else if c == 3 && x != paddle_x {
                paddle_x = x;
            }
            if ball_x < paddle_x {
                input = -1;
            } else if ball_x > paddle_x {
                input =  1;
            } else {
                input = 0;
            }

            if redraw {
                print!("{}[2J", 27 as char);
                std::io::stdout().flush();
                for y in ymin..=ymax {
                    for x in xmin..=xmax {
                        let c = match tiles.get(&(x, y)).unwrap_or(&0) {
                            0 => ' ',
                            1 => '█',
                            2 => '▒',
                            3 => '▔',
                            4 => '●',
                            _ => unreachable!(),
                        };
                        print!("{}", c);
                    }
                    print!("\n");
                }
                std::io::stdout().flush();
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    }
    println!("Part 2: {}", tiles.get(&(-1, 0)).unwrap());
}
