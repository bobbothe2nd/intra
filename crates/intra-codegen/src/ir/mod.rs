//! Core compute IR structures

use crate::ir::data::{Instruction, MemorySpace, Terminator, RegisterClass};

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
pub struct BlockId(pub usize);

#[derive(Debug, Clone)]
pub struct Module {
    pub name: Symbol,
    pub funcs: Box<[Function]>,
    pub data: Box<[StaticData]>,
    pub regs: Box<[RegisterClass]>,
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
    pub ops: Box<[Instruction]>,
    pub term: Terminator,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StaticData {
    pub sym: Symbol,
    pub mem: MemorySpace,
    pub size: u32,
    pub align: u16,
}
