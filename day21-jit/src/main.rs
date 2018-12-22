extern crate inkwell;

use std::io::{self, Read};
use std::collections::HashSet;

use inkwell::OptimizationLevel;
use inkwell::AddressSpace;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::targets::{InitializationConfig, Target};
use inkwell::values::{PointerValue, VectorValue, BasicValueEnum};
use inkwell::IntPredicate;

#[derive(Debug, Eq, PartialEq)]
struct Registers([usize; 6]);

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Op {
    Add,
    Mul,
    And,
    Or,
    Set,
    Gt,
    Eq,
}

#[derive(Eq, PartialEq, Debug)]
enum Source { Register, Immediate }

use crate::Op::*;
use crate::Source::*;
type Opcode = (Op, Source, Source);
fn str_to_opcode(s: &str) -> Opcode {
    match s {
        "addr" => (Add, Register, Register),
        "addi" => (Add, Register, Immediate),
        "mulr" => (Mul, Register, Register),
        "muli" => (Mul, Register, Immediate),
        "banr" => (And, Register, Register),
        "bani" => (And, Register, Immediate),
        "borr" => (Or, Register, Register),
        "bori" => (Or, Register, Immediate),
        "setr" => (Set, Register, Immediate),
        "seti" => (Set, Immediate, Immediate),
        "gtir" => (Gt, Immediate, Register),
        "gtri" => (Gt, Register, Immediate),
        "gtrr" => (Gt, Register, Register),
        "eqir" => (Eq, Immediate, Register),
        "eqri" => (Eq, Register, Immediate),
        "eqrr" => (Eq, Register, Register),
        _ => unimplemented!(),
    }
}

#[derive(Debug)]
struct Instruction {
    op: Opcode,
    a: usize,
    b: usize,
    c: usize,
    breakpoint: bool,
}

