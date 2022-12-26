//Generate LLVM-IR

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManagerSubType;
use inkwell::types::AnyTypeEnum;
use inkwell::types::BasicType;
use crate::fileinfo;
use inkwell::debug_info::AsDIScope;
use inkwell::values::AggregateValue;

use core::panic;
use std::error::Error;
use crate::parser;
pub mod types;
mod builtin_types;
use crate::errors;

extern crate guess_host_triple;

pub struct InkwellTypes<'ctx> {
    i8tp: &'ctx inkwell::types::IntType<'ctx>,
    i16tp: &'ctx inkwell::types::IntType<'ctx>,
    i32tp: &'ctx inkwell::types::IntType<'ctx>,
    i64tp: &'ctx inkwell::types::IntType<'ctx>,
    i128tp: &'ctx inkwell::types::IntType<'ctx>,
    f32tp: &'ctx inkwell::types::FloatType<'ctx>,
    f64tp: &'ctx inkwell::types::FloatType<'ctx>,
    voidtp: &'ctx inkwell::types::VoidType<'ctx>,
}

pub struct Namespaces<'ctx> {
    locals: std::collections::HashMap<String, (Option<inkwell::values::PointerValue<'ctx>>, types::DataType, types::DataMutablility, types::DataOwnership)>,
    functions: std::collections::HashMap<String, (inkwell::values::FunctionValue<'ctx>, types::DataType)>,
    structs: std::collections::HashMap<String, (types::DataType, inkwell::types::AnyTypeEnum<'ctx>)>,
}

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    types: std::collections::HashMap<String, types::Type<'ctx>>,
    info: &'ctx fileinfo::FileInfo<'ctx>,
    inkwell_types: InkwellTypes<'ctx>,
    namespaces: Namespaces<'ctx>,
    dibuilder: inkwell::debug_info::DebugInfoBuilder<'ctx>,
    dicompile_unit: inkwell::debug_info::DICompileUnit<'ctx>,
    expected_rettp: Option<types::DataType>,
}

//Codegen functions
impl<'ctx> CodeGen<'ctx> {
    fn get_variable(&self, name: &String) -> Option<&(Option<inkwell::values::PointerValue<'ctx>>, types::DataType, types::DataMutablility, types::DataOwnership)>{
        if self.namespaces.locals.iter().find(|x| *x.0 == *name) != None {
            return self.namespaces.locals.get(name);
        }
        
        return None
    }
    
    fn get_function(&self, name: &String) -> Option<(inkwell::values::PointerValue<'ctx>, types::DataType)>{
        if self.namespaces.functions.iter().find(|x| *x.0 == *name) != None {
            return Some((self.namespaces.functions.get(name).unwrap().0.as_global_value().as_pointer_value(), self.namespaces.functions.get(name).unwrap().1.clone()));
        }

        return None;
    }
    
    fn build_struct_tp_from_types(ctx: &'ctx Context, inktypes: &InkwellTypes<'ctx>, types: &Vec<types::DataType>) -> inkwell::types::AnyTypeEnum<'ctx> {
        let mut basictypes: Vec<inkwell::types::BasicTypeEnum> = Vec::new();

        for tp in types {
            let any: Option<AnyTypeEnum> = Self::get_anytp_from_tp(ctx, &inktypes, tp.clone());
            
            if any.is_some() {
                let res: Option<inkwell::types::BasicTypeEnum> = Self::get_basic_from_any(any.unwrap());

                if res.is_some() {
                    basictypes.push(res.unwrap());
                }
            }
        }

        return inkwell::types::AnyTypeEnum::StructType(ctx.struct_type(&basictypes[..], false));
    }
    
    fn get_datatype_from_str(structs: &std::collections::HashMap<String, (types::DataType, inkwell::types::AnyTypeEnum<'ctx>)>, str_rep: &String) -> Option<types::DataType> {
        if *str_rep == types::BasicDataType::I32.to_string() {
            return Some(types::new_datatype(types::BasicDataType::I32, types::BasicDataType::I32.to_string(), None, Vec::new(), Vec::new(), None, false));
        }
        else if *str_rep == types::BasicDataType::U32.to_string() {
            return Some(types::new_datatype(types::BasicDataType::U32, types::BasicDataType::U32.to_string(), None, Vec::new(), Vec::new(), None, false));
        }
        else if *str_rep == types::BasicDataType::I8.to_string() {
            return Some(types::new_datatype(types::BasicDataType::I8, types::BasicDataType::I8.to_string(), None, Vec::new(), Vec::new(), None, false));
        }
        else if *str_rep == types::BasicDataType::U8.to_string() {
            return Some(types::new_datatype(types::BasicDataType::U8, types::BasicDataType::U8.to_string(), None, Vec::new(), Vec::new(), None, false));
        }
        else if *str_rep == types::BasicDataType::I16.to_string() {
            return Some(types::new_datatype(types::BasicDataType::I16, types::BasicDataType::I16.to_string(), None, Vec::new(), Vec::new(), None, false));
        }
        else if *str_rep == types::BasicDataType::U16.to_string() {
            return Some(types::new_datatype(types::BasicDataType::U16, types::BasicDataType::U16.to_string(), None, Vec::new(), Vec::new(), None, false));
        }
        else if *str_rep == types::BasicDataType::I64.to_string() {
            return Some(types::new_datatype(types::BasicDataType::I64, types::BasicDataType::I64.to_string(), None, Vec::new(), Vec::new(), None, false));
        }
        else if *str_rep == types::BasicDataType::U64.to_string() {
            return Some(types::new_datatype(types::BasicDataType::U64, types::BasicDataType::U64.to_string(), None, Vec::new(), Vec::new(), None, false));
        }
        else if *str_rep == types::BasicDataType::I128.to_string() {
            return Some(types::new_datatype(types::BasicDataType::I128, types::BasicDataType::I128.to_string(), None, Vec::new(), Vec::new(), None, false));
        }
        else if *str_rep == types::BasicDataType::U128.to_string() {
            return Some(types::new_datatype(types::BasicDataType::U128, types::BasicDataType::U128.to_string(), None, Vec::new(), Vec::new(), None, false));
        }
        else if *str_rep == types::BasicDataType::F32.to_string() {
            return Some(types::new_datatype(types::BasicDataType::F32, types::BasicDataType::F32.to_string(), None, Vec::new(), Vec::new(), None, false));
        }
        else if *str_rep == types::BasicDataType::F64.to_string() {
            return Some(types::new_datatype(types::BasicDataType::F64, types::BasicDataType::F64.to_string(), None, Vec::new(), Vec::new(), None, false));
        }
        else if *str_rep == types::BasicDataType::Unit.to_string() {
            return Some(types::new_datatype(types::BasicDataType::Unit, types::BasicDataType::Unit.to_string(), None, Vec::new(), Vec::new(), None, false));
        }
        else if structs.get(str_rep).is_some() {
            return Some(structs.get(str_rep).unwrap().0.clone());
        }

        return None;
    }

    fn get_anytp_from_tp(ctx: &'ctx Context, types: &InkwellTypes<'ctx>, tp: types::DataType) -> Option<inkwell::types::AnyTypeEnum<'ctx>> {
        match tp.tp {
            types::BasicDataType::I32 |
            types::BasicDataType::U32 => {
                return Some(inkwell::types::AnyTypeEnum::IntType(*types.i32tp));
            }
            types::BasicDataType::I8 |
            types::BasicDataType::U8 => {
                return Some(inkwell::types::AnyTypeEnum::IntType(*types.i8tp));
            }
            types::BasicDataType::I16 |
            types::BasicDataType::U16 => {
                return Some(inkwell::types::AnyTypeEnum::IntType(*types.i16tp));
            }
            types::BasicDataType::I64 |
            types::BasicDataType::U64 => {
                return Some(inkwell::types::AnyTypeEnum::IntType(*types.i64tp));
            }
            types::BasicDataType::I128 |
            types::BasicDataType::U128 => {
                return Some(inkwell::types::AnyTypeEnum::IntType(*types.i128tp));
            }
            types::BasicDataType::F32 => {
                return Some(inkwell::types::AnyTypeEnum::FloatType(*types.f32tp));
            }
            types::BasicDataType::F64 => {
                return Some(inkwell::types::AnyTypeEnum::FloatType(*types.f64tp));
            }
            types::BasicDataType::Unit => {
                return Some(inkwell::types::AnyTypeEnum::VoidType(*types.voidtp));
            }
            types::BasicDataType::Func => {
                return None;
            }
            types::BasicDataType::Struct => {
                return Some(Self::build_struct_tp_from_types(ctx, types, &tp.types));
            }
            types::BasicDataType::Unknown => {
                return None;
            }    
        }
    }

    pub fn get_llvm_from_type(ctx: &'ctx Context, structs: &std::collections::HashMap<String, (types::DataType, inkwell::types::AnyTypeEnum<'ctx>)>, types: &InkwellTypes<'ctx>, info: &fileinfo::FileInfo, arg: &parser::Type, node: &parser::Node) -> (types::DataType, inkwell::types::AnyTypeEnum<'ctx>) {
        if arg.isfn {
            let args: &Vec<parser::Type> = &arg.args.as_ref().unwrap().args;
            let mut datatypes: Vec<types::DataType> = Vec::new();
            let mut mutability: Vec<types::DataMutablility> = Vec::new();
            let mut inktypes: Vec<inkwell::types::BasicMetadataTypeEnum> = Vec::new();
            
            for arg in args {
                let (data, tp) = Self::get_llvm_from_type(ctx, structs, types, info, &arg, node);
                datatypes.push(data);
                mutability.push(arg.mutability);
                let res: Option<inkwell::types::BasicMetadataTypeEnum> = Self::get_basicmeta_from_any(tp);

                if res.is_some() {
                    inktypes.push(res.unwrap());
                }
            }
            
            let rettp_full: (types::DataType, inkwell::types::AnyTypeEnum) = Self::get_llvm_from_type(ctx, structs, types, info, &arg.args.as_ref().unwrap().rettp.last().unwrap(), node);
            let tp: inkwell::types::AnyTypeEnum = rettp_full.1;
            let fntp: inkwell::types::FunctionType;
            
            if tp.is_int_type() {
                fntp = tp.into_int_type().fn_type(&inktypes[..], false);
            }
            else if tp.is_float_type() {
                fntp = tp.into_float_type().fn_type(&inktypes[..], false);
            }
            else if tp.is_function_type() {
                fntp = tp.into_function_type().ptr_type(inkwell::AddressSpace::Generic).fn_type(&inktypes[..], false);
            }
            else if tp.is_void_type() {
                fntp = tp.into_void_type().fn_type(&inktypes[..], false);
            }
            else if tp.is_struct_type() {
                fntp = tp.into_struct_type().fn_type(&inktypes[..], false);
            }
            else {
                panic!("Unexpected type");
            }

            let mut names: Option<Vec<String>> = None;
            if node.tp == parser::NodeType::FUNC {
                names=Some(node.data.func.as_ref().unwrap().args.name.clone());
            }

            return (types::new_datatype(types::BasicDataType::Func, types::BasicDataType::Func.to_string(), names, datatypes, mutability, Some(rettp_full.0.clone()), false), inkwell::types::AnyTypeEnum::FunctionType(fntp));
        }
        else {
            let tp: Option<types::DataType> = Self::get_datatype_from_str(structs ,&arg.data.as_ref().unwrap());
            if tp.is_none() {
                let fmt: String = format!("Unknown type '{}'.", &arg.data.as_ref().unwrap());
                errors::raise_error(&fmt, errors::ErrorType::UnknownType, &node.pos, info);
            }
            let anytp: Option<inkwell::types::AnyTypeEnum> = Self::get_anytp_from_tp(ctx, &types, tp.as_ref().unwrap().clone());
            if anytp.is_none() {
                unimplemented!();
            }
            return (tp.unwrap(), anytp.unwrap());
        }
    }

    fn mangle_name_main(&self, name: &String) -> String {
        let mut new: String = name.clone();
        if *name == String::from("main") {
            new = String::from("_") + new.as_str();      
        }
        return new;
    }

    fn get_type_from_data(&self, data: &types::Data) -> &types::Type<'ctx> {
        return self.types.get(&data.tp.to_string()).unwrap();
    }

    fn get_basicmeta_from_any(tp: inkwell::types::AnyTypeEnum<'ctx>) -> Option<inkwell::types::BasicMetadataTypeEnum> {
        if tp.is_int_type() {
            return Some(inkwell::types::BasicMetadataTypeEnum::IntType(tp.into_int_type()));
        }
        else if tp.is_float_type() {
            return Some(inkwell::types::BasicMetadataTypeEnum::FloatType(tp.into_float_type()));
        }
        else if tp.is_function_type() {
            return Some(inkwell::types::BasicMetadataTypeEnum::PointerType(tp.into_function_type().ptr_type(inkwell::AddressSpace::Generic)));
        }
        else if tp.is_void_type() {
            return None;
        }
        else if tp.is_struct_type() {
            return Some(inkwell::types::BasicMetadataTypeEnum::StructType(tp.into_struct_type()));
        }
        else {
            panic!("Unexpected type");
        }
    }

    fn get_basic_from_any(tp: inkwell::types::AnyTypeEnum<'ctx>) -> Option<inkwell::types::BasicTypeEnum> {
        if tp.is_int_type() {
            return Some(inkwell::types::BasicTypeEnum::IntType(tp.into_int_type()));
        }
        else if tp.is_float_type() {
            return Some(inkwell::types::BasicTypeEnum::FloatType(tp.into_float_type()));
        }
        else if tp.is_function_type() {
            return Some(inkwell::types::BasicTypeEnum::PointerType(tp.into_function_type().ptr_type(inkwell::AddressSpace::Generic)));
        }
        else if tp.is_void_type() {
            return None;
        }
        else if tp.is_struct_type() {
            return Some(inkwell::types::BasicTypeEnum::StructType(tp.into_struct_type()));
        }
        else {
            panic!("Unexpected type");
        }
    }

    fn build_binary(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let binary: &parser::nodes::BinaryNode = node.data.binary.as_ref().unwrap();

        let left: types::Data = self.compile_expr(&binary.left, false);
        let right: types::Data = self.compile_expr(&binary.right, false);

        let mut args: Vec<types::Data> = Vec::new();

        let tp: &types::Type = self.get_type_from_data(&left);

        let tp_str: &String = &left.tp.name.clone();

        args.push(left);
        args.push(right);

        let traittp: types::TraitType = match node.data.binary.as_ref().unwrap().op {
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
        };

        let t: &types::Trait = match tp.traits.get(&traittp.to_string()) {
            Some (v) => {
                v
            }
            None => {
                let fmt: String = format!("Type '{}' has no trait '{}'.", tp_str, &traittp.to_string());
                errors::raise_error(&fmt, errors::ErrorType::MissingTrait, &node.pos, self.info);
            }
        };

        let func = t.function;

        let data: types::Data = (func)(&self, args, &node.pos);

        return data;
    }
    
    fn build_let(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let right: types::Data = self.compile_expr(&node.data.letn.as_ref().unwrap().expr, true);

        let name: String = node.data.letn.as_ref().unwrap().name.clone();
        if self.get_variable(&name) != None {
            let fmt: String = format!("Name '{}' is already defined in namespace.", name);
            errors::raise_error(&fmt, errors::ErrorType::RedefinitionAttempt, &node.pos, self.info);
        }

        if right.data == None{
            let fmt: String = format!("Cannot assign to '{}'.", right.tp.to_string());
            errors::raise_error(&fmt, errors::ErrorType::CannotAssign, &node.pos, self.info);
        }

        let ptr: inkwell::values::PointerValue = self.builder.build_alloca(right.data.unwrap().get_type(), name.as_str());

        self.builder.build_store(ptr, right.data.unwrap());

        let mut tp: types::DataType = right.tp;
        let rt_tp: types::DataType = tp.clone();
        if node.data.letn.as_ref().unwrap().tp != None {
            (tp, _) = Self::get_llvm_from_type(&self.context, &self.namespaces.structs, &self.inkwell_types, self.info, &node.data.letn.as_ref().unwrap().tp.as_ref().unwrap(), node);
            if tp != rt_tp {
                let fmt: String = format!("Expected '{}' type, got '{}' type.", tp.to_string(), rt_tp.to_string());
                errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
            }
        }

        self.namespaces.locals.insert(name, (Some(ptr), tp, node.data.letn.as_ref().unwrap().mutability, types::DataOwnership {owned: true, transferred: None}));

        let data: types::Data = types::Data {
            data: None,
            tp: types::new_datatype(types::BasicDataType::Unit, types::BasicDataType::Unit.to_string(), None, Vec::new(), Vec::new(), None, false),
            owned: true,
        };
        return data;
    }
    
    fn build_loadname(&mut self, node: &parser::Node, give_ownership: bool) -> types::Data<'ctx> {
        let name: String = node.data.identifier.as_ref().unwrap().name.clone();

        let (ptr, tp) = match self.get_variable(&name) {
            None => {
                let res: Option<(inkwell::values::PointerValue, types::DataType)> = self.get_function(&name);
                if res==None {
                    let fmt: String = format!("Name '{}' is not defined.", name);
                    errors::raise_error(&fmt, errors::ErrorType::NameNotFound, &node.pos, self.info);
                }
                let data: types::Data = types::Data {
                    data: Some(inkwell::values::BasicValueEnum::PointerValue(res.as_ref().unwrap().0)),
                    tp: res.unwrap().1,
                    owned: true,
                };
                return data;
            }
            Some(v) => {
                if !self.get_variable(&name).unwrap().3.owned {
                    let transferred: String = String::from("Name '{}' was transferred here.");
                    let fmt: String = format!("Name '{}' is not owned.", name);
                    errors::raise_error_multi(errors::ErrorType::NameNotOwned, vec![transferred, fmt], vec![&self.get_variable(&name).unwrap().3.transferred.as_ref().unwrap(), &node.pos], self.info);
                }
                (v.0, v.1.clone())
            }
        };

        let owner: types::DataOwnership = self.get_variable(&name).unwrap().3.clone();

        if give_ownership {
            self.namespaces.locals.insert(name.clone(), (self.get_variable(&name).unwrap().0, self.get_variable(&name).unwrap().1.clone(), self.get_variable(&name).unwrap().2, types::DataOwnership {owned: false, transferred: Some(node.pos.clone())}));
        }

        if ptr.is_some() {
            let data: types::Data = types::Data {
                data: Some(self.builder.build_load(ptr.unwrap(), name.as_str())),
                tp,
                owned: owner.owned,
            };
            return data;
        }
        let data: types::Data = types::Data {
            data: None,
            tp,
            owned: true,
        };
        return data;
    }
    
    fn build_func(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let name: &String = &node.data.func.as_ref().unwrap().name;
        if self.get_function(&name) != None {
            let fmt: String = format!("Function '{}' is already defined.", name);
            errors::raise_error(&fmt, errors::ErrorType::RedefinitionAttempt, &node.pos, self.info);
        }

        // Argument and return types
        let args = &node.data.func.as_ref().unwrap().args;

        let mut datatypes: Vec<types::DataType> = Vec::new();
        let mut mutability: Vec<types::DataMutablility> = Vec::new();
        let mut inktypes: Vec<inkwell::types::BasicMetadataTypeEnum> = Vec::new();

        for arg in &args.args {
            let (data, tp) = Self::get_llvm_from_type(&self.context, &self.namespaces.structs, &self.inkwell_types, &self.info, &arg, node);
            datatypes.push(data);
            mutability.push(arg.mutability);


            let res: Option<inkwell::types::BasicMetadataTypeEnum> = Self::get_basicmeta_from_any(tp);

            if res.is_some() {
                inktypes.push(res.unwrap());
            }
        }
        
        let rettp_full: (types::DataType, inkwell::types::AnyTypeEnum) = Self::get_llvm_from_type(&self.context, &self.namespaces.structs, &self.inkwell_types, &self.info, &args.rettp.last().unwrap(), node);

        self.expected_rettp = Some(rettp_full.0.clone());
        
        let tp: inkwell::types::AnyTypeEnum = rettp_full.1;
        let fn_type: inkwell::types::FunctionType;
        
        if tp.is_int_type() {
            fn_type = tp.into_int_type().fn_type(&inktypes[..], false);
        }
        else if tp.is_float_type() {
            fn_type = tp.into_float_type().fn_type(&inktypes[..], false);
        }
        else if tp.is_function_type() {
            fn_type = tp.into_function_type().ptr_type(inkwell::AddressSpace::Generic).fn_type(&inktypes[..], false);
        }
        else if tp.is_void_type() {
            fn_type = tp.into_void_type().fn_type(&inktypes[..], false);
        }
        else if tp.is_struct_type() {
            fn_type = tp.into_struct_type().fn_type(&inktypes[..], false);
        }
        else {
            panic!("Unexpected type");
        }

        //Main function specifics
        let mangled_name = self.mangle_name_main(&name);
        if self.get_function(&mangled_name) != None {
            let fmt: String = format!("Mangled function 'main' name '{}' is already defined.", mangled_name);
            errors::raise_error(&fmt, errors::ErrorType::RedefinitionAttempt, &node.pos, self.info);
        }
        if name == "main" {
            if datatypes.len() != 0 {
                let fmt: String = format!("Expected 0 arguments, got {}.", datatypes.len());
                errors::raise_error(&fmt, errors::ErrorType::ArgumentCountMismatch, &node.pos, self.info);
            }

            if fn_type.get_return_type() != None {
                let fmt: String = format!("Expected 'void' return type, got '{}'.", &rettp_full.0.name);
                errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
            }
        }
        //

        let func: inkwell::values::FunctionValue = self.module.add_function(mangled_name.as_str(), fn_type, None);

        
        self.namespaces.functions.insert(name.clone(), (func, types::new_datatype(types::BasicDataType::Func, types::BasicDataType::Func.to_string(), Some(node.data.func.as_ref().unwrap().args.name.clone()), datatypes.clone(), mutability.clone(), Some(rettp_full.0.clone()), false)));
        
        // Add debug information
        let mut diparamtps: Vec<inkwell::debug_info::DIType> = Vec::new();

        let direttp: inkwell::debug_info::DIBasicType = self.dibuilder.create_basic_type(
            tp.print_to_string().to_str().unwrap(),
            std::mem::size_of_val(&tp) as u64,
            0x00,
            inkwell::debug_info::DIFlagsConstants::PUBLIC).unwrap();

        for tp in fn_type.get_param_types() {
            diparamtps.push(self.dibuilder.create_basic_type(
                tp.print_to_string().to_str().unwrap(),
                std::mem::size_of_val(&tp) as u64,
                0x00,
                inkwell::debug_info::DIFlagsConstants::PUBLIC).unwrap().as_type());
        }

        let sub_type = self.dibuilder.create_subroutine_type(
            self.dicompile_unit.get_file(),
            Some(direttp.as_type()),
            &diparamtps[..],
            inkwell::debug_info::DIFlagsConstants::PUBLIC);

        let func_scope: inkwell::debug_info::DISubprogram = self.dibuilder.create_function(
            self.dicompile_unit.as_debug_info_scope(),
            name,
            Some(&mangled_name),
            self.dicompile_unit.get_file(),
            node.pos.line as u32,
            sub_type,
            true, //Needs to be dynamic
            true,
            node.pos.line as u32,
            inkwell::debug_info::DIFlagsConstants::PUBLIC,
            true);

        func.set_subprogram(func_scope);

        let lexical_block = self.dibuilder.create_lexical_block(
            func_scope.as_debug_info_scope(),
            self.dicompile_unit.get_file(),
            node.pos.line as u32,
            node.pos.startcol as u32);

        let location = self.dibuilder.create_debug_location(
            self.context,
            node.pos.line as u32,
            node.pos.startcol as u32,
            lexical_block.as_debug_info_scope(),
            None);


        //Continue function compilation
        let basic_block: inkwell::basic_block::BasicBlock = self.context.append_basic_block(func, "entry");
        self.builder.set_current_debug_location(self.context, location);

        let mut attr: inkwell::attributes::Attribute = self.context.create_enum_attribute(inkwell::attributes::Attribute::get_named_enum_kind_id("noinline"), 0);

        func.add_attribute(inkwell::attributes::AttributeLoc::Function, attr);

        attr = self.context.create_enum_attribute(inkwell::attributes::Attribute::get_named_enum_kind_id("optnone"), 0);

        func.add_attribute(inkwell::attributes::AttributeLoc::Function, attr);
        
        self.builder.position_at_end(basic_block); 

        
        //Setup locals
        let prev_locals: std::collections::HashMap<String, (Option<inkwell::values::PointerValue>, types::DataType, types::DataMutablility, types::DataOwnership)> = self.namespaces.locals.to_owned();
        self.namespaces.locals = std::collections::HashMap::new();
        
        //Setup arguments
        let mut idx: u32 = 0;
        let mut idx_mut: usize = 0;
        for (name, tp) in std::iter::zip(&args.name, &datatypes) { 
            let mut argv: Option<inkwell::values::BasicValueEnum> = None;
            if *tp != types::BasicDataType::Unit {
                argv = func.get_nth_param(idx);
                idx += 1;
            }

            let ptr: inkwell::values::PointerValue;
            if argv.is_some() {
                ptr = self.builder.build_alloca(argv.unwrap().get_type(), name.as_str());
            
                self.builder.build_store(ptr, argv.unwrap());

                self.namespaces.locals.insert(name.to_string(), (Some(ptr), tp.clone(), mutability.get(idx_mut).unwrap().clone(), types::DataOwnership {owned: true, transferred: None}));
            }
            else {
                self.namespaces.locals.insert(name.to_string(), (None, tp.clone(), types::DataMutablility::Immutable, types::DataOwnership {owned: true, transferred: None}));
            }
            idx_mut += 1;
        }

        /////// Code generation start:

        let retv: types::Data = self.compile(&node.data.func.as_ref().unwrap().blocks, true);
        
        //Reset locals
        self.namespaces.locals = prev_locals;

        /////// End
        
        //Check if last stmt. is a return

        if node.data.func.as_ref().unwrap().blocks.len()==0 || node.data.func.as_ref().unwrap().blocks.last().unwrap().tp != parser::NodeType::RETURN {
            if retv.tp != rettp_full.0.tp && name!="main"{
                let fmt: String = format!("Expected '{}' return type, got '{}'.", &rettp_full.0.name, retv.tp.name);
                errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
            }


            if rettp_full.0.tp != types::BasicDataType::Unit {
                self.builder.build_return(Some(&retv.data.unwrap())); 
            }
            else {
                self.builder.build_return(None);
            }
        }
        
        let pass_manager_builder: inkwell::passes::PassManagerBuilder = inkwell::passes::PassManagerBuilder::create();
        pass_manager_builder.set_optimization_level(inkwell::OptimizationLevel::Aggressive);
        let manager = inkwell::passes::PassManager::create(&self.module);
        pass_manager_builder.populate_function_pass_manager(&manager);

        unsafe { func.run_in_pass_manager(&manager); }
        if node.data.func.as_ref().unwrap().blocks.len() > 0 && !retv.owned {
            let fmt: String = format!("Return value is not owned.");
            errors::raise_error(&fmt, errors::ErrorType::ReturnValueNotOwned, &node.data.func.as_ref().unwrap().blocks.last().unwrap().pos, self.info);
        }
        
        let data: types::Data = types::Data {
            data: Some(inkwell::values::BasicValueEnum::PointerValue(func.as_global_value().as_pointer_value())),
            tp: self.namespaces.functions.get(&name.clone()).unwrap().1.clone(),
            owned: true,
        };
        return data;
    }
    
    fn build_assign(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let right: types::Data = self.compile_expr(&node.data.assign.as_ref().unwrap().expr, true);

        let name: String = node.data.assign.as_ref().unwrap().name.clone();

        if right.data == None{
            let fmt: String = format!("Cannot assign to '{}'.", right.tp.to_string());
            errors::raise_error(&fmt, errors::ErrorType::CannotAssign, &node.pos, self.info);
        }

        if self.namespaces.locals.get(&name).unwrap().2 == types::DataMutablility::Immutable {
            let fmt: String = format!("Cannot assign to immutable variable.");
            errors::raise_error(&fmt, errors::ErrorType::ImmutableAssign, &node.pos, self.info);
        }

        if self.namespaces.locals.get(&name).unwrap().1 != right.tp {
            let fmt: String = format!("Expected '{}' type, got '{}' type.", self.namespaces.locals.get(&name).unwrap().1.tp.to_string(), right.tp.to_string());
            errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
        }

        let ptr: Option<inkwell::values::PointerValue> = self.namespaces.locals.get(&name).unwrap().0;

        if ptr.is_some() {
            self.builder.build_store(ptr.unwrap(), right.data.unwrap());

            self.namespaces.locals.insert(name, (ptr, right.tp.clone(), types::DataMutablility::Mutable, types::DataOwnership {owned: true, transferred: None}));
        }

        return right;
    }
    
    fn build_call(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let callable: types::Data = self.compile_expr(&node.data.call.as_ref().unwrap().name, false);

        let mut args: Vec<types::Data> = Vec::new();
        let tp_name: &String = &callable.tp.name.clone();
        args.push(callable);

        for arg in &node.data.call.as_ref().unwrap().args{
            let v: types::Data = self.compile_expr(arg, true); 
            args.push(v);
        }

        let tp: &types::Type = self.get_type_from_data(&args.first().unwrap());

        let t: &types::Trait = match tp.traits.get(&types::TraitType::Call.to_string()) {
            Some (v) => {
                v
            }
            None => {
                let fmt: String = format!("Type '{}' has no trait '{}'.", tp_name, &types::TraitType::Call.to_string());
                errors::raise_error(&fmt, errors::ErrorType::MissingTrait, &node.pos, self.info);
            }
        };

        let func = t.function;

        let data: types::Data = (func)(&self, args, &node.pos);

        return data;
    }

    fn build_return(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let retv: types::Data = self.compile_expr(&node.data.ret.as_ref().unwrap().expr, true);

        if self.expected_rettp==None {
            let fmt: String = format!("Cannot return outside of function.");
            errors::raise_error(&fmt, errors::ErrorType::ReturnOutsideOfFunction, &node.pos, self.info);            
        }

        if retv.tp != *self.expected_rettp.as_ref().unwrap() {
            let fmt: String = format!("Expected '{}' return type, got '{}'.", &self.expected_rettp.as_ref().unwrap().name, retv.tp.name);
            errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
        }


        if retv.data.is_some() {
            if !retv.owned {
                let fmt: String = format!("Return value is not owned.");
                errors::raise_error(&fmt, errors::ErrorType::ReturnValueNotOwned, &node.data.ret.as_ref().unwrap().expr.pos, self.info);
            }
            self.builder.build_return(Some(&retv.data.unwrap())); 
        }
        else {
            self.builder.build_return(None);
        }

        
        return retv;
    }

    fn build_to(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let left: types::Data = self.compile_expr(&node.data.to.as_ref().unwrap().left, false);     
        let arg: &parser::Type = &node.data.to.as_ref().unwrap().tp;  
        if arg.isfn {
            let fmt: String = format!("Non primitive cast from '{}' to 'fn'.", left.tp.name);
            errors::raise_error(&fmt, errors::ErrorType::InvalidCast, &node.pos, self.info);
        }
        let tp_name: &String = &arg.data.as_ref().unwrap();

        let tp: types::DataType = if Self::get_datatype_from_str(&self.namespaces.structs, tp_name).is_none() {
            let fmt: String = format!("Unknown type '{}'.", tp_name);
            errors::raise_error(&fmt, errors::ErrorType::UnknownType, &node.pos, self.info);
        } else {
            Self::get_datatype_from_str(&self.namespaces.structs, tp_name).unwrap()
        };

        let anytp: Option<inkwell::types::AnyTypeEnum> = Self::get_anytp_from_tp(self.context, &self.inkwell_types, tp.clone());

        if !anytp.is_none() && anytp.unwrap().is_int_type() && left.data.unwrap().is_int_value() {
            let res: inkwell::values::IntValue = self.builder.build_int_cast(left.data.unwrap().into_int_value(), anytp.unwrap().into_int_type(), "icast");

            match tp.tp {
                types::BasicDataType::I8 => {
                    builtin_types::i8type::check_overflow_literal(self, &res.to_string(), &node.pos);
                }
                types::BasicDataType::I16 => {
                    builtin_types::i16type::check_overflow_literal(self, &res.to_string(), &node.pos);
                }
                types::BasicDataType::I32 => {
                    builtin_types::i32type::check_overflow_literal(self, &res.to_string(), &node.pos);
                }
                types::BasicDataType::I64 => {
                    builtin_types::i64type::check_overflow_literal(self, &res.to_string(), &node.pos);
                }
                types::BasicDataType::I128 => {
                    builtin_types::i128type::check_overflow_literal(self, &res.to_string(), &node.pos);
                }
                types::BasicDataType::U8 => {
                    builtin_types::u8type::check_overflow_literal(self, &res.to_string(), &node.pos);
                }
                types::BasicDataType::U16 => {
                    builtin_types::u16type::check_overflow_literal(self, &res.to_string(), &node.pos);
                }
                types::BasicDataType::U32 => {
                    builtin_types::u32type::check_overflow_literal(self, &res.to_string(), &node.pos);
                }
                types::BasicDataType::U64 => {
                    builtin_types::u64type::check_overflow_literal(self, &res.to_string(), &node.pos);
                }
                types::BasicDataType::U128 => {
                    builtin_types::u128type::check_overflow_literal(self, &res.to_string(), &node.pos);
                }
                _ => {
                    unreachable!();
                }
            }

            return types::Data {
                data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
                tp: tp.clone(),
                owned: true,
            };
        }
        else if !anytp.is_none() && anytp.unwrap().is_float_type() && left.data.unwrap().is_float_value() {
            let res: inkwell::values::FloatValue = self.builder.build_float_cast(left.data.unwrap().into_float_value(), anytp.unwrap().into_float_type(), "fcast");

            match tp.tp {
                types::BasicDataType::F32 => {
                    builtin_types::f32type::check_overflow_literal(self, &res.to_string(), &node.pos);
                }
                types::BasicDataType::F64 => {
                    builtin_types::f64type::check_overflow_literal(self, &res.to_string(), &node.pos);
                }
                _ => {
                    unreachable!();
                }
            }

            return types::Data {
                data: Some(inkwell::values::BasicValueEnum::FloatValue(res)),
                tp: tp.clone(),
                owned: true,
            };
        }
        else if !anytp.is_none() && anytp.unwrap().is_float_type() && left.data.unwrap().is_int_value() {
            let res: inkwell::values::FloatValue = left.data.unwrap().into_int_value().const_signed_to_float(anytp.unwrap().into_float_type());
            
            match tp.tp {
                types::BasicDataType::F32 => {
                    builtin_types::f32type::check_overflow_literal(self, &builtin_types::float_repr(res), &node.pos);
                }
                types::BasicDataType::F64 => {
                    builtin_types::f64type::check_overflow_literal(self, &builtin_types::float_repr(res), &node.pos);
                }
                _ => {
                    unreachable!();
                }
            }

            return types::Data {
                data: Some(inkwell::values::BasicValueEnum::FloatValue(res)),
                tp: tp.clone(),
                owned: true,
            };
        }
        else if !anytp.is_none() && anytp.unwrap().is_int_type() && left.data.unwrap().is_float_value() {
            let res: inkwell::values::IntValue = if builtin_types::int_issigned(tp.clone()) {
                left.data.unwrap().into_float_value().const_to_signed_int(anytp.unwrap().into_int_type())
            }
            else {
                left.data.unwrap().into_float_value().const_to_unsigned_int(anytp.unwrap().into_int_type())
            };
            
            match tp.tp {
                types::BasicDataType::I8 => {
                    builtin_types::i8type::check_overflow_literal(self, &builtin_types::int_repr(res), &node.pos);
                }
                types::BasicDataType::I16 => {
                    builtin_types::i16type::check_overflow_literal(self, &builtin_types::int_repr(res), &node.pos);
                }
                types::BasicDataType::I32 => {
                    builtin_types::i32type::check_overflow_literal(self, &builtin_types::int_repr(res), &node.pos);
                }
                types::BasicDataType::I64 => {
                    builtin_types::i64type::check_overflow_literal(self, &builtin_types::int_repr(res), &node.pos);
                }
                types::BasicDataType::I128 => {
                    builtin_types::i128type::check_overflow_literal(self, &builtin_types::int_repr(res), &node.pos);
                }
                types::BasicDataType::U8 => {
                    builtin_types::u8type::check_overflow_literal(self, &builtin_types::int_repr(res), &node.pos);
                }
                types::BasicDataType::U16 => {
                    builtin_types::u16type::check_overflow_literal(self, &builtin_types::int_repr(res), &node.pos);
                }
                types::BasicDataType::U32 => {
                    builtin_types::u32type::check_overflow_literal(self, &builtin_types::int_repr(res), &node.pos);
                }
                types::BasicDataType::U64 => {
                    builtin_types::u64type::check_overflow_literal(self, &builtin_types::int_repr(res), &node.pos);
                }
                types::BasicDataType::U128 => {
                    builtin_types::u128type::check_overflow_literal(self, &builtin_types::int_repr(res), &node.pos);
                }
                _ => {
                    unreachable!();
                }
            }

            return types::Data {
                data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
                tp: tp.clone(),
                owned: true,
            };
        }
        else {
            let fmt: String = format!("Non primitive cast from '{}' to '{}'.", left.tp.name, tp_name);
            errors::raise_error(&fmt, errors::ErrorType::InvalidCast, &node.pos, self.info);
        }
    }

    fn build_as(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let left: types::Data = self.compile_expr(&node.data.to.as_ref().unwrap().left, false);     
        let arg: &parser::Type = &node.data.to.as_ref().unwrap().tp;  
        if arg.isfn {
            let fmt: String = format!("Non primitive cast from '{}' to 'fn'.", left.tp.name);
            errors::raise_error(&fmt, errors::ErrorType::InvalidCast, &node.pos, self.info);
        }
        let tp_name: &String = &arg.data.as_ref().unwrap();

        let tp: types::DataType = if Self::get_datatype_from_str(&self.namespaces.structs, tp_name).is_none() {
            let fmt: String = format!("Unknown type '{}'.", tp_name);
            errors::raise_error(&fmt, errors::ErrorType::UnknownType, &node.pos, self.info);
        } else {
            Self::get_datatype_from_str(&self.namespaces.structs, tp_name).unwrap()
        };

        let anytp: Option<inkwell::types::AnyTypeEnum> = Self::get_anytp_from_tp(self.context, &self.inkwell_types, tp.clone());

        if !anytp.is_none() && anytp.unwrap().is_int_type() && left.data.unwrap().is_int_value() {
            let res: inkwell::values::IntValue = self.builder.build_int_cast(left.data.unwrap().into_int_value(), anytp.unwrap().into_int_type(), "cast");

            return types::Data {
                data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
                tp: tp.clone(),
                owned: true,
            };
        }
        else if !anytp.is_none() && anytp.unwrap().is_float_type() && left.data.unwrap().is_float_value() {
            let res: inkwell::values::FloatValue = self.builder.build_float_cast(left.data.unwrap().into_float_value(), anytp.unwrap().into_float_type(), "fcast");

            return types::Data {
                data: Some(inkwell::values::BasicValueEnum::FloatValue(res)),
                tp: tp.clone(),
                owned: true,
            };
        }
        else if !anytp.is_none() && anytp.unwrap().is_float_type() && left.data.unwrap().is_int_value() {
            let res: inkwell::values::FloatValue = left.data.unwrap().into_int_value().const_signed_to_float(anytp.unwrap().into_float_type());
            
            return types::Data {
                data: Some(inkwell::values::BasicValueEnum::FloatValue(res)),
                tp: tp.clone(),
                owned: true,
            };
        }
        else if !anytp.is_none() && anytp.unwrap().is_int_type() && left.data.unwrap().is_float_value() {
            let res: inkwell::values::IntValue = if builtin_types::int_issigned(left.tp) {
                left.data.unwrap().into_float_value().const_to_signed_int(left.data.unwrap().get_type().into_int_type())
            }
            else {
                left.data.unwrap().into_float_value().const_to_unsigned_int(left.data.unwrap().get_type().into_int_type())
            };

            return types::Data {
                data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
                tp: tp.clone(),
                owned: true,
            };
        } 
        else {
            let fmt: String = format!("Non primitive cast from '{}' to '{}'.", left.tp.name, tp_name);
            errors::raise_error(&fmt, errors::ErrorType::InvalidCast, &node.pos, self.info);
        }
    }

    fn build_ref(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        return self.compile_expr(&node.data.unary.as_ref().unwrap().right, false);
    }

    fn build_unary(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let unary: &parser::nodes::UnaryNode = node.data.unary.as_ref().unwrap();

        let right: types::Data = self.compile_expr(&unary.right, false);

        let mut args: Vec<types::Data> = Vec::new();

        let tp: &types::Type = self.get_type_from_data(&right);

        let tp_str: &String = &right.tp.name.clone();

        args.push(right);

        let traittp: types::TraitType = match node.data.unary.as_ref().unwrap().op {
            parser::nodes::UnaryOpType::NEG => {
                types::TraitType::Neg
            }
            parser::nodes::UnaryOpType::POS => {
                types::TraitType::Pos                
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
                let fmt: String = format!("Type '{}' has no trait '{}'.", tp_str, &traittp.to_string());
                errors::raise_error(&fmt, errors::ErrorType::MissingTrait, &node.pos, self.info);
            }
        };

        let func = t.function;

        let data: types::Data = (func)(&self, args, &node.pos);

        return data;
    }

    fn build_struct(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let mut names: Vec<String> = Vec::new();
        let mut types: Vec<(types::DataType, AnyTypeEnum)> = Vec::new();
        let mut simpletypes: Vec<types::DataType> = Vec::new();
        let mut mutabilitites: Vec<types::DataMutablility> = Vec::new();

        for member in &node.data.st.as_ref().unwrap().members {
            names.push(member.0.clone());
            types.push(Self::get_llvm_from_type(self.context, &self.namespaces.structs, &self.inkwell_types, self.info, member.1, node));
            simpletypes.push(Self::get_llvm_from_type(self.context, &self.namespaces.structs, &self.inkwell_types, self.info, member.1, node).0);
            mutabilitites.push(types::DataMutablility::Mutable);
        }

        self.namespaces.structs.insert(node.data.st.as_ref().unwrap().name.clone(), (types::new_datatype(types::BasicDataType::Struct, node.data.st.as_ref().unwrap().name.clone(), Some(names), simpletypes.clone(), mutabilitites, None, false), Self::build_struct_tp_from_types(self.context, &self.inkwell_types, &simpletypes)));

        let data: types::Data = types::Data {
            data: None,
            tp: types::new_datatype(types::BasicDataType::Unit, types::BasicDataType::Unit.to_string(), None, Vec::new(), Vec::new(), None, false),
            owned: true,
        };
        return data;
    }

    fn compile_expr(&mut self, node: &parser::Node, give_ownership: bool) -> types::Data<'ctx> {
        match node.tp {
            parser::NodeType::I32 => {
                let self_data: &String = &node.data.num.as_ref().unwrap().left;
                builtin_types::i32type::check_overflow_literal(self, self_data, &node.pos);
                let selfv: inkwell::values::IntValue = match self.inkwell_types.i32tp.const_int_from_string(self_data.as_str(), inkwell::types::StringRadix::Decimal) {
                    None => {
                        let fmt: String = format!("Invalid i32 literal '{}'.", self_data);
                        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, &node.pos, self.info);
                    }
            
                    Some(v) => {
                        v
                    }
            
                };
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: types::new_datatype(types::BasicDataType::I32, types::BasicDataType::I32.to_string(), None, Vec::new(), Vec::new(), None, false), owned: true}
            }
            parser::NodeType::BINARY => {
                self.build_binary(node)
            }
            parser::NodeType::LET => {
                self.build_let(node)
            }
            parser::NodeType::IDENTIFIER => {
                self.build_loadname(node, give_ownership)
            }
            parser::NodeType::FUNC => {
                self.build_func(node)
            }
            parser::NodeType::ASSIGN => {
                self.build_assign(node)
            }
            parser::NodeType::CALL => {
                self.build_call(node)
            }
            parser::NodeType::RETURN => {
                self.build_return(node)
            }
            parser::NodeType::U32 => {
                let self_data: &String = &node.data.num.as_ref().unwrap().left;
                builtin_types::u32type::check_overflow_literal(self, self_data, &node.pos);
                let selfv: inkwell::values::IntValue = match self.inkwell_types.i32tp.const_int_from_string(self_data.as_str(), inkwell::types::StringRadix::Decimal) {
                    None => {
                        let fmt: String = format!("Invalid u32 literal '{}'.", self_data);
                        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, &node.pos, self.info);
                    }
            
                    Some(v) => {
                        v
                    }
            
                };
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: types::new_datatype(types::BasicDataType::U32, types::BasicDataType::U32.to_string(), None, Vec::new(), Vec::new(), None, false), owned: true}
            }
            parser::NodeType::I8 => {
                let self_data: &String = &node.data.num.as_ref().unwrap().left;
                builtin_types::i8type::check_overflow_literal(self, self_data, &node.pos);
                let selfv: inkwell::values::IntValue = match self.inkwell_types.i8tp.const_int_from_string(self_data.as_str(), inkwell::types::StringRadix::Decimal) {
                    None => {
                        let fmt: String = format!("Invalid i8 literal '{}'.", self_data);
                        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, &node.pos, self.info);
                    }
            
                    Some(v) => {
                        v
                    }
            
                };
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: types::new_datatype(types::BasicDataType::I8, types::BasicDataType::I8.to_string(), None, Vec::new(), Vec::new(), None, false), owned: true}
            }
            parser::NodeType::U8 => {
                let self_data: &String = &node.data.num.as_ref().unwrap().left;
                builtin_types::u8type::check_overflow_literal(self, self_data, &node.pos);
                let selfv: inkwell::values::IntValue = match self.inkwell_types.i8tp.const_int_from_string(self_data.as_str(), inkwell::types::StringRadix::Decimal) {
                    None => {
                        let fmt: String = format!("Invalid u8 literal '{}'.", self_data);
                        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, &node.pos, self.info);
                    }
            
                    Some(v) => {
                        v
                    }
            
                };
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: types::new_datatype(types::BasicDataType::U8, types::BasicDataType::U8.to_string(), None, Vec::new(), Vec::new(), None, false), owned: true}
            }
            parser::NodeType::I16 => {
                let self_data: &String = &node.data.num.as_ref().unwrap().left;
                builtin_types::i16type::check_overflow_literal(self, self_data, &node.pos);
                let selfv: inkwell::values::IntValue = match self.inkwell_types.i16tp.const_int_from_string(self_data.as_str(), inkwell::types::StringRadix::Decimal) {
                    None => {
                        let fmt: String = format!("Invalid i16 literal '{}'.", self_data);
                        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, &node.pos, self.info);
                    }
            
                    Some(v) => {
                        v
                    }
            
                };
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: types::new_datatype(types::BasicDataType::I16, types::BasicDataType::I16.to_string(), None, Vec::new(), Vec::new(), None, false), owned: true}
            }
            parser::NodeType::U16 => {
                let self_data: &String = &node.data.num.as_ref().unwrap().left;
                builtin_types::u16type::check_overflow_literal(self, self_data, &node.pos);
                let selfv: inkwell::values::IntValue = match self.inkwell_types.i16tp.const_int_from_string(self_data.as_str(), inkwell::types::StringRadix::Decimal) {
                    None => {
                        let fmt: String = format!("Invalid u16 literal '{}'.", self_data);
                        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, &node.pos, self.info);
                    }
            
                    Some(v) => {
                        v
                    }
            
                };
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: types::new_datatype(types::BasicDataType::U16, types::BasicDataType::U16.to_string(), None, Vec::new(), Vec::new(), None, false), owned: true}
            }
            parser::NodeType::I64 => {
                let self_data: &String = &node.data.num.as_ref().unwrap().left;
                builtin_types::i64type::check_overflow_literal(self, self_data, &node.pos);
                let selfv: inkwell::values::IntValue = match self.inkwell_types.i64tp.const_int_from_string(self_data.as_str(), inkwell::types::StringRadix::Decimal) {
                    None => {
                        let fmt: String = format!("Invalid i64 literal '{}'.", self_data);
                        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, &node.pos, self.info);
                    }
            
                    Some(v) => {
                        v
                    }
            
                };
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: types::new_datatype(types::BasicDataType::I64, types::BasicDataType::I64.to_string(), None, Vec::new(), Vec::new(), None, false), owned: true}
            }
            parser::NodeType::U64 => {
                let self_data: &String = &node.data.num.as_ref().unwrap().left;
                builtin_types::u64type::check_overflow_literal(self, self_data, &node.pos);
                let selfv: inkwell::values::IntValue = match self.inkwell_types.i64tp.const_int_from_string(self_data.as_str(), inkwell::types::StringRadix::Decimal) {
                    None => {
                        let fmt: String = format!("Invalid u64 literal '{}'.", self_data);
                        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, &node.pos, self.info);
                    }
            
                    Some(v) => {
                        v
                    }
            
                };
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: types::new_datatype(types::BasicDataType::U64, types::BasicDataType::U64.to_string(), None, Vec::new(), Vec::new(), None, false), owned: true}
            }
            parser::NodeType::I128 => {
                let self_data: &String = &node.data.num.as_ref().unwrap().left;
                builtin_types::i128type::check_overflow_literal(self, self_data, &node.pos);
                let selfv: inkwell::values::IntValue = match self.inkwell_types.i128tp.const_int_from_string(self_data.as_str(), inkwell::types::StringRadix::Decimal) {
                    None => {
                        let fmt: String = format!("Invalid i128 literal '{}'.", self_data);
                        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, &node.pos, self.info);
                    }
            
                    Some(v) => {
                        v
                    }
            
                };
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: types::new_datatype(types::BasicDataType::I128, types::BasicDataType::I128.to_string(), None, Vec::new(), Vec::new(), None, false), owned: true}
            }
            parser::NodeType::U128 => {
                let self_data: &String = &node.data.num.as_ref().unwrap().left;
                builtin_types::u128type::check_overflow_literal(self, self_data, &node.pos);
                let selfv: inkwell::values::IntValue = match self.inkwell_types.i128tp.const_int_from_string(self_data.as_str(), inkwell::types::StringRadix::Decimal) {
                    None => {
                        let fmt: String = format!("Invalid u128 literal '{}'.", self_data);
                        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, &node.pos, self.info);
                    }
            
                    Some(v) => {
                        v
                    }
            
                };
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: types::new_datatype(types::BasicDataType::U128, types::BasicDataType::U128.to_string(), None, Vec::new(), Vec::new(), None, false), owned: true}
            }
            parser::NodeType::TO => {
                self.build_to(node)
            }
            parser::NodeType::AS => {
                self.build_as(node)
            }
            parser::NodeType::F32 => {
                let self_data: &String = &node.data.num.as_ref().unwrap().left;
                builtin_types::f32type::check_overflow_literal(self, self_data, &node.pos);
                let selfv: inkwell::values::FloatValue = self.inkwell_types.f32tp.const_float_from_string(self_data.as_str());
                types::Data {data: Some(inkwell::values::BasicValueEnum::FloatValue(selfv)), tp: types::new_datatype(types::BasicDataType::F32, types::BasicDataType::F32.to_string(), None, Vec::new(), Vec::new(), None, false), owned: true}
            }
            parser::NodeType::F64 => {
                let self_data: &String = &node.data.num.as_ref().unwrap().left;
                builtin_types::f64type::check_overflow_literal(self, self_data, &node.pos);
                let selfv: inkwell::values::FloatValue = self.inkwell_types.f64tp.const_float_from_string(self_data.as_str());
                types::Data {data: Some(inkwell::values::BasicValueEnum::FloatValue(selfv)), tp: types::new_datatype(types::BasicDataType::F64, types::BasicDataType::F64.to_string(), None, Vec::new(), Vec::new(), None, false), owned: true}
            }
            parser::NodeType::REF => {
                self.build_ref(node)
            }
            parser::NodeType::UNARY => {
                self.build_unary(node)
            }
            parser::NodeType::STRUCT => {
                self.build_struct(node)
            }
        }
    }

    fn compile(&mut self, nodes: &Vec<parser::Node>, infn: bool) -> types::Data<'ctx>{
        let mut retv: types::Data = types::Data {
            data: None,
            tp: types::new_datatype(types::BasicDataType::Unit, types::BasicDataType::Unit.to_string(), None, Vec::new(), Vec::new(), None, false),
            owned: true
        };

        for node in nodes {
            if infn && node.tp == parser::NodeType::FUNC {
                let fmt: String = format!("Cannot define nested functions.");
                errors::raise_error(&fmt, errors::ErrorType::NestedFunctions, &node.pos, self.info);
            }
            retv = self.compile_expr(node, false);
        }
        return retv;
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
        i8tp: &context.i8_type(),
        i16tp: &context.i16_type(),
        i32tp: &context.i32_type(),
        i64tp: &context.i64_type(),
        i128tp: &context.i128_type(),
        f32tp: &context.f32_type(),
        f64tp: &context.f64_type(),
        voidtp: &context.void_type(),
    };

    let namespaces: Namespaces = Namespaces {
        locals: std::collections::HashMap::new(),
        functions: std::collections::HashMap::new(),
        structs: std::collections::HashMap::new(),
    };

    
    //Setup debug info
    module.add_basic_value_flag("Debug Info Version", inkwell::module::FlagBehavior::Error, inkwelltypes.i32tp.const_int(1, false));
    let (dibuilder, compile_unit) = module.create_debug_info_builder(
        true,
        inkwell::debug_info::DWARFSourceLanguage::C,
        &info.name,
        &info.dir,
        "Kestrel",
        true,
        "",
        0,
        "",
        inkwell::debug_info::DWARFEmissionKind::Full,
        0,
        false,
        false);

    let mut codegen: CodeGen = CodeGen {
        context: &context,
        module: module,
        builder: context.create_builder(),
        types: std::collections::HashMap::new(),
        info,
        inkwell_types: inkwelltypes,
        namespaces: namespaces,
        dibuilder: dibuilder,
        dicompile_unit: compile_unit,
        expected_rettp: None, 
    };
    
    //Pass manager (optimizer)
    let pass_manager_builder: inkwell::passes::PassManagerBuilder = inkwell::passes::PassManagerBuilder::create();
    pass_manager_builder.set_optimization_level(inkwell::OptimizationLevel::Aggressive);
    let manager: inkwell::passes::PassManager<Module> = inkwell::passes::PassManager::create(());
    pass_manager_builder.populate_module_pass_manager(&manager);

    //Setup builtin types
    builtin_types::init(&mut codegen);

    //Compile code
    codegen.compile(&nodes, false);

    //Make the real main function
    if codegen.get_function(&String::from("main")) == None {
        let fmt: String = format!("Function 'main' is not defined.");
        errors::raise_error_no_pos(&fmt, errors::ErrorType::NameNotFound);
    }

    let (main, _) = codegen.namespaces.functions.get(&String::from("main")).unwrap();

    let main_tp: inkwell::types::FunctionType = codegen.inkwell_types.i32tp.fn_type(&[inkwell::types::BasicMetadataTypeEnum::IntType(*codegen.inkwell_types.i32tp), inkwell::types::BasicMetadataTypeEnum::PointerType(codegen.inkwell_types.i8tp.ptr_type(inkwell::AddressSpace::Generic).ptr_type(inkwell::AddressSpace::Generic))], false);
    let realmain: inkwell::values::FunctionValue = codegen.module.add_function("main", main_tp, None);
    let basic_block: inkwell::basic_block::BasicBlock = codegen.context.append_basic_block(realmain, "entry");

    let mut attr: inkwell::attributes::Attribute = codegen.context.create_enum_attribute(inkwell::attributes::Attribute::get_named_enum_kind_id("noinline"), 0);

    realmain.add_attribute(inkwell::attributes::AttributeLoc::Function, attr);

    attr = codegen.context.create_enum_attribute(inkwell::attributes::Attribute::get_named_enum_kind_id("optnone"), 0);

    realmain.add_attribute(inkwell::attributes::AttributeLoc::Function, attr);
    
    codegen.builder.position_at_end(basic_block); 

    codegen.builder.build_call(*main, &[], "res");

    codegen.builder.build_return(Some(&codegen.inkwell_types.i32tp.const_int(0, false)));

    //

    //Generate debug info
    codegen.dibuilder.finalize();

    //Optimize
    unsafe { codegen.module.run_in_pass_manager(&manager) };

    codegen.module.print_to_file(std::path::Path::new("a.ll"))?;

    std::process::Command::new("llc").arg("a.ll").output().expect("Failed to execute llc");

    std::process::Command::new("gcc").arg("a.s").arg("-oa.out").output().expect("Failed to execute gcc");

    Ok(())
}