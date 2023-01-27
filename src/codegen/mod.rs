//Generate LLVM-IR

use inflector::Inflector;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManagerSubType;
use inkwell::types::AnyTypeEnum;
use inkwell::types::BasicType;
use crate::fileinfo;
use inkwell::debug_info::AsDIScope;
use itertools::izip;

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
    booltp: &'ctx inkwell::types::IntType<'ctx>,
    dynptrtp: &'ctx inkwell::types::StructType<'ctx>,
}

#[derive(PartialEq, Clone)]
pub enum ForwardDeclarationType {
    Forward,
    Real,
}

#[derive(PartialEq, Clone, Debug)]
pub enum InitializationStatus {
    Initialized,
    Uninitialized,
}

#[derive(PartialEq, Clone)]
pub enum TemplateFunctionInstance {
    Unrelated,
    Instance,
    Namespace,
}

pub struct Namespaces<'ctx> {
    locals: Vec<std::collections::HashMap<String, (Option<inkwell::values::PointerValue<'ctx>>, types::DataType<'ctx>, types::DataMutablility, types::DataOwnership, parser::Position, InitializationStatus)>>,
    functions: std::collections::HashMap<String, (inkwell::values::FunctionValue<'ctx>, types::DataType<'ctx>, ForwardDeclarationType)>,
    structs: std::collections::HashMap<String, (types::DataType<'ctx>, Option<inkwell::types::AnyTypeEnum<'ctx>>, std::collections::HashMap<String, i32>, ForwardDeclarationType)>,
    template_functions_sig: std::collections::HashMap<String, (parser::Node, TemplateFunctionInstance)>,
    template_functions: Vec<(String, types::DataType<'ctx>, inkwell::values::FunctionValue<'ctx>)>,
    structid: std::collections::HashMap<String, i32>,
    structid_from: std::collections::HashMap<i32, String>,
    structid_max: i32,
}

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    types: std::collections::HashMap<String, types::Type<'ctx>>,
    info: &'ctx fileinfo::FileInfo<'ctx>,
    inkwell_types: InkwellTypes<'ctx>,
    datatypes: std::collections::HashMap<String, types::DataType<'ctx>>,
    namespaces: Namespaces<'ctx>,
    dibuilder: inkwell::debug_info::DebugInfoBuilder<'ctx>,
    dicompile_unit: inkwell::debug_info::DICompileUnit<'ctx>,
    expected_rettp: Option<types::DataType<'ctx>>,
    traits: std::collections::HashMap<String, types::TraitSignature<'ctx>>,
    enclosing_block: Option<inkwell::basic_block::BasicBlock<'ctx>>,
    start_block: Option<inkwell::basic_block::BasicBlock<'ctx>>,
    end_block: Option<inkwell::basic_block::BasicBlock<'ctx>>,
    loop_flow_broken: bool,
    vtables: Option<inkwell::values::GlobalValue<'ctx>>,
    vtables_vec: Vec<Vec<inkwell::values::PointerValue<'ctx>>>,
}

//Codegen functions
impl<'ctx> CodeGen<'ctx> {
    fn get_variable(&self, name: &String) -> (Option<&(Option<inkwell::values::PointerValue<'ctx>>, types::DataType<'ctx>, types::DataMutablility, types::DataOwnership, parser::Position, InitializationStatus)>, usize){
        for index in (0..self.namespaces.locals.len()).rev(){
            if self.namespaces.locals.get(index).unwrap().iter().find(|x| *x.0 == *name) != None {
                return (self.namespaces.locals.get(index).unwrap().get(name), index);
            }
        }
        
        return (None, 0);
    }
    
    fn get_function(&self, name: &String) -> Option<(inkwell::values::PointerValue<'ctx>, types::DataType<'ctx>, ForwardDeclarationType)>{
        if self.namespaces.functions.iter().find(|x| *x.0 == *name) != None {
            return Some((self.namespaces.functions.get(name).unwrap().0.as_global_value().as_pointer_value(), self.namespaces.functions.get(name).unwrap().1.clone(), self.namespaces.functions.get(name).unwrap().2.clone()));
        }

        return None;
    }
    
