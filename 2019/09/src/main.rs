use std::io::Read;
use vm::Vm;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut vm = Vm::from_str(&input);
    vm.input(1);
    let out = vm.run();
    assert!(out.len() == 1);
    println!("Part 1: {} ", out[0]);

    let mut vm = Vm::from_str(&input);
    vm.input(2);
    println!("Part 2: {} ", vm.run()[0]);
}
