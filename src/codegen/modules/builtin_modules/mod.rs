mod std_mod;
use crate::codegen::CodeGen;

pub fn init_builtin_modules(codegen: &mut CodeGen) {
    std_mod::init_std(codegen)
}