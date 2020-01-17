use std::io::Read;
use std::str::FromStr;
use assembunny::Vm;

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
    {   // This isn't blindingly fast, but the simple loop transformation
        // makes it < 30 seconds, which is good enough.
        let mut vm = vm.clone();
        vm.regs[0] = 12;
        vm.run();
        println!("Part 2: {}", vm.regs[0]);
    }
}
