extern crate inkwell;

use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::AddressSpace;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target};
use std::error::Error;

/// Convenience type alias for the `sum` function.
///
/// Calling this is innately `unsafe` because there's no guarantee it doesn't
/// do `unsafe` operations internally.
type RunFunction = unsafe extern "C" fn();

static mut i: usize = 0;

//  The callback should return 1 if we should terminate
unsafe extern "C" fn callback(reg: *mut i32) -> i32 {
    i += 1;
    println!("HIIIII {}", i);
    return (i == 1) as i32;
}

fn main() -> Result<(), Box<std::error::Error>> {
    Target::initialize_native(&InitializationConfig::default())?;

    let context = Context::create();
    let module = context.create_module("cb");
    let builder = context.create_builder();
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;

    /*  Install our global callback into the system */
    let i32_type = context.i32_type();
    let cb_type = i32_type.fn_type(&[i32_type.ptr_type(AddressSpace::Generic).into()], false);
    let cb_func = module.add_function("cb", cb_type, None);
    execution_engine.add_global_mapping(&cb_func, callback as usize);

    // Here is our JITted function
    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[cb_type.ptr_type(AddressSpace::Global).into()], false);
    let function = module.add_function("run", fn_type, None);

    // HEEEEERE WE GO
    let run_block = context.append_basic_block(&function, "run");
    builder.position_at_end(&run_block);

    // Install the block that lets us exit from the program
    let exit_block = context.append_basic_block(&function, "exit");
    builder.position_at_end(&exit_block);
    builder.build_return(None);
    builder.position_at_end(&run_block);

    let six = i32_type.const_int(6, false);
    let regs = builder.build_array_alloca(i32_type, six, "regs");

    let cb_result = builder.build_call(cb_func, &[regs.into()], "cb_call").try_as_basic_value().left().unwrap();
    builder.build_conditional_branch(*cb_result.as_int_value(), &exit_block, &run_block);

    module.print_to_stderr();

    let run_fn: JitFunction<RunFunction> = unsafe { execution_engine.get_function("run").ok() }.unwrap();
    unsafe { run_fn.call() };

    Ok(())
}
