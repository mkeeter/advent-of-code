use std::collections::HashSet;
use std::cmp::min;
use std::io::Read;
use vm::Vm;

use z3::ast::Ast;
use z3::ast::{BV, Bool};

type Scan = u16;

fn test(mut vm: Vm, plan: &str, speed: &str) -> Result<i64, Vec<char>> {
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

fn solve(input: &str, range: u64, speed: &str) -> i64 {
    let mut jumps: Vec<Vec<Scan>> = Vec::new();
    let mut dead: HashSet<Scan> = HashSet::new();

    for i in 0..10 {
        let tape = solve_z3(range, &jumps, &dead);
        println!("Running test");
        let vm = Vm::from_str(input);
        let r = test(vm, &tape, speed);

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
                let pattern = (i+1..(i + 1 + range as usize))
                    .map(|j| fall[j] != '.')
                    .enumerate()
                    .fold(0, |acc, (i, b)| if b { acc | (1 << i) } else { acc });
                dead.insert(pattern);
            }
        }
        // Mark with an x any block where a jump will set you up for
        // death, because you'll be on x blocks until falling off the level
        for i in 0..(fall.len() - 4) {
            let mut any_safe = false;
            for j in (i + 4)..fall.len() {
                if fall[j] == '.' {
                    break;
                } else if fall[j] != 'x' {
                    any_safe = true;
                    break;
                }
            }
            if !any_safe {
                fall[i] = 'x';
                let pattern = (i+1..(i + 1 + range as usize))
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
                let pattern = (i+1..min(i + 1 + range as usize, fall.len()))
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

fn build_model<'a>(instructions: usize,
                   range: u64,
                   in_bits: u32,
                   op_bits: u32,
                   scan: u16,
                   ctx: &'a z3::Context,
                   ops: &[z3::ast::BV<'a>],
                   in_srcs: &[z3::ast::BV<'a>],
                   out_dests: &[z3::ast::Bool<'a>],
                   solver: &z3::Solver) -> z3::ast::Bool<'a>
{
    // Initialize T and J registers
    let mut t_prev = z3::ast::Bool::from_bool(&ctx, false);
    let mut j_prev = z3::ast::Bool::from_bool(&ctx, false);

    // Build set of scanner values
    let mut sensors = Vec::new();
    for i in 0..range {
        sensors.push(Bool::from_bool(&ctx,
            (scan & (1 << i)) != 0));
    }

    for i in 0..instructions {
        let op = &ops[i];
        let in_src = &in_srcs[i];
        let out_dest = &out_dests[i];

        let lhs_val = z3::ast::Bool::fresh_const(&ctx, &format!("lhs_{}", i));
        let rhs_val = z3::ast::Bool::fresh_const(&ctx, &format!("rhs_{}", i));
        let out_val = z3::ast::Bool::fresh_const(&ctx, &format!("out_{}", i));

        let t = z3::ast::Bool::fresh_const(&ctx, &format!("T_{}", i));
        let j = z3::ast::Bool::fresh_const(&ctx, &format!("J_{}", i));

        // Assign LHS based on many input registers
        let mut v = in_src._eq(&BV::from_u64(&ctx, range as u64, in_bits))
            .ite(&t_prev, &j_prev);
        for j in (0..range).rev() {
            v = in_src._eq(&BV::from_u64(&ctx, j as u64, in_bits)).ite(&sensors[j as usize], &v);
        }
        solver.assert(&lhs_val._eq(&v));

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

        // Assign output to either the T or J register
        //  If out_dest is set, then store in T
        //  Otherwise, store in J
        solver.assert(&t._eq(&out_dest.ite(&out_val, &t_prev)));
        solver.assert(&j._eq(&out_dest.not().ite(&out_val, &j_prev)));

        t_prev  = t;
        j_prev = j;
    }
    j_prev
}

fn solve_z3(range: u64,
            jumps: &Vec<Vec<Scan>>,
            dead: &HashSet<Scan>) -> String
{
    let in_bits = 4;
    let op_bits = 2;
    assert!((1 << in_bits) >= range + 2);

    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    let instructions = 2;
    let ops = (0..instructions)
        .map(|i| BV::new_const(&ctx, format!("op_{}", i), op_bits))
        .collect::<Vec<_>>();
    let in_srcs = (0..instructions)
        .map(|i| BV::new_const(&ctx, format!("in_src_{}", i), in_bits))
        .collect::<Vec<_>>();
    let out_dests = (0..instructions)
        .map(|i| Bool::new_const(&ctx, format!("out_dest_{}", i)))
        .collect::<Vec<_>>();

    for in_src in in_srcs.iter() {
        solver.assert(&in_src.bvult(&BV::from_u64(&ctx, range as u64 + 2, in_bits)));
    }
    for op in ops.iter() {
        solver.assert(&op._eq(&BV::from_u64(&ctx, 3, op_bits)).not());
    }

    for hole in jumps.iter() {
        let options = hole.iter()
            .map(|j| build_model(
                    instructions, range, in_bits, op_bits, *j,
                    &ctx, &ops, &in_srcs, &out_dests, &solver))
            .collect::<Vec<_>>();
        let opts = options[1..].iter().collect::<Vec<&_>>();
        solver.assert(&options[0].or(&opts));
    }
    for d in dead.iter() {
        let m = build_model(
                instructions, range, in_bits, op_bits, *d,
                &ctx, &ops, &in_srcs, &out_dests, &solver);
        solver.assert(&m.not());
    }
    //println!("{}", solver);
    println!("{:?}", solver.check());

    let model = solver.get_model();

    let mut tape = String::new();
    for i in 0..instructions {
        let op = model.eval(&ops[i]).unwrap();
        let in_src = model.eval(&in_srcs[i]).unwrap();
        let out_dest = model.eval(&out_dests[i]).unwrap();
        let op = match op.as_u64().unwrap() {
            0 => "AND",
            1 => "NOT",
            2 => "OR",
            _ => unreachable!(),
        };
        tape += &format!("{} ", op);
        tape += &format!("{} ", if in_src.as_u64().unwrap() == range {
            'T'
        } else if in_src.as_u64().unwrap() == range + 1 {
            'J'
        } else {
            ('A' as u8 + in_src.as_u64().unwrap() as u8) as char
        });
        tape += &format!("{}", if out_dest.as_bool().unwrap() {
            'T'
        } else {
            'J'
        });
        tape += "\n";
    }
    println!("Got tape:\n{}", tape);
    tape
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve(&input, 4, "WALK"));
    //solve(&input, 9, "RUN");
}
