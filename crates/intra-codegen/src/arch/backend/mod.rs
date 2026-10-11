use crate::ir::opcodes::Opcode;

pub mod x86;

#[derive(Debug)]
pub enum LowerError {
    Unreachable,
    UnsupportedOpcode(Opcode),
    UnsupportedRegister,
    RegisterExhaustion,
    InvalidOperands,
    MissingOutput,
}
