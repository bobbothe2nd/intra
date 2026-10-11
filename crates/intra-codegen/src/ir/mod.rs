//! Core compute IR structures

use crate::{error::{Error, ErrorCode, Location}, ir::data::{Instruction, MemorySpace, RegisterClass, Terminator}};

pub mod data;
pub mod opcodes;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Symbol(pub(crate) Box<str>);

impl Symbol {
    pub fn new(sym: &str) -> Self {
        Self(Box::from(sym))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuncId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstrId(pub u32);

#[derive(Debug, Clone)]
pub struct Module {
    pub name: Symbol,
    pub funcs: Box<[Function]>,
    pub data: Box<[StaticData]>,
    pub regs: Box<[RegisterClass]>,
}

impl Module {
    pub fn get_instr(&self, loc: Location) -> Result<&Instruction, Error> {
        let func = self.funcs.get(loc.func.0 as usize).ok_or_else(|| Error::new(format!("failed to find function in module, `{loc:?}"), ErrorCode::SpanUnresolved))?;
        let block = func.blocks.get(loc.block.0 as usize).ok_or_else(|| Error::new(format!("failed to find block in function, `{loc:?}"), ErrorCode::SpanUnresolved))?;
        block.instrs.get(loc.instr.0 as usize).ok_or_else(|| Error::new(format!("failed to find instruction in block, `{loc:?}"), ErrorCode::SpanUnresolved))
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub sym: Symbol,
    pub params: Box<[RegisterClass]>,
    pub out: Option<RegisterClass>,
    pub blocks: Box<[Block]>,
    pub calling_conv: CallingConv,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CallingConv {
    SysV64,
    Win64,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub instrs: Box<[Instruction]>,
    pub term: Terminator,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StaticData {
    pub sym: Symbol,
    pub mem: MemorySpace,
    pub size: u32,
    pub align: u16,
}
