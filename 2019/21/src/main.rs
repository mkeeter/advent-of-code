use std::collections::HashSet;
use std::cmp::min;
use std::io::Read;
use vm::Vm;

type Scan = u16;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Plan {
    active: u16,
    value: u16
}

fn plan(jumps: &[Vec<Scan>], dead: &HashSet<Scan>, scan: usize) -> Option<Plan> {
    if jumps.len() == 0 {
        return Some(Plan { active: 0, value: 0 });
    }
    // Plan without this jump as a constraint
    let n = plan(&jumps[1..], dead, scan).unwrap();

    // Then try every possible version of this jump
    'outer: for i in jumps[0].iter() {
        // Make a new plan which jumps at the given point
        let p = Plan{ active: (1 << scan) - 1, value: *i };

        // Turn off conflicting values
        let conflicting = p.active & n.active & (p.value ^ n.value);
        let active = (p.active | n.active) ^ conflicting;

        // Combine values with the recursed solution
        let value = ((p.value & p.active) | (n.value & n.active)) & active;

        if *i == 8 {
            println!("Trouble spot: {:b} {:b}", active, value);
        }

        // If this plan kills anyone, then keep searching
        if dead.iter().any(|d| (d & active) == value) {
            continue;
        }
        if *i == 8 {
            println!("plan doesn't kill anyone?");
        }

        // The plan has to still work
        for j in jumps.iter() {
            if !j.iter().any(|d| (d & active) == value) {
                continue 'outer;
            }
        }
        if *i == 8 {
            println!("plan doesn't not work?");
        }

        println!("Returning plan from {:?}: {:b} {:b}", jumps, active, value);

        // Otherwise, we've found a good plan!
        return Some(Plan { active: active, value: value});
    }
    None
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

    for i in 0..10 {
        let p: Plan = plan(&jumps, &dead, range).unwrap();
        println!("Got plan value: {:b}: active: {:b}", p.value, p.active);

        let mut started = false;
        let mut s = String::new();
        for i in 0..range {
            let c = ('A' as u8 + i as u8) as char;
            if p.active & (1 << i) != 0 {
                if p.value & (1 << i) != 0 {
                    if !started {
                        s += &format!("OR {} J\n", c);
                    } else {
                        s += &format!("AND {} J\n", c);
                    }
                } else {
                    if !started {
                        s += &format!("NOT {} J\n", c);
                    } else {
                        s += &format!("NOT {} T\n", c);
                        s += &format!("AND T J\n");
                    }
                }
                started = true;
            }
        }
        s += "WALK\n";
        println!("Executing plan\n{}", s);

        let vm = Vm::from_str(input);
        let r = run(vm, &s, speed);

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
                    .enumerate()
                    .fold(0, |acc, (i, b)| if b { acc | (1 << i) } else { acc });
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
                    .enumerate()
                    .fold(0, |acc, (i, b)| if b { acc | (1 << i) } else { acc });
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
            println!("    {:?}", d);
        }
    }
    0
}

fn solve_z3(range: u64) {
    use z3::ast::Ast;

    let in_bits = 4;
    let op_bits = 2;
    assert!((1 << in_bits) >= range + 2);

    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    use z3::ast::{BV, Bool};
    let sensors = (0..range)
        .map(|i| Bool::new_const(&ctx,
            format!("{}", ('A' as u8 + i as u8) as char)))
        .collect::<Vec<_>>();

    let mut t_prev = z3::ast::Bool::from_bool(&ctx, false);
    let mut j_prev = z3::ast::Bool::from_bool(&ctx, false);

    let instructions = 1;
    let ops = (0..instructions)
        .map(|i| BV::new_const(&ctx, format!("op_{}", i), op_bits))
        .collect::<Vec<_>>();
    let in_srcs = (0..instructions)
        .map(|i| BV::new_const(&ctx, format!("in_src_{}", i), in_bits))
        .collect::<Vec<_>>();
    let out_dests = (0..instructions)
        .map(|i| Bool::new_const(&ctx, format!("out_dest_{}", i)))
        .collect::<Vec<_>>();

    for i in 0..instructions {

        let op = &ops[i];
        let in_src = &in_srcs[i];
        let out_dest = &out_dests[i];

        let lhs_val = z3::ast::Bool::new_const(&ctx, format!("lhs_{}", i));
        let rhs_val = z3::ast::Bool::new_const(&ctx, format!("rhs_{}", i));
        let out_val = z3::ast::Bool::new_const(&ctx, format!("out_{}", i));

        let t = z3::ast::Bool::new_const(&ctx, format!("T_{}", i));
        let j = z3::ast::Bool::new_const(&ctx, format!("J_{}", i));

        // Assign LHS based on many input registers
        let mut v = in_src._eq(&BV::from_u64(&ctx, range as u64, in_bits)).ite(&t_prev, &j_prev);
        for j in (0..range).rev() {
            v = in_src._eq(&BV::from_u64(&ctx, j as u64, in_bits)).ite(&sensors[j as usize], &v);
        }
        solver.assert(&lhs_val._eq(&v));
        solver.assert(&in_src.bvult(&BV::from_u64(&ctx, range as u64 + 2, in_bits)));

        // Assign RHS based on one of the two output registers
        solver.assert(&rhs_val._eq(&out_dest.ite(&t_prev, &j_prev)));

        // Calculate output
        solver.assert(&out_val._eq(
                /* AND opcode = 0 */
                &op._eq(&BV::from_u64(&ctx, 0, op_bits)).ite(&lhs_val.and(&[&rhs_val]),
                /* NOT opcode = 1 */
                &op._eq(&BV::from_u64(&ctx, 1, op_bits)).ite(&lhs_val.not(),
                /* OR opcode = 2 */
                &lhs_val.or(&[&rhs_val])))));
        solver.assert(&op._eq(&BV::from_u64(&ctx, 3, op_bits)).not());

        // Assign output to either the T or J register
        solver.assert(&t._eq(&out_dest.ite(&out_val, &t_prev)));
        solver.assert(&j._eq(&out_dest.not().ite(&out_val, &j_prev)));

        t_prev  = t;
        j_prev = j;
    }
    solver.assert(&j_prev);
    solver.assert(&sensors[0].implies(&j_prev));
    println!("{}, {:?}", solver, solver.check());
    println!("{}", solver.get_model());

    let model = solver.get_model();

    for i in 0..instructions {
        let op = model.eval(&ops[i]).unwrap();
        let in_src = model.eval(&in_srcs[i]).unwrap();
        let out_dest = model.eval(&out_dests[i]).unwrap();
        print!("{:#02}:", i);
        print!("{} ", match op.as_u64().unwrap() {
            0 => "AND",
            1 => "NOT",
            2 => "OR ",
            _ => unreachable!(),
        });
        print!("{} ", if in_src.as_u64().unwrap() == range {
            'V'
        } else if in_src.as_u64().unwrap() == range + 1 {
            'J'
        } else {
            ('A' as u8 + in_src.as_u64().unwrap() as u8) as char
        });
        print!("{}", if out_dest.as_bool().unwrap() {
            'V'
        } else {
            'J'
        });
        print!("\n");
    }
}

fn main() {
    solve_z3(9);

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    //println!("Part 1: {}", solve(&input, 4, "WALK"));
    //solve(&input, 9, "RUN");


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
