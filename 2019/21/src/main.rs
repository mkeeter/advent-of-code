use std::io::Read;
use vm::Vm;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut vm = Vm::from_str(&input);
    for c in "NOT C J\nAND A J\nAND D J\nNOT A T\nOR T J\nWALK\n".chars() {
        vm.input(c as i64);
    }
    while let Some(i) = vm.run_until() {
        if i < 255 && i > 0 {
            print!("{}", i as u8 as char);
        } else {
            println!("Got output {}\n", i);
        }
    }
}
