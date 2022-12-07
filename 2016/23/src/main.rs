use assembunny::Vm;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let vm = Vm::from_str(&input).unwrap();

    {
        let mut vm = vm.clone();
        vm.regs[0] = 7;
        vm.run();
        println!("Part 1: {}", vm.regs[0]);
    }
    {
        let mut vm = vm.clone();
        vm.regs[0] = 12;
        vm.run();
        println!("Part 2: {}", vm.regs[0]);
    }
}