//  The callback should return 1 if we should terminate
static mut seen: Option<HashSet<i64>> = None;
static mut prev: i64 = 0;
unsafe extern "C" fn callback(reg: *const i64) -> i64 {
    if seen.is_none() {
        seen = Some(HashSet::new());
    }

    let target = *reg.offset(3);
    if prev == 0 {
        println!("{}", target);
    }
    if seen.as_ref().unwrap().contains(&target) {
        println!("{}", prev);
        return 1;
    }
    seen.as_mut().unwrap().insert(target);
    prev = target;
    return 0;
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut ip_reg = 0;
    let mut tape = buffer
        .lines()
        .filter_map(|line| {
            let words = line.split(' ').collect::<Vec<_>>();
            if words[0] == "#ip" {
                ip_reg = str::parse::<usize>(words[1]).unwrap();
                None
            } else {
                let op = str_to_opcode(words[0]);
                let a = str::parse::<usize>(words[1]).unwrap();
                let b = str::parse::<usize>(words[2]).unwrap();
                let c = str::parse::<usize>(words[3]).unwrap();
                Some(Instruction { op: op, a: a, b: b, c: c, breakpoint: false})
            }
        })
        .collect::<Vec<Instruction>>();
    tape[28].breakpoint = true;

    Target::initialize_native(&InitializationConfig::default())?;

    let context = Context::create();
    let module = context.create_module("cb");
    let builder = context.create_builder();
    let execution_engine = module.create_jit_execution_engine(
        OptimizationLevel::Aggressive)?;

    //  Install our global callback into the system
    let i64_type = context.i64_type();
    let cb_type = i64_type.fn_type(&[i64_type.array_type(6).ptr_type(AddressSpace::Generic).into()], false);
    let cb_func = module.add_function("cb", cb_type, None);
    execution_engine.add_global_mapping(&cb_func, callback as usize);

    // Here is our JITted function, which takes no arguments and returns void
    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);
    let function = module.add_function("jit", fn_type, None);

    // Set up the main blocks
    let setup_block = context.append_basic_block(&function, "setup");
    let jump_block = context.insert_basic_block_after(&setup_block, "jump_table");

    builder.position_at_end(&setup_block);
    let regs = builder.build_alloca(i64_type.array_type(6), "regs");

    let get = |i: usize| -> PointerValue {
        let reg_ptr = builder.build_ptr_to_int(regs, i64_type, "");
        let offset = i64_type.const_int((i * 8) as u64, false);
        let sum = builder.build_int_add(reg_ptr, offset, "");
        builder.build_int_to_ptr(sum, i64_type.ptr_type(AddressSpace::Generic), "")
    };

    for i in 0..6 {
        builder.build_store(get(i), i64_type.const_zero());
    }

    let mut instruction_blocks = Vec::new();
    for i in 0..tape.len() {
        instruction_blocks.push(
            context.insert_basic_block_after(
                if i == 0 { &setup_block }
                else {  instruction_blocks.last().unwrap() },
                &format!("i{}", i)));
    }
    let exit_block = context.insert_basic_block_after(
        instruction_blocks.last().unwrap(), "exit");

    builder.build_call(cb_func, &[regs.into()], "first_call");
    builder.build_unconditional_branch(&instruction_blocks[0]);

    // Write out the actual instructions
    for (i, line) in tape.iter().enumerate() {
        builder.position_at_end(&instruction_blocks[i]);

        let a = match line.op.1 {
            Source::Immediate => i64_type.const_int(line.a as u64, false),
            Source::Register  => *builder.build_load(get(line.a), "a")
                                         .as_int_value()
        };
        let b = match line.op.2 {
            Source::Immediate => i64_type.const_int(line.b as u64, false),
            Source::Register  => *builder.build_load(get(line.b), "b")
                                         .as_int_value()
        };

        let value = match line.op.0 {
            Add => builder.build_int_add(a, b, ""),
            Mul => builder.build_int_mul(a, b, ""),
            And => builder.build_and(a, b, ""),
            Or => builder.build_or(a, b, ""),
            Set => a,
            Gt => builder.build_int_z_extend(
                    builder.build_int_compare(IntPredicate::UGT, a, b, ""),
                    i64_type, ""),
            Eq => builder.build_int_z_extend(
                    builder.build_int_compare(IntPredicate::EQ, a, b, ""),
                    i64_type, ""),
        };
        builder.build_store(get(line.c), value);

        // Run the callback, exiting if it returns true
        if line.breakpoint {
            let cb_result = builder
                .build_call(cb_func, &[regs.into()], "cb_call")
                .try_as_basic_value()
                .left()
                .unwrap();
            builder.build_conditional_branch(
                *cb_result.as_int_value(), &exit_block, &jump_block);
        } else {
            builder.build_unconditional_branch(&jump_block);
        }
    }

    // Write out the jump table
    builder.position_at_end(&jump_block);
    let ip = *builder.build_load(get(ip_reg), "ip").as_int_value();
    let ip = builder.build_int_add(ip, i64_type.const_int(1, false), "");
    builder.build_store(get(ip_reg), ip);
    let mut jump_blocks = Vec::new();
    for i in 0..tape.len() {
        jump_blocks.push(
            context.insert_basic_block_after(
                if i == 0 { &jump_block }
                else {  jump_blocks.last().unwrap() },
                &format!("j{}", i)));
    }
    for i in 0..tape.len() {
        builder.position_at_end(&jump_blocks[i]);
        let eq = builder.build_int_compare(
            IntPredicate::EQ, ip, i64_type.const_int(i as u64, false), "");
        builder.build_conditional_branch(eq,
            &instruction_blocks[i], jump_blocks.get(i + 1).unwrap_or(&exit_block));
    }
    builder.position_at_end(&jump_block);
    builder.build_unconditional_branch(&jump_blocks[0]);

    // Install the block that lets us exit from the program
    builder.position_at_end(&exit_block);
    builder.build_call(cb_func, &[regs.into()], "final_call");
    builder.build_return(None);

    module.print_to_stderr();

    type RunFunction = unsafe extern "C" fn();
    println!("BUILDING\n");
    let run_fn: JitFunction<RunFunction> = unsafe { execution_engine.get_function("jit")? };
    println!("RUNNING\n");
    unsafe { run_fn.call() };

    Ok(())
}