    fn build_struct_tp_from_types(ctx: &'ctx Context, inktypes: &InkwellTypes<'ctx>, types: &Vec<types::DataType<'ctx>>) -> inkwell::types::AnyTypeEnum<'ctx> {
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

    pub fn array_repr(arrtp: inkwell::types::ArrayType) -> String {
        let mut res: String = String::from("");
        res+=arrtp.get_element_type().print_to_string().to_str().unwrap();
        res+="[";
        res+=arrtp.len().to_string().as_str();
        res+="]";
        return res;
    }

    fn get_anytp_from_tp(ctx: &'ctx Context, types: &InkwellTypes<'ctx>, tp: types::DataType<'ctx>) -> Option<inkwell::types::AnyTypeEnum<'ctx>> {
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
            types::BasicDataType::Void => {
                return Some(inkwell::types::AnyTypeEnum::VoidType(*types.voidtp));
            }
            types::BasicDataType::Func |
            types::BasicDataType::WrapperFunc => {
                return None;
            }
            types::BasicDataType::Struct => {
                return Some(Self::build_struct_tp_from_types(ctx, types, &tp.types));
            }
            types::BasicDataType::Array => {
                return Some(inkwell::types::AnyTypeEnum::ArrayType(tp.arrtp.unwrap()));
            }
            types::BasicDataType::Bool => {
                return Some(inkwell::types::AnyTypeEnum::IntType(*types.booltp));
            }
            types::BasicDataType::Enum => {
                return Some(inkwell::types::AnyTypeEnum::IntType(*types.i32tp));
            }
            types::BasicDataType::Dyn => {
                return Some(inkwell::types::AnyTypeEnum::StructType(*types.dynptrtp));
            }
            types::BasicDataType::Unknown => {
                return None;
            }    
        }
    }

    pub fn get_llvm_from_type(ctx: &'ctx Context, structs: &std::collections::HashMap<String, (types::DataType<'ctx>, Option<inkwell::types::AnyTypeEnum<'ctx>>, std::collections::HashMap<String, i32>, ForwardDeclarationType)>, types: &InkwellTypes<'ctx>, datatypes: &std::collections::HashMap<String, types::DataType<'ctx>>, traits: &std::collections::HashMap<String, types::TraitSignature<'ctx>>, info: &fileinfo::FileInfo, arg: &parser::Type, node: &parser::Node) -> (types::DataType<'ctx>, inkwell::types::AnyTypeEnum<'ctx>) {
        if arg.isfn {
            let args: &Vec<parser::Type> = &arg.args.as_ref().unwrap().args;
            let mut datatypes_: Vec<types::DataType> = Vec::new();
            let mut mutability: Vec<types::DataMutablility> = Vec::new();
            let mut inktypes: Vec<inkwell::types::BasicMetadataTypeEnum> = Vec::new();
            
            for arg in args {
                let (data, tp) = Self::get_llvm_from_type(ctx, structs, types, datatypes, traits, info, &arg, node);
                datatypes_.push(data);
                mutability.push(arg.mutability);
                let res: Option<inkwell::types::BasicMetadataTypeEnum> = Self::get_basicmeta_from_any(tp);

                if res.is_some() {
                    inktypes.push(res.unwrap());
                }
            }
            
            let rettp_full: (types::DataType, inkwell::types::AnyTypeEnum) = Self::get_llvm_from_type(ctx, structs, types, datatypes, traits, info, &arg.args.as_ref().unwrap().rettp.last().unwrap(), node);
            let tp: inkwell::types::AnyTypeEnum = rettp_full.1;
            let fntp: inkwell::types::FunctionType;
            
            if tp.is_int_type() {
                fntp = tp.into_int_type().fn_type(&inktypes[..], false);
            }
            else if tp.is_float_type() {
                fntp = tp.into_float_type().fn_type(&inktypes[..], false);
            }
            else if tp.is_function_type() {
                fntp = tp.into_function_type().ptr_type(inkwell::AddressSpace::from(0u16)).fn_type(&inktypes[..], false);
            }
            else if tp.is_void_type() {
                fntp = tp.into_void_type().fn_type(&inktypes[..], false);
            }
            else if tp.is_struct_type() {
                fntp = tp.into_struct_type().fn_type(&inktypes[..], false);
            }
            else if tp.is_array_type() {
                fntp = tp.into_array_type().fn_type(&inktypes[..], false);
            }
            else {
                panic!("Unexpected type");
            }

            let mut names: Option<Vec<String>> = None;
            if node.tp == parser::NodeType::FUNC {
                names=Some(node.data.func.as_ref().unwrap().args.name.clone());
            }
            let mut tp: types::DataType = datatypes.get(&types::BasicDataType::Func.to_string()).unwrap().clone();
            tp.rettp = Some(Box::new(rettp_full.0.clone()));
            tp.names = names;
            return (tp.clone(), inkwell::types::AnyTypeEnum::FunctionType(fntp));
        }
        else if arg.isarr {
            let (_, tp) = Self::get_llvm_from_type(ctx, structs, types, datatypes, traits, info, &arg.arrtp.as_ref().unwrap(), node);
            let len: u32 = match u32::from_str_radix(arg.arrlen.as_ref().unwrap().first().unwrap().as_str(), 10) {
                Ok(v) => {
                    v
                }
                Err(_) => {
                    let fmt: String = format!("Value '{}' out of range for 'u32'.", arg.arrlen.as_ref().unwrap().first().unwrap());
                    errors::raise_error(&fmt, errors::ErrorType::ArrayLengthOutOfRange, &node.pos, info);
                }
            };

            if len == 0 {
                let fmt: String = format!("Cannot define zero-length array.");
                errors::raise_error(&fmt, errors::ErrorType::ZeroLengthArray, &node.pos, info);
            }

            let mut arrtp: inkwell::types::ArrayType;

            if tp.is_int_type() {
                arrtp = tp.into_int_type().array_type(len);
            }
            else if tp.is_float_type() {
                arrtp = tp.into_float_type().array_type(len);
            }
            else if tp.is_function_type() {
                let fmt: String = format!("Cannot define array of 'fn'.",);
                errors::raise_error(&fmt, errors::ErrorType::CannotDefineFnArray, &node.pos, info);
            }
            else if tp.is_void_type() {
                let fmt: String = format!("Cannot define array of 'void'.");
                errors::raise_error(&fmt, errors::ErrorType::CannotDefineVoidArray, &node.pos, info);
            }
            else if tp.is_struct_type() {
                arrtp = tp.into_struct_type().array_type(len);
            }
            else if tp.is_array_type() {
                arrtp = tp.into_array_type().array_type(len);
            }
            else {
                panic!("Unexpected type");
            }
            
            for item in arg.arrlen.as_ref().unwrap().split_at(1).1.to_vec() {
                let len: u32 = u32::from_str_radix(item.as_str(), 10).unwrap();
                arrtp = arrtp.array_type(len);
            }

            let mut tp: types::DataType = datatypes.get(&types::BasicDataType::Array.to_string()).unwrap().clone();
            tp.name = Self::array_repr(arrtp);
            tp.arrtp = Some(arrtp);
            return (tp.clone(), inkwell::types::AnyTypeEnum::ArrayType(arrtp));
        }
        else if arg.isdyn {
            let traitnm: String = arg.data.as_ref().unwrap().to_owned();
            if !traits.contains_key(&traitnm) {
                let fmt: String = format!("Trait '{}' not found.", traitnm);
                errors::raise_error(&fmt, errors::ErrorType::TraitNotFound, &node.pos, info);
            }
            let tp: types::DataType = types::new_dyn_datatype(traitnm, arg.mutability);
            
            return (tp, inkwell::types::AnyTypeEnum::StructType(*types.dynptrtp));
        }
        else {
            let tp: Option<&types::DataType> = datatypes.get(arg.data.as_ref().unwrap());
            if tp.is_none() {
                let fmt: String = format!("Unknown type '{}'.", &arg.data.as_ref().unwrap());
                errors::raise_error(&fmt, errors::ErrorType::UnknownType, &node.pos, info);
            }
            let anytp: Option<inkwell::types::AnyTypeEnum> = Self::get_anytp_from_tp(ctx, &types, tp.unwrap().clone());
            if anytp.is_none() {
                unimplemented!();
            }
            return (tp.unwrap().clone(), anytp.unwrap());
        }
    }

    fn mangle_name_main(&self, name: &String) -> String {
        let mut new: String = name.clone();
        if *name == String::from("main") {
            new = String::from("_") + new.as_str();      
        }
        return new;
    }

    fn get_type_from_data(types: std::collections::HashMap<String, types::Type<'ctx>>, data: &types::Data) -> types::Type<'ctx> {
        return types.get(&data.tp.to_string()).unwrap().clone();
    }

    fn get_basicmeta_from_any(tp: inkwell::types::AnyTypeEnum<'ctx>) -> Option<inkwell::types::BasicMetadataTypeEnum> {
        if tp.is_int_type() {
            return Some(inkwell::types::BasicMetadataTypeEnum::IntType(tp.into_int_type()));
        }
        else if tp.is_float_type() {
            return Some(inkwell::types::BasicMetadataTypeEnum::FloatType(tp.into_float_type()));
        }
        else if tp.is_function_type() {
            return Some(inkwell::types::BasicMetadataTypeEnum::PointerType(tp.into_function_type().ptr_type(inkwell::AddressSpace::from(0u16))));
        }
        else if tp.is_void_type() {
            return None;
        }
        else if tp.is_struct_type() {
            return Some(inkwell::types::BasicMetadataTypeEnum::StructType(tp.into_struct_type()));
        }
        else if tp.is_array_type() {
            return Some(inkwell::types::BasicMetadataTypeEnum::ArrayType(tp.into_array_type()));
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
            return Some(inkwell::types::BasicTypeEnum::PointerType(tp.into_function_type().ptr_type(inkwell::AddressSpace::from(0u16))));
        }
        else if tp.is_void_type() {
            return None;
        }
        else if tp.is_struct_type() {
            return Some(inkwell::types::BasicTypeEnum::StructType(tp.into_struct_type()));
        }
        else if tp.is_array_type() {
            return Some(inkwell::types::BasicTypeEnum::ArrayType(tp.into_array_type()));
        }
        else {
            panic!("Unexpected type");
        }
    }

    fn append_struct_to_vtables(&mut self, tbl: Vec<inkwell::values::PointerValue<'ctx>>, idx: i32) {
        let mut structs: Vec<inkwell::values::BasicValueEnum> = Vec::new();
        for table in &self.vtables_vec {
            let mut ptrs: Vec<inkwell::values::BasicValueEnum> = Vec::new();
            for ptr in table {
                ptrs.push(inkwell::values::BasicValueEnum::PointerValue(*ptr));
            }
            structs.push(inkwell::values::BasicValueEnum::StructValue(self.context.const_struct(&ptrs[..], false)));
        }

        let mut ptrs: Vec<inkwell::values::BasicValueEnum> = Vec::new();
        for ptr in &tbl {
            ptrs.push(inkwell::values::BasicValueEnum::PointerValue(*ptr));
        }

        if idx as usize >= self.vtables_vec.len() {
            structs.push(inkwell::values::BasicValueEnum::StructValue(self.context.const_struct(&ptrs[..], false)));
            
            self.vtables_vec.push(tbl);
        }
        else {
            structs.insert(idx as usize, inkwell::values::BasicValueEnum::StructValue(self.context.const_struct(&ptrs[..], false)));
        }

        let st: inkwell::values::BasicValueEnum = inkwell::values::BasicValueEnum::StructValue(self.context.const_struct(&structs[..], false));

        let glbl: inkwell::values::GlobalValue = self.module.add_global(st.get_type(), Some(inkwell::AddressSpace::from(0u16)), "vtables");
        glbl.set_constant(true);
        glbl.set_initializer(&st);
        
        self.vtables = Some(glbl);
    }

    fn call_trait(&mut self, t: &types::Trait<'ctx>, mut args: Vec<types::Data<'ctx>>, node: &parser::Node) -> types::Data<'ctx> {
        if t.function.is_some() {
            let func = t.function.unwrap();

            return (func)(&self, args, &node.pos);
        }
        else {
            let func: inkwell::values::PointerValue = t.inkfunc.unwrap();

            args.insert(0, types::Data {
                data: Some(inkwell::values::BasicValueEnum::PointerValue(func)),
                tp: self.datatypes.get(&types::BasicDataType::Func.to_string()).unwrap().clone(),
                owned: true,
            });

            return builtin_types::functype::fn_call(self, args, &node.pos);
        }
    }

    fn build_binary(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let binary: &parser::nodes::BinaryNode = node.data.binary.as_ref().unwrap();

        let left: types::Data = self.compile_expr(&binary.left, false, false);
        let right: types::Data = self.compile_expr(&binary.right, false, false);

        let mut args: Vec<types::Data> = Vec::new();

        let tp: types::Type = Self::get_type_from_data(self.types.clone(), &left);

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
            parser::nodes::BinaryOpType::GT => {
                types::TraitType::Gt
            }
            parser::nodes::BinaryOpType::GTE => {
                types::TraitType::Ge
            }
            parser::nodes::BinaryOpType::LT => {
                types::TraitType::Lt
            }
            parser::nodes::BinaryOpType::LTE => {
                types::TraitType::Le
            }
            parser::nodes::BinaryOpType::EQ => {
                types::TraitType::Eq
            }
            parser::nodes::BinaryOpType::NE => {
                types::TraitType::Ne
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

        let data: types::Data = self.call_trait(t, args, node);

        if binary.isassign {
            let name: &String = &binary.left.data.identifier.as_ref().unwrap().name;
            let ptr: Option<inkwell::values::PointerValue> = self.get_variable(name).0.unwrap().0;

            if ptr.is_some() {
                self.builder.build_store(ptr.unwrap(), data.data.unwrap());

                self.namespaces.locals.last_mut().unwrap().insert(name.to_owned(), (ptr, data.tp.clone(), types::DataMutablility::Mutable, types::DataOwnership {owned: true, transferred: None}, node.pos.clone(), InitializationStatus::Initialized));
            }
        }

        return data;
    }
    
    fn build_let(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let name: String = node.data.letn.as_ref().unwrap().name.clone();
        
        if !name.is_snake_case() {
            errors::show_warning(errors::WarningType::ExpectedSnakeCase, vec![String::from(""), name.to_snake_case()], vec![String::from("Expected snake case"), String::from("Convert to this: ")], &node.pos, self.info)
        }

        if name.get(0..1).unwrap() == "_" {
            let data: types::Data = types::Data {
                data: None,
                tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
                owned: true,
            };
            return data;
        }

        if self.namespaces.locals.last().unwrap().get(&name).is_some() {
            let fmt: String = format!("Name '{}' is already defined in namespace.", &name);
            let here: String = format!("'{}' defined here.", name);
            errors::raise_error_multi(errors::ErrorType::RedefinitionAttempt, vec![here, fmt], vec![&self.namespaces.locals.last().unwrap().get(&name).unwrap().4, &node.pos], self.info);
        }

        if node.data.letn.as_ref().unwrap().expr.is_some() {
            if  node.data.letn.as_ref().unwrap().tp != None &&
                node.data.letn.as_ref().unwrap().tp.as_ref().unwrap().isdyn {
                let right: types::Data = self.compile_expr(&node.data.letn.as_ref().unwrap().expr.as_ref().unwrap(), true, false);
                
                let ptr: inkwell::values::PointerValue = self.builder.build_alloca(inkwell::types::BasicTypeEnum::StructType(*self.inkwell_types.dynptrtp), name.as_str());

                let typ: types::Type = Self::get_type_from_data(self.types.clone(), &right);
                let (dyntp, _) = Self::get_llvm_from_type(&self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, &node.data.letn.as_ref().unwrap().tp.as_ref().unwrap(), node);
                
                if !typ.traits.contains_key(&dyntp.name) {
                    let fmt: String = format!("'{}' type does not implement '{}' trait.", right.tp.to_string(), dyntp.name);
                    errors::raise_error(&fmt, errors::ErrorType::MissingTrait, &node.pos, self.info);
                }

                if right.tp.tp != types::BasicDataType::Struct {
                    let fmt: String = format!("Expected struct.");
                    errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
                }

                for sig in self.traits.get(&dyntp.name).unwrap().trait_sig.as_ref().unwrap() {
                    if sig.template_types.len() > 0 {
                        let fmt: String = format!("Trait '{}' is not trait object safe because function '{}' is templated.", dyntp.name, sig.name);
                        errors::raise_error(&fmt, errors::ErrorType::TraitIsNotTraitObjSafe, &node.pos, self.info);
                    }
                }

                let idptr = self.builder.build_struct_gep(ptr, 0u32, "idptr").expect("GEP error");
                self.builder.build_store(idptr, self.inkwell_types.i32tp.const_int(*self.namespaces.structid.get(&right.tp.name).unwrap() as u64, false));

                let itmptr = self.builder.build_struct_gep(ptr, 1u32, "item").expect("GEP error");
                let structptr: inkwell::values::PointerValue = self.builder.build_alloca(right.data.unwrap().get_type(), "struct_ptr");
                self.builder.build_store(structptr, right.data.unwrap());
                self.builder.build_store(itmptr, self.builder.build_pointer_cast(structptr, itmptr.get_type().get_element_type().into_pointer_type(), "st_bitcast"));


                self.namespaces.locals.last_mut().unwrap().insert(name, (Some(ptr), dyntp, node.data.letn.as_ref().unwrap().mutability, types::DataOwnership {owned: true, transferred: None}, node.pos.clone(), InitializationStatus::Initialized));
            }
            else { 
                let right: types::Data = self.compile_expr(&node.data.letn.as_ref().unwrap().expr.as_ref().unwrap(), true, false);
                if right.data.is_some(){
                    let rt_tp: types::DataType = right.tp.clone();
                    if node.data.letn.as_ref().unwrap().tp != None {
                        let (tp, _) = Self::get_llvm_from_type(&self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, &node.data.letn.as_ref().unwrap().tp.as_ref().unwrap(), node);
                        if tp != rt_tp {
                            let fmt: String = format!("Expected '{}' type, got '{}' type.", tp.to_string(), rt_tp.to_string());
                            errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
                        }
                    }

                    
                    let ptr: inkwell::values::PointerValue = self.builder.build_alloca(right.data.unwrap().get_type(), name.as_str());
                        
                    self.builder.build_store(ptr, right.data.unwrap());

                    self.namespaces.locals.last_mut().unwrap().insert(name, (Some(ptr), right.tp, node.data.letn.as_ref().unwrap().mutability, types::DataOwnership {owned: true, transferred: None}, node.pos.clone(), InitializationStatus::Initialized));
                }
                else {
                    self.namespaces.locals.last_mut().unwrap().insert(name, (None, right.tp, node.data.letn.as_ref().unwrap().mutability, types::DataOwnership {owned: true, transferred: None}, node.pos.clone(), InitializationStatus::Initialized));            
                }
            }
        }
        else {
            if node.data.letn.as_ref().unwrap().tp.is_none() {
                let fmt: String = format!("Expected specified type.");
                errors::raise_error(&fmt, errors::ErrorType::ExpectedSpecifiedType, &node.pos, self.info);
            }
            
            if  node.data.letn.as_ref().unwrap().tp != None &&
                node.data.letn.as_ref().unwrap().tp.as_ref().unwrap().isdyn {                
                let ptr: inkwell::values::PointerValue = self.builder.build_alloca(inkwell::types::BasicTypeEnum::StructType(*self.inkwell_types.dynptrtp), name.as_str());

                let (dyntp, _) = Self::get_llvm_from_type(&self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, &node.data.letn.as_ref().unwrap().tp.as_ref().unwrap(), node);

                self.namespaces.locals.last_mut().unwrap().insert(name, (Some(ptr), dyntp, node.data.letn.as_ref().unwrap().mutability, types::DataOwnership {owned: true, transferred: None}, node.pos.clone(), InitializationStatus::Initialized));
            }
            else { 
                let (tp, inktp) = Self::get_llvm_from_type(&self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, &node.data.letn.as_ref().unwrap().tp.as_ref().unwrap(), node);

                if tp.tp != types::BasicDataType::Void{
                    let ptr: inkwell::values::PointerValue = self.builder.build_alloca(Self::get_basic_from_any(inktp).unwrap(), name.as_str());
    
                    let rt_tp: types::DataType = tp.clone();
                    if node.data.letn.as_ref().unwrap().tp != None {
    
                        if tp != rt_tp {
                            let fmt: String = format!("Expected '{}' type, got '{}' type.", tp.to_string(), rt_tp.to_string());
                            errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
                        }
                    }
    
                    self.namespaces.locals.last_mut().unwrap().insert(name, (Some(ptr), tp, node.data.letn.as_ref().unwrap().mutability, types::DataOwnership {owned: true, transferred: None}, node.pos.clone(), InitializationStatus::Uninitialized));
                }
                else {
                    self.namespaces.locals.last_mut().unwrap().insert(name, (None, tp, node.data.letn.as_ref().unwrap().mutability, types::DataOwnership {owned: true, transferred: None}, node.pos.clone(), InitializationStatus::Uninitialized));            
                }
            }
        }

        let data: types::Data = types::Data {
            data: None,
            tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
            owned: true,
        };
        return data;
    }
    
    fn build_loadname(&mut self, node: &parser::Node, give_ownership: bool, get_ptr: bool) -> types::Data<'ctx> {
        let name: String = node.data.identifier.as_ref().unwrap().name.clone();

        let (ptr, tp) = match self.get_variable(&name).0 {
            None => {
                let res: Option<(inkwell::values::PointerValue, types::DataType, ForwardDeclarationType)> = self.get_function(&name);
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
                if !self.get_variable(&name).0.unwrap().3.owned {
                    let transferred: String = String::from(format!("'{}' was transferred here.", name));
                    let fmt: String = format!("Name '{}' is not owned.", name);
                    errors::raise_error_multi(errors::ErrorType::NameNotOwned, vec![transferred, fmt], vec![&self.get_variable(&name).0.unwrap().3.transferred.as_ref().unwrap(), &node.pos], self.info);
                }
                (v.0, v.1.clone())
            }
        };

        if self.get_variable(&name).0.unwrap().5 == InitializationStatus::Uninitialized {
            let fmt: String = format!("Name '{}' is not necessarily initialized.", name);
            errors::raise_error(&fmt, errors::ErrorType::NameNotInitialized, &node.pos, self.info);
        }

        let owner: types::DataOwnership = self.get_variable(&name).0.unwrap().3.clone();

        if give_ownership {
            let var = self.get_variable(&name);
            let mut locals = self.namespaces.locals.last().unwrap().clone();
            locals.insert(name.clone(), (var.0.unwrap().0.clone(), var.0.unwrap().1.clone(), var.0.unwrap().2.clone(), types::DataOwnership {owned: false, transferred: Some(node.pos.clone())}, var.0.unwrap().4.clone(), var.0.unwrap().5.clone()));

            self.namespaces.locals.pop();
            self.namespaces.locals.push(locals);
        }

        if ptr.is_some() {
            if get_ptr {
                let data: types::Data = types::Data {
                    data: Some(inkwell::values::BasicValueEnum::PointerValue(ptr.unwrap())),
                    tp,
                    owned: owner.owned,
                };
                return data;
            }
            
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
    
    fn build_func(&mut self, node: &parser::Node, altnm: Option<String>, template_types: Option<Vec<types::DataType<'ctx>>>, rettp_opt: Option<types::DataType<'ctx>>) -> types::Data<'ctx> {
        let mut name: String = if altnm.is_none() { node.data.func.as_ref().unwrap().name.clone() } else { altnm.as_ref().unwrap().clone() };

        if altnm.is_none(){
            if node.data.func.as_ref().unwrap().methodname.is_some() {
                name = node.data.func.as_ref().unwrap().methodname.as_ref().unwrap().to_string();
            }
            if node.data.func.as_ref().unwrap().namespacename.is_some() {
                name = node.data.func.as_ref().unwrap().namespacename.as_ref().unwrap().to_string();
            }

            if !name.is_snake_case() {
                errors::show_warning(errors::WarningType::ExpectedSnakeCase, vec![String::from(""), name.to_snake_case()], vec![String::from("Expected snake case"), String::from("Convert to this: ")], &node.pos, self.info)
            }
        }

        if self.get_function(&name) != None && self.get_function(&name).unwrap().2 != ForwardDeclarationType::Forward {
            let fmt: String = format!("Function '{}' is already defined.", name);
            errors::raise_error(&fmt, errors::ErrorType::RedefinitionAttempt, &node.pos, self.info);
        }

        if node.data.func.as_ref().unwrap().template_types.len() > 0 && template_types.is_none() {
            let mut name: String = node.data.func.as_ref().unwrap().name.clone();

            let mut instance: TemplateFunctionInstance = TemplateFunctionInstance::Unrelated;
        
            if node.data.func.as_ref().unwrap().methodname.is_some() {
                name += (String::from(".")+node.data.func.as_ref().unwrap().methodname.as_ref().unwrap().as_str()).as_str();
                instance = TemplateFunctionInstance::Instance;
            }
            if node.data.func.as_ref().unwrap().namespacename.is_some() {
                name += (String::from(".")+node.data.func.as_ref().unwrap().namespacename.as_ref().unwrap().as_str()).as_str();
                instance = TemplateFunctionInstance::Namespace;
            }
            
            self.namespaces.template_functions_sig.insert(name.to_owned(), (node.clone(), instance));
                
            let data: types::Data = types::Data {
                data: None,
                tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
                owned: true,
            };
            return data;
        }

        // Argument and return types
        let args = &node.data.func.as_ref().unwrap().args;

        let mut datatypes: Vec<types::DataType> = Vec::new();
        let mut mutability: Vec<types::DataMutablility> = Vec::new();
        let mut inktypes: Vec<inkwell::types::BasicMetadataTypeEnum> = Vec::new();

        if template_types.is_none() {
            for arg in &args.args {
                let (data, tp) = Self::get_llvm_from_type(&self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, &arg, node);
                datatypes.push(data);
                mutability.push(arg.mutability);


                let res: Option<inkwell::types::BasicMetadataTypeEnum> = Self::get_basicmeta_from_any(tp);

                if res.is_some() {
                    inktypes.push(res.unwrap());
                }
            }
        }
        else {
            for tp in izip![template_types.as_ref().unwrap(), &args.args] {
                mutability.push(tp.1.mutability);

                datatypes.push(tp.0.clone());
                let any = Self::get_anytp_from_tp(self.context, &self.inkwell_types, tp.0.clone());
                if any.is_none() {
                    unimplemented!();
                }
                let res: Option<inkwell::types::BasicMetadataTypeEnum> = Self::get_basicmeta_from_any(any.unwrap().clone());

                if res.is_some() {
                    inktypes.push(res.unwrap());
                }
            }
        }

        let rettp_tp: types::DataType;
        let rettp_any: inkwell::types::AnyTypeEnum;
        
        if rettp_opt.is_none() {
            let rettp_full: (types::DataType, inkwell::types::AnyTypeEnum) = Self::get_llvm_from_type(&self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, &args.rettp.last().unwrap(), node);
            rettp_tp = rettp_full.0;
            rettp_any = rettp_full.1;
        }
        else {
            rettp_tp = rettp_opt.unwrap().to_owned();
            let any_opt: Option<inkwell::types::AnyTypeEnum> = Self::get_anytp_from_tp(self.context, &self.inkwell_types, rettp_tp.clone());
            if any_opt.is_none() {
                unimplemented!();
            }
            rettp_any = any_opt.unwrap().to_owned();
        }

        self.expected_rettp = Some(rettp_tp.clone());
        
        let tp: inkwell::types::AnyTypeEnum = rettp_any;
        let fn_type: inkwell::types::FunctionType;
        
        if tp.is_int_type() {
            fn_type = tp.into_int_type().fn_type(&inktypes[..], false);
        }
        else if tp.is_float_type() {
            fn_type = tp.into_float_type().fn_type(&inktypes[..], false);
        }
        else if tp.is_function_type() {
            fn_type = tp.into_function_type().ptr_type(inkwell::AddressSpace::from(0u16)).fn_type(&inktypes[..], false);
        }
        else if tp.is_void_type() {
            fn_type = tp.into_void_type().fn_type(&inktypes[..], false);
        }
        else if tp.is_struct_type() {
            fn_type = tp.into_struct_type().fn_type(&inktypes[..], false);
        }
        else if tp.is_array_type() {
            fn_type = tp.into_array_type().fn_type(&inktypes[..], false);
        }
        else {
            panic!("Unexpected type");
        }

        //Main function specifics
        let mangled_name = self.mangle_name_main(&name);
        
        if self.get_function(&mangled_name) != None && self.get_function(&mangled_name).unwrap().2 != ForwardDeclarationType::Forward {
            let fmt: String = format!("Mangled function 'main' name '{}' is already defined.", mangled_name);
            errors::raise_error(&fmt, errors::ErrorType::RedefinitionAttempt, &node.pos, self.info);
        }
        if name == "main" {
            if datatypes.len() != 0 {
                let fmt: String = format!("Expected 0 arguments, got {}.", datatypes.len());
                errors::raise_error(&fmt, errors::ErrorType::ArgumentCountMismatch, &node.pos, self.info);
            }

            if fn_type.get_return_type() != None {
                let fmt: String = format!("Expected 'void' return type, got '{}'.", &rettp_tp);
                errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
            }
        }
        //

        let func: inkwell::values::FunctionValue;

        let mut dtp: types::DataType = self.datatypes.get(&types::BasicDataType::Func.to_string()).unwrap().clone();
        dtp.names = Some(node.data.func.as_ref().unwrap().args.name.clone());
        dtp.types = datatypes.clone();
        dtp.mutability =mutability.clone();
        dtp.rettp =  Some(Box::new(rettp_tp.clone()));

        if template_types.is_some() {
            if self.module.get_function(mangled_name.as_str()).is_some() {
                func = self.module.get_function(mangled_name.as_str()).replace(self.module.add_function(mangled_name.as_str(), fn_type, None)).unwrap();
            }
            else {
                func = self.module.add_function(mangled_name.as_str(), fn_type, None);
            }
            self.namespaces.template_functions.push((name.clone(), dtp.clone(), func.clone()));
        }
        else if  node.data.func.as_ref().unwrap().methodname.is_some() ||
            node.data.func.as_ref().unwrap().namespacename.is_some() {
            let structnm: &String = &node.data.func.as_ref().unwrap().name;

            if self.namespaces.structs.get(structnm).is_none() {
                let fmt: String = format!("Struct '{}' is not defined.", structnm);
                errors::raise_error(&fmt, errors::ErrorType::StructNotDefined, &node.pos, self.info);
            }

            func = self.module.add_function(&(structnm.to_owned()+"."+mangled_name.as_str()), fn_type, None);
    
            let mut s: (types::DataType, Option<AnyTypeEnum>, std::collections::HashMap<String, i32>, ForwardDeclarationType) = self.namespaces.structs.get(structnm).unwrap().clone();
            let mut isinstance: bool = true;
            if node.data.func.as_ref().unwrap().namespacename.is_some() {
                isinstance = false;
            }
            s.0.methods.insert(name.clone(), types::Method {
                tp: types::MethodType::Fn,
                builtin: None,
                func: Some(func.as_global_value().as_pointer_value()),
                functp: dtp.clone(),
                isinstance: isinstance,
            });

            self.namespaces.structs.insert(structnm.to_owned(), (s.0, s.1, s.2, s.3));
        }
        else {
            if self.module.get_function(mangled_name.as_str()).is_some() {
                func = self.module.get_function(mangled_name.as_str()).replace(self.module.add_function(mangled_name.as_str(), fn_type, None)).unwrap();
            }
            else {
                func = self.module.add_function(mangled_name.as_str(), fn_type, None);
            }
            self.namespaces.functions.insert(name.clone(), (func, dtp.clone(), ForwardDeclarationType::Real));
        }
        
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
            &name,
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
        self.enclosing_block = Some(basic_block);
        self.builder.set_current_debug_location(self.context, location);

        let mut attr: inkwell::attributes::Attribute = self.context.create_enum_attribute(inkwell::attributes::Attribute::get_named_enum_kind_id("noinline"), 0);

        func.add_attribute(inkwell::attributes::AttributeLoc::Function, attr);

        attr = self.context.create_enum_attribute(inkwell::attributes::Attribute::get_named_enum_kind_id("optnone"), 0);

        func.add_attribute(inkwell::attributes::AttributeLoc::Function, attr);
        
        self.builder.position_at_end(basic_block); 
        
        //Setup locals
        let prev_locals = self.namespaces.locals.to_owned();
        self.namespaces.locals = Vec::new();
        self.namespaces.locals.push(std::collections::HashMap::new());
        
        //Setup arguments
        let mut idx: u32 = 0;
        let mut idx_mut: usize = 0;
        for (name, tp) in std::iter::zip(&args.name, &datatypes) { 
            if name.get(0..1).unwrap() == "_" {
                continue;
            }
            let mut argv: Option<inkwell::values::BasicValueEnum> = None;
            if *tp != types::BasicDataType::Void {
                argv = func.get_nth_param(idx);
                idx += 1;
            }

            let ptr: inkwell::values::PointerValue;
        if argv.is_some() {
                ptr = self.builder.build_alloca(argv.unwrap().get_type(), name.as_str());
            
                self.builder.build_store(ptr, argv.unwrap());
                self.namespaces.locals.last_mut().unwrap().insert(name.to_string(), (Some(ptr), tp.clone(), mutability.get(idx_mut).unwrap().clone(), types::DataOwnership {owned: true, transferred: None}, node.pos.clone(), InitializationStatus::Initialized));
            }
            else {
                self.namespaces.locals.last_mut().unwrap().insert(name.to_string(), (None, tp.clone(), types::DataMutablility::Immutable, types::DataOwnership {owned: true, transferred: None}, node.pos.clone(), InitializationStatus::Initialized));
            }
            idx_mut += 1;
        }

        /////// Code generation start:

        let retv: types::Data = self.compile(&node.data.func.as_ref().unwrap().blocks, true, true);
        
        //Reset locals
        self.namespaces.locals = prev_locals;

        /////// End
        
        //Check if last stmt. is a return
        if node.data.func.as_ref().unwrap().blocks.len()==0 || node.data.func.as_ref().unwrap().blocks.last().unwrap().tp != parser::NodeType::RETURN {
            if retv.tp != rettp_tp.tp && name!="main"{
                let fmt: String = format!("Expected '{}' return type, got '{}'.", &rettp_tp, retv.tp);
                errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
            }

            if rettp_tp.tp != types::BasicDataType::Void {
                self.builder.build_return(Some(&retv.data.unwrap())); 
            }
            else {
                self.builder.build_return(None);
            }
        }
        
        let pass_manager_builder: inkwell::passes::PassManagerBuilder = inkwell::passes::PassManagerBuilder::create();
        pass_manager_builder.set_optimization_level(inkwell::OptimizationLevel::Aggressive);
        let manager = inkwell::passes::PassManager::create(&self.module);
        manager.add_cfg_simplification_pass();
        pass_manager_builder.populate_function_pass_manager(&manager);


        unsafe { func.run_in_pass_manager(&manager); }
        
        if node.data.func.as_ref().unwrap().blocks.len() > 0 && !retv.owned {
            let fmt: String = format!("Return value is not owned.");
            errors::raise_error(&fmt, errors::ErrorType::ReturnValueNotOwned, &node.data.func.as_ref().unwrap().blocks.last().unwrap().pos, self.info);
        }

        let data: types::Data = types::Data {
            data: None,
            tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
            owned: true,
        };
        return data;
    }
    
    fn build_assign(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let right: types::Data = self.compile_expr(&node.data.assign.as_ref().unwrap().expr, true, false);

        let name: String = node.data.assign.as_ref().unwrap().name.clone();
        
        if self.get_variable(&name).0.is_none() {
            let fmt: String = format!("Name '{}' is not defined in namespace.", name);
            errors::raise_error(&fmt, errors::ErrorType::NameNotFound, &node.pos, self.info);
        }

        if right.data == None{
            let fmt: String = format!("Cannot assign to '{}'.", right.tp.to_string());
            errors::raise_error(&fmt, errors::ErrorType::CannotAssign, &node.pos, self.info);
        }

        if  self.get_variable(&name).0.unwrap().2 == types::DataMutablility::Immutable &&
            self.get_variable(&name).0.unwrap().5 == InitializationStatus::Initialized {
            let fmt: String = format!("Cannot assign to immutable variable.");
            errors::raise_error(&fmt, errors::ErrorType::ImmutableAssign, &node.pos, self.info);
        }

        if self.get_variable(&name).0.unwrap().1 != right.tp {
            let fmt: String = format!("Expected '{}' type, got '{}' type.", self.get_variable(&name).0.unwrap().1.tp.to_string(), right.tp.to_string());
            errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
        }

        let ptr: Option<inkwell::values::PointerValue> = self.get_variable(&name).0.unwrap().0;

        if ptr.is_some() {
            let ptr: inkwell::values::PointerValue = self.get_variable(&name).0.unwrap().0.unwrap().clone();
            if self.get_variable(&name).0.unwrap().1.is_dyn {
                let typ: types::Type = Self::get_type_from_data(self.types.clone(), &right);
                let (dyntp, _) = Self::get_llvm_from_type(&self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, &node.data.letn.as_ref().unwrap().tp.as_ref().unwrap(), node);

                if !typ.traits.contains_key(&dyntp.name) {
                    let fmt: String = format!("'{}' type does not implement '{}' trait.", right.tp.to_string(), dyntp.name);
                    errors::raise_error(&fmt, errors::ErrorType::MissingTrait, &node.pos, self.info);
                }

                if right.tp.tp != types::BasicDataType::Struct {
                    let fmt: String = format!("Expected struct, got '{}'.", right.tp.tp);
                    errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
                }

                for sig in self.traits.get(&dyntp.name).unwrap().trait_sig.as_ref().unwrap() {
                    if sig.template_types.len() > 0 {
                        let fmt: String = format!("Trait '{}' is not trait object safe because function '{}' is templated.", dyntp.name, sig.name);
                        errors::raise_error(&fmt, errors::ErrorType::TraitIsNotTraitObjSafe, &node.pos, self.info);
                    }
                }

                let idptr = self.builder.build_struct_gep(ptr, 0u32, "idptr").expect("GEP error");
                self.builder.build_store(idptr, self.inkwell_types.i32tp.const_int(*self.namespaces.structid.get(&right.tp.name).unwrap() as u64, false));

                let itmptr = self.builder.build_struct_gep(ptr, 1u32, "item").expect("GEP error");
                let structptr: inkwell::values::PointerValue = self.builder.build_malloc(right.data.unwrap().get_type(), "struct_ptr").expect("Malloc error");
                self.builder.build_store(structptr, right.data.unwrap());
                self.builder.build_store(itmptr, self.builder.build_pointer_cast(structptr, itmptr.get_type().get_element_type().into_pointer_type(), "st_bitcast"));
                
                let idx: usize = self.get_variable(&name).1;

                self.namespaces.locals.get_mut(idx).unwrap().insert(name, (Some(ptr), dyntp, node.data.letn.as_ref().unwrap().mutability, types::DataOwnership {owned: true, transferred: None}, node.pos.clone(), InitializationStatus::Initialized));
            }
            else {
                self.builder.build_store(ptr, right.data.unwrap());

                let idx: usize = self.get_variable(&name).1;
                
                self.namespaces.locals.get_mut(idx).unwrap().insert(name, (Some(ptr), right.tp.clone(), types::DataMutablility::Mutable, types::DataOwnership {owned: true, transferred: None}, node.pos.clone(), InitializationStatus::Initialized));
            }
        }

        return right;
    }
    
    fn build_call(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let mut args: Vec<types::Data> = Vec::new();
        let mut tp_name: String = String::from("");
        let mut tp: Option<types::Type> = None;

        let mut have_template_method: bool = false;

        if node.data.call.as_ref().unwrap().name.tp == parser::NodeType::ATTR {
            let attr: &String = &node.data.call.as_ref().unwrap().name.data.attr.as_ref().unwrap().attr;

            let base: types::Data = self.compile_expr(&node.data.call.as_ref().unwrap().name.data.attr.as_ref().unwrap().name, false, true);

            if base.tp.is_dyn {
                let idptr: inkwell::values::PointerValue = self.builder.build_struct_gep(base.data.unwrap().into_pointer_value(), 0u32, "id_ptr").expect("GEP error");

                let vtable: inkwell::values::PointerValue = unsafe { self.builder.build_in_bounds_gep(self.vtables.unwrap().as_pointer_value(), &[self.builder.build_load(idptr, "id").into_int_value(), self.inkwell_types.i32tp.const_zero()], "vtable") };
                
                let idx: usize = self.traits.get(&base.tp.name).unwrap().trait_sig.as_ref().unwrap().iter().position(|x| &x.name == attr).unwrap();
                
                let method: inkwell::values::PointerValue = self.builder.build_load( unsafe { self.builder.build_in_bounds_gep(vtable, &[self.inkwell_types.i32tp.const_int(idx as u64, false), self.inkwell_types.i32tp.const_zero()], "method_ptr") }, "method").into_pointer_value();

                let mut mtp: types::DataType = self.datatypes.get(&types::BasicDataType::Func.to_string()).unwrap().clone();

                let mut tsig: Option<types::TemplateTraitSignature> = None;
                for sig in self.traits.get(&base.tp.name).unwrap().trait_sig.as_ref().unwrap() {
                    if &sig.name == attr {
                        tsig = Some(sig.clone());
                        break;
                    }
                }

                if tsig.is_none() {
                    let fmt: String = format!("Type '{}' has no method '{}'.", base.tp, attr);
                    errors::raise_error(&fmt, errors::ErrorType::StructAttrNotFound, &node.pos, self.info);
                }

                let func_args = tsig.unwrap().args;

                let mut datatypes: Vec<types::DataType> = Vec::new();
                let mut names: Vec<String> = Vec::new();
                
                for arg in &func_args.args {
                    let (data, _) = Self::get_llvm_from_type(&self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, &arg, node);
                    
                    if data.tp != types::BasicDataType::Void {
                        names.push(String::from(""));
                    }   

                    datatypes.push(data);                  
                }
                
                let rettp_full: (types::DataType, inkwell::types::AnyTypeEnum) = Self::get_llvm_from_type(&self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, &func_args.rettp.last().unwrap(), node);
                let rettp_tp: types::DataType = rettp_full.0;

                mtp.types = datatypes;
                mtp.names = Some(names);
                mtp.rettp = Some(Box::new(rettp_tp));
                
                args.push(types::Data {
                    data: Some(inkwell::values::BasicValueEnum::PointerValue(method)),
                    tp: mtp,
                    owned: true,
                });

                args.push(types::Data {
                    data: Some(self.builder.build_load(base.data.unwrap().into_pointer_value(), "instance")),
                    tp: base.tp.clone(),
                    owned: base.owned,
                });

                tp_name = self.datatypes.get(&types::BasicDataType::Func.to_string()).unwrap().clone().name.clone();
                
                tp = Some(Self::get_type_from_data(self.types.clone(), args.first().unwrap()));
            }
            else if base.tp.methods.get(attr).is_some() {
                let method: &types::Method = base.tp.methods.get(attr).unwrap();
                if method.tp == types::MethodType::Fn {
                    let data: types::Data = types::Data {
                        data: Some(inkwell::values::BasicValueEnum::PointerValue(method.func.unwrap())),
                        tp: method.functp.clone(),
                        owned: true,
                    };

                    args.push(data.clone());
                    if method.isinstance {
                        args.push(types::Data {
                            data: Some(self.builder.build_load(base.data.unwrap().into_pointer_value(), &base.tp.name)),
                            tp: base.tp.clone(),
                            owned: base.owned,
                        });
                    }

                    tp_name = method.functp.name.clone();
                    
                    tp = Some(Self::get_type_from_data(self.types.clone(), &data));
                }
                else {
                    let mut tp_: types::DataType = self.datatypes.get(&types::BasicDataType::WrapperFunc.to_string()).unwrap().clone();
                    tp_.wrapperfn = method.builtin;
                    tp_name = tp_.name.clone();
                    let data: types::Data = types::Data {
                        data: None,
                        tp: tp_,
                        owned: true,
                    };

                    tp = Some(Self::get_type_from_data(self.types.clone(), &data.clone()));

                    args.push(data);
                    if method.isinstance {
                        args.push(base.clone());
                    }
                }
            }
            else if self.namespaces.template_functions_sig.contains_key(&(base.tp.name.clone()+"."+node.data.call.as_ref().unwrap().name.data.attr.as_ref().unwrap().attr.to_owned().as_str()).to_owned()) {
                have_template_method = true;
            }
            else{
                let fmt: String = format!("Type '{}' has no method '{}'.", base.tp, attr);
                errors::raise_error(&fmt, errors::ErrorType::StructAttrNotFound, &node.pos, self.info);
            }
        }
        else if node.data.call.as_ref().unwrap().name.tp == parser::NodeType::IDENTIFIER &&
                self.namespaces.template_functions_sig.contains_key(&node.data.call.as_ref().unwrap().name.data.identifier.as_ref().unwrap().name) {
            // Do nothing yet
        }
        else {
            let callable: types::Data = self.compile_expr(&node.data.call.as_ref().unwrap().name, false, false);
            tp_name = callable.tp.name.clone();
            args.push(callable);
            tp = Some(Self::get_type_from_data(self.types.clone(), &args.first().unwrap()));
        }

        for arg in &node.data.call.as_ref().unwrap().args{
            let v: types::Data = self.compile_expr(arg, true, false); 
            if v.tp.tp != types::BasicDataType::Struct {
                args.push(v);
            }
            else {
                args.push(types::Data {
                    data: Some(self.builder.build_load(v.data.unwrap().into_pointer_value(), &v.tp.name)),
                    tp: v.tp.clone(),
                    owned: v.owned,
                });
            }
        }

        if  node.data.call.as_ref().unwrap().name.tp == parser::NodeType::IDENTIFIER &&
            self.namespaces.template_functions_sig.contains_key(&node.data.call.as_ref().unwrap().name.data.identifier.as_ref().unwrap().name) {
            let func: parser::Node = self.namespaces.template_functions_sig.get(&node.data.call.as_ref().unwrap().name.data.identifier.as_ref().unwrap().name).unwrap().0.to_owned();
            let mut fn_types: Vec<types::DataType> = Vec::new();
            let mut templates: std::collections::HashMap<String, types::DataType> = std::collections::HashMap::new();
            
            for (data, arg) in izip![&args, &func.data.func.as_ref().unwrap().args.args] {
                if  !arg.isarr &&
                    !arg.isfn &&
                    !arg.isdyn && !self.datatypes.contains_key(&arg.data.as_ref().unwrap().clone()) {
                    if !templates.contains_key(&arg.data.as_ref().unwrap().clone()) {
                        if !func.data.func.as_ref().unwrap().template_types.contains(&arg.data.as_ref().unwrap().clone()) {
                            let fmt: String = format!("Unknown type '{}'.", arg.data.as_ref().unwrap().clone());
                            errors::raise_error(&fmt, errors::ErrorType::UnknownTemplateType, &node.pos, self.info);    
                        }
                        templates.insert(arg.data.as_ref().unwrap().clone(), data.tp.clone());
                    }
                    fn_types.push(templates.get(&arg.data.as_ref().unwrap().clone()).unwrap().to_owned());
                }
                else {
                    fn_types.push(Self::get_llvm_from_type(self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, arg, node).0.to_owned());
                }
            }

            let rettp: &parser::Type = func.data.func.as_ref().unwrap().args.rettp.last().unwrap();
            let rettp_tp: types::DataType;
            if  !rettp.isarr &&
                !rettp.isfn &&
                !rettp.isdyn &&
                !self.datatypes.contains_key(&rettp.data.as_ref().unwrap().clone()) {
                if !templates.contains_key(&rettp.data.as_ref().unwrap().clone()) {
                    let fmt: String = format!("Unknown type '{}'.", rettp.data.as_ref().unwrap().clone());
                    errors::raise_error(&fmt, errors::ErrorType::UnknownTemplateType, &node.pos, self.info); 
                }
                rettp_tp = templates.get(&rettp.data.as_ref().unwrap().clone()).unwrap().to_owned();
            }
            else {
                rettp_tp = Self::get_llvm_from_type(self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, rettp, node).0.to_owned();
            }
            
            let enclosing_block: inkwell::basic_block::BasicBlock = self.enclosing_block.unwrap();
            self.build_func(&func, None, Some(fn_types), Some(rettp_tp));
            self.enclosing_block = Some(enclosing_block);

            let func_v = self.namespaces.template_functions.last().unwrap().to_owned();
            self.namespaces.template_functions.pop();
            if !self.namespaces.template_functions.contains(&func_v) {
                self.namespaces.template_functions.push(func_v.to_owned());
            }
            
            tp_name = func_v.1.name.clone();
            let callable: types::Data = types::Data {
                data: Some(inkwell::values::BasicValueEnum::PointerValue(func_v.2.as_global_value().as_pointer_value())),
                tp: func_v.1.clone(),
                owned: true,
            };

            args.insert(0usize, callable);
            tp = Some(Self::get_type_from_data(self.types.clone(), &args.first().unwrap()));
            self.builder.position_at_end(self.enclosing_block.unwrap());
        }

        if have_template_method {
            let base: types::Data = self.compile_expr(&node.data.call.as_ref().unwrap().name.data.attr.as_ref().unwrap().name, false, true);

            if  self.namespaces.template_functions_sig.contains_key(&(base.tp.name.clone()+"."+node.data.call.as_ref().unwrap().name.data.attr.as_ref().unwrap().attr.to_owned().as_str()).to_owned()) {
                let func: parser::Node = self.namespaces.template_functions_sig.get(&(base.tp.name.clone()+"."+node.data.call.as_ref().unwrap().name.data.attr.as_ref().unwrap().attr.to_owned().as_str()).to_owned()).unwrap().0.to_owned();
                let instance_meth: TemplateFunctionInstance = self.namespaces.template_functions_sig.get(&(base.tp.name.clone()+"."+node.data.call.as_ref().unwrap().name.data.attr.as_ref().unwrap().attr.to_owned().as_str()).to_owned()).unwrap().1.to_owned();
                
                let mut fn_types: Vec<types::DataType> = Vec::new();
                let mut templates: std::collections::HashMap<String, types::DataType> = std::collections::HashMap::new();
                
                let mut idx: i32 = 0;
                for arg in &func.data.func.as_ref().unwrap().args.args {
                    if  !arg.isarr &&
                        !arg.isfn &&
                        !arg.isdyn && !self.datatypes.contains_key(&arg.data.as_ref().unwrap().clone()) && idx > 0 {
                        if !templates.contains_key(&arg.data.as_ref().unwrap().clone()) {
                            if !func.data.func.as_ref().unwrap().template_types.contains(&arg.data.as_ref().unwrap().clone()) {
                                let fmt: String = format!("Unknown type '{}'.", arg.data.as_ref().unwrap().clone());
                                errors::raise_error(&fmt, errors::ErrorType::UnknownTemplateType, &node.pos, self.info);    
                            }
                            templates.insert(arg.data.as_ref().unwrap().clone(), args.get((idx-1) as usize).unwrap().tp.clone());
                        }
                        fn_types.push(templates.get(&arg.data.as_ref().unwrap().clone()).unwrap().to_owned());
                    }
                    else if !arg.isarr &&
                            !arg.isfn &&
                            !arg.isdyn &&
                            idx == 0 &&
                            !self.datatypes.contains_key(&arg.data.as_ref().unwrap().clone()) && instance_meth != TemplateFunctionInstance::Instance {
                        if !templates.contains_key(&arg.data.as_ref().unwrap().clone()) {
                            if !func.data.func.as_ref().unwrap().template_types.contains(&arg.data.as_ref().unwrap().clone()) {
                                let fmt: String = format!("Unknown type '{}'.", arg.data.as_ref().unwrap().clone());
                                errors::raise_error(&fmt, errors::ErrorType::UnknownTemplateType, &node.pos, self.info);    
                            }
                            templates.insert(arg.data.as_ref().unwrap().clone(), args.get(0).unwrap().tp.clone());
                        }
                        fn_types.push(templates.get(&arg.data.as_ref().unwrap().clone()).unwrap().to_owned());
                    }
                    else if idx == 0 && !self.datatypes.contains_key(&arg.data.as_ref().unwrap().clone()) && instance_meth == TemplateFunctionInstance::Instance  {
                        let fmt: String = format!("First argument for template method may not be template.");
                        errors::raise_error(&fmt, errors::ErrorType::MethodTemplateFunctionHasFirstTemplate, &node.pos, self.info);
                    }
                    else {
                        fn_types.push(Self::get_llvm_from_type(self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, arg, node).0.to_owned());
                    }
                    idx += 1;
                }

                let rettp: &parser::Type = func.data.func.as_ref().unwrap().args.rettp.last().unwrap();
                let rettp_tp: types::DataType;
                if  !rettp.isarr &&
                    !rettp.isfn &&
                    !rettp.isdyn &&
                    !self.datatypes.contains_key(&rettp.data.as_ref().unwrap().clone()) {
                    if !templates.contains_key(&rettp.data.as_ref().unwrap().clone()) {
                        let fmt: String = format!("Unknown type '{}'.", rettp.data.as_ref().unwrap().clone());
                        errors::raise_error(&fmt, errors::ErrorType::UnknownTemplateType, &node.pos, self.info); 
                    }
                    rettp_tp = templates.get(&rettp.data.as_ref().unwrap().clone()).unwrap().to_owned();
                }
                else {
                    rettp_tp = Self::get_llvm_from_type(self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, rettp, node).0.to_owned();
                }
                
                let enclosing_block: inkwell::basic_block::BasicBlock = self.enclosing_block.unwrap();
                self.build_func(&func, None, Some(fn_types), Some(rettp_tp));
                self.enclosing_block = Some(enclosing_block);

                let func_v = self.namespaces.template_functions.last().unwrap().to_owned();
                self.namespaces.template_functions.pop();
                if !self.namespaces.template_functions.contains(&func_v) {
                    self.namespaces.template_functions.push(func_v.to_owned());
                }
                
                tp_name = func_v.1.name.clone();
                let callable: types::Data = types::Data {
                    data: Some(inkwell::values::BasicValueEnum::PointerValue(func_v.2.as_global_value().as_pointer_value())),
                    tp: func_v.1.clone(),
                    owned: true,
                };

                args.insert(0usize, callable.clone());
                tp = Some(Self::get_type_from_data(self.types.clone(), &callable));
                self.builder.position_at_end(self.enclosing_block.unwrap());
                if instance_meth == TemplateFunctionInstance::Instance {
                    args.insert(1usize, types::Data {
                        data: Some(self.builder.build_load(base.data.unwrap().into_pointer_value(), &base.tp.name)),
                        tp: base.tp.clone(),
                        owned: base.owned,
                    });
                }
            }
        }


        let t: &types::Trait = match tp.as_ref().unwrap().traits.get(&types::TraitType::Call.to_string()) {
            Some (v) => {
                v
            }
            None => {
                let fmt: String = format!("Type '{}' has no trait '{}'.", tp_name, &types::TraitType::Call.to_string());
                errors::raise_error(&fmt, errors::ErrorType::MissingTrait, &node.pos, self.info);
            }
        };

        let data: types::Data = self.call_trait(t, args, node);

        return data;
    }

    fn build_return(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let retv: types::Data = if node.data.ret.as_ref().unwrap().expr.is_some() { self.compile_expr(&node.data.ret.as_ref().unwrap().expr.as_ref().unwrap(), true, false) } else { types::Data {
            data: None,
            tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
            owned: true,
        } };

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
                errors::raise_error(&fmt, errors::ErrorType::ReturnValueNotOwned, if node.data.ret.as_ref().unwrap().expr.is_some() { &node.data.ret.as_ref().unwrap().expr.as_ref().unwrap().pos } else { &node.pos }, self.info);
            }
            self.builder.build_return(Some(&retv.data.unwrap())); 
        }
        else {
            self.builder.build_return(None);
        }
  
        return retv;
    }

    fn build_as(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let left: types::Data = self.compile_expr(&node.data.to.as_ref().unwrap().left, false, false);     
        let arg: &parser::Type = &node.data.to.as_ref().unwrap().tp;  
        if arg.isfn {
            let fmt: String = format!("Non primitive cast from '{}' to 'fn'.", left.tp);
            errors::raise_error(&fmt, errors::ErrorType::InvalidCast, &node.pos, self.info);
        }
        let tp_name: &String = &arg.data.as_ref().unwrap();

        let tp: &types::DataType = if self.datatypes.get(tp_name).is_none() {
            let fmt: String = format!("Unknown type '{}'.", tp_name);
            errors::raise_error(&fmt, errors::ErrorType::UnknownType, &node.pos, self.info);
        } else {
            self.datatypes.get(tp_name).unwrap()
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
            let res: inkwell::values::IntValue = if builtin_types::int_issigned(tp.clone()) {
                self.builder.build_float_to_signed_int(left.data.unwrap().into_float_value(), anytp.unwrap().into_int_type(), "ftoi")
            }
            else {
                self.builder.build_float_to_unsigned_int(left.data.unwrap().into_float_value(), anytp.unwrap().into_int_type(), "ftoui")
            };

            return types::Data {
                data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
                tp: tp.clone(),
                owned: true,
            };
        } 
        else {
            let fmt: String = format!("Non primitive cast from '{}' to '{}'.", left.tp, tp_name);
            errors::raise_error(&fmt, errors::ErrorType::InvalidCast, &node.pos, self.info);
        }
    }

    fn build_ref(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        return self.compile_expr(&node.data.unary.as_ref().unwrap().right, false, false);
    }

    fn build_unary(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let unary: &parser::nodes::UnaryNode = node.data.unary.as_ref().unwrap();

        let right: types::Data = self.compile_expr(&unary.right, false, false);

        let mut args: Vec<types::Data> = Vec::new();

        let tp: types::Type = Self::get_type_from_data(self.types.clone(), &right);

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

        let data: types::Data = self.call_trait(t, args, node);
        
        return data;
    }

    fn build_struct(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        if self.namespaces.structs.get(&node.data.st.as_ref().unwrap().name.clone()).is_some() && self.namespaces.structs.get(&node.data.st.as_ref().unwrap().name.clone()).unwrap().3 != ForwardDeclarationType::Forward {
            let fmt: String = format!("Struct '{}' is already defined.", node.data.st.as_ref().unwrap().name.clone());
            errors::raise_error(&fmt, errors::ErrorType::RedefinitionAttempt, &node.pos, self.info);
        }
        if self.datatypes.get(&node.data.st.as_ref().unwrap().name.clone()).is_some() && self.namespaces.structs.get(&node.data.st.as_ref().unwrap().name.clone()).unwrap().3 != ForwardDeclarationType::Forward {
            let fmt: String = format!("Type '{}' is already defined.", node.data.st.as_ref().unwrap().name.clone());
            errors::raise_error(&fmt, errors::ErrorType::TypeRedefinitionAttempt, &node.pos, self.info);
        }

        self.namespaces.structid.insert(node.data.st.as_ref().unwrap().name.clone(), self.namespaces.structid_max);
        self.namespaces.structid_from.insert(self.namespaces.structid_max, node.data.st.as_ref().unwrap().name.clone());
        self.namespaces.structid_max += 1;
        
        if !node.data.st.as_ref().unwrap().name.is_camel_case() {
            errors::show_warning(errors::WarningType::ExpectedCamelCase, vec![String::from(""), node.data.st.as_ref().unwrap().name.to_camel_case()], vec![String::from("Expected camel case"), String::from("Convert to this: ")], &node.pos, self.info)
        }

        let mut names: Vec<String> = Vec::new();
        let mut types: Vec<(types::DataType, AnyTypeEnum)> = Vec::new();
        let mut simpletypes: Vec<types::DataType> = Vec::new();
        let mut mutabilitites: Vec<types::DataMutablility> = Vec::new();
        let mut idxmapping: std::collections::HashMap<String, i32> = std::collections::HashMap::new();

        let mut idx = 0;
        for member in &node.data.st.as_ref().unwrap().names {
            if !member.is_snake_case() {
                errors::show_warning(errors::WarningType::ExpectedSnakeCase, vec![String::from(""), member.to_camel_case()], vec![String::from("Expected snake case"), String::from("Convert to this: ")], &node.pos, self.info)
            }
            if names.contains(&member.clone()) {
                let fmt: String = format!("Field '{}' is already declared.", member.clone());
                errors::raise_error(&fmt, errors::ErrorType::FieldRedeclaration, &node.pos, self.info);
            }
            names.push(member.clone());
            types.push(Self::get_llvm_from_type(self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, node.data.st.as_ref().unwrap().members.get(member).unwrap(), node));
            simpletypes.push(Self::get_llvm_from_type(self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, node.data.st.as_ref().unwrap().members.get(member).unwrap(), node).0);
            mutabilitites.push(node.data.st.as_ref().unwrap().members.get(member).unwrap().mutability);
            idxmapping.insert(member.clone(), idx);
            idx+=1;
        }

        let mut tp: types::DataType = self.datatypes.get(&types::BasicDataType::Struct.to_string()).unwrap().clone();
        tp.name = node.data.st.as_ref().unwrap().name.clone();
        tp.names = Some(names);
        tp.types = simpletypes.clone();
        tp.mutability = mutabilitites;

        self.datatypes.insert(node.data.st.as_ref().unwrap().name.clone(), tp.clone());
        self.namespaces.structs.insert(node.data.st.as_ref().unwrap().name.clone(), (tp, Some(Self::build_struct_tp_from_types(self.context, &self.inkwell_types, &simpletypes)), idxmapping, ForwardDeclarationType::Real));
        
        let data: types::Data = types::Data {
            data: None,
            tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
            owned: true,
        };
        return data;
    }

    fn build_initstruct(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let mut members: std::collections::HashMap<String, types::Data> = std::collections::HashMap::new();
        let name: String = node.data.initst.as_ref().unwrap().name.clone();

        if self.namespaces.structs.get(&name).is_none() {
            let fmt: String = format!("Struct '{}' is not defined.", name);
            errors::raise_error(&fmt, errors::ErrorType::StructNotDefined, &node.pos, self.info);
        }

        let s: (types::DataType, Option<AnyTypeEnum>, std::collections::HashMap<String, i32>, ForwardDeclarationType) = self.namespaces.structs.get(&name).unwrap().clone();

        for member in &node.data.initst.as_ref().unwrap().members_vec {
            if members.contains_key(member) {
                let fmt: String = format!("Field '{}' is already declared.", member);
                errors::raise_error(&fmt, errors::ErrorType::FieldReinitialization, &node.pos, self.info);
            }
            members.insert(member.clone(), self.compile_expr(&(&node.data.initst.as_ref().unwrap().members).get(member).unwrap().clone(), true, false));
        }
        
        if s.0.names.as_ref().unwrap().len() != members.len() {
            let fmt: String = format!("Expected {} members, got {}.", s.0.names.as_ref().unwrap().len(), members.len());
            errors::raise_error(&fmt, errors::ErrorType::InvalidMemberCount, &node.pos, self.info);
        }

        for member in &members {
            if !s.0.names.as_ref().unwrap().contains(member.0) {
                let fmt: String = format!("Member '{}' does not exist.", member.0);
                errors::raise_error(&fmt, errors::ErrorType::MemberNameNotFound, &node.pos, self.info);
            }
        }

        for (member, tp, name) in izip!(&node.data.initst.as_ref().unwrap().members_vec, &s.0.types, s.0.names.as_ref().unwrap()) {
            if members.get(member).unwrap().tp != *tp {
                let fmt: String = format!("Expected '{}' type for member '{}', got '{}'.", tp, name, members.get(member).unwrap().tp);
                errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
            }
        }
        
        let ptr: inkwell::values::PointerValue = self.builder.build_alloca(s.1.unwrap().into_struct_type(), name.as_str());
        
        for member in &node.data.initst.as_ref().unwrap().members_vec {
            if members.get(member).unwrap().data.is_some() {
                let itmptr: inkwell::values::PointerValue = self.builder.build_struct_gep(ptr, *s.2.get(member).unwrap() as u32, &member.as_str()).expect("GEP Error");
                self.builder.build_store(itmptr, members.get(member).unwrap().data.unwrap());
            }
        }
        
        let data: types::Data = types::Data {
            data: Some(self.builder.build_load(ptr, name.as_str())),
            tp: s.0.clone(),
            owned: true,
        };
        return data;
    }

    fn build_attrload(&mut self, node: &parser::Node, get_ptr: bool) -> types::Data<'ctx> {
        let base: types::Data = self.compile_expr(&node.data.attr.as_ref().unwrap().name, false, true);

        if base.tp.tp != types::BasicDataType::Struct {
            let fmt: String = format!("Expected struct, got '{}'.", base.tp.tp);
            errors::raise_error(&fmt, errors::ErrorType::GetAttrOfNonStruct, &node.pos, self.info);
        }

        let attr: String = node.data.attr.as_ref().unwrap().attr.clone();

        if !base.tp.names.as_ref().unwrap().contains(&attr) {
            let fmt: String = format!("Type '{}' has no attribute '{}'.", base.tp, attr);
            errors::raise_error(&fmt, errors::ErrorType::StructAttrNotFound, &node.pos, self.info);
        }

        let mut idx: u32 = 0;
        for (attrn, tp) in izip![base.tp.names.as_ref().unwrap(), &base.tp.types] {
            if tp.tp == types::BasicDataType::Void {
                if attrn == &attr {
                    let data: types::Data = types::Data {
                        data: None,
                        tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
                        owned: true,
                    };
                    return data;
                }
                continue;
            }
            if attrn == &attr {
                break;
            }
            idx+=1;
        }

        let itmptr: inkwell::values::PointerValue = self.builder.build_struct_gep(base.data.unwrap().into_pointer_value(), idx, base.tp.name.as_str()).expect("GEP Error");
        if get_ptr {
            let data: types::Data = types::Data {
                data: Some(inkwell::values::BasicValueEnum::PointerValue(itmptr)),
                tp: base.tp.types.get(idx as usize).unwrap().clone(),
                owned: true,
            };
            return data;
        }
        
        let val = self.builder.build_load(itmptr, attr.as_str());

        let data: types::Data = types::Data {
            data: Some(val),
            tp: base.tp.types.get(idx as usize).unwrap().clone(),
            owned: true,
        };
        return data;
    }

