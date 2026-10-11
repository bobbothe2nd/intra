pub mod backend;

use crate::{arch::backend::x86::{X86Lowerer, X86Module}, error::Errors, ir::Module};

pub fn lower_ir_x86(module: &Module) -> Result<X86Module, Errors> {
    X86Lowerer::new(module).lower()
}
