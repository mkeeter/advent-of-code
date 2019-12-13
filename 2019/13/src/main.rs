use std::io::Read;
use std::collections::HashMap;
use std::cmp::Ordering;

use vm::Vm;

fn get_pixel_with(vm: &mut Vm, input: i64) -> Option<(i64, i64, i64)> {
    if let Some(x) = vm.run_until_with(input) {
        Some((x, vm.run_until_with(input).unwrap(),
                 vm.run_until_with(input).unwrap()))
    } else {
        None
    }
}

fn draw(tiles: &HashMap<(i64, i64), i64>) {
    let xmin = tiles.keys().map(|p| p.0).min().unwrap();
    let xmax = tiles.keys().map(|p| p.0).max().unwrap();
    let ymin = tiles.keys().map(|p| p.1).min().unwrap();
    let ymax = tiles.keys().map(|p| p.1).max().unwrap();

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            println!("{}", match tiles.get(&(x, y)).unwrap_or(&0) {
                0 => ' ',
                1 => '█',
                2 => '▒',
                3 => '▔',
                4 => '●',
                _ => unreachable!(),
            });
        }
        print!("\n");
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut vm = Vm::from_str(&input);

    let mut tiles = HashMap::new();
    while vm.running() {
        let x = vm.run_until();
        let y = vm.run_until();
        let c = vm.run_until();
        if vm.running() {
            tiles.insert((x.unwrap(), y.unwrap()), c.unwrap());
        } else {
            break;
        }
    }
    println!("Part 1: {}", tiles.values().filter(|v| **v == 2).count());

    let mut vm = Vm::from_str(&input);
    vm.poke(0, 2);

    // Build the initial game state
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut input = 0;
    while tiles.values().any(|v| *v == 2) {
        while let Some((x, y, c)) = get_pixel_with(&mut vm, input) {
            let redraw = tiles.get(&(x, y)) != Some(&c);
            tiles.insert((x, y), c);

            match c {
                3 => paddle_x = x,
                4 => ball_x = x,
                _ => (),
            };

            input = match ball_x.cmp(&paddle_x) {
                Ordering::Less => -1,
                Ordering::Greater =>  1,
                Ordering::Equal => 0,
            };

            if redraw && false {
                draw(&tiles);
            }
        }
    }
    println!("Part 2: {}", tiles.get(&(-1, 0)).unwrap());
}
