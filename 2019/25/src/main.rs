use std::str::FromStr;
use vm::Vm;

fn fuzz(vm: &mut Vm, dir: &str) {
    let mut output = String::new();
    for c in "inv\n".chars() {
        vm.input(c as u8 as i64);
    }
    while !vm.needs_input() {
        if let Some(i) = vm.step() {
            output += &format!("{}", i as u8 as char);
        }
    }
    let inv: Vec<String> = output.lines()
        .filter(|i| i.starts_with("- "))
        .map(|i| i.replace("- ", "").to_owned())
        .collect();

    for i in 0..(1 << inv.len()) {
        let mut dropped = Vec::new();
        for (j, item) in inv.iter().enumerate() {
            if i & (1 << j) != 0 {
                dropped.push(item);
            }
        }
        println!("Testing {:?}", dropped);
        for d in dropped.iter() {
            for c in format!("drop {}\n", d).chars() {
                vm.input(c as u8 as i64);
            }
        }
        for c in dir.chars() {
            vm.input(c as u8 as i64);
        }
        output.clear();
        while !vm.needs_input() && vm.running() {
            if let Some(i) = vm.step() {
                output += &format!("{}", i as u8 as char);
            }
        }
        if !output.contains("Droids on this ship are") {
            break;
        }
        for d in dropped.iter() {
            for c in format!("take {}\n", d).chars() {
                vm.input(c as u8 as i64);
            }
        }
    }
    println!("{}", output);
}

fn main() {
    let input = include_str!("../input");
    let mut vm = Vm::from_str(&input).unwrap();

    loop {
        if vm.needs_input() {
            let mut cmd = String::new();
            std::io::stdin().read_line(&mut cmd).unwrap();
            if cmd.starts_with("fuzz") {
                let dir = cmd.split(" ").nth(1).unwrap();
                fuzz(&mut vm, dir);
                break;
            } else {
                for c in cmd.chars() {
                    vm.input(c as u8 as i64);
                }
            }
        } else {
            if let Some(i) = vm.step() {
                print!("{}", i as u8 as char);
            }
        }
    }
}
