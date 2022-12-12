use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use inkwell::values::FunctionValue;

use std::error::Error;

type SumFunc = unsafe extern "C" fn(f64, f64, f64) -> f64;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    fn compile(&self) -> Option<(FunctionValue, JitFunction<SumFunc>)> {
        let f64_type = self.context.f64_type();
        let fn_type = f64_type.fn_type(&[f64_type.into(), f64_type.into(), f64_type.into()], false);
        let function = self.module.add_function("sum", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0)?.into_float_value();
        let y = function.get_nth_param(1)?.into_float_value();
        let z = function.get_nth_param(2)?.into_float_value();

        let sum = self.builder.build_float_add(x, y, "sum");
        let sum = self.builder.build_float_add(sum, z, "sum");

        self.builder.build_return(Some(&sum));

        return Some((function, unsafe { self.execution_engine.get_function("sum").ok()? }));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("sum");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
    let codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
        execution_engine,
    };

    let (func, sum) = codegen.compile().ok_or("Unable to JIT compile function")?;

    func.print_to_stderr();

    let x = 1f64;
    let y = 2f64;
    let z = 3f64;

    unsafe {
        println!("{} + {} + {} = {}", x, y, z, sum.call(x, y, z));
        assert_eq!(sum.call(x, y, z), x + y + z);
    }

    Ok(())
}