use std::collections::HashSet;
use std::io::BufRead;

//include!(concat!(env!("OUT_DIR"), "/prog.rs"));

fn main() {
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut words = line.split(' ');
        match words.next().unwrap() {
            "inp" => {
                let reg = words.next().unwrap();
            }
            "add" => {
                let a = words.next().unwrap();
                let b = words.next().unwrap();
            }
            "mul" => {
                let a = words.next().unwrap();
                let b = words.next().unwrap();
            }
            "div" => {
                let a = words.next().unwrap();
                let b = words.next().unwrap();
            }
            "mod" => {
                let a = words.next().unwrap();
                let b = words.next().unwrap();
            }
            "eql" => {
                let a = words.next().unwrap();
                let b = words.next().unwrap();
            }
            _ => panic!("Invalid instruction {}", line),
        }
    }
    //println!("{}", include_str!(concat!(env!("OUT_DIR"), "/prog.rs")));

    /*
    for pass in 0..14 {
        let f = PASSES[14 - pass - 1];
        let a = f((1, 1, 1, 1), 1);
        let di = f((1, 1, 1, 1), 2);
        let dy = f((1, 2, 1, 1), 1);
        let dz = f((1, 1, 2, 1), 1);
        let dw = f((1, 1, 1, 2), 1);
        let dy_di = di.1 - a.1;
        let dy_dy = dy.1 - a.1;
        let dy_dz = dz.1 - a.1;
        let dy_dw = dw.1 - a.1;
        println!("{} {} {} {}", dy_di, dy_dy, dy_dz, dy_dw);
    }

    let mut states: Vec<(i64, Registers)> = vec![(0i64, (0, 0, 0, 0))];
    for pass in 0..14 {
        println!("Pass {}: {}", pass, states.len());
        let f = PASSES[14 - pass - 1];
        let mut next = HashSet::new();
        for s in states.iter() {
            for i in 1..=9 {
                let out = f(s.1, i);
                //println!("{:?} x {} => {:?}", s.1, i, out);
                next.insert((s.0 * 10 + i as i64, out));
            }
        }
        // Pass 0: Y = W + 6, Z = W + 6
        // Pass 0: Y = W + 8
        states = next.into_iter().collect();
    }
    println!("{:?}", states.iter().filter(|(_, i)| i.2 == 0).max_by_key(|(s, _)| s).unwrap());
    */
}
