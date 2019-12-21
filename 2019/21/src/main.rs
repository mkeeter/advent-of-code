use std::collections::HashSet;
use std::io::Read;
use vm::Vm;

type Scan = Vec<bool>;

enum Op {
    Lit(char),
    And(Box<Op>, Box<Op>),
    Or(Box<Op>, Box<Op>),
    Not(Box<Op>),
}

impl Op {
    fn expand(o: &Op) -> Op {
        match o {
            Lit(c) => Lit(c),
            Not(i) => match i.expand() {
                Lit(c) => Not(Lit(c)),
                And(a, b) => Or(Not(a), Not(b)),

            }

    }
}


fn plan(jumps: &[Vec<Scan>]) -> String {
    let mut plans: Vec<Vec<bool>> = jumps[0].clone();
    for j in jumps[1..].iter() {

    }
    // Any (remaining) plan is a good plan!
    if let Some(p) = plans.pop() {
        p;
        "omg";
    } else {
    }
    "WALK\n".to_owned()
}

fn run(mut vm: Vm, plan: &str) -> Result<i64, Vec<char>> {
    for c in plan.chars() {
        vm.input(c as i64);
    }
    let mut line = String::new();
    while let Some(i) = vm.run_until() {
        if i < 255 && i > 0 {
            let c = i as u8 as char;
            print!("{}", c);
            line.push(c);
        } else {
            return Ok(i);
        }
    }
    Err(line.split('\n')
        .rev()
        .nth(2)
        .unwrap()
        .chars()
        .map(|c| if c == '@' { '.' } else { c } )
        .collect::<Vec<char>>())
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut jumps: Vec<Vec<Scan>> = Vec::new();
    let mut dead: HashSet<Scan> = HashSet::new();

    loop {
        println!("Got plan {:?}", plan(&jumps));

        let vm = Vm::from_str(&input);
        let r = run(vm, "WALK\n");

        if r.is_ok() {
            println!("Part 1: {}", r.unwrap());
            break;
        }

        let mut fall = r.err().unwrap();

        // Assign numbers to contiguous blocks of floor
        let mut in_gap = false;
        let mut index = 1;
        for i in 0..fall.len() {
            if fall[i] == '#' {
                if in_gap {
                    in_gap = false;
                    index += 1;
                }
                fall[i] = ('0' as u8 + index) as char;
            } else {
                in_gap = true;
            }
        }
        // Mark with x any block where a jump will kill you
        for i in 0..fall.len() {
            if *fall.get(i + 4).unwrap_or(&'#') == '.' {
                fall[i] = 'x';
                let pattern = (i+1..i+5)
                    .map(|j| fall[j] != '.')
                    .collect::<Scan>();
                dead.insert(pattern);
            }
        }
        // Leave numbers only where jumps will change chunks
        for i in 0..fall.len() {
            let c = fall[i];
            let d = *fall.get(i + 4).unwrap_or(&c);
            if c == d {
                fall[i] = '#';
            }
        }
        // Unpack into valid binary patterns
        let mut prev = 0;
        for (i, c) in fall.iter().enumerate() {
            if char::is_numeric(*c) {
                let j = *c as u8 - '0' as u8;
                if j != prev {
                    jumps.push(Vec::new());
                    prev = j;
                }
                let pattern = (i+1..i+5)
                    .map(|j| fall[j] != '.')
                    .collect::<Scan>();
                println!("Pushing {:?} to {:?}", pattern, jumps.last_mut());
                jumps.last_mut().unwrap().push(pattern);
            }
        }
        // Filter out any jumps that will kill you
        jumps = jumps.into_iter()
            .map(|p| p.into_iter()
                 .filter(|q| !dead.contains(&*q))
                 .collect())
            .collect::<Vec<Vec<Scan>>>();

        println!("{:?}", jumps);
        println!("{:?}", dead);
        break;
    }

    /*
    let mut vm = Vm::from_str(&input);
    for c in "NOT C J\nAND A J\nAND D J\nNOT A T\nOR T J\nRUN\n".chars() {
        vm.input(c as i64);
    }
    let mut line = String::new();
    while let Some(i) = vm.run_until() {
        if i < 255 && i > 0 {
            let c = i as u8 as char;
            print!("{}", c);
            line.push(c);
        } else {
            println!("Part 1: {}\n", i);
        }
    }
    let fall = line.split('\n')
        .rev()
        .nth(2)
        .unwrap()
        .chars()
        .map(|c| if c == '@' { '.' } else { c } )
        .collect::<Vec<char>>();
    println!("Fell on {:?}", fall);
    */
}
