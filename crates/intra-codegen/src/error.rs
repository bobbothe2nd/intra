use core::fmt::{self, Debug, Display, Formatter, Write};

use crate::ir::{BlockId, FuncId, InstrId, Symbol, data::{Constant, Instruction, Operand, RegisterClass, ScalarType}};

#[derive(Debug, Clone)]
pub struct Errors {
    errors: Vec<Error>,
}

impl Errors {
    pub const fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn accum(&mut self, err: Error) {
        self.errors.push(err);
    }

    pub fn try_accum(&mut self, res: Result<(), Error>) {
        if let Err(err) = res {
            self.errors.push(err);
        }
    }

    pub fn append(&mut self, errs: Errors) {
        self.errors.extend(errs.errors);
    }

    pub fn wrap(self) -> Result<(), Self> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }

    pub fn wrap_or<T>(self, f: impl FnOnce() -> T) -> Result<T, Self> {
        if self.errors.is_empty() {
            Ok(f())
        } else {
            Err(self)
        }
    }

    pub fn try_wrap_or<T>(self, f: impl FnOnce() -> Result<T, Self>) -> Result<T, Self> {
        if self.errors.is_empty() {
            f()
        } else {
            Err(self)
        }
    }
}

impl From<Error> for Errors {
    fn from(value: Error) -> Self {
        Self { errors: vec![value] }
    }
}

impl From<Vec<Error>> for Errors {
    fn from(errors: Vec<Error>) -> Self {
        Self { errors }
    }
}

impl Display for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for err in &self.errors {
            write!(f, "{err}\n")?;
        }

        Ok(())
    }
}

impl core::error::Error for Errors {}

#[derive(Debug, Clone)]
pub struct Error {
    msg: Box<str>,
    span: Span,
    code: u16,
}

impl Error {
    #[inline]
    pub fn new_spanned<S: Into<Box<str>>>(msg: S, span: Span, code: ErrorCode) -> Self {
        Self { msg: msg.into(), span, code: code as u16 }
    }

    #[inline]
    pub fn new<S: Into<Box<str>>>(msg: S, code: ErrorCode) -> Self {
        Self { msg: msg.into(), span: Span(None), code: code as u16 }
    }

    #[inline]
    pub const fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    #[inline]
    pub const fn with_span(mut self, span: Span) -> Self {
        self.set_span(span);
        self
    }

    #[inline]
    pub const fn strip_span(&mut self) {
        self.span = Span(None);
    }

    #[inline]
    pub const fn without_span(mut self) -> Self {
        self.strip_span();
        self
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.span.0 {
            Some(loc) => write!(f, "error[E{:0>4}]: {}\n  --> {:?}", self.code, self.msg, loc),
            None => write!(f, "error[E{:0>4}]: {}\n  --> internal error", self.code, self.msg),
        }
    }
}

impl core::error::Error for Error {}

#[derive(Debug, Clone)]
pub struct Span(Option<Location>);

impl Span {
    pub const fn new(loc: Location) -> Self {
        Self(Some(loc))
    }

    pub const fn unbound() -> Self {
        Self(None)
    }
}

#[derive(Clone)]
pub struct Location {
    pub func: FuncId,
    pub block: BlockId,
    pub instr: InstrId,
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "func{}::block{}::instr{}", self.func.0, self.block.0, self.instr.0)
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

#[repr(u16)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    UnsupportedRegister,
    UnsupportedOpcode,
    RegisterExhaustion,
    InvalidOperands,
    InvalidInstruction,
    SpanUnresolved,
    MissingOutput,
    InvalidRegister,
}
