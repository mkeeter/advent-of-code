use std::cmp::min;
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

use vm::Vm;

use z3::ast::Ast;
use z3::ast::{Bool, BV};

fn test(mut vm: Vm, plan: &str, speed: &str) -> Result<i64, Vec<char>> {
    // Feed the plan and the speed into the VM
    for c in plan.chars() {
        vm.input(c as i64);
    }
    for c in speed.chars() {
        vm.input(c as i64);
    }
    vm.input(b'\n' as i64);

    let mut line = String::new();
    while let Some(i) = vm.run_until() {
        if i < 255 && i > 0 {
            let c = i as u8 as char;
            line.push(c);
        } else {
            return Ok(i);
        }
    }
    Err(line
        .split('\n')
        .rev()
        .nth(2)
        .unwrap()
        .chars()
        .map(|c| if c == '@' { '.' } else { c })
        .collect::<Vec<char>>())
}

fn solve(input: &str, range: usize, speed: &str) -> i64 {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let mut solver = Solver::new(&ctx, range);

    loop {
        let vm = Vm::from_str(input).unwrap();
        let tape = solver.plan();
        match test(vm, &tape, speed) {
            Ok(i) => return i,
            Err(r) => solver.append(&r),
        }
    }
}

struct Solver<'ctx> {
    ctx: &'ctx z3::Context,
    ops: Vec<BV<'ctx>>,
    in_srcs: Vec<BV<'ctx>>,
    out_dests: Vec<Bool<'ctx>>,

    in_bits: u32,
    op_bits: u32,
    range: usize,
    instructions: usize,

    solver: z3::Solver<'ctx>,
    cache: HashMap<u16, Bool<'ctx>>,
}

impl<'ctx> Solver<'ctx> {
    fn new(ctx: &'ctx z3::Context, range: usize) -> Solver<'ctx> {
        let in_bits = 4;
        let op_bits = 2;
        assert!((1 << in_bits) >= range + 2);

        let instructions = 15;

        let solver = z3::Solver::new(&ctx);
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
        let lim = BV::from_u64(&ctx, range as u64 + 2, in_bits);
        for in_src in in_srcs.iter() {
            solver.assert(&in_src.bvult(&lim));
        }
        // Opcodes are < 3
        let three = BV::from_u64(&ctx, 3, op_bits);
        for op in ops.iter() {
            solver.assert(&op._eq(&three).not());
        }

        Solver {
            ctx,
            ops,
            in_srcs,
            out_dests,

            in_bits,
            op_bits,
            range,
            instructions,

            solver,
            cache: HashMap::new(),
        }
    }

    fn eval(&mut self, scan: u16) -> z3::ast::Bool<'ctx> {
        if let Some(b) = self.cache.get(&scan) {
            return b.clone();
        }

        // Initialize T and J registers
        let mut t_prev = z3::ast::Bool::from_bool(&self.ctx, false);
        let mut j_prev = z3::ast::Bool::from_bool(&self.ctx, false);

        // Build set of scanner values
        let mut sensors = Vec::new();
        for i in 0..self.range {
            sensors.push(Bool::from_bool(&self.ctx, (scan & (1 << i)) != 0));
        }

        for i in 0..self.instructions {
            let op = &self.ops[i];
            let in_src = &self.in_srcs[i];
            let out_dest = &self.out_dests[i];

            let lhs_val = Bool::fresh_const(&self.ctx, &format!("lhs_{}", i));
            let rhs_val = Bool::fresh_const(&self.ctx, &format!("rhs_{}", i));
            let out_val = Bool::fresh_const(&self.ctx, &format!("out_{}", i));

            let t = Bool::fresh_const(&self.ctx, &format!("T_{}", i));
            let j = Bool::fresh_const(&self.ctx, &format!("J_{}", i));

            // Assign LHS based on many input registers
            let mut v = in_src
                ._eq(&BV::from_u64(&self.ctx, self.range as u64, self.in_bits))
                .ite(&t_prev, &j_prev);
            for j in (0..self.range).rev() {
                v = in_src
                    ._eq(&BV::from_u64(&self.ctx, j as u64, self.in_bits))
                    .ite(&sensors[j as usize], &v);
            }
            self.solver.assert(&lhs_val._eq(&v));

            // Assign RHS based on one of the two output registers
            self.solver
                .assert(&rhs_val._eq(&out_dest.ite(&t_prev, &j_prev)));

            // Calculate output
            self.solver.assert(&out_val._eq(
                /* AND opcode = 0 */
                &op._eq(&BV::from_u64(&self.ctx, 0, self.op_bits)).ite(
                    &lhs_val.and(&[&rhs_val]),
                    /* NOT opcode = 1 */
                    &op._eq(&BV::from_u64(&self.ctx, 1, self.op_bits)).ite(
                        &lhs_val.not(),
                        /* OR opcode = 2 */
                        &lhs_val.or(&[&rhs_val]),
                    ),
                ),
            ));

            // Assign output to either the T or J register
            // - If out_dest is set, then store in T
            // - Otherwise, store in J
            self.solver.assert(&t._eq(&out_dest.ite(&out_val, &t_prev)));
            self.solver
                .assert(&j._eq(&out_dest.not().ite(&out_val, &j_prev)));

            t_prev = t;
            j_prev = j;
        }
        self.cache.insert(scan, j_prev.clone());
        j_prev
    }

    fn append(&mut self, d: &[char]) {
        let mut jumped_at = Vec::new();
        for i in 0..d.len() {
            let scan = d[(i + 1)..min(d.len(), i + 1 + self.range)]
                .iter()
                .enumerate()
                .filter(|(_j, c)| **c == '#')
                .fold(0, |acc, (j, _c)| acc | (1 << j));

            // We're aerial if we jumped within the last few tiles
            let mut aerial = Bool::from_bool(&self.ctx, false);
            for j in &jumped_at[i.saturating_sub(3)..i] {
                aerial = aerial.or(&[j]);
            }

            // We only jump if the program says to jump,
            // and we memoize program evaluations
            let j = self.eval(scan);

            // We also only jump if we're not in mid-air
            jumped_at.push(j.and(&[&aerial.not()]));

            // Record whether we're aerial or not
            if d[i] != '#' {
                self.solver.assert(&aerial);
            }
        }
    }

    fn plan(&self) -> String {
        if self.solver.check() != z3::SatResult::Sat {
            panic!("Failed to find plan");
        }

        let model = self.solver.get_model();

        // Convert from solver model values to an instruction string
        let mut tape = String::new();
        for i in 0..self.instructions {
            let op = model.eval(&self.ops[i]).unwrap().as_u64().unwrap();
            let in_src = model.eval(&self.in_srcs[i]).unwrap().as_u64().unwrap() as usize;
            let out_dest = model.eval(&self.out_dests[i]).unwrap().as_bool().unwrap();

            let op = match op {
                0 => "AND",
                1 => "NOT",
                2 => "OR",
                _ => panic!("Invalid opcode"),
            };

            let lhs = if in_src == self.range {
                'T'
            } else if in_src == self.range + 1 {
                'J'
            } else {
                (b'A' + in_src as u8) as char
            };

            let rhs = if out_dest { 'T' } else { 'J' };

            tape += &format!("{} {} {}\n", op, lhs, rhs);
        }
        tape
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve(&input, 4, "WALK"));
    println!("Part 2: {}", solve(&input, 9, "RUN"));
}
