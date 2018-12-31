extern crate inkwell;

use std::io::{self, Read};
use std::collections::{HashSet, VecDeque, HashMap};

use inkwell::OptimizationLevel;
use inkwell::AddressSpace;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::targets::{InitializationConfig, Target};
use inkwell::IntPredicate;
use inkwell::values::{FunctionValue, PointerValue, PhiValue, IntValue};
use inkwell::types::{IntType, PointerType};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::module::Module;

#[derive(Debug, Eq, PartialEq)]
struct Registers([usize; 6]);

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
static mut SEEN: Option<HashSet<i64>> = None;
static mut PREV: i64 = 0;
unsafe extern "C" fn callback(reg: *const i64) -> bool {
    // Initialize static global
    if SEEN.is_none() {
        SEEN = Some(HashSet::new());
    }
    let seen = SEEN.as_mut().unwrap();

    let target = *reg.offset(3);
    if PREV == 0 && target != 0 {
        println!("Part 1: {}", target);
    }

    if seen.contains(&target) {
        println!("Part 2: {}", PREV);
        return true;
    } else {
        seen.insert(target);
        PREV = target;
        return false;
    }
}

fn build_setup_block(context: &Context,
                     module: &Module,
                     builder: &Builder,
                     i64_type: IntType) ->
    (BasicBlock, PointerValue, Vec<PointerValue>)
{
    println!("Creating function block");

    // Here is our JITted function, which takes no arguments and returns void
    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);
    let function = module.add_function("jit", fn_type, None);

    // The setup block initializes our registers to all zeros
    println!("  Initializing register array");
    let setup_block = context.append_basic_block(&function, "setup");
    builder.position_at_end(&setup_block);
    let reg_array = builder.build_alloca(i64_type.array_type(6), "reg_array");

    // Build an array of the register addresses, for store + load operations
    let reg = {
        let mut reg = Vec::new();
        let mut reg_ptr = builder.build_ptr_to_int(reg_array, i64_type, "reg_addr_int");
        let reg_offset = i64_type.const_int(8, false);
        for i in 0..6 {
            let r = builder.build_int_to_ptr(
                reg_ptr,
                i64_type.ptr_type(AddressSpace::Generic),
                &format!("reg{}", i));
            builder.build_store(r, i64_type.const_zero());
            reg.push(r);
            reg_ptr = builder.build_int_add(reg_ptr, reg_offset,
                                            &format!("reg_{}_addr_int", i));
        }
        reg
    };

    (setup_block, reg_array, reg)
}

////////////////////////////////////////////////////////////////////////////////

