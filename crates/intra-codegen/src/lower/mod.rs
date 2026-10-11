use crate::ir::{Function, Module};

pub trait IrLowerer {
    type Output;
    type Error;

    fn lower_module(&mut self, module: &Module) -> Result<Self::Output, Self::Error>;
    fn lower_function(&mut self, func: &Function) -> Result<Self::Output, Self::Error>;
}
