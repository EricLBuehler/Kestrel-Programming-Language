use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use inkwell::values::{FunctionValue, AnyValue};

use std::error::Error;
use std::fs;
use std::io::Write;

type SumFunc = unsafe extern "C" fn(i64, i64, i64) -> i64;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    fn compile(&self) -> Option<(FunctionValue, JitFunction<SumFunc>)> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
        let function = self.module.add_function("main", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0)?.into_int_value();
        let y = function.get_nth_param(1)?.into_int_value();
        let z = function.get_nth_param(2)?.into_int_value();

        let sum = self.builder.build_int_add(x, y, "sum");
        let sum = self.builder.build_int_add(sum, z, "sum");

        self.builder.build_return(Some(&sum));

        return Some((function, unsafe { self.execution_engine.get_function("main").ok()? }));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("module");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
    let codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
        execution_engine,
    };

    let (func, sum) = codegen.compile().ok_or("Unable to JIT compile function")?;

    let x = 1i64;
    let y = 2i64;
    let z = 3i64;

    unsafe {
        println!("{} + {} + {} = {}", x, y, z, sum.call(x, y, z));
        assert_eq!(sum.call(x, y, z), x + y + z);
    }

    let output = func.print_to_string();

    let mut file = fs::File::create("output.ll")?;
    write!(file, "{}", output.to_string()).expect("Unable to write output to file.");
    drop(file);

    Ok(())
}