use std::io::Read;
use std::collections::HashMap;
use vm::Vm;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut vm = Vm::from_str(&input);
    println!("Hello, world!");

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
}
