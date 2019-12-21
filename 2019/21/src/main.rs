use std::collections::HashSet;
use std::cmp::min;
use std::io::Read;
use vm::Vm;

type Scan = u16;

struct Plan {
    active: u16,
    floors: u16
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Lit {
    Val(char),
    Not(char),
}

// OR of ANDs
#[derive(Clone, Debug)]
struct Dnf(Vec<HashSet<Lit>>);

// AND of ORs
#[derive(Clone, Debug)]
struct Cnf(Vec<HashSet<Lit>>);

impl Dnf {
    fn to_cnf(&self) -> Cnf {
        let mut out = self.0[0].iter().map(|i| {
                let mut h = HashSet::new();
                h.insert(*i);
                h
            })
            .collect::<Vec<HashSet<Lit>>>();
        for i in self.0[1..].iter() {
            let mut next = Vec::new();
            for o in out.iter() {
                for c in i.iter() {
                    let mut q = o.clone();
                    q.insert(*c);
                    next.push(q);
                }
            }
            out = next;
        }
        // Any clause with both X and !X must be true
        let mut out = out.into_iter().filter(|c| !c.iter()
            .any(|k| match k {
                Lit::Val(i) => c.contains(&Lit::Not(*i)),
                Lit::Not(i) => c.contains(&Lit::Val(*i)),
            }))
            .collect::<Vec<HashSet<Lit>>>();

        out = out.iter().enumerate()
            .filter(|(i, p)| !out.iter()
                    .enumerate()
                    .any(|(j, s)| j != *i && p.is_subset(s)))
            .map(|(i, p)| p.clone())
            .collect::<Vec<HashSet<Lit>>>();
        Cnf(out)
    }
}

fn plan(jumps: &[Vec<Scan>]) -> String {
    println!("Planning!");
    let mut out = Vec::new();
    for j in jumps {
        let mut dnf = Dnf(Vec::new());
        for k in j.iter() {
            let w = k.iter()
                .enumerate()
                .map(|(i, b)| {
                    let k = (i as u8 + 'A' as u8) as char;
                    if *b {
                        Lit::Val(k)
                    } else {
                        Lit::Not(k)
                    }
                })
                .collect::<HashSet<_>>();
            dnf.0.push(w);
        }
        for c in dnf.to_cnf().0 {
            out.push(c);
        }
        println!("  cnf: {:?}", dnf.to_cnf());
    }
    out = out.iter().enumerate()
        .filter(|(i, p)| !out.iter()
                .enumerate()
                .any(|(j, s)| j != *i && s.is_subset(p)))
        .map(|(i, p)| p.clone())
        .collect::<Vec<HashSet<Lit>>>();
    println!("Final CNF: {:?}", out);

    // Still hard-coded for now
    //"NOT C J\nNOT B T\nOR T J\nNOT A T\nOR T J\nAND D J\n".to_owned()
    //"NOT C J\nNOT B T\nOR T J\nNOT A T\nOR T J\nAND D J\n".to_owned()
    "WALK\n".to_owned()
}

fn run(mut vm: Vm, plan: &str, speed: &str) -> Result<i64, Vec<char>> {
    for c in plan.chars() {
        vm.input(c as i64);
    }
    for c in speed.chars() {
        vm.input(c as i64);
    }
    vm.input('\n' as u8 as i64);
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

fn solve(input: &str, range: usize, speed: &str) -> i64 {
    let mut jumps: Vec<Vec<Scan>> = Vec::new();
    let mut dead: HashSet<Scan> = HashSet::new();

    loop {
        let p = plan(&jumps);

        let vm = Vm::from_str(input);
        let r = run(vm, &p, speed);

        if r.is_ok() {
            return r.unwrap();
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
                let pattern = (i+1..i+range + 1)
                    .map(|j| fall[j] != '.')
                    .collect::<Scan>();
                dead.insert(pattern);
            }
        }
        for c in fall.iter() {
            print!("{}", c);
        }
        print!("\n");
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
                let pattern = (i+1..min(i+range+1, fall.len()))
                    .map(|j| fall[j] != '.')
                    .collect::<Scan>();
                println!("Pushing {} {:?} to {:?}", i, pattern, jumps.last_mut());
                jumps.last_mut().unwrap().push(pattern);
            }
        }
        // Filter out any jumps that will kill you
        jumps = jumps.into_iter()
            .map(|p| p.into_iter()
                 .filter(|q| !dead.contains(&*q))
                 .collect())
            .collect::<Vec<Vec<Scan>>>();

        println!("We must jump in the following cases:");
        for j in jumps.iter() {
            println!("   Case:");
            for c in j.iter() {
                println!("        {:?}", c);
            }
        }
        println!("We may not jump in the following cases:");
        for d in dead.iter() {
            println!("    {:?}", dead);
        }

        println!("Got plan {:?}", plan(&jumps));
        break;
    }
    0
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve(&input, 4, "WALK"));
    solve(&input, 9, "RUN");


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
