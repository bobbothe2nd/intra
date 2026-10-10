use crate::ir::{BlockId, Symbol, opcodes::Opcode};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Terminator {
    Jmp(BlockId),
    Loop(BlockId),
    Branch {
        cond: VReg,
        then_block: BlockId,
        else_block: BlockId,
    },
    Return(Option<VReg>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpDescriptor {
    pub opcode: Opcode,
    pub inputs: Box<[OperandClass]>,
    pub output: Option<VReg>,
    pub effects: Effects,
    pub properties: Properties,
}

#[derive(Clone, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub inputs: Box<[Operand]>,
    pub output: Option<VReg>,
}

#[derive(Clone, PartialEq)]
pub enum Operand {
    Reg(VReg),
    Imm(Constant),
    Sym(Symbol),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Constant {
    Bool(bool),
    Int {
        val: i64,
        ty: ScalarType,
    },
    Float {
        val: f64,
        ty: ScalarType,
    },
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VReg {
    pub class: RegisterClass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Effects {
    pub mem: MemoryEffects,
    pub sync: SyncScope,
    pub ord: MemoryOrdering,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Properties {
    pub mem: MemorySpace,
    pub scope: ExecutionScope,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum RegisterClass {
    Scalar(ScalarType),

    Vector {
        elem: ScalarType,
        lanes: u16,
    },

    Pointer {
        mem: MemorySpace,
        pointee_size: u32,
    },

    Matrix {
        elem: ScalarType,
        rows: u16,
        cols: u16,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperandClass {
    Symbol,

    Shape {
        rank: u16,
    },

    Imm(Constant),

    Register(RegisterClass),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScalarType {
    Bool,

    I8,
    I16,
    I32,
    I64,

    F16,
    F32,
    F64,
}

impl ScalarType {
    #[inline]
    pub const fn bits(self) -> usize {
        match self {
            Self::Bool
            | Self::I8 => 8,
            Self::F16
            | Self::I16 => 16,
            Self::F32
            | Self::I32 => 32,
            Self::F64
            | Self::I64 => 64,
        }
    }

    #[inline]
    pub const fn size(self) -> usize {
        match self {
            Self::Bool
            | Self::I8 => 1,
            Self::F16
            | Self::I16 => 2,
            Self::F32
            | Self::I32 => 4,
            Self::F64
            | Self::I64 => 8,
        }
    }

    #[inline]
    pub const fn prefix(self) -> &'static str {
        match self {
            Self::Bool => "",
            Self::F16
            | Self::F32
            | Self::F64 => "f",
            Self::I8
            | Self::I16
            | Self::I32
            | Self::I64 => "i",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperandHint {
    Unknown,
    Symbol,
    Shape,
    Pointer,
    Scalar(ScalarHint),
    Vector(ScalarHint),
    Matrix(ScalarHint),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScalarHint {
    Unknown,
    Bool,
    Integer,
    SignedInteger,
    UnsignedInteger,
    Float,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeInference {
    pub inputs: &'static [OperandHint],
    pub outputs: Option<OperandHint>,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MemorySpace {
    /// Data visible across the relevant device or execution context. Examples include GPU global memory and ordinary host buffers.
    Global = 3,

    /// Data in a target-specific shared or cooperatively accessible memory space, such as GPU shared memory or a scratchpad.
    Shared = 2,

    /// Per-execution scratch data. A backend might map it to registers, stack storage, or private device memory.
    Local = 1,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExecutionScope {
    Invocation = 1,
    Subgroup = 2,
    Workgroup = 3,
    Device = 4,
    System = 5,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SyncScope {
    None = 0,
    Local = 1,
    Subgroup = 2,
    Workgroup = 3,
    Device = 4,
    System = 5,
}

impl SyncScope {
    #[inline]
    pub const fn exec_scope(self) -> ExecutionScope {
        match self {
            Self::Local
            | Self::None => ExecutionScope::Invocation,
            Self::Subgroup => ExecutionScope::Subgroup,
            Self::Workgroup => ExecutionScope::Workgroup,
            Self::Device => ExecutionScope::Device,
            Self::System => ExecutionScope::System,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryOrdering {
    Relaxed,
    Acquire,
    Release,
    AcquireRelease,
    SequentiallyConsistent,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryEffects {
    None,
    Read,
    Write,
    ReadWrite,
}