    fn build_attrasssign(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let base: types::Data = self.compile_expr(&node.data.attrassign.as_ref().unwrap().name, false, true);

        if base.tp.tp != types::BasicDataType::Struct {
            let fmt: String = format!("Expected struct, got '{}'.", base.tp.tp);
            errors::raise_error(&fmt, errors::ErrorType::GetAttrOfNonStruct, &node.pos, self.info);
        }

        let attr: String = node.data.attrassign.as_ref().unwrap().attr.clone();

        if !base.tp.names.as_ref().unwrap().contains(&attr) {
            let fmt: String = format!("Struct '{}' has no attribute '{}'.", base.tp, attr);
            errors::raise_error(&fmt, errors::ErrorType::StructAttrNotFound, &node.pos, self.info);
        }

        let mut idx: u32 = 0;
        for (attrn, tp) in izip![base.tp.names.as_ref().unwrap(), &base.tp.types] {
            if tp.tp == types::BasicDataType::Void {
                if attrn == &attr {
                    let data: types::Data = types::Data {
                        data: None,
                        tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
                        owned: true,
                    };
                    return data;
                }
                continue;
            }
            if attrn == &attr {
                break;
            }
            idx+=1;
        }

        if base.tp.mutability.get(idx as usize).unwrap() == &types::DataMutablility::Immutable{
            let fmt: String = format!("Attribute '{}' is immutable.", base.tp);
            errors::raise_error(&fmt, errors::ErrorType::ImmutableAttr, &node.pos, self.info);
        }

        let itmptr: inkwell::values::PointerValue = self.builder.build_struct_gep(base.data.unwrap().into_pointer_value(), idx, base.tp.name.as_str()).expect("GEP Error");
        
        let expr: types::Data = self.compile_expr(&node.data.attrassign.as_ref().unwrap().expr, true, false);

        if &expr.tp != base.tp.types.get(idx as usize).unwrap() {
            let fmt: String = format!("Expected '{}' type, got '{}' type.", expr.tp, base.tp.types.get(idx as usize).unwrap());
            errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
        }

        if expr.data.is_some() {
            self.builder.build_store(itmptr, expr.data.unwrap());
        }

        let data: types::Data = types::Data {
            data: None,
            tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
            owned: true,
        };
        return data;
    }

