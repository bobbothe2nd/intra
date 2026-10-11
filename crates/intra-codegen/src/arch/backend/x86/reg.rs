use core::fmt::{self, Display, Formatter};

use crate::{arch::backend::{LowerError, x86::{X86Lowerer, X86Operand}}, ir::data::{Constant, Instruction, Operand, RegisterClass, VReg}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum X86RegClass {
    Gpr,
    Xmm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct X86VReg {
    pub id: u32,
    pub class: X86RegClass,
}

impl Display for X86VReg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.class {
            X86RegClass::Gpr => return match self.id {
                0 => f.write_str("rax"),
                1 => f.write_str("rbx"),
                2 => f.write_str("rcx"),
                3 => f.write_str("rdx"),
                4..12 => write!(f, "r{}", self.id + 4),
                _ => Err(fmt::Error),
            },
            X86RegClass::Xmm => f.write_str("xmm")?,
        }

        write!(f, "{}", self.id)
    }
}

impl X86Lowerer {
    pub fn alloc(&mut self, reg: VReg) -> Result<X86VReg, LowerError> {
        let id = self.next_vreg;
        self.next_vreg += 1;

        Ok(X86VReg { id, class: reg.class.try_into()? })
    }

    pub fn alloc_out(&mut self, instr: &Instruction) -> Result<X86VReg, LowerError> {
        match instr.output {
            Some(reg) => self.alloc(reg),
            None => Err(LowerError::MissingOutput),
        }
    }

    pub fn alloc_opt(&mut self, reg: Option<VReg>) -> Result<Option<X86VReg>, LowerError> {
        Ok(match reg {
            Some(reg) => Some(self.alloc(reg)?),
            None => None,
        })
    }

    pub fn alloc_operand(&mut self, operand: &Operand) -> Result<X86Operand, LowerError> {
        match operand {
            Operand::Reg(reg) => Ok(X86Operand::VReg(self.alloc(*reg)?)),

            Operand::Imm(Constant::Int { val, .. }) => {
                Ok(X86Operand::Imm(*val))
            }

            _ => Err(LowerError::InvalidOperands),
        }
    }
}

impl TryFrom<RegisterClass> for X86RegClass {
    type Error = LowerError;

    fn try_from(value: RegisterClass) -> Result<Self, Self::Error> {
        match value {
            RegisterClass::Scalar(_) => Ok(Self::Gpr),
            _ => Err(LowerError::UnsupportedRegister),
        }
    }
}
