use std::io::Read;
use itertools::Itertools;
use std::str::FromStr;

use vm::Vm;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    ////////////////////////////////////////////////////////////////////////////
    // Part 1
    let best = (0..5).permutations(5)
        .map(|ps| {
            // Build a fresh set of VMs and queues
            let mut vms = vec![Vm::from_str(&input).unwrap(); 5];
            for (i, vm) in vms.iter_mut().enumerate() {
                vm.input(ps[i] as i64);
            }
            vms[0].input(0);

            loop {
                for i in 0..vms.len() {
                    if let Some(out) = vms[i].step() {
                        if let Some(vm) = vms.get_mut(i + 1) {
                            vm.input(out);
                        } else {
                            return out;
                        }
                    }
                }
            }
        }).max().unwrap();
    println!("Part 1: {}", best);

    ////////////////////////////////////////////////////////////////////////////
    let best = (5..10).permutations(5)
        .map(|ps| {
            // Build a fresh set of VMs and queues
            let mut vms = vec![Vm::from_str(&input).unwrap(); 5];
            for (i, vm) in vms.iter_mut().enumerate() {
                vm.input(ps[i] as i64);
            }
            vms[0].input(0);

            let mut output = 0;
            while vms.iter().any(Vm::running) {
                for i in 0..vms.len() {
                    if let Some(out) = vms[i].step() {
                        if i == vms.len() - 1 {
                            output = out;
                        }
                        vms[(i + 1) % 5].input(out);
                    }
                }
            }
            output
        }).max().unwrap();
    println!("Part 2: {}", best);
}