    fn build_string(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let data: Vec<u8> = node.data.str.as_ref().unwrap().data.as_str().as_bytes().to_vec();
        
        let arraytp: inkwell::types::ArrayType = self.inkwell_types.i8tp.array_type(data.len() as u32);

        let mut arrdata: Vec<inkwell::values::IntValue> = Vec::new();
        for c in data {
            arrdata.push(self.inkwell_types.i8tp.const_int(c as u64, false));
        }

        let array: inkwell::values::ArrayValue = self.inkwell_types.i8tp.const_array(&arrdata[..]);

        let types: Vec<types::DataType> = vec![self.datatypes.get(&types::BasicDataType::I8.to_string()).unwrap().clone()];

        let mut tp: types::DataType = self.datatypes.get(&types::BasicDataType::Array.to_string()).unwrap().clone();
        tp.name = Self::array_repr(arraytp);
        tp.arrtp = Some(arraytp);
        tp.types = types;

        let data: types::Data = types::Data {
            data: Some(inkwell::values::BasicValueEnum::ArrayValue(array)),
            tp,
            owned: true,
        };
        return data;
    }

    fn build_char(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let mut data: std::str::Chars = node.data.num.as_ref().unwrap().left.chars();
        
        let selfv: inkwell::values::IntValue = self.inkwell_types.i32tp.const_int((data.next().unwrap()).into(), false);
        return types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: self.datatypes.get(&types::BasicDataType::U32.to_string()).unwrap().clone(), owned: true}
    }

    fn build_array(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let elements: &Vec<parser::Node> = &node.data.arr.as_ref().unwrap().elements;

        let mut data_elem: Vec<types::Data> = Vec::new();

        if elements.len() == 0 {
            let fmt: String = format!("Cannot define zero-length array.");
            errors::raise_error(&fmt, errors::ErrorType::ZeroLengthArray, &node.pos, self.info);
        }

        data_elem.push(self.compile_expr(elements.first().unwrap(), true, false));
        let firsttp_: Option<inkwell::types::AnyTypeEnum> = Self::get_anytp_from_tp(self.context, &self.inkwell_types, data_elem.first().unwrap().tp.clone());
        if firsttp_.is_none() {
            let fmt: String = format!("Cannot define array of 'void'.");
            errors::raise_error(&fmt, errors::ErrorType::CannotDefineVoidArray, &node.pos, self.info);
        }
        if firsttp_.unwrap().is_function_type() {
            let fmt: String = format!("Cannot define array of 'fn'.");
            errors::raise_error(&fmt, errors::ErrorType::CannotDefineFnArray, &node.pos, self.info);
        }
        let firsttp: inkwell::types::BasicTypeEnum = Self::get_basic_from_any(firsttp_.unwrap()).unwrap();

        for element in elements[1..].to_vec() {
            data_elem.push(self.compile_expr(&element, true, false));
            let tp_: Option<inkwell::types::AnyTypeEnum> = Self::get_anytp_from_tp(self.context, &self.inkwell_types, data_elem.last().unwrap().tp.clone());
            if tp_.is_none() {
                let fmt: String = format!("Expected '{}' type, got 'void' type.", data_elem.first().unwrap().tp.to_string());
                errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &element.pos, self.info);
            }
            let tp: inkwell::types::BasicTypeEnum = Self::get_basic_from_any(tp_.unwrap()).unwrap();
            if tp != firsttp {
                let fmt: String = format!("Expected '{}' type, got '{}' type.", data_elem.first().unwrap().tp.to_string(), data_elem.last().unwrap().tp.to_string());
                errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &element.pos, self.info);
            }
        }

        let firstdatatp: types::DataType = data_elem.first().unwrap().tp.clone();

        let arraytp: inkwell::types::ArrayType = firsttp.array_type(elements.len() as u32);
        let array: inkwell::values::PointerValue = self.builder.build_alloca(arraytp, "arr");

        for element in data_elem {
            let ptr: inkwell::values::PointerValue = unsafe { self.builder.build_gep(array, &[self.inkwell_types.i8tp.const_int(0, false), self.inkwell_types.i8tp.const_int(0, false)], &element.tp.name) };
            self.builder.build_store(ptr, element.data.unwrap());
        }        

        let mut tp: types::DataType = self.datatypes.get(&types::BasicDataType::Array.to_string()).unwrap().clone();
        tp.name = Self::array_repr(arraytp);
        tp.arrtp = Some(arraytp);
        tp.types = vec![firstdatatp];

        let data: types::Data = types::Data {
            data: Some(inkwell::values::BasicValueEnum::PointerValue(array)),
            tp,
            owned: true,
        };
        return data;
    }

    fn build_impl(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let traitnm: &String = &node.data.impln.as_ref().unwrap().traitnm;
        let structnm: &String = &node.data.impln.as_ref().unwrap().structnm;

        if !self.traits.contains_key(traitnm) {
            let fmt: String = format!("Trait '{}' not found.", traitnm.to_string());
            errors::raise_error(&fmt, errors::ErrorType::TraitNotFound, &node.pos, self.info);
        }

        let mut traitsig: types::TraitSignature = self.traits.get(traitnm).unwrap().clone();

        let mut functions: std::collections::HashMap<String, inkwell::values::PointerValue> = std::collections::HashMap::new();

        if traitsig.traittp == types::TraitMetatype::Builtin {
            if traitsig.name != node.data.impln.as_ref().unwrap().functions.last().unwrap().data.func.as_ref().unwrap().name {
                let fmt: String = format!("Trait '{}' expected function '{}'.", traitnm.to_string(), traitsig.name);
                errors::raise_error(&fmt, errors::ErrorType::TraitExpectProperFunctionName, &node.pos, self.info);
            }

            let nargs: usize = traitsig.nargs.unwrap();

            if nargs != node.data.impln.as_ref().unwrap().functions.last().unwrap().data.func.as_ref().unwrap().args.args.len() {
                let fmt: String = format!("Trait '{}' expected function with '{}' arguments.", traitnm.to_string(), traitsig.nargs.unwrap());
                errors::raise_error(&fmt, errors::ErrorType::ArgumentCountMismatch, &node.pos, self.info);
            }
            
            let func: types::Data = self.build_func(&node.data.impln.as_ref().unwrap().functions.last().unwrap(), Some(structnm.to_owned() + "." + node.data.impln.as_ref().unwrap().functions.last().unwrap().data.func.as_ref().unwrap().name.as_str()), None, None);

            if !self.namespaces.structs.contains_key(structnm) {
                let fmt: String = format!("Struct '{}' is not defined.", structnm);
                errors::raise_error(&fmt, errors::ErrorType::StructNotDefined, &node.pos, self.info);
            }
            
            let mut tp: types::Type = self.types.get(structnm).unwrap().clone();

            let rettp: types::DataType = Self::get_llvm_from_type(self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, node.data.impln.as_ref().unwrap().functions.last().unwrap().data.func.as_ref().unwrap().args.rettp.first().unwrap(), node).0;

            let traittp: Option<types::TraitType> = types::get_traittp_from_str(traitnm.to_owned());
            if traittp.as_ref().unwrap() == &types::TraitType::Call {
                let fmt: String = format!("Cannot implement 'Call' trait.");
                errors::raise_error(&fmt, errors::ErrorType::CannotImplementCallTrait, &node.pos, self.info);
            }

            tp.traits.insert(traitnm.to_owned(), builtin_types::create_trait_ink(func.data.unwrap().into_pointer_value(), nargs, traittp.unwrap(), rettp));
            
            self.types.insert(structnm.to_owned(), tp);

            functions.insert(traitsig.name, func.data.unwrap().into_pointer_value());
        }
        else {
            if !self.namespaces.structs.contains_key(structnm) {
                let fmt: String = format!("Struct '{}' is not defined.", structnm);
                errors::raise_error(&fmt, errors::ErrorType::StructNotDefined, &node.pos, self.info);
            }

            let traittp: Option<types::TraitType> = types::get_traittp_from_str(traitnm.to_owned());
            if traittp.is_some() {
                let fmt: String = format!("Cannot implement builtin trait '{}'.", traittp.unwrap().to_string());
                errors::raise_error(&fmt, errors::ErrorType::CannotImplementBuiltinTrait, &node.pos, self.info);
            }
            
            let mut tp: types::Type = self.types.get(structnm).unwrap().clone();
            
            tp.traits.insert(traitnm.to_owned(), builtin_types::create_empty_trait());  
            self.types.insert(structnm.to_owned(), tp);

            for var in traitsig.vars.as_ref().unwrap() {
                if !self.namespaces.structs.get(structnm).unwrap().0.names.as_ref().unwrap().contains(var.0) {
                    let fmt: String = format!("Struct '{}' does not implement required member '{}'.", structnm, var.0);
                    errors::raise_error(&fmt, errors::ErrorType::CannotImplementBuiltinTrait, &node.pos, self.info);                    
                }
                let idx = self.namespaces.structs.get(structnm).unwrap().0.names.as_ref().unwrap().iter().position(|x| x==var.0).unwrap();
                if self.namespaces.structs.get(structnm).unwrap().0.types.get(idx).unwrap() != &Self::get_llvm_from_type(self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, var.1, node).0 {
                    let fmt: String = format!("Struct '{}' does not implement required member '{}' of type '{}'.", structnm, var.0, self.namespaces.structs.get(structnm).unwrap().0.types.get(idx).unwrap());
                    errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);     
                }
            }
            
            if node.data.impln.as_ref().unwrap().functions.len() != traitsig.trait_sig.as_ref().unwrap().len() {
                let fmt: String = format!("Trait '{}' expected {} functions.", traitnm.to_string(), traitsig.trait_sig.unwrap().len());
                errors::raise_error(&fmt, errors::ErrorType::ExpectedNFunctionsDefined, &node.pos, self.info);
            }

            for function in &node.data.impln.as_ref().unwrap().functions {
                if function.data.func.as_ref().unwrap().template_types.len() > 0 {
                    let fmt: String = format!("Implementation functions may not be templated.");
                    errors::raise_error(&fmt, errors::ErrorType::ImplTemplatedFunction, &node.pos, self.info);
                }

                let mut found: bool = false;
                for sig in traitsig.trait_sig.as_ref().unwrap() {
                    if function.data.func.as_ref().unwrap().name == sig.name {
                        found = true;
                        break;
                    }
                }

                if !found {
                    let fmt: String = format!("Function '{}' is not defined in trait '{}'.", function.data.func.as_ref().unwrap().name, traitnm.to_string());
                    errors::raise_error(&fmt, errors::ErrorType::FunctionNotDefinedInTrait, &node.pos, self.info);
                }
                
                let mut redundant: bool = false;
                for func in &node.data.impln.as_ref().unwrap().functions {
                    if  (function.data.func.as_ref().unwrap().name == func.data.func.as_ref().unwrap().name) &&
                        func != function {
                        redundant = true;
                        break;
                    }
                }

                if redundant {
                    let fmt: String = format!("Function '{}' is redefined in impl for trait '{}' on struct '{}'.", function.data.func.as_ref().unwrap().name, traitnm.to_string(), structnm.to_string());
                    errors::raise_error(&fmt, errors::ErrorType::FunctionRedefinedInImpl, &node.pos, self.info);
                }
            }

            let mut ptrs: Vec<inkwell::values::PointerValue<'ctx>> = Vec::new();
            
            for (sig, function) in izip![traitsig.trait_sig.as_ref().unwrap(), &node.data.impln.as_ref().unwrap().functions] {
                let nargs: usize = sig.args.args.len();

                if nargs != function.data.func.as_ref().unwrap().args.args.len() {
                    let fmt: String = format!("Trait '{}' expected function with '{}' arguments.", traitnm.to_string(), nargs);
                    errors::raise_error(&fmt, errors::ErrorType::ArgumentCountMismatch, &node.pos, self.info);
                }

                let mut template_indices: std::collections::HashMap<String, Vec<usize>> = std::collections::HashMap::new();
                let mut all_indices: Vec<usize> = Vec::new();
                let mut standard_indices: Vec<usize> = Vec::new();
                for template in &sig.template_types {
                    let mut indices: Vec<usize> = Vec::new();
                    let mut idx: usize = 0;
                    for arg in &sig.args.args {
                        if  !arg.isarr &&
                            !arg.isfn &&
                            !arg.isdyn &&
                            !self.datatypes.contains_key(arg.data.as_ref().unwrap()) &&
                            sig.template_types.contains(arg.data.as_ref().unwrap()) &&
                            arg.data.as_ref().unwrap() == template {
                            indices.push(idx);
                            all_indices.push(idx);
                        }
                        idx += 1;
                    }
                    template_indices.insert(template.to_owned(), indices);
                }

                for idx in 0..sig.args.args.len() {
                    if all_indices.contains(&idx) {
                        continue;
                    }
                    standard_indices.push(idx);
                }
                
                let _ = self.build_func(&function, Some(structnm.to_owned() + "." + function.data.func.as_ref().unwrap().name.as_str()), None, None);
                let functp: types::DataType = self.namespaces.functions.get(&(structnm.to_owned() + "." + function.data.func.as_ref().unwrap().name.as_str())).unwrap().1.to_owned();
                
                for (template, indices) in &template_indices {
                    let firsttp: types::DataType = functp.types.get(indices.get(0).unwrap().to_owned()).unwrap().to_owned();
                    if indices.len() > 1 {
                        for index in indices[1..].to_vec() {
                            if functp.types.get(index).unwrap().to_owned() != firsttp {
                                let fmt: String = format!("Expected '{}' type, got '{}' type for '{}' template type.", firsttp, functp.types.get(index).unwrap().to_owned(), template);
                                errors::raise_error(&fmt, errors::ErrorType::ImplFunctionTemplateTypeMismatch, &node.pos, self.info);
                            }
                        }
                    }
                }

                for idx in &standard_indices {
                    let tp: types::DataType = functp.types.get(idx.to_owned()).unwrap().to_owned();

                    if tp != Self::get_llvm_from_type(self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, sig.args.args.get(idx.to_owned()).as_ref().unwrap(), function).0 {
                        let fmt: String = format!("Expected '{}' type, got '{}' type.", tp, Self::get_llvm_from_type(self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, sig.args.args.get(idx.to_owned()).as_ref().unwrap(), function).0);
                        errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
                    }
                }

                if  !sig.args.rettp.first().unwrap().isarr &&
                    !sig.args.rettp.first().unwrap().isfn &&
                    !sig.args.rettp.first().unwrap().isdyn &&
                    !self.datatypes.contains_key(sig.args.rettp.first().unwrap().data.as_ref().unwrap()) &&
                    sig.template_types.contains(sig.args.rettp.first().unwrap().data.as_ref().unwrap()) {
                    let tp: types::DataType = functp.types.get(template_indices.get(sig.args.rettp.first().unwrap().data.as_ref().unwrap()).unwrap().get(0).unwrap().to_owned()).unwrap().to_owned();

                    if tp != *functp.rettp.as_ref().unwrap().to_owned() {
                        let fmt: String = format!("Expected '{}' return  type, got '{}' return type.", tp, *functp.rettp.as_ref().unwrap().to_owned());
                        errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
                    }
                }

                //Add as method
                let mut s: (types::DataType, Option<AnyTypeEnum>, std::collections::HashMap<String, i32>, ForwardDeclarationType) = self.namespaces.structs.get(structnm).unwrap().clone();
                
                s.0.methods.insert(function.data.func.as_ref().unwrap().name.clone(), types::Method {
                    tp: types::MethodType::Fn,
                    builtin: None,
                    func: Some(self.namespaces.functions.get(&(structnm.to_owned() + "." + function.data.func.as_ref().unwrap().name.as_str())).unwrap().0.as_global_value().as_pointer_value()),
                    functp: functp.clone(),
                    isinstance: true,
                });

                self.namespaces.structs.insert(structnm.to_owned(), (s.0, s.1, s.2, s.3));  

                ptrs.push(self.namespaces.functions.get(&(structnm.to_owned() + "." + function.data.func.as_ref().unwrap().name.as_str())).unwrap().0.as_global_value().as_pointer_value());
                functions.insert(function.data.func.as_ref().unwrap().name.to_owned(), self.namespaces.functions.get(&(structnm.to_owned() + "." + function.data.func.as_ref().unwrap().name.as_str())).unwrap().0.as_global_value().as_pointer_value());                  
            }

            traitsig.implementations.insert(structnm.to_owned(), functions);
            
            self.traits.insert(traitsig.name.to_owned(), traitsig);   

            let idx: i32 = self.namespaces.structid.get(structnm).unwrap().clone();

            self.append_struct_to_vtables(ptrs, idx);
        }

        let data: types::Data = types::Data {
            data: None,
            tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
            owned: true,
        };
        return data;
    }

    fn build_namespaceload(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let attr: &String = &node.data.attr.as_ref().unwrap().attr;

        //Check for enums
        if  self.datatypes.get(&node.data.attr.as_ref().unwrap().name.data.identifier.as_ref().unwrap().name).is_some() &&
            self.datatypes.get(&node.data.attr.as_ref().unwrap().name.data.identifier.as_ref().unwrap().name).unwrap().tp == types::BasicDataType::Enum{
            let mut tp: types::DataType = self.datatypes.get(&node.data.attr.as_ref().unwrap().name.data.identifier.as_ref().unwrap().name).unwrap().clone();
            let name: String = node.data.attr.as_ref().unwrap().attr.clone();
            
            if !tp.names.as_ref().unwrap().contains(&name) {
                let fmt: String = format!("Type '{}' has no namespace attribute '{}'.", node.data.attr.as_ref().unwrap().name.data.identifier.as_ref().unwrap().name, attr);
                errors::raise_error(&fmt, errors::ErrorType::NamespaceAttrNotFound, &node.pos, self.info);
            }
            
            let idx: usize = tp.names.as_ref().unwrap().iter().position(|x| x == &name).unwrap() as usize;
            let enum_tp: types::DataType = tp.types.get(idx).unwrap().clone();
            tp.enum_tp = Some(Box::new(enum_tp));
            tp.enum_num = Some(Box::new(self.inkwell_types.i32tp.const_int(idx as u64, false)));

            let data: Option<inkwell::values::BasicValueEnum>;
            if node.data.attr.as_ref().unwrap().expr.is_none() {
                data = Some(inkwell::values::BasicValueEnum::IntValue(self.inkwell_types.i32tp.const_int(idx as u64, false)));
            }
            else {
                let dat: types::Data = self.compile_expr(&node.data.attr.as_ref().unwrap().expr.as_ref().unwrap(), true, false);
                if dat.tp != *tp.enum_tp.as_ref().unwrap().clone() {
                    let fmt: String = format!("Expected '{}' type, got '{}' type.", *tp.enum_tp.unwrap().clone(), dat.tp);
                    errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
                }
                data = dat.data;
            }

            return types::Data {
                data,
                tp: tp.clone(),
                owned: true
            };
        }

        if self.namespaces.structs.get(&node.data.attr.as_ref().unwrap().name.data.identifier.as_ref().unwrap().name).is_none() {
            let fmt: String = format!("Struct '{}' is not defined.", &node.data.attr.as_ref().unwrap().name.data.identifier.as_ref().unwrap().name);
            errors::raise_error(&fmt, errors::ErrorType::StructNotDefined, &node.pos, self.info);
        }

        let st = self.namespaces.structs.get(&node.data.attr.as_ref().unwrap().name.data.identifier.as_ref().unwrap().name).unwrap().clone();

        //First check methods
        let method_: Option<&types::Method> = st.0.methods.get(attr);
        if method_.is_some() {
            let method: &types::Method = method_.unwrap();
            if !method.isinstance {
                if method.tp == types::MethodType::Fn {
                    let data: types::Data = types::Data {
                        data: Some(inkwell::values::BasicValueEnum::PointerValue(method.func.unwrap())),
                        tp: method.functp.clone(),
                        owned: true,
                    };

                    return data;
                }
                else {
                    let mut tp_: types::DataType = self.datatypes.get(&types::BasicDataType::WrapperFunc.to_string()).unwrap().clone();
                    tp_.wrapperfn = method.builtin;
                    let data: types::Data = types::Data {
                        data: None,
                        tp: tp_,
                        owned: true,
                    };

                    return data;
                }
            }
        }

        //Last case
        let fmt: String = format!("Type '{}' has no namespace attribute '{}'.", node.data.attr.as_ref().unwrap().name.data.identifier.as_ref().unwrap().name, attr);
        errors::raise_error(&fmt, errors::ErrorType::NamespaceAttrNotFound, &node.pos, self.info);
    }

    fn build_if(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let end_block: inkwell::basic_block::BasicBlock = self.context.append_basic_block(self.enclosing_block.unwrap().get_parent().unwrap(), "if_end");
        let else_block: inkwell::basic_block::BasicBlock = self.context.append_basic_block(self.enclosing_block.unwrap().get_parent().unwrap(), "else");

        let mut enclosing_block: inkwell::basic_block::BasicBlock = self.enclosing_block.unwrap();

        
        let mut collected_locals: Vec<std::collections::HashMap<String, usize>> = Vec::new();

        let mut idx: usize = 0;
        for ifn in &node.data.ifn.as_ref().unwrap().ifs {
            self.builder.position_at_end(enclosing_block);            
            let right: types::Data = self.compile_expr(&ifn.0, false, false);
            
            let mut args: Vec<types::Data> = Vec::new();

            let tp: types::Type = Self::get_type_from_data(self.types.clone(), &right);

            let tp_str: &String = &right.tp.name.clone();

            args.push(right);

            let traittp: types::TraitType = types::TraitType::Bool;

            let t: &types::Trait = match tp.traits.get(&traittp.to_string()) {
                Some (v) => {
                    v
                }
                None => {
                    let fmt: String = format!("Type '{}' has no trait '{}'.", tp_str, &traittp.to_string());
                    errors::raise_error(&fmt, errors::ErrorType::MissingTrait, &node.pos, self.info);
                }
            };

            let data: types::Data = self.call_trait(t, args, node);

            if data.tp != self.datatypes.get(&types::BasicDataType::Bool.to_string()).unwrap().clone() {
                let fmt: String = format!("Expected 'bool' type, got '{}' type.", data.tp);
                errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
            }
            
            let then_block: inkwell::basic_block::BasicBlock = self.context.append_basic_block(self.enclosing_block.unwrap().get_parent().unwrap(), "if");

            let _ = then_block.move_after(self.enclosing_block.unwrap());
            
            let elseif_block: inkwell::basic_block::BasicBlock;

            if idx!=node.data.ifn.as_ref().unwrap().ifs.len()-1 {
                elseif_block = self.context.append_basic_block(self.enclosing_block.unwrap().get_parent().unwrap(), "else_if");

                let _ = elseif_block.move_after(then_block);
                
                enclosing_block = elseif_block;                
            }
            else {
                elseif_block = else_block;
            }

            self.builder.build_conditional_branch(data.data.unwrap().into_int_value(), then_block, elseif_block);

            self.builder.position_at_end(then_block);
            
            self.namespaces.locals.push(std::collections::HashMap::new());


            let mut start_locals: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
            let mut end_locals: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
            
            let mut lvl: usize = 0;
            for local in &self.namespaces.locals {
                for item in local {
                    if item.1.5 == InitializationStatus::Uninitialized {
                        start_locals.insert(item.0.clone(), lvl);
                    }
                }
                lvl += 1;
            }
            
            let loop_flow_broken_old = self.loop_flow_broken;

            self.compile(&ifn.1, true, false);

            self.loop_flow_broken = loop_flow_broken_old;

            let mut lvl: usize = 0;
            for local in &self.namespaces.locals {
                for item in local {
                    if  item.1.5 == InitializationStatus::Initialized && start_locals.get(item.0).is_some() &&
                        start_locals.get(item.0).unwrap() == &lvl {
                        
                        end_locals.insert(item.0.clone(), lvl);

                    }
                }
                lvl += 1;
            }

            for var in &end_locals {
                let mut var_val = self.namespaces.locals.get_mut(var.1.to_owned()).unwrap().get(&var.0.to_owned()).unwrap().to_owned();
                var_val.5 = InitializationStatus::Uninitialized;

                self.namespaces.locals.get_mut(var.1.to_owned()).unwrap().insert(var.0.to_owned(), var_val);
            }

            collected_locals.push(end_locals);


            self.namespaces.locals.pop();

            self.builder.build_unconditional_branch(end_block);

            self.enclosing_block = Some(then_block);

            idx+=1;
        }
        
        
        let _ = else_block.move_after(self.enclosing_block.unwrap());
        let _ = end_block.move_after(else_block); 

        if node.data.ifn.as_ref().unwrap().else_opt.is_some() {
            self.builder.position_at_end(else_block);
            
            self.namespaces.locals.push(std::collections::HashMap::new());


            let mut start_locals: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
            let mut end_locals: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
            
            let mut lvl: usize = 0;
            for local in &self.namespaces.locals {
                for item in local {
                    if item.1.5 == InitializationStatus::Uninitialized {
                        start_locals.insert(item.0.clone(), lvl);
                    }
                }
                lvl += 1;
            }

            let loop_flow_broken_old = self.loop_flow_broken;

            self.compile(&node.data.ifn.as_ref().unwrap().else_opt.as_ref().unwrap(), true, false);

            self.loop_flow_broken = loop_flow_broken_old;

            let mut lvl: usize = 0;
            for local in &self.namespaces.locals {
                for item in local {
                    if  item.1.5 == InitializationStatus::Initialized && start_locals.get(item.0).is_some() &&
                        start_locals.get(item.0).unwrap() == &lvl {
                        
                        end_locals.insert(item.0.clone(), lvl);
                    }
                }
                lvl += 1;
            }

            for var in &end_locals {
                let mut var_val = self.namespaces.locals.get_mut(var.1.to_owned()).unwrap().get(&var.0.to_owned()).unwrap().to_owned();
                var_val.5 = InitializationStatus::Uninitialized;

                self.namespaces.locals.get_mut(var.1.to_owned()).unwrap().insert(var.0.to_owned(), var_val);
            }

            collected_locals.push(end_locals);


            self.builder.build_unconditional_branch(self.enclosing_block.unwrap());

            self.namespaces.locals.pop();
        }
        else {
            self.builder.position_at_end(else_block);
            self.builder.build_unconditional_branch(end_block);
        }

        //Only if there is an else clause to catch all other cases
        if node.data.ifn.as_ref().unwrap().else_opt.is_some() {
            let mut common: Vec<(String, usize)> = Vec::new();
            let mut common_init: Vec<(String, usize)> = Vec::new();

            //Get all of the initialized variables
            for local_set in &collected_locals {
                for item in local_set {
                    if !common.contains(&(item.0.to_owned(), item.1.to_owned())) {
                        common.push((item.0.to_owned(), item.1.to_owned()));
                    }
                }
            }

            //Get all of the initialized variables that all have been commonly init
            'outer: for var in &common {
                for local_set in &collected_locals {
                    if local_set.len() == 0 {
                        continue 'outer;
                    }
                    for local in local_set {
                        if local != (&var.0, &var.1) {
                            continue 'outer;
                        }
                    }
                }
                common_init.push((var.0.to_owned(), var.1.to_owned()));
            }

            for var in common_init {
                let mut var_val = self.namespaces.locals.get_mut(var.1).unwrap().get(&var.0).unwrap().to_owned();
                var_val.5 = InitializationStatus::Initialized;

                self.namespaces.locals.get_mut(var.1).unwrap().insert(var.0, var_val);
            }
        }




        self.enclosing_block = Some(end_block);

        self.builder.position_at_end(end_block);
        
        let data: types::Data = types::Data {
            data: None,
            tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
            owned: true,
        };
        return data;
    }

    fn build_loop(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let loop_block: inkwell::basic_block::BasicBlock = self.context.append_basic_block(self.enclosing_block.unwrap().get_parent().unwrap(), "loop");
        let end_block: inkwell::basic_block::BasicBlock = self.context.append_basic_block(self.enclosing_block.unwrap().get_parent().unwrap(), "loop_end");

        let start_block_old = self.start_block;
        let end_block_old = self.end_block;
        let loop_flow_broken_old = self.loop_flow_broken;

        self.start_block = Some(loop_block);
        self.end_block = Some(end_block);

        self.builder.build_unconditional_branch(loop_block);

        self.builder.position_at_end(loop_block);

        self.compile(&node.data.loopn.as_ref().unwrap().block, true, true);

        self.builder.build_unconditional_branch(loop_block);

        self.builder.position_at_end(end_block);

        self.end_block = end_block_old;
        self.start_block = start_block_old;
        self.loop_flow_broken = loop_flow_broken_old;
        
        let data: types::Data = types::Data {
            data: None,
            tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
            owned: true,
        };
        return data;
    }

    fn build_break(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        if self.end_block.is_none() {
            let fmt: String = format!("Cannot break outside of loop.");
            errors::raise_error(&fmt, errors::ErrorType::BreakOutsideOfLoop, &node.pos, self.info);         
        }

        if self.loop_flow_broken {
            self.builder.build_unconditional_branch(self.end_block.unwrap());
            self.loop_flow_broken = true;
        }
        
        let data: types::Data = types::Data {
            data: None,
            tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
            owned: true,
        };
        return data;
    }

    fn build_continue(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        if self.start_block.is_none() {
            let fmt: String = format!("Cannot continue outside of loop.");
            errors::raise_error(&fmt, errors::ErrorType::ContinueOutsideOfLoop, &node.pos, self.info);         
        }

        if self.loop_flow_broken {
            self.builder.build_unconditional_branch(self.start_block.unwrap());
            self.loop_flow_broken = true;
        }
        
        let data: types::Data = types::Data {
            data: None,
            tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
            owned: true,
        };
        return data;
    }

    fn build_while(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        let loop_block: inkwell::basic_block::BasicBlock = self.context.append_basic_block(self.enclosing_block.unwrap().get_parent().unwrap(), "loop_head");
        let loop_then_block: inkwell::basic_block::BasicBlock = self.context.append_basic_block(self.enclosing_block.unwrap().get_parent().unwrap(), "loop_then");
        let end_block: inkwell::basic_block::BasicBlock = self.context.append_basic_block(self.enclosing_block.unwrap().get_parent().unwrap(), "loop_end");

        let start_block_old = self.start_block;
        let end_block_old = self.end_block;
        let loop_flow_broken_old = self.loop_flow_broken;

        self.start_block = Some(loop_block);
        self.end_block = Some(end_block);

        self.builder.build_unconditional_branch(loop_block);

        self.builder.position_at_end(loop_block);            

        let right: types::Data = self.compile_expr(&node.data.loopn.as_ref().unwrap().expr.as_ref().unwrap(), false, false);
      
        
        let mut args: Vec<types::Data> = Vec::new();

        let tp: types::Type = Self::get_type_from_data(self.types.clone(), &right);

        let tp_str: &String = &right.tp.name.clone();

        args.push(right);

        let traittp: types::TraitType = types::TraitType::Bool;

        let t: &types::Trait = match tp.traits.get(&traittp.to_string()) {
            Some (v) => {
                v
            }
            None => {
                let fmt: String = format!("Type '{}' has no trait '{}'.", tp_str, &traittp.to_string());
                errors::raise_error(&fmt, errors::ErrorType::MissingTrait, &node.pos, self.info);
            }
        };

        let data: types::Data = self.call_trait(t, args, node);

        if data.tp != self.datatypes.get(&types::BasicDataType::Bool.to_string()).unwrap().clone() {
            let fmt: String = format!("Expected 'bool' type, got '{}' type.", data.tp);
            errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
        }

        self.builder.build_conditional_branch(data.data.unwrap().into_int_value(), loop_then_block, end_block);

        self.builder.position_at_end(loop_then_block);

        self.compile(&node.data.loopn.as_ref().unwrap().block, true, true);

        self.builder.build_unconditional_branch(loop_block);

        self.builder.position_at_end(end_block);

        self.end_block = end_block_old;
        self.start_block = start_block_old;
        self.loop_flow_broken = loop_flow_broken_old;
        
        let data: types::Data = types::Data {
            data: None,
            tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
            owned: true,
        };
        return data;
    }

    fn build_enum(&mut self, node: &parser::Node) -> types::Data<'ctx> {        
        if !node.data.enumn.as_ref().unwrap().name.is_camel_case() {
            errors::show_warning(errors::WarningType::ExpectedCamelCase, vec![String::from(""), node.data.st.as_ref().unwrap().name.to_camel_case()], vec![String::from("Expected camel case"), String::from("Convert to this: ")], &node.pos, self.info)
        }

        if self.datatypes.get(&node.data.enumn.as_ref().unwrap().name.clone()).is_some() && self.namespaces.structs.get(&node.data.enumn.as_ref().unwrap().name.clone()).unwrap().3 != ForwardDeclarationType::Forward {
            let fmt: String = format!("Type '{}' is already defined.", node.data.enumn.as_ref().unwrap().name.clone());
            errors::raise_error(&fmt, errors::ErrorType::TypeRedefinitionAttempt, &node.pos, self.info);
        }

        let mut names: Vec<String> = Vec::new();
        
        for member in &node.data.enumn.as_ref().unwrap().variants {
            if !member.is_camel_case() {
                errors::show_warning(errors::WarningType::ExpectedCamelCase, vec![String::from(""), member.to_camel_case()], vec![String::from("Expected camel case"), String::from("Convert to this: ")], &node.pos, self.info)
            }
            if names.contains(&member.clone()) {
                let fmt: String = format!("Variant '{}' is already declared.", member.clone());
                errors::raise_error(&fmt, errors::ErrorType::VariantRedeclaration, &node.pos, self.info);
            }
            names.push(member.clone());
        }

        let mut types: Vec<types::DataType> = Vec::new();
        
        for tp in &node.data.enumn.as_ref().unwrap().tps {
            if tp.is_some() {
                types.push(Self::get_llvm_from_type(self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, tp.as_ref().unwrap(), node).0);
            }
            else {
                types.push(self.datatypes.get(&types::BasicDataType::I32.to_string()).unwrap().clone());
            }
        }

        let mut tp: types::DataType = self.datatypes.get(&types::BasicDataType::Enum.to_string()).unwrap().clone();
        tp.name = node.data.enumn.as_ref().unwrap().name.clone();
        tp.names = Some(names);
        tp.types = types;

        self.datatypes.insert(node.data.enumn.as_ref().unwrap().name.clone(), tp.clone());
        builtin_types::add_simple_type(self, std::collections::HashMap::new(), types::BasicDataType::Enum, &node.data.enumn.as_ref().unwrap().name.clone());

        let data: types::Data = types::Data {
            data: None,
            tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
            owned: true,
        };
        return data;
    }

    fn build_trait(&mut self, node: &parser::Node) -> types::Data<'ctx> {
        self.traits.insert(node.data.traitn.as_ref().unwrap().traitname.clone(), types::TraitSignature {
                nargs: None, trait_sig: Some(node.data.traitn.as_ref().unwrap().functions.clone()), name: node.data.traitn.as_ref().unwrap().traitname.clone(), traittp: types::TraitMetatype::User,
                vars: Some(node.data.traitn.as_ref().unwrap().vars.clone()),
                implementations: std::collections::HashMap::new(),
            });

        let data: types::Data = types::Data {
            data: None,
            tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
            owned: true,
        };
        return data;
    }

    fn compile_expr(&mut self, node: &parser::Node, give_ownership: bool, get_ptr: bool) -> types::Data<'ctx> {
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
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: self.datatypes.get(&types::BasicDataType::I32.to_string()).unwrap().clone(), owned: true}
            }
            parser::NodeType::BINARY => {
                self.build_binary(node)
            }
            parser::NodeType::LET => {
                self.build_let(node)
            }
            parser::NodeType::IDENTIFIER => {
                self.build_loadname(node, give_ownership, get_ptr)
            }
            parser::NodeType::FUNC => {
                self.build_func(node, None, None, None)
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
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: self.datatypes.get(&types::BasicDataType::U32.to_string()).unwrap().clone(), owned: true}
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
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: self.datatypes.get(&types::BasicDataType::I8.to_string()).unwrap().clone(), owned: true}
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
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: self.datatypes.get(&types::BasicDataType::U8.to_string()).unwrap().clone(), owned: true}
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
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: self.datatypes.get(&types::BasicDataType::I16.to_string()).unwrap().clone(), owned: true}
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
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: self.datatypes.get(&types::BasicDataType::U16.to_string()).unwrap().clone(), owned: true}
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
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: self.datatypes.get(&types::BasicDataType::I64.to_string()).unwrap().clone(), owned: true}
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
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: self.datatypes.get(&types::BasicDataType::U64.to_string()).unwrap().clone(), owned: true}
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
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: self.datatypes.get(&types::BasicDataType::I128.to_string()).unwrap().clone(), owned: true}
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
                types::Data {data: Some(inkwell::values::BasicValueEnum::IntValue(selfv)), tp: self.datatypes.get(&types::BasicDataType::U128.to_string()).unwrap().clone(), owned: true}
            }
            parser::NodeType::AS => {
                self.build_as(node)
            }
            parser::NodeType::F32 => {
                let self_data: &String = &node.data.num.as_ref().unwrap().left;
                builtin_types::f32type::check_overflow_literal(self, self_data, &node.pos);
                let selfv: inkwell::values::FloatValue = self.inkwell_types.f32tp.const_float_from_string(self_data.as_str());
                types::Data {data: Some(inkwell::values::BasicValueEnum::FloatValue(selfv)), tp: self.datatypes.get(&types::BasicDataType::F32.to_string()).unwrap().clone(), owned: true}
            }
            parser::NodeType::F64 => {
                let self_data: &String = &node.data.num.as_ref().unwrap().left;
                builtin_types::f64type::check_overflow_literal(self, self_data, &node.pos);
                let selfv: inkwell::values::FloatValue = self.inkwell_types.f64tp.const_float_from_string(self_data.as_str());
                types::Data {data: Some(inkwell::values::BasicValueEnum::FloatValue(selfv)), tp: self.datatypes.get(&types::BasicDataType::F64.to_string()).unwrap().clone(), owned: true}
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
            parser::NodeType::INITSTRUCT => {
                self.build_initstruct(node)
            }
            parser::NodeType::ATTR => {
                self.build_attrload(node, get_ptr)
            }
            parser::NodeType::ATTRASSIGN => {
                self.build_attrasssign(node)
            }
            parser::NodeType::STRING => {
                self.build_string(node)
            }
            parser::NodeType::CHAR => {
                self.build_char(node)
            }
            parser::NodeType::ARRAY => {
                self.build_array(node)
            }
            parser::NodeType::IMPL => {
                types::Data {
                    data: None,
                    tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
                    owned: true,
                }
            }
            parser::NodeType::NAMESPACE => {
                self.build_namespaceload(node)
            }
            parser::NodeType::IF => {
                self.build_if(node)
            }
            parser::NodeType::LOOP => {
                self.build_loop(node)
            }
            parser::NodeType::BREAK => {
                self.build_break(node)
            }
            parser::NodeType::CONTINUE => {
                self.build_continue(node)
            }
            parser::NodeType::WHILE => {
                self.build_while(node)
            }
            parser::NodeType::ENUM |
            parser::NodeType::TRAIT |
            parser::NodeType::VOID => {
                types::Data {
                    data: None,
                    tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
                    owned: true,
                }
            }
        }
    }

    fn compile(&mut self, nodes: &Vec<parser::Node>, infn: bool, toplvl: bool) -> types::Data<'ctx>{
        let mut retv: types::Data = types::Data {
            data: None,
            tp: self.datatypes.get(&types::BasicDataType::Void.to_string()).unwrap().clone(),
            owned: true
        };
        let mut idx: usize = 0;
        for node in nodes {
            if infn && node.tp == parser::NodeType::FUNC {
                let fmt: String = format!("Cannot define nested functions.");
                errors::raise_error(&fmt, errors::ErrorType::NestedFunctions, &node.pos, self.info);
            }

            if  !infn && node.tp != parser::NodeType::FUNC &&
                node.tp != parser::NodeType::STRUCT &&
                node.tp != parser::NodeType::IMPL &&
                node.tp != parser::NodeType::ENUM &&
                node.tp != parser::NodeType::TRAIT {
                let fmt: String = format!("Invalid global scope statement.");
                errors::raise_error(&fmt, errors::ErrorType::GlobalScopeStmt, &node.pos, self.info);
            }

            if  infn && (node.tp == parser::NodeType::FUNC ||
                node.tp == parser::NodeType::STRUCT ||
                node.tp == parser::NodeType::IMPL ||
                node.tp == parser::NodeType::ENUM ||
                node.tp == parser::NodeType::TRAIT) {
                let fmt: String = format!("Invalid local scope statement.");
                errors::raise_error(&fmt, errors::ErrorType::LocalScopeStmt, &node.pos, self.info);
            }

            retv = self.compile_expr(node, false, false);

            //Handle expressions that modify control flow
            if idx != nodes.len()-1 && toplvl {
                if  node.tp == parser::NodeType::CONTINUE ||
                    node.tp == parser::NodeType::BREAK ||
                    node.tp == parser::NodeType::RETURN {
                    errors::show_warning(errors::WarningType::UnreachableCode, vec![String::from("")], vec![String::from("Any code following this expression in this block is unreachable.")], &node.pos, self.info);    
                    break;
                }
            }

            idx += 1;
        }
        return retv;
    }

    fn forward_declare(&mut self, nodes: &Vec<parser::Node>){
        for node in nodes {
            if node.tp == parser::NodeType::FUNC {
                if  (node.data.func.as_ref().unwrap().methodname.is_some() ||
                    node.data.func.as_ref().unwrap().namespacename.is_some()) &&
                    node.data.func.as_ref().unwrap().template_types.len() == 0 {
                    continue;
                }

                let name: &String = &node.data.func.as_ref().unwrap().name;

                if !name.is_snake_case() {
                    errors::show_warning(errors::WarningType::ExpectedSnakeCase, vec![String::from(""), name.to_snake_case()], vec![String::from("Expected snake case"), String::from("Convert to this: ")], &node.pos, self.info)
                }

                if self.get_function(&name) != None {
                    let fmt: String = format!("Function '{}' is already defined.", name);
                    errors::raise_error(&fmt, errors::ErrorType::RedefinitionAttempt, &node.pos, self.info);
                }

                if node.data.func.as_ref().unwrap().template_types.len() > 0 {
                    let mut name: String = node.data.func.as_ref().unwrap().name.clone();

                    let mut instance: TemplateFunctionInstance = TemplateFunctionInstance::Unrelated;
                
                    if node.data.func.as_ref().unwrap().methodname.is_some() {
                        name += (String::from(".")+node.data.func.as_ref().unwrap().methodname.as_ref().unwrap().as_str()).as_str();
                        instance = TemplateFunctionInstance::Instance;
                    }
                    if node.data.func.as_ref().unwrap().namespacename.is_some() {
                        name += (String::from(".")+node.data.func.as_ref().unwrap().namespacename.as_ref().unwrap().as_str()).as_str();
                        instance = TemplateFunctionInstance::Namespace;
                    }
                    
                    self.namespaces.template_functions_sig.insert(name.to_owned(), (node.clone(), instance));
                    continue;
                }

                // Argument and return types
                let args = &node.data.func.as_ref().unwrap().args;

                let mut datatypes: Vec<types::DataType> = Vec::new();
                let mut mutability: Vec<types::DataMutablility> = Vec::new();
                let mut inktypes: Vec<inkwell::types::BasicMetadataTypeEnum> = Vec::new();

                for arg in &args.args {
                    let (data, tp) = Self::get_llvm_from_type(&self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, &arg, node);
                    datatypes.push(data);
                    mutability.push(arg.mutability);


                    let res: Option<inkwell::types::BasicMetadataTypeEnum> = Self::get_basicmeta_from_any(tp);

                    if res.is_some() {
                        inktypes.push(res.unwrap());
                    }
                }
                
                let rettp_full: (types::DataType, inkwell::types::AnyTypeEnum) = Self::get_llvm_from_type(&self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, &args.rettp.last().unwrap(), node);

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
                    fn_type = tp.into_function_type().ptr_type(inkwell::AddressSpace::from(0u16)).fn_type(&inktypes[..], false);
                }
                else if tp.is_void_type() {
                    fn_type = tp.into_void_type().fn_type(&inktypes[..], false);
                }
                else if tp.is_struct_type() {
                    fn_type = tp.into_struct_type().fn_type(&inktypes[..], false);
                }
                else if tp.is_array_type() {
                    fn_type = tp.into_array_type().fn_type(&inktypes[..], false);
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
                        let fmt: String = format!("Expected 'void' return type, got '{}'.", &rettp_full.0);
                        errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, &node.pos, self.info);
                    }
                }
                //

                let func: inkwell::values::FunctionValue = self.module.add_function(mangled_name.as_str(), fn_type, None);

                let mut tp: types::DataType = self.datatypes.get(&types::BasicDataType::Func.to_string()).unwrap().clone();
                tp.names = Some(node.data.func.as_ref().unwrap().args.name.clone());
                tp.types = datatypes.clone();
                tp.mutability =mutability.clone();
                tp.rettp =  Some(Box::new(rettp_full.0.clone()));

                self.namespaces.functions.insert(name.clone(), (func, tp, ForwardDeclarationType::Forward));
            }
            else if node.tp == parser::NodeType::STRUCT {
                if !node.data.st.as_ref().unwrap().name.is_camel_case() {
                    errors::show_warning(errors::WarningType::ExpectedCamelCase, vec![String::from(""), node.data.st.as_ref().unwrap().name.to_camel_case()], vec![String::from("Expected camel case"), String::from("Convert to this: ")], &node.pos, self.info)
                }

                self.namespaces.structid_max += 1;
                self.namespaces.structid.insert(node.data.st.as_ref().unwrap().name.clone(), self.namespaces.structid_max);
                self.namespaces.structid_from.insert(self.namespaces.structid_max, node.data.st.as_ref().unwrap().name.clone());
                    
                let mut names: Vec<String> = Vec::new();
                let mut types: Vec<(types::DataType, AnyTypeEnum)> = Vec::new();
                let mut simpletypes: Vec<types::DataType> = Vec::new();
                let mut mutabilitites: Vec<types::DataMutablility> = Vec::new();
                let mut idxmapping: std::collections::HashMap<String, i32> = std::collections::HashMap::new();

                let mut idx = 0;
                for member in &node.data.st.as_ref().unwrap().members {
                    if !member.0.is_camel_case() {
                        errors::show_warning(errors::WarningType::ExpectedSnakeCase, vec![String::from(""), member.0.to_camel_case()], vec![String::from("Expected snake case"), String::from("Convert to this: ")], &node.pos, self.info)
                    }
                    if names.contains(&member.0.clone()) {
                        let fmt: String = format!("Field '{}' is already declared.", member.0.clone());
                        errors::raise_error(&fmt, errors::ErrorType::FieldRedeclaration, &node.pos, self.info);
                    }
                    names.push(member.0.clone());
                    types.push(Self::get_llvm_from_type(self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, member.1, node));
                    simpletypes.push(Self::get_llvm_from_type(self.context, &self.namespaces.structs, &self.inkwell_types, &self.datatypes, &self.traits, self.info, member.1, node).0);
                    mutabilitites.push(types::DataMutablility::Mutable);
                    idxmapping.insert(member.0.clone(), idx);
                    idx+=1;
                }

                let mut tp: types::DataType = self.datatypes.get(&types::BasicDataType::Struct.to_string()).unwrap().clone();
                tp.name = node.data.st.as_ref().unwrap().name.clone();
                tp.names = Some(names);
                tp.types = simpletypes.clone();
                tp.mutability = mutabilitites;
                
                self.datatypes.insert(node.data.st.as_ref().unwrap().name.clone(), tp.clone());
                self.namespaces.structs.insert(node.data.st.as_ref().unwrap().name.clone(), (tp, Some(Self::build_struct_tp_from_types(self.context, &self.inkwell_types, &simpletypes)), idxmapping, ForwardDeclarationType::Forward));
                builtin_types::add_simple_type(self, std::collections::HashMap::new(), types::BasicDataType::Struct, &node.data.st.as_ref().unwrap().name.clone());
            }
            else if node.tp == parser::NodeType::ENUM {
                self.build_enum(node);    
            }
            else if node.tp == parser::NodeType::IMPL {
                self.build_impl(node);
            }
            else if node.tp == parser::NodeType::TRAIT {
                self.build_trait(node);
            }
        }
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

    let dynptrtp: inkwell::types::StructType = context.struct_type(&[inkwell::types::BasicTypeEnum::IntType(context.i32_type()), inkwell::types::BasicTypeEnum::PointerType(context.opaque_struct_type("st_data").ptr_type(inkwell::AddressSpace::from(0u16)))], false);
    
    let inkwelltypes = InkwellTypes {
        i8tp: &context.i8_type(),
        i16tp: &context.i16_type(),
        i32tp: &context.i32_type(),
        i64tp: &context.i64_type(),
        i128tp: &context.i128_type(),
        f32tp: &context.f32_type(),
        f64tp: &context.f64_type(),
        voidtp: &context.void_type(),
        booltp: &context.bool_type(),
        dynptrtp: &dynptrtp,
    };

    let namespaces: Namespaces = Namespaces {
        locals: Vec::new(),
        functions: std::collections::HashMap::new(),
        structs: std::collections::HashMap::new(),
        template_functions_sig: std::collections::HashMap::new(),
        template_functions: Vec::new(),
        structid: std::collections::HashMap::new(),
        structid_from: std::collections::HashMap::new(),
        structid_max: -1,
    };

    
    //Setup debug info
    module.add_basic_value_flag("Debug Info Version", inkwell::module::FlagBehavior::Error, inkwelltypes.i32tp.const_int(3, false));
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
        datatypes: std::collections::HashMap::new(),
        info,
        inkwell_types: inkwelltypes,
        namespaces: namespaces,
        dibuilder: dibuilder,
        dicompile_unit: compile_unit,
        expected_rettp: None, 
        traits: std::collections::HashMap::new(),
        enclosing_block: None,
        start_block: None,
        end_block: None,
        loop_flow_broken: false,
        vtables: None,
        vtables_vec: Vec::new(),
    };
    
    //Pass manager (optimizer)
    let pass_manager_builder: inkwell::passes::PassManagerBuilder = inkwell::passes::PassManagerBuilder::create();
    pass_manager_builder.set_optimization_level(inkwell::OptimizationLevel::Aggressive);
    let manager: inkwell::passes::PassManager<Module> = inkwell::passes::PassManager::create(());
    pass_manager_builder.populate_module_pass_manager(&manager);

    //Setup builtin types and structs
    builtin_types::init(&mut codegen);
    builtin_types::init_traits(&mut codegen);
    builtin_types::init_structs(&mut codegen);

    //Generate forward-declaration functions
    codegen.forward_declare(&nodes);

    //Compile code
    codegen.compile(&nodes, false, false);

    //Make the real main function
    if codegen.get_function(&String::from("main")) == None {
        let fmt: String = format!("Function 'main' is not defined.");
        errors::raise_error_no_pos(&fmt, errors::ErrorType::NameNotFound);
    }

    let (main, _, _) = codegen.namespaces.functions.get(&String::from("main")).unwrap();

    let main_tp: inkwell::types::FunctionType = codegen.inkwell_types.i32tp.fn_type(&[inkwell::types::BasicMetadataTypeEnum::IntType(*codegen.inkwell_types.i32tp), inkwell::types::BasicMetadataTypeEnum::PointerType(codegen.inkwell_types.i8tp.ptr_type(inkwell::AddressSpace::from(0u16)).ptr_type(inkwell::AddressSpace::from(0u16)))], false);
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

    let mut res: std::process::Output = std::process::Command::new("llc").arg("a.ll").output().expect("Failed to execute llc");
    if !res.status.success() {
        println!("Stderr:\n{}\n\nStdout:{}", std::str::from_utf8(&res.stderr[..]).expect("Unable to convert for stderr (llc)"), std::str::from_utf8(&res.stdout[..]).expect("Unable to convert for stdout (llc)"));
        panic!("Failed to run llc (exit code {})", res.status.to_string());
    }

    res = std::process::Command::new("gcc").arg("a.s").arg("-oa.o").arg("-c").output().expect("Failed to execute gcc");
    if !res.status.success() {
        println!("Stderr:\n{}\n\nStdout:{}", std::str::from_utf8(&res.stderr[..]).expect("Unable to convert for stderr (gcc)"), std::str::from_utf8(&res.stdout[..]).expect("Unable to convert for stdout (gcc)"));
        panic!("Failed to run gcc (exit code {})", res.status.to_string());
    }

    res = std::process::Command::new("gcc").arg("a.s").arg("-oa.out").arg("-no-pie").output().expect("Failed to execute gcc");
    if !res.status.success() {
        println!("Stderr:\n{}\n\nStdout:{}", std::str::from_utf8(&res.stderr[..]).expect("Unable to convert for stderr (gcc)"), std::str::from_utf8(&res.stdout[..]).expect("Unable to convert for stdout (gcc)"));
        panic!("Failed to run gcc (exit code {})", res.status.to_string());
    }

    Ok(())
}