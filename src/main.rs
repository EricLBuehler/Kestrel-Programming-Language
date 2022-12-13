use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::OptimizationLevel;

use std::error::Error;

type SumFunc = unsafe extern "C" fn(u8) -> u8;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    fn compile(&self) -> Option<JitFunction<SumFunc>> {
        let i8_type = self.context.i8_type();
        let i32_type = self.context.i32_type();
        let char_ptr= i8_type.ptr_type(inkwell::AddressSpace::Generic);

        let printf_type = i32_type.fn_type(&[char_ptr.into()], true);
        let printf = self.module.add_function("printf", printf_type, Some(inkwell::module::Linkage::External));
        
        let fn_type = i32_type.fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        let mut attr = self.context.create_enum_attribute(inkwell::attributes::Attribute::get_named_enum_kind_id("noinline"), 0);

        function.add_attribute(inkwell::attributes::AttributeLoc::Function, attr);

        attr = self.context.create_enum_attribute(inkwell::attributes::Attribute::get_named_enum_kind_id("optnone"), 0);

        function.add_attribute(inkwell::attributes::AttributeLoc::Function, attr);

        self.builder.position_at_end(basic_block);    
        
        

        const MESSAGE: &str = "Hello, world!\n\0";
        let arr_type = self.context.i8_type().array_type(MESSAGE.len()  as u32);
        
        
        let global = self.module.add_global(arr_type, None, "mystring");
        
        
        let mut arr = Vec::new();
        for chr in MESSAGE.as_bytes() {
            arr.push(chr.clone());
        }

        let vec = arr.iter().map(|&x| {i8_type.const_int(x as u64, false)} ).collect::<Vec<_>>();

        let arr = i8_type.const_array(&vec[..]);

        global.set_initializer(&arr);
        global.set_constant(true);
        global.set_alignment(std::mem::size_of::<u8>() as u32);
        global.set_visibility(inkwell::GlobalVisibility::Default);
        global.set_unnamed_address(inkwell::values::UnnamedAddress::Global);
        
        let value = unsafe { self.builder.build_in_bounds_gep(global.as_pointer_value(), &[i32_type.const_int(0, false), i32_type.const_int(0, false)], "val") };

        self.builder.build_call(printf, &[value.into()], "printf");

        self.builder.build_return(Some(&i32_type.const_int(0, false),));

        unsafe { self.execution_engine.get_function("main").ok() }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("module");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
    let codegen = CodeGen {
        context: &context,
        module: module,
        builder: context.create_builder(),
        execution_engine,
    };

    let _ = codegen.compile().ok_or("Unable to JIT compile function")?;

    codegen.module.print_to_file(std::path::Path::new("output.ll"))?;

    Ok(())
}