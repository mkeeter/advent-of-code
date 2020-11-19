use std::collections::HashSet;

use std::io::Read;
use std::str::FromStr;
use assembunny::Vm;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let vm = Vm::from_str(&input).unwrap();

    for i in 0.. {
        let mut vm = vm.clone();
        vm.regs[0] = i;
        let mut seq = Vec::new();
        let mut seen = HashSet::new();
        while let Some(i) = vm.next() {
            if seen.contains(&vm) {
                break;
            } else {
                seen.insert(vm.clone());
            }
            seq.push(i);
        }
        if seq.iter().zip([0, 1].iter().cycle()).all(|(a, b)| a == b) {
            println!("Part 1: {}", i);
            break;
        }
    }
    println!("Part 2: ☆");
}
