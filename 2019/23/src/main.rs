use std::collections::HashSet;
use std::io::Read;
use std::str::FromStr;

use vm::Vm;

const NUM_MACHINES: usize = 50;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut nat: Option<(i64, i64)> = None;
    let mut idle = [0; NUM_MACHINES];
    let mut seen = HashSet::new();

    let mut vms = vec![Vm::from_str(&input).unwrap(); NUM_MACHINES];
    for (i, vm) in vms.iter_mut().enumerate() {
        vm.input(i as i64);
    }
    loop {
        // The network is idle if every machine has tried to receive
        // two packets without sending anything.
        if idle.iter().all(|i| *i > 1) {
            let nat = nat.unwrap();
            if !seen.insert(nat.1) {
                println!("Part 2: {}", nat.1);
                break;
            }
            vms[0].input(nat.0);
            vms[0].input(nat.1);
            idle = [0; NUM_MACHINES];
        }
        for i in 0..vms.len() {
            idle[i] += vms[i].needs_input() as i64;
            if let Some(addr) = vms[i].step_with(-1) {
                idle[i] = 0;

                let x = vms[i].run_until().unwrap();
                let y = vms[i].run_until().unwrap();
                if addr == 255 {
                    if nat.is_none() {
                        println!("Part 1: {}", y);
                    }
                    nat = Some((x, y))
                } else {
                    idle[addr as usize] = 0;
                    vms[addr as usize].input(x);
                    vms[addr as usize].input(y);
                }
            }
        }
    }
}
