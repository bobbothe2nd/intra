use crate::{arch::backend::{LowerError, x86::{instr::X86Instr, reg::X86VReg}}, ir::{Block, Function, Symbol, data::Terminator}};

pub mod instr;

mod reg;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X86Operand {
    VReg(X86VReg),
    Imm(i64),
}

pub struct X86Module {
    pub funcs: Box<[X86Function]>,
}

pub struct X86Function {
    pub sym: Symbol,
    pub blocks: Box<[X86Block]>,
}

pub struct X86Block {
    pub label: X86Label,
    pub instrs: Box<[X86Instr]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct X86Label(pub u32);

pub struct X86Lowerer {
    next_vreg: u32,
}

impl X86Lowerer {
    pub fn lower_function(
        &mut self,
        func: &Function,
    ) -> Result<X86Function, LowerError> {
        let labels: Box<[X86Label]> = func
            .blocks
            .iter()
            .enumerate()
            .map(|(i, _)| X86Label(i as u32))
            .collect();

        let mut blocks = Vec::with_capacity(func.blocks.len());

        for (block, &label) in func.blocks.iter().zip(labels.iter()) {
            blocks.push(self.lower_block(block, &labels, label)?);
        }

        Ok(X86Function {
            sym: func.sym.clone(),
            blocks: blocks.into_boxed_slice(),
        })
    }

    fn lower_term(
        &mut self,
        term: &Terminator,
        labels: &[X86Label],
        out: &mut Vec<X86Instr>,
    ) -> Result<(), LowerError> {
        match term {
            Terminator::Jmp(block) => {
                out.push(X86Instr::Jmp {
                    target: labels[block.0],
                });
            }

            Terminator::Branch {
                cond,
                then_block,
                else_block,
            } => {
                out.push(X86Instr::Test {
                    reg: self.alloc(*cond)?,
                });

                out.push(X86Instr::Jnz {
                    target: labels[then_block.0],
                });

                out.push(X86Instr::Jmp {
                    target: labels[else_block.0],
                });
            }

            Terminator::Return(value) => {
                out.push(X86Instr::Ret {
                    value: self.alloc_opt(*value)?,
                });
            }

            Terminator::Loop(block) => {
                out.push(X86Instr::Jmp {
                    target: labels[block.0],
                });
            }
        }

        Ok(())
    }

    fn lower_block(
        &mut self,
        block: &Block,
        labels: &[X86Label],
        label: X86Label,
    ) -> Result<X86Block, LowerError> {
        let mut instrs = Vec::new();

        for inst in &block.ops {
            self.lower_instr(inst, &mut instrs)?;
        }

        self.lower_term(&block.term, labels, &mut instrs)?;

        Ok(X86Block {
            label,
            instrs: instrs.into_boxed_slice(),
        })
    }
}
