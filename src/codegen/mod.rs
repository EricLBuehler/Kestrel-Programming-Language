//Generate LLVM-IR

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManagerSubType;

use std::error::Error;
use crate::parser;
mod types;
mod builtin_types;
use crate::errors;

extern crate guess_host_triple;

pub struct InkwellTypes<'ctx> {
    i32tp: &'ctx inkwell::types::IntType<'ctx>,
}

pub struct Namespaces<'ctx> {
    global: std::collections::HashMap<String, (inkwell::values::PointerValue<'ctx>, types::DataType)>,
}

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    types: std::collections::HashMap<String, types::Type<'ctx>>,
    info: &'ctx crate::fileinfo::FileInfo<'ctx>,
    inkwell_types: InkwellTypes<'ctx>,
    namespaces: Namespaces<'ctx>,
}

//Codegen functions
impl<'ctx> CodeGen<'ctx> {
    fn build_binary(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let binary: &parser::nodes::BinaryNode = node.data.binary.as_ref().unwrap();

        let left: types::Data = self.compile_expr(&binary.left);
        let right: types::Data = self.compile_expr(&binary.right);

        let mut args: Vec<&types::Data> = Vec::new();

        args.push(&left);
        args.push(&right);

        let tp: &types::Type = self.types.get(&left.tp.to_string()).unwrap();

        let traittp = match node.data.binary.as_ref().unwrap().op {
            parser::nodes::BinaryOpType::ADD => {
                types::TraitType::Add
            }
            parser::nodes::BinaryOpType::MUL => {
                types::TraitType::Mul
            }
            parser::nodes::BinaryOpType::SUB => {
                types::TraitType::Sub
            }
            parser::nodes::BinaryOpType::DIV => {
                types::TraitType::Div
            }
            _ => {
                unreachable!();
            }
        };

        let t: &types::Trait = match tp.traits.get(&traittp.to_string()) {
            Some (v) => {
                v
            }
            None => {
                let fmt: String = format!("type {} has no trait {}.", &left.tp.to_string(), &traittp.to_string());
                errors::raise_error(&fmt, errors::ErrorType::MissingTrait, &node.pos, self.info);
            }
        };

        let func = t.function;

        let data: types::Data = (func)(&self, args, &node.pos);

        return data;
    }
    
    fn build_let(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let right: types::Data = self.compile_expr(&node.data.letn.as_ref().unwrap().expr);

        let name: String = node.data.letn.as_ref().unwrap().name.clone();
        if self.namespaces.global.iter().find(|x| *x.0 == name) != None {
            let fmt: String = format!("name {} is already defined in namespace.", name);
            errors::raise_error(&fmt, errors::ErrorType::RedefinitionAttempt, &node.pos, self.info);
        }

        let ptr: inkwell::values::PointerValue = self.builder.build_alloca(right.data.unwrap().get_type(), name.as_str());

        self.builder.build_store(ptr, right.data.unwrap());

        self.namespaces.global.insert(name, (ptr, right.tp));

        let data: types::Data = types::Data {
            data: None,
            tp: types::DataType::Unit,
        };
        return data;
    }
    
    fn build_loadname(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let name: String = node.data.identifier.as_ref().unwrap().name.clone();

        if self.namespaces.global.iter().find(|x| *x.0 == name) == None {
        }

        let (ptr, tp) = match self.namespaces.global.get(&name) {
            None => {
                let fmt: String = format!("name {} is not defined in namespace.", name);
                errors::raise_error(&fmt, errors::ErrorType::RedefinitionAttempt, &node.pos, self.info);
            }
            Some(v) => {
                (v.0, v.1.clone())
            }
        };

        let data: types::Data = types::Data {
            data: Some(self.builder.build_load(ptr, &name)),
            tp,
        };
        return data;
    }

