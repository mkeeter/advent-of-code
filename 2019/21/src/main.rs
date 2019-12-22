use std::cmp::min;
use std::collections::HashSet;
use std::io::Read;
use vm::Vm;

use z3::ast::Ast;
use z3::ast::{BV, Bool};

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
    let mut danger_zones: Vec<Vec<char>> = Vec::new();

    loop {
        let tape = solve_z3(range, &danger_zones);
        println!("Running test");
        let vm = Vm::from_str(input);
        match test(vm, &tape, speed) {
            Ok(i) => return i,
            Err(r) => danger_zones.push(r),
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
            danger_zones: &Vec<Vec<char>>) -> String
{
    let in_bits = 4;
    let op_bits = 2;
    assert!((1 << in_bits) >= range + 2);

    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    let instructions = 15;
    let ops = (0..instructions)
        .map(|i| BV::new_const(&ctx, format!("op_{}", i), op_bits))
        .collect::<Vec<_>>();
    let in_srcs = (0..instructions)
        .map(|i| BV::new_const(&ctx, format!("in_src_{}", i), in_bits))
        .collect::<Vec<_>>();
    let out_dests = (0..instructions)
        .map(|i| Bool::new_const(&ctx, format!("out_dest_{}", i)))
        .collect::<Vec<_>>();

    // input registers are <= range + 1
    // (range is T, range + 1 is J)
    for in_src in in_srcs.iter() {
        solver.assert(&in_src.bvult(
                &BV::from_u64(&ctx, range as u64 + 2, in_bits)));
    }
    // Opcodes are < 3
    for op in ops.iter() {
        solver.assert(&op._eq(&BV::from_u64(&ctx, 3, op_bits)).not());
    }

    for d in danger_zones {
        let mut jumped_at = Vec::new();
        for i in 0..d.len() {
            let scan = d[(i + 1)..min(d.len(), i + 1 + range as usize)].iter()
                .enumerate()
                .filter(|(_j, c)| **c == '#')
                .fold(0, |acc, (j, _c)| acc | (1 << j));

            // We're aerial if we jumped within the last few tiles
            let mut aerial = Bool::from_bool(&ctx, false);
            for j in i.saturating_sub(3)..i {
                aerial = aerial.or(&[&jumped_at[j]]);
            }

            // We only jump if the program says to jump
            let jump = build_model(
                instructions, range, in_bits, op_bits, scan,
                &ctx, &ops, &in_srcs, &out_dests, &solver);

            // We also only jump if we're not in mid-air
            jumped_at.push(jump.and(&[&aerial.not()]));

            // Record whether we're aerial or not
            if d[i] != '#' {
                solver.assert(&aerial);
            }
        }
    }
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

    //println!("Part 1: {}", solve(&input, 4, "WALK"));
    println!("Part 2: {}", solve(&input, 9, "RUN"));
}
