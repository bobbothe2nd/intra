use crate::{arch::backend::{LowerError, x86::{X86Label, X86Lowerer, X86Operand, reg::X86Reg}}, ir::{data::Instruction, opcodes::Opcode}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X86Instr {
    Mov {
        dst: X86Reg,
        src: X86Operand,
    },

    Add {
        dst: X86Reg,
        src: X86Operand,
    },

    Test {
        reg: X86Reg,
    },

    Jmp {
        target: X86Label,
    },

    Jnz {
        target: X86Label,
    },

    Ret {
        value: Option<X86Reg>,
    },
}

impl X86Lowerer<'_> {
    pub fn lower_instr(&mut self, instr: &Instruction, collect: &mut Vec<X86Instr>) -> Result<(), LowerError> {
        match instr.opcode {
            Opcode::BuiltinMove => {
                let [src] = instr.inputs.as_ref() else {
                    return Err(LowerError::InvalidOperands);
                };

                let dst = self.alloc_out(instr)?;
                let src = self.alloc_operand(src)?;

                collect.push(X86Instr::Mov { dst, src });
            }

            Opcode::CoreAddI => {
                let [lhs, rhs] = instr.inputs.as_ref() else {
                    return Err(LowerError::InvalidOperands);
                };

                let dst = self.alloc_out(instr)?;

                let lhs = self.alloc_operand(lhs)?;
                let rhs = self.alloc_operand(rhs)?;

                let X86Operand::Reg(lhs) = lhs else {
                    return Err(LowerError::InvalidOperands);
                };

                collect.extend([
                    X86Instr::Mov {
                        dst,
                        src: X86Operand::Reg(lhs),
                    },
                    X86Instr::Add {
                        dst,
                        src: rhs,
                    },
                ]);
            }

            opcode => {
                return Err(LowerError::UnsupportedOpcode(opcode));
            }
        }

        Ok(())
    }
}
