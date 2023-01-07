//Generate LLVM-IR

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManagerSubType;

use std::error::Error;

extern crate guess_host_triple;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    fn compile(&self) {
        let i8_type: inkwell::types::IntType = self.context.i8_type();
        let i32_type: inkwell::types::IntType = self.context.i32_type();
        let char_ptr: inkwell::types::PointerType = i8_type.ptr_type(inkwell::AddressSpace::from(0u16));

        let printf_type: inkwell::types::FunctionType = i32_type.fn_type(&[char_ptr.into()], true);
        let printf: inkwell::values::FunctionValue = self.module.add_function("printf", printf_type, Some(inkwell::module::Linkage::External));
        
        let fn_type: inkwell::types::FunctionType = i32_type.fn_type(&[], false);
        let function: inkwell::values::FunctionValue = self.module.add_function("main", fn_type, None);
        let basic_block: inkwell::basic_block::BasicBlock = self.context.append_basic_block(function, "entry");

        let mut attr: inkwell::attributes::Attribute = self.context.create_enum_attribute(inkwell::attributes::Attribute::get_named_enum_kind_id("noinline"), 0);

        function.add_attribute(inkwell::attributes::AttributeLoc::Function, attr);

        attr = self.context.create_enum_attribute(inkwell::attributes::Attribute::get_named_enum_kind_id("optnone"), 0);

        function.add_attribute(inkwell::attributes::AttributeLoc::Function, attr);

        self.builder.position_at_end(basic_block); 
        
        let mut msg: String = String::from("Hello, world!");
        msg.push('\0');

        let arr_type: inkwell::types::ArrayType = self.context.i8_type().array_type(msg.len()  as u32);
        
        
        let global: inkwell::values::GlobalValue = self.module.add_global(arr_type, None, "mystring");
        
        
        let mut arr: Vec<u8> = Vec::new();
        for chr in msg.as_bytes() {
            arr.push(chr.clone());
        }

        let vec: Vec<inkwell::values::IntValue> = arr.iter().map(|&x| {i8_type.const_int(x as u64, false)} ).collect::<Vec<_>>();

        let arr: inkwell::values::ArrayValue = i8_type.const_array(&vec[..]);

        global.set_initializer(&arr);
        global.set_constant(true);
        global.set_alignment(std::mem::size_of::<u8>() as u32);
        global.set_visibility(inkwell::GlobalVisibility::Default);
        global.set_unnamed_address(inkwell::values::UnnamedAddress::Global);
        
        let value: inkwell::values::PointerValue = unsafe { self.builder.build_in_bounds_gep(global.as_pointer_value(), &[i32_type.const_int(0, false), i32_type.const_int(0, false)], "val") };

        self.builder.build_call(printf, &[value.into()], "printf");

        self.builder.build_return(Some(&i32_type.const_int(0, false),));
    }
}

pub fn generate_code(module_name: &str, source_name: &str) -> Result<(), Box<dyn Error>> {
    let context: inkwell::context::Context = Context::create();
    let module: inkwell::module::Module = context.create_module(module_name);
    
    let mut triple: String = String::from("");
    guess_host_triple::guess_host_triple()
    .map(|t| triple = String::from(t))
    .unwrap_or_else(|| triple = String::from("unknown-unknown-unknown"));

    module.set_triple(&inkwell::targets::TargetTriple::create(triple.as_str()));
    module.set_source_file_name(source_name);

    let codegen: CodeGen = CodeGen {
        context: &context,
        module: module,
        builder: context.create_builder(),
    };

    let pass_manager_builder: inkwell::passes::PassManagerBuilder = inkwell::passes::PassManagerBuilder::create();
    pass_manager_builder.set_optimization_level(inkwell::OptimizationLevel::Aggressive);
    let manager: inkwell::passes::PassManager<Module> = inkwell::passes::PassManager::create(());
    pass_manager_builder.populate_module_pass_manager(&manager);

    codegen.compile();

    unsafe { codegen.module.run_in_pass_manager(&manager) };

    codegen.module.print_to_file(std::path::Path::new("output.ll"))?;

    Ok(())
}

fn main() {
    let res: Result<(), Box<dyn std::error::Error>> = generate_code("module", "filename");

    match res {
        Ok(_) => {}

        Err(err) => {
            panic!("{}",err.to_string());
        }
    }
}