// IR generator for cases when all of the jumps are safe
fn build_safe_ir(context: &Context,
                 module: &Module,
                 builder: &Builder,
                 i64_type: IntType,

                 cb_func: FunctionValue,
                 tape: &Vec<Instruction>,
                 ip_reg: usize)
{
    let (setup_block, reg_array, reg) = build_setup_block(context, module, builder, i64_type);

    struct Block {
        phi: Vec<PhiValue>,
        input: Vec<IntValue>,
        output: Vec<IntValue>,
        block: BasicBlock,
    };

    // Each instruction gets one i block, plus an optional j block
    println!("  Creating instruction blocks");
    let mut instruction_blocks: Vec<Block> = Vec::new();
    for i in 0..tape.len() {
        let b = context.insert_basic_block_after(
                if i == 0 { &setup_block }
                else {  &instruction_blocks.last().unwrap().block },
                &format!("i{}", i));
        builder.position_at_end(&b);

        let mut block = Block {
            phi: Vec::new(),
            input: Vec::new(),
            output: Vec::new(),
            block: b
        };
        for j in 0..6 {
            if j == ip_reg {
                block.input.push(i64_type.const_int(i as u64, false));
            } else {
                let phi = builder.build_phi(i64_type, &format!("r{}_{}", j, i));
                if i == 0 {
                    phi.add_incoming(&[(&i64_type.const_int(0, false), &setup_block)]);
                }
                block.phi.push(phi);
                block.input.push(phi.as_basic_value().into_int_value());
            }
        }
        block.output = block.input.clone();
        instruction_blocks.push(block);
    }

    // Finally, the exit block is at the end of our instructions
    let exit_block = context.insert_basic_block_after(
        &instruction_blocks.last().unwrap().block, "exit");
    builder.position_at_end(&exit_block);
    let mut exit_phi = Vec::new();
    for j in 0..6 {
        if j != ip_reg {
            let phi = builder.build_phi(i64_type, &format!("r{}_exit", j));
            if instruction_blocks.len() == 0 {
                phi.add_incoming(&[(&i64_type.const_int(0, false),
                                    &setup_block)]);
            }
            exit_phi.push(phi);
        }
    }

    builder.position_at_end(&setup_block);
    builder.build_call(cb_func, &[reg_array.into()], "first_call");
    builder.build_unconditional_branch(
        instruction_blocks.get(0).map(|b| &b.block).unwrap_or(&exit_block));

    println!("  Writing instruction");
    for (i, line) in tape.iter().enumerate() {
        builder.position_at_end(&instruction_blocks[i].block);

        let a = match line.op.1 {
            Source::Immediate => i64_type.const_int(line.a as u64, false),
            Source::Register  => instruction_blocks[i].input[line.a],
        };
        let b = match line.op.2 {
            Source::Immediate => i64_type.const_int(line.b as u64, false),
            Source::Register  => instruction_blocks[i].input[line.b],
        };

        let name = format!("r{}_{}_", line.c, i);
        let value = match line.op.0 {
            Add => builder.build_int_add(a, b, &name),
            Mul => builder.build_int_mul(a, b, &name),
            And => builder.build_and(a, b, &name),
            Or => builder.build_or(a, b, &name),
            Set => a,
            Gt => builder.build_int_z_extend(
                    builder.build_int_compare(IntPredicate::UGT, a, b, ""),
                    i64_type, &name),
            Eq => builder.build_int_z_extend(
                    builder.build_int_compare(IntPredicate::EQ, a, b, ""),
                    i64_type, &name),
        };
        instruction_blocks[i].output[line.c] = value;

        let get = |t: usize, prev: &BasicBlock| -> &BasicBlock {
            let (phi, block) = instruction_blocks.get(t)
                .map(|b| (&b.phi, &b.block))
                .unwrap_or((&exit_phi, &exit_block));

                let mut k = 0;
                for j in 0..6 {
                    if j != ip_reg {
                        phi[k].add_incoming(&[(&instruction_blocks[i].output[j], prev)]);
                        k += 1;
                    }
                }
                block
        };

        // Build an optional jump block if this is a jumpy instruction
        let jump_block = if line.c == ip_reg {
            let jb = context.insert_basic_block_after(
                &instruction_blocks[i].block,
                &format!("i{}_jmp", i));
            builder.position_at_end(&jb);

            // If this is a fixed jump (from seti), then only add that target.
            if line.op.0 == Set && line.op.1 == Immediate {
                println!("    Found fixed absolute jump at {}", i);
                builder.build_unconditional_branch(get(line.a + 1, &jb));
            // If this is a jump with a fixed offset, then only add it
            } else if line.op.0 == Add && line.op.2 == Immediate {
                println!("    Found fixed relative jump at {}", i);
                builder.build_unconditional_branch(get(i + line.b + 1, &jb));
            // Otherwise, it must be jumping either 1 or 2 forward
            // (because otherwise we'd be calling the unsafe IR builder)
            } else if line.op.0 == Add {
                println!("    Found one-or-two jump at {}", i);
                let eq = builder.build_int_compare(
                    IntPredicate::EQ, value,
                    i64_type.const_int((i + 1) as u64, false),
                    &format!("cmp_{}_{}", i, i + 1));
                builder.build_conditional_branch(eq, get(i + 1, &jb),
                                                     get(i + 2, &jb));
            }
            builder.position_at_end(&instruction_blocks[i].block);
            Some(jb)
        } else {
            None
        };

        // If there's an indirect jump, then head there after the optional
        // callback (with a check to see if the callback requested an exit)
        let next = jump_block.as_ref()
            .unwrap_or_else(|| get(i + 1, &instruction_blocks[i].block));
        if line.breakpoint {
            let cb_result = builder
                .build_call(cb_func, &[reg_array.into()], "cb_call")
                .try_as_basic_value()
                .left()
                .unwrap();
            builder.build_conditional_branch(
                *cb_result.as_int_value(),
                get(instruction_blocks.len(), &instruction_blocks[i].block),
                next);
        } else {
            builder.build_unconditional_branch(next);
        }
    }

    println!("  Building exit block");
    builder.position_at_end(&exit_block);
    builder.build_call(cb_func, &[reg_array.into()], "final_call");
    builder.build_return(None);
}

