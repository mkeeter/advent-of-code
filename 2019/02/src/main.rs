use std::io::Read;
use vm::Vm;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let mut vm = Vm::from_str(&input);
    vm.poke(1, 12);
    vm.poke(2, 2);
    vm.run();
    println!("Part 1: {}", vm.peek(0));

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut vm = Vm::from_str(&input);
            vm.poke(1, noun);
            vm.poke(2, verb);
            vm.run();
            if vm.peek(0) == 1969_07_20 {
                println!("Part 2: {}", 100 * noun + verb);
                break 'outer;
            }
        }
    }

    Ok(())
}