    fn compile_expr(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        match node.tp {
            parser::NodeType::I32 => {
                let self_data: &String = &node.data.int.as_ref().unwrap().left;
                let selfv: inkwell::values::IntValue = match self.inkwell_types.i32tp.const_int_from_string(self_data.as_str(), inkwell::types::StringRadix::Decimal) {
                    None => {
                        let fmt: String = format!("invalid i32 literal {}.", self_data);
                        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, &node.pos, self.info);
                    }
            
                    Some(v) => {
                        v
                    }
            
                };
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: types::DataType::I32}
            }
            parser::NodeType::BINARY => {
                self.build_binary(node)
            }
            parser::NodeType::LET => {
                self.build_let(node)
            }
            parser::NodeType::IDENTIFIER => {
                self.build_loadname(node)
            }
        }
    }

    fn compile(&mut self, mut nodes: Vec<parser::Node>) {
        // Generic header
        let i32_type: inkwell::types::IntType = self.context.i32_type();
        let fn_type: inkwell::types::FunctionType = i32_type.fn_type(&[], false);
        let main: inkwell::values::FunctionValue = self.module.add_function("main", fn_type, None);
        let basic_block: inkwell::basic_block::BasicBlock = self.context.append_basic_block(main, "entry");

        let mut attr: inkwell::attributes::Attribute = self.context.create_enum_attribute(inkwell::attributes::Attribute::get_named_enum_kind_id("noinline"), 0);

        main.add_attribute(inkwell::attributes::AttributeLoc::Function, attr);

        attr = self.context.create_enum_attribute(inkwell::attributes::Attribute::get_named_enum_kind_id("optnone"), 0);

        main.add_attribute(inkwell::attributes::AttributeLoc::Function, attr);
        
        self.builder.position_at_end(basic_block); 

        /////// Code generation start:
                
        for node in &mut nodes {
            self.compile_expr(node);
        }

        /////// End

        self.builder.build_return(Some(&i32_type.const_int(0, false))); //TODO: replace this with something user-defined
        
        let pass_manager_builder: inkwell::passes::PassManagerBuilder = inkwell::passes::PassManagerBuilder::create();
        pass_manager_builder.set_optimization_level(inkwell::OptimizationLevel::Aggressive);
        let manager = inkwell::passes::PassManager::create(&self.module);
        pass_manager_builder.populate_function_pass_manager(&manager);

        unsafe { main.run_in_pass_manager(&manager); }
    }
}

pub fn generate_code(module_name: &str, source_name: &str, nodes: Vec<parser::Node>, info: &crate::fileinfo::FileInfo) -> Result<(), Box<dyn Error>> {
    let context: inkwell::context::Context = Context::create();
    let module: inkwell::module::Module = context.create_module(module_name);
    
    let mut triple: String = String::from("");
    guess_host_triple::guess_host_triple()
    .map(|t| triple = String::from(t))
    .unwrap_or_else(|| triple = String::from("unknown-unknown-unknown"));

    module.set_triple(&inkwell::targets::TargetTriple::create(triple.as_str()));
    module.set_source_file_name(source_name);

    let inkwelltypes = InkwellTypes {
        i32tp: &context.i32_type(),
    };

    let namespaces: Namespaces = Namespaces {
        global: std::collections::HashMap::new(),
    };

    let mut codegen: CodeGen = CodeGen {
        context: &context,
        module: module,
        builder: context.create_builder(),
        types: std::collections::HashMap::new(),
        info,
        inkwell_types: inkwelltypes,
        namespaces: namespaces,
    };
    
    let pass_manager_builder: inkwell::passes::PassManagerBuilder = inkwell::passes::PassManagerBuilder::create();
    pass_manager_builder.set_optimization_level(inkwell::OptimizationLevel::Aggressive);
    let manager: inkwell::passes::PassManager<Module> = inkwell::passes::PassManager::create(());
    pass_manager_builder.populate_module_pass_manager(&manager);

    builtin_types::init(&mut codegen);



    codegen.compile(nodes);

    unsafe { codegen.module.run_in_pass_manager(&manager) };

    codegen.module.print_to_file(std::path::Path::new("a.ll"))?;

    std::process::Command::new("llc").arg("a.ll").output().expect("Failed to execute llc");

    std::process::Command::new("gcc").arg("a.s").arg("-oa").output().expect("Failed to execute llc");

    Ok(())
}