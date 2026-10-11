use core::fmt::{self, Display, Formatter};

use crate::{arch::backend::{LowerError, x86::{X86Lowerer, X86Operand}}, ir::data::{Constant, Instruction, Operand, RegisterClass, VReg}};

#[derive(Debug, Clone, Copy, Eq, Hash)]
pub enum X86RegClass {
    Gpr64,
    Gpr32,
    Gpr16,
    Mm,
    Xmm,
    Ymm,
    Zmm,
    Tmm,
}

impl PartialEq for X86RegClass {
    fn eq(&self, other: &Self) -> bool {
        let both_gpr = matches!(self, Self::Gpr16 | Self::Gpr32 | Self::Gpr64) && matches!(other, Self::Gpr16 | Self::Gpr32 | Self::Gpr64);
        both_gpr || match (self, other) {
            (Self::Mm, Self::Mm) => true,
            (Self::Xmm, Self::Xmm) => true,
            (Self::Ymm, Self::Ymm) => true,
            (Self::Zmm, Self::Zmm) => true,
            (Self::Tmm, Self::Tmm) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct X86Reg {
    pub id: u32,
    pub class: X86RegClass,
}

impl Display for X86Reg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.class {
            X86RegClass::Gpr64 => return match self.id {
                0 => f.write_str("rax"),
                1 => f.write_str("rbx"),
                2 => f.write_str("rcx"),
                3 => f.write_str("rdx"),
                4 => f.write_str("rsi"),
                5 => f.write_str("rdi"),
                6 => f.write_str("rbp"),
                7 => f.write_str("rsp"),
                8..16 => write!(f, "r{}", self.id + 4),
                _ => Err(fmt::Error),
            },
            X86RegClass::Gpr32 => return match self.id {
                0 => f.write_str("eax"),
                1 => f.write_str("ebx"),
                2 => f.write_str("ecx"),
                3 => f.write_str("edx"),
                4 => f.write_str("esi"),
                5 => f.write_str("edi"),
                6 => f.write_str("ebp"),
                7 => f.write_str("esp"),
                _ => Err(fmt::Error),
            },
            X86RegClass::Gpr16 => return match self.id {
                0 => f.write_str("ax"),
                1 => f.write_str("bx"),
                2 => f.write_str("cx"),
                3 => f.write_str("dx"),
                4 => f.write_str("si"),
                5 => f.write_str("di"),
                6 => f.write_str("bp"),
                7 => f.write_str("sp"),
                _ => Err(fmt::Error),
            },
            X86RegClass::Mm => f.write_str("mm")?,
            X86RegClass::Xmm => f.write_str("xmm")?,
            X86RegClass::Ymm => f.write_str("ymm")?,
            X86RegClass::Zmm => f.write_str("zmm")?,
            X86RegClass::Tmm => f.write_str("tmm")?,
        }

        write!(f, "{}", self.id)
    }
}

impl X86Lowerer<'_> {
    fn alloc_physical(
        &mut self,
        class: X86RegClass,
    ) -> Result<X86Reg, LowerError> {
        const ALLOCATABLE_GPRS: &[u32] = &[
            0,    2,  3,  4,  5,
            8, 9, 10, 11, 12, 13, 14, 15,
        ];

        let used = self
            .allocs
            .iter()
            .flatten()
            .filter(|reg| reg.class == class)
            .count();

        let id = match class {
            X86RegClass::Gpr64
            | X86RegClass::Gpr32
            | X86RegClass::Gpr16 => {
                *ALLOCATABLE_GPRS
                    .get(used)
                    .ok_or(LowerError::RegisterExhaustion)?
            }

            _ => {
                used as u32
            }
        };

        Ok(X86Reg { id, class })
    }

    pub fn alloc(&mut self, reg: VReg) -> Result<X86Reg, LowerError> {
        if let Some(reg) = self.allocs[reg.0] {
            return Ok(reg);
        }

        let class = self.module.regs[reg.0].try_into()?;

        let physical = self.alloc_physical(class)?;

        self.allocs[reg.0] = Some(physical);

        Ok(physical)
    }

    pub fn alloc_out(&mut self, instr: &Instruction) -> Result<X86Reg, LowerError> {
        match instr.output {
            Some(reg) => self.alloc(reg),
            None => Err(LowerError::MissingOutput),
        }
    }

    pub fn alloc_opt(&mut self, reg: Option<VReg>) -> Result<Option<X86Reg>, LowerError> {
        Ok(match reg {
            Some(reg) => Some(self.alloc(reg)?),
            None => None,
        })
    }

    pub fn alloc_operand(&mut self, operand: &Operand) -> Result<X86Operand, LowerError> {
        match operand {
            Operand::Reg(reg) => Ok(X86Operand::Reg(self.alloc(*reg)?)),

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
            RegisterClass::Scalar(scalar) => Ok(match scalar.bits() {
                64 => Self::Gpr64,
                32 => Self::Gpr32,
                16 => Self::Gpr16,
                _ => return Err(LowerError::UnsupportedRegister),
            }),
            _ => Err(LowerError::UnsupportedRegister),
        }
    }
}
