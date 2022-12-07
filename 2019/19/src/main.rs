use std::collections::VecDeque;
use std::io::Read;
use std::str::FromStr;

use vm::Vm;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let vm = Vm::from_str(&input).unwrap();
    let run = |x, y| -> i64 {
        let mut vm = vm.clone();
        vm.input(x);
        vm.input(y);
        vm.run_until().unwrap()
    };

    let count = (0..50)
        .flat_map(|y| (0..50).map(move |x| (x, y)))
        .filter(|&(x, y)| run(x, y) == 1)
        .count();
    println!("Part 1: {}", count);

    let mut xmin = 0;
    let mut length = 1;
    let mut buf = VecDeque::new();
    for y in 4.. {
        // Walk forward to find the first point in the beam
        while run(xmin, y) != 1 {
            xmin += 1;
        }
        // Estimate the end of this run
        let mut xmax = xmin + length;
        // Walk forward while inside the beam
        while run(xmax, y) == 1 {
            xmax += 1;
        }
        // Walk backwards while not barely outside the beam
        while run(xmax - 1, y) != 1 {
            xmax -= 1;
        }
        // Update the length estimate
        length = xmax - xmin;

        if length >= 100 {
            buf.push_back((xmin, xmax));
            if buf.len() > 100 {
                buf.pop_front();
            }
        } else {
            buf.clear();
        }

        // Check for a 100x100 square within this region
        if buf.len() == 100 {
            let span_start = buf.back().unwrap().0;
            let span_end = span_start + 100;
            if buf.iter().all(|&(xmin, xmax)| {
                span_start <= xmax && span_start >= xmin && span_end <= xmax && span_end >= xmin
            }) {
                let corner = (span_start, y - 100 + 1);
                println!("Part 2: {}", corner.0 * 10000 + corner.1);
                break;
            }
        }
    }
}
