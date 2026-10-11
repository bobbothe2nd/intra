use crate::{arch::backend::x86::{instr::X86Instr, reg::X86Reg}, error::{Error, Errors, Location, Span}, ir::{Block, BlockId, FuncId, Function, InstrId, Module, StaticData, Symbol, data::Terminator}};

pub mod instr;

pub mod reg;

mod alloc;

pub struct X86Lowerer<'a> {
    module: &'a Module,
    allocs: Vec<Option<X86Reg>>,
}

impl<'a> X86Lowerer<'a> {
    pub fn new(module: &'a Module) -> Self {
        Self {
            module,
            allocs: Vec::new(),
        }
    }

    pub fn lower(
        &mut self,
    ) -> Result<X86Module, Errors> {
        let mut errors = Errors::new();

        let mut funcs = Vec::with_capacity(self.module.funcs.len());

        for (func_id, func) in self.module.funcs.iter().enumerate() {
            match self.lower_function(FuncId(func_id as u32), func) {
                Ok(val) => funcs.push(val),
                Err(errs) => errors.append(errs),
            }
        }

        errors.wrap_or(||X86Module {
            funcs: funcs.into_boxed_slice(),
            data: self.module.data.clone(),
        })
    }

    fn lower_function(
        &mut self,
        func_id: FuncId,
        func: &Function,
    ) -> Result<X86Function, Errors> {
        self.allocs.clear();
        self.allocs.resize(self.module.regs.len(), None);

        let labels: Box<[X86Label]> = func
            .blocks
            .iter()
            .enumerate()
            .map(|(i, _)| X86Label(i as u32))
            .collect();

        let mut errors = Errors::new();

        let mut blocks = Vec::with_capacity(func.blocks.len());

        for (block_id, (block, &label)) in func.blocks.iter().zip(labels.iter()).enumerate() {
            match self.lower_block(func_id, block, BlockId(block_id as u32), &labels, label) {
                Ok(val) => blocks.push(val),
                Err(errs) => errors.append(errs),
            }
        }

        errors.wrap_or(|| X86Function {
            sym: func.sym.clone(),
            blocks: blocks.into_boxed_slice(),
        })
    }

    fn lower_term(
        &mut self,
        term: &Terminator,
        labels: &[X86Label],
        out: &mut Vec<X86Instr>,
    ) -> Result<(), Error> {
        match term {
            Terminator::Jmp(block) => {
                out.push(X86Instr::Jmp {
                    target: labels[block.0 as usize],
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
                    target: labels[then_block.0 as usize],
                });

                out.push(X86Instr::Jmp {
                    target: labels[else_block.0 as usize],
                });
            }

            Terminator::Return(value) => {
                out.push(X86Instr::Ret {
                    value: self.alloc_opt(*value)?,
                });
            }

            Terminator::Loop(block) => {
                out.push(X86Instr::Jmp {
                    target: labels[block.0 as usize],
                });
            }
        }

        Ok(())
    }

    fn lower_block(
        &mut self,
        func: FuncId,
        block: &Block,
        block_id: BlockId,
        labels: &[X86Label],
        label: X86Label,
    ) -> Result<X86Block, Errors> {
        let mut instrs = Vec::new();

        let mut errors = Errors::new();

        for (id, instr) in block.instrs.iter().enumerate() {
            errors.try_accum(self.lower_instr(instr, &mut instrs).map_err(|err| err.with_span(Span::new(Location {
                func: func,
                block: block_id,
                instr: InstrId(id as u32),
            }))));
        }

        errors.try_accum(self.lower_term(&block.term, labels, &mut instrs));

        errors.wrap_or(|| X86Block {
            label,
            instrs: instrs.into_boxed_slice(),
        })
    }
}

pub struct X86Module {
    pub funcs: Box<[X86Function]>,
    pub data: Box<[StaticData]>,
}

pub struct X86Function {
    pub sym: Symbol,
    pub blocks: Box<[X86Block]>,
}

pub struct X86Block {
    pub label: X86Label,
    pub instrs: Box<[X86Instr]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X86Operand {
    Reg(X86Reg),
    Imm(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct X86Label(pub u32);
