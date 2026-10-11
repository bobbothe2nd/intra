use core::fmt::{self, Debug, Display, Formatter, Write};

use crate::ir::{Symbol, data::{Constant, Instruction, Operand, RegisterClass, ScalarType, VReg}};

#[derive(Debug, Clone)]
pub struct Error {
    msg: Box<str>,
    loc: Location,
    code: u16,
}

impl Error {
    #[inline]
    pub fn new(msg: &str, loc: Location, code: u16) -> Self {
        Self { msg: msg.into(), loc, code }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "error[E{:0>4}]: {}\n  --> {:?}", self.code, self.msg, self.loc)
    }
}

impl core::error::Error for Error {}

#[derive(Debug, Clone)]
pub struct Location {
    module: Box<str>,
    id: usize,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}/instr{}", self.module, self.id)
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.opcode, f)?;

        for (i, input) in self.inputs.iter().enumerate() {
            if i != 0 {
                f.write_char(',')?;
            }

            write!(f, " {:?}", input)?;
        }

        Ok(())
    }
}

impl Debug for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Imm(imm) => Display::fmt(&imm, f),
            Self::Sym(sym) => f.write_str(&sym.0),
            Self::Reg(reg) => Debug::fmt(&reg, f),
        }
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(val) => Display::fmt(&val, f),
            Self::Float { val, .. } => Display::fmt(&val, f),
            Self::Int { val, .. } => Display::fmt(&val, f),
        }
    }
}

impl Debug for RegisterClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scalar(val) => write!(f, "r{}", val.bits()),
            Self::Matrix { elem, rows, cols } => write!(f, "tile{}{}", rows * cols * (elem.size() as u16), elem.prefix()),
            Self::Pointer { pointee_size, .. } => write!(f, "m{pointee_size}"),
            Self::Vector { elem, lanes } => write!(f, "{elem:?}x{lanes}"),
        }
    }
}

impl Debug for ScalarType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Bool => "bool",
            Self::F16 => "f16",
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
        })
    }
}

impl Debug for VReg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.class, f)
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