////////////////////////////////////////////////////////////////////////////////

// IR generator for cases where jumps can't be proven safe
fn build_unsafe_ir(context: &Context,
                   module: &Module,
                   builder: &Builder,
                   i64_type: IntType,

                   cb_func: FunctionValue,
                   tape: &Vec<Instruction>,
                   ip_reg: usize)
{
    let (setup_block, reg_array, reg) = build_setup_block(context, module, builder, i64_type);

    // Each instruction gets one i block, plus an optional j block
    println!("  Creating instruction blocks");
    let mut instruction_blocks = Vec::new();
    for i in 0..tape.len() {
        instruction_blocks.push(
            context.insert_basic_block_after(
                if i == 0 { &setup_block }
                else {  instruction_blocks.last().unwrap() },
                &format!("i{}", i)));
    }

    // Finally, the exit block is at the end of our instructions
    let exit_block = context.insert_basic_block_after(
        instruction_blocks.last().unwrap(), "exit");

    builder.build_call(cb_func, &[reg_array.into()], "first_call");
    builder.build_unconditional_branch(&instruction_blocks[0]);

    // Write out the actual instructions
    println!("  Writing instruction");
    for (i, line) in tape.iter().enumerate() {
        builder.position_at_end(&instruction_blocks[i]);

        builder.build_store(reg[ip_reg], i64_type.const_int(i as u64, false));
        let a = match line.op.1 {
            Source::Immediate => i64_type.const_int(line.a as u64, false),
            Source::Register  => *builder.build_load(reg[line.a], "a")
                                         .as_int_value()
        };
        let b = match line.op.2 {
            Source::Immediate => i64_type.const_int(line.b as u64, false),
            Source::Register  => *builder.build_load(reg[line.b], "b")
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
        builder.build_store(reg[line.c], value);

        // Increment address register by 1
        let ip = *builder.build_load(reg[ip_reg], "ip").as_int_value();
        let ip = builder.build_int_add(ip, i64_type.const_int(1, false), "ip");
        builder.build_store(reg[ip_reg], ip);

        // If this is an instruction that could change the instruction
        // register, then we build a long list of conditional jumps (and
        // hope that the compiler optimizes it to a jump table).
        let jump_table_block = if line.c == ip_reg {
            // Decide which targets to put at the top of the jump table:
            let mut target_list = Vec::new();

            // If this is a fixed jump (from seti), then only add that target.
            if line.op.0 == Set && line.op.1 == Immediate {
                println!("    Found fixed absolute jump at {}", i);
                target_list.push(line.a + 1);
            // If this is a jump with a fixed offset, then only add it
            } else if line.op.0 == Add && line.op.2 == Immediate {
                println!("    Found fixed relative jump at {}", i);
                target_list.push(i + line.b + 1);
            // Otherwise, prioritize the next two slots
            } else if line.op.0 == Add {
                println!("    Found basic jump at {}", i);
                target_list.push(i + 1);
                target_list.push(i + 2);
                for j in 0..tape.len() {
                    if !target_list.contains(&j) {
                        target_list.push(j);
                    }
                }
            }

            // If we either got no targets or ended up with invalid targets,
            // then deploy the safe table (which includes every single target)
            if target_list.is_empty() || target_list.iter().any(|i| *i >= tape.len()) {
                println!("    Building expensive jump table at {}", i);
                target_list.clear();
                for i in 0..tape.len() {
                    target_list.push(i);
                }
            }

            // Create the blocks themselves
            let mut jump_blocks = VecDeque::new();
            for j in target_list.iter() {
                jump_blocks.push_back(
                    context.insert_basic_block_after(
                        if *j == target_list[0] {
                            &instruction_blocks[i]
                        } else {
                            jump_blocks.back().unwrap()
                        },
                        &format!("i{}j{}", i, j)));
            }
            // Build the logic within each block
            for j in 0..target_list.len() {
                builder.position_at_end(&jump_blocks[j]);
                let t = target_list[j];
                let eq = builder.build_int_compare(
                    IntPredicate::EQ, ip, i64_type.const_int(t as u64, false),
                    &format!("cmp_{}_{}", i, t));
                builder.build_conditional_branch(eq,
                    &instruction_blocks[t],
                    jump_blocks.get(j + 1)
                               .unwrap_or(&exit_block));
            }
            builder.position_at_end(&instruction_blocks[i]);
            Some(jump_blocks.pop_front().unwrap())
        } else {
            None
        };

        // If there's an indirect jump, then head there after the optional
        // callback (with a check to see if the callback requested an exit)
        let next = jump_table_block.as_ref().unwrap_or(
            instruction_blocks.get(i + 1).unwrap_or(
            &exit_block));
        if line.breakpoint {
            let cb_result = builder
                .build_call(cb_func, &[reg_array.into()], "cb_call")
                .try_as_basic_value()
                .left()
                .unwrap();
            builder.build_conditional_branch(
                *cb_result.as_int_value(), &exit_block, next);
        } else {
            builder.build_unconditional_branch(next);
        }
    }

    // Install the block that lets us exit from the program
    println!("  Building exit block");
    builder.position_at_end(&exit_block);
    builder.build_call(cb_func, &[reg_array.into()], "final_call");
    builder.build_return(None);

}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    println!("Parsing instructions...");
    let mut ip_reg = 0;
    let tape = buffer
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
                let bp = *words.get(4).unwrap_or(&"") == "#break";
                Some(Instruction { op: op, a: a, b: b, c: c, breakpoint: bp})
            }
        })
        .collect::<Vec<Instruction>>();
    println!("  Found {} instructions with {} breakpoints", tape.len(),
             tape.iter().filter(|i| i.breakpoint).count());

    let mut unsafe_landing_zones = HashSet::new();
    let mut jumps = HashMap::new();

    for (i, line) in tape.iter().enumerate() {
        if line.c == ip_reg {
            let t: &mut HashSet<_> = jumps.entry(i).or_insert(HashSet::new());
            if line.op.0 == Set && line.op.1 == Immediate {
                println!("Found jump from {} to {}", i, line.a + 1);
                t.insert(line.a + 1);
            } else if line.op.0 == Add && line.op.2 == Immediate {
                println!("Found jump from {} to {}", i, i + line.a + 1);
                t.insert(i + line.a + 1);
            } else if line.op.0 == Add && line.op.2 == Register && i > 0 &&
                (tape[i - 1].op.0 == Eq || tape[i - 1].op.0 == Gt) &&
                ((line.b == ip_reg && line.a == tape[i - 1].c) ||
                 (line.a == ip_reg && line.b == tape[i - 1].c))
            {
                println!("Found jump from {} to {} or {}", i, i + 1, i + 2);
                t.insert(i + 1);
                t.insert(i + 2);
                unsafe_landing_zones.insert(i);
            } else {
                println!("Found unconstrained jump");
                for j in 0..tape.len() {
                    t.insert(j);
                }
            }
        }
    }
    println!("{:?}\n{:?}\n", unsafe_landing_zones, jumps);
    let jump_targets = jumps.values().flat_map(|i| i).cloned().collect();
    let proved_safe =
        unsafe_landing_zones.intersection(&jump_targets).count() == 0;

    println!("Building JIT engine");
    Target::initialize_native(&InitializationConfig::default())?;

    let context = Context::create();
    let module = context.create_module("cb");
    let builder = context.create_builder();
    let execution_engine = module.create_jit_execution_engine(
        OptimizationLevel::Aggressive)?;

    let i64_type = context.i64_type();

    //  Install our global callback into the system
    let i1_type = context.custom_width_int_type(1);
    let cb_type = i1_type.fn_type(
        &[i64_type.array_type(6).ptr_type(AddressSpace::Generic).into()], false);
    let cb_func = module.add_function("cb", cb_type, None);
    execution_engine.add_global_mapping(&cb_func, callback as usize);

    if !proved_safe {
        println!("Cannot prove program safe; generating inefficient IR");
        build_unsafe_ir(&context, &module, &builder, i64_type,
                        cb_func, &tape, ip_reg);
    } else {
        println!("Proved program safe; generating efficient IR");
        build_safe_ir(&context, &module, &builder, i64_type,
                      cb_func, &tape, ip_reg);
    }
    module.print_to_stderr();

    println!("Compiling...");
    type RunFunction = unsafe extern "C" fn();
    let run_fn: JitFunction<RunFunction> = unsafe { execution_engine.get_function("jit")? };

    println!("Running...");
    unsafe { run_fn.call() };

    Ok(())
}
