use std::io::Read;
use std::str::FromStr;

use vm::Vm;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut vms = vec![Vm::from_str(&input).unwrap(); 50];
    for (i, vm) in vms.iter_mut().enumerate() {
        vm.input(i as i64);
    }
    'outer: loop {
        for i in 0..vms.len() {
            if let Some(addr) = vms[i].step_with(-1) {
                let x = vms[i].run_until().unwrap();
                let y = vms[i].run_until().unwrap();
                println!("Sending packet {} {} {}", addr, x, y);
                if addr == 255 {
                    println!("Part 1: {}", y);
                    break 'outer;
                } else {
                    vms[addr as usize].input(x);
                    vms[addr as usize].input(y);
                }
            }
        }
    }
}
