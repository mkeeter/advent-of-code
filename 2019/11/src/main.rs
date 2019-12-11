use std::io::Read;
use std::collections::HashMap;
use vm::Vm;

fn paint(input: &str, s: bool) -> HashMap<(i32, i32), bool> {
    let mut vm = Vm::from_str(&input);

    let mut panels : HashMap<(i32, i32), bool> = HashMap::new();
    panels.insert((0, 0), s);

    let mut pos = (0, 0);
    let mut dir = (0, 1);
    while vm.running() {
        // Color
        loop {
            if !vm.running() {
                break;
            } else if vm.blocked() {
                vm.input(panels.get(&pos).cloned().unwrap_or(false) as i64);
            }
            if let Some(color) = vm.step() {
                panels.insert(pos, color != 0);
                break;
            }
        }

        // Rotation
        loop {
            if !vm.running() {
                break;
            } else if vm.blocked() {
                vm.input(panels.get(&pos).cloned().unwrap_or(false) as i64);
            }
            if let Some(rot) = vm.step() {
                if rot == 0 {
                    dir = (-dir.1, dir.0);
                } else {
                    dir = (dir.1, -dir.0);
                }
                break;
            }
        }
        pos.0 += dir.0;
        pos.1 += dir.1;
    }
    panels
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", paint(&input, false).len());

    println!("Part 2:");
    let p = paint(&input, true);
    let (xmin, xmax, ymin, ymax) = p.keys()
        .fold((std::i32::MAX, std::i32::MIN,
               std::i32::MAX, std::i32::MIN),
        |(xmin, xmax, ymin, ymax), (x, y)| {
            (xmin.min(*x), xmax.max(*x),
             ymin.min(*y), ymax.max(*y))});
    for y in (ymin..=ymax).rev() {
        for x in xmin..=xmax {
            let c = p.get(&(x, y))
                .cloned()
                .unwrap_or(false);
            if c {
                print!("█");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}
