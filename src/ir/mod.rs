//! Core compute IR structures

use crate::ir::data::{Instruction, MemorySpace, Terminator, VReg};

pub mod data;
pub mod opcodes;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Symbol(pub(crate) Box<str>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub usize);

#[derive(Debug, Clone)]
pub struct Module {
    pub name: Box<str>,
    pub funcs: Box<[Function]>,
    pub data: Box<[StaticData]>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub sym: Symbol,
    pub params: Box<[VReg]>,
    pub out: Option<VReg>,
    pub blocks: Box<[Block]>,
    pub calling_conv: CallingConv,
}

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
    pub(crate) sym: Box<str>,
    pub(crate) mem: MemorySpace,
}
