use core::fmt::{self, Display, Formatter};

use crate::{error::Error, ir::data::RegisterClass};

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

impl TryFrom<RegisterClass> for X86RegClass {
    type Error = Error;

    fn try_from(value: RegisterClass) -> Result<Self, Self::Error> {
        match value {
            RegisterClass::Scalar(scalar) => Ok(match scalar.bits() {
                64 => Self::Gpr64,
                32 => Self::Gpr32,
                16 => Self::Gpr16,
                _ => return Err(lower_error!(@InvalidRegister "GPR" "!(16/32/64)")),
            }),
            _ => Err(lower_error!(@UnsupportedRegister "!GPR")),
        }
    }
}
