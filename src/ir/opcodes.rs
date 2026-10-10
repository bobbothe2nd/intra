use core::fmt::{self, Debug, Display, Formatter};

use crate::ir::data::{ExecutionScope, MemoryEffects, MemoryOrdering, OperandHint, ScalarHint, SyncScope, TypeInference};

staticly::static_enum! {
    #[repr(u16)]
    #[non_exhaustive]
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Opcode {
        BuiltinCompute = "compute",
        BuiltinLoad = "load",
        BuitlinStore = "store",
        BuiltinSync = "sync",
        BuiltinCall = "call",

        CoreAddU = "addu",
        CoreAddI = "addi",
        CoreAddF = "addf",
        CoreAddP = "addp",
        CoreMulU = "mulu",
        CoreMulI = "muli",
        CoreMulF = "mulf",
        CoreMulP = "mulp",
        CoreSubU = "subu",
        CoreSubI = "subi",
        CoreSubF = "subf",
        CoreSubP = "subp",
        CoreDivU = "divu",
        CoreDivI = "divi",
        CoreDivF = "divf",
        CoreDivP = "divp",
        CoreCast = "cast",

        BitCast = "bitcast",
        BitXor = "xor",
        BitOr = "or",
        BitAnd = "and",
        BitXnor = "xnor",
        BitNand = "nand",
        BitNor = "nor",
        BitShl = "shl",
        BitShr = "shr",

        MathExpF = "expf",
        MathLog2F = "log2f",
        MathLog10F = "log10f",

        CoreCmpEqI = "cmpeqi",
        CoreCmpNeI = "cmpnei",
        CoreCmpLtU = "cmpltu",
        CoreCmpLeU = "cmpleu",
        CoreCmpGtU = "cmpgtu",
        CoreCmpGeU = "cmpgeu",
        CoreCmpLtI = "cmplti",
        CoreCmpLeI = "cmplei",
        CoreCmpGtI = "cmpgti",
        CoreCmpGeI = "cmpgei",
        CoreCmpEqF = "cmpeqf",
        CoreCmpNeF = "cmpnef",
        CoreCmpLtF = "cmpltf",
        CoreCmpLeF = "cmplef",
        CoreCmpGtF = "cmpgtf",
        CoreCmpGeF = "cmpgef",

        VecAdd = "vadd",
        VecMul = "vmul",
        VecSub = "vsub",
        VecDiv = "vdiv",
        VecHAdd = "vhadd",
        VecHMul = "vhmul",
        VecHSub = "vhsub",
        VecHDiv = "vhdiv",
    }
}

impl Debug for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Opcode {
    pub const fn type_inference(self) -> TypeInference {
        let inputs: &[OperandHint] = match self {
            Self::CoreAddF
            | Self::CoreDivF
            | Self::CoreMulF
            | Self::CoreSubF
            | Self::MathExpF
            | Self::MathLog10F
            | Self::MathLog2F => &[OperandHint::Scalar(ScalarHint::Float), OperandHint::Scalar(ScalarHint::Float)],
            Self::CoreAddI
            | Self::CoreDivI
            | Self::CoreMulI
            | Self::CoreSubI => &[OperandHint::Scalar(ScalarHint::SignedInteger), OperandHint::Scalar(ScalarHint::SignedInteger)],
            Self::CoreAddU
            | Self::CoreDivU
            | Self::CoreMulU
            | Self::CoreSubU => &[OperandHint::Scalar(ScalarHint::UnsignedInteger), OperandHint::Scalar(ScalarHint::UnsignedInteger)],
            Self::VecAdd
            | Self::VecDiv
            | Self::VecHAdd
            | Self::VecHDiv
            | Self::VecHMul
            | Self::VecHSub
            | Self::VecMul
            | Self::VecSub => &[OperandHint::Vector(ScalarHint::Unknown), OperandHint::Vector(ScalarHint::Unknown)],
            Self::BuiltinCompute => &[OperandHint::Shape, OperandHint::Symbol],
            Self::BuiltinCall => &[OperandHint::Symbol],
            Self::BuiltinLoad => &[OperandHint::Pointer],
            Self::BuitlinStore => &[OperandHint::Pointer, OperandHint::Unknown],
            Self::CoreAddP
            | Self::CoreSubP
            | Self::CoreMulP
            | Self::CoreDivP => &[OperandHint::Pointer, OperandHint::Scalar(ScalarHint::Integer)],
            Self::CoreCmpEqI
            | Self::CoreCmpNeI
            | Self::BitAnd
            | Self::BitNand
            | Self::BitNor
            | Self::BitOr
            | Self::BitShl
            | Self::BitShr
            | Self::BitXnor
            | Self::BitXor => &[OperandHint::Scalar(ScalarHint::Integer), OperandHint::Scalar(ScalarHint::Integer)],
            Self::CoreCmpGeU
            | Self::CoreCmpGtU
            | Self::CoreCmpLeU
            | Self::CoreCmpLtU => &[OperandHint::Scalar(ScalarHint::UnsignedInteger), OperandHint::Scalar(ScalarHint::UnsignedInteger)],
            Self::CoreCmpGeI
            | Self::CoreCmpLtI
            | Self::CoreCmpLeI
            | Self::CoreCmpGtI => &[OperandHint::Scalar(ScalarHint::SignedInteger), OperandHint::Scalar(ScalarHint::SignedInteger)],
            Self::CoreCast
            | Self::BitCast => &[OperandHint::Unknown],
            Self::CoreCmpEqF
            | Self::CoreCmpGeF
            | Self::CoreCmpGtF
            | Self::CoreCmpLeF
            | Self::CoreCmpLtF
            | Self::CoreCmpNeF => &[OperandHint::Scalar(ScalarHint::Float), OperandHint::Scalar(ScalarHint::Float)],
            Self::BuiltinSync => &[],
        };

        debug_assert!(inputs.len() == self.arity() as usize);

        let outputs = match self {
            Self::CoreAddF
            | Self::CoreDivF
            | Self::CoreMulF
            | Self::CoreSubF
            | Self::MathExpF
            | Self::MathLog10F
            | Self::MathLog2F => Some(OperandHint::Scalar(ScalarHint::Float)),
            Self::CoreCmpEqI
            | Self::CoreCmpGeI
            | Self::CoreCmpGtI
            | Self::CoreCmpLeI
            | Self::CoreCmpLtI
            | Self::CoreCmpGeU
            | Self::CoreCmpGtU
            | Self::CoreCmpLeU
            | Self::CoreCmpLtU
            | Self::CoreCmpNeI
            | Self::CoreCmpEqF
            | Self::CoreCmpGeF
            | Self::CoreCmpGtF
            | Self::CoreCmpLeF
            | Self::CoreCmpLtF
            | Self::CoreCmpNeF => Some(OperandHint::Scalar(ScalarHint::Bool)),
            Self::CoreAddI
            | Self::CoreDivI
            | Self::CoreMulI
            | Self::CoreSubI => Some(OperandHint::Scalar(ScalarHint::SignedInteger)),
            Self::CoreAddU
            | Self::CoreDivU
            | Self::CoreMulU
            | Self::CoreSubU => Some(OperandHint::Scalar(ScalarHint::UnsignedInteger)),
            Self::CoreCast
            | Self::BitCast => Some(OperandHint::Unknown),
            Self::BitAnd
            | Self::BitNand
            | Self::BitNor
            | Self::BitOr
            | Self::BitShl
            | Self::BitShr
            | Self::BitXnor
            | Self::BitXor => Some(OperandHint::Scalar(ScalarHint::Integer)),
            Self::CoreAddP
            | Self::CoreSubP
            | Self::CoreMulP
            | Self::CoreDivP => Some(OperandHint::Pointer),
            Self::VecAdd
            | Self::VecDiv
            | Self::VecHAdd
            | Self::VecHDiv
            | Self::VecHMul
            | Self::VecHSub
            | Self::VecMul
            | Self::VecSub => Some(OperandHint::Vector(ScalarHint::Unknown)),
            Self::BuiltinLoad => Some(OperandHint::Unknown),
            Self::BuiltinSync
            | Self::BuiltinCompute
            | Self::BuitlinStore
            | Self::BuiltinCall => None,
        };

        TypeInference {
            inputs,
            outputs,
        }
    }

    pub const fn mem_effects(self) -> MemoryEffects {
        match self {
            Self::CoreAddF
            | Self::CoreAddI
            | Self::CoreAddU
            | Self::CoreDivF
            | Self::CoreDivI
            | Self::CoreDivU
            | Self::CoreMulF
            | Self::CoreMulI
            | Self::CoreMulU
            | Self::CoreSubF
            | Self::CoreSubI
            | Self::CoreSubU
            | Self::VecAdd
            | Self::VecDiv
            | Self::VecHAdd
            | Self::VecHDiv
            | Self::VecHMul
            | Self::VecHSub
            | Self::VecMul
            | Self::VecSub
            | Self::MathExpF
            | Self::MathLog10F
            | Self::MathLog2F
            | Self::BuiltinCompute
            | Self::BuiltinSync
            | Self::BuiltinCall
            | Self::CoreCmpEqI
            | Self::CoreCmpGeI
            | Self::CoreCmpGtI
            | Self::CoreCmpLeI
            | Self::CoreCmpLtI
            | Self::CoreCmpGeU
            | Self::CoreCmpGtU
            | Self::CoreCmpLeU
            | Self::CoreCmpLtU
            | Self::CoreCmpNeI
            | Self::CoreCmpEqF
            | Self::CoreCmpGeF
            | Self::CoreCmpGtF
            | Self::CoreCmpLeF
            | Self::CoreCmpLtF
            | Self::CoreCmpNeF
            | Self::BitAnd
            | Self::BitNand
            | Self::BitNor
            | Self::BitOr
            | Self::BitShl
            | Self::BitShr
            | Self::BitXnor
            | Self::BitXor
            | Self::CoreAddP
            | Self::CoreCast
            | Self::BitCast
            | Self::CoreSubP
            | Self::CoreDivP
            | Self::CoreMulP => MemoryEffects::None,
            Self::BuitlinStore => MemoryEffects::Write,
            Self::BuiltinLoad => MemoryEffects::Read,
        }
    }

    pub const fn mem_ord(self) -> MemoryOrdering {
        MemoryOrdering::Relaxed
    }

    pub const fn sync_scope(self, scope: ExecutionScope) -> SyncScope {
        if matches!(self, Self::BuiltinSync) {
            match scope {
                ExecutionScope::Invocation => SyncScope::Local,
                ExecutionScope::Subgroup => SyncScope::Subgroup,
                ExecutionScope::Workgroup => SyncScope::Workgroup,
                ExecutionScope::Device => SyncScope::Device,
                ExecutionScope::System => SyncScope::System,
            }
        } else {
            SyncScope::None
        }
    }

    pub const fn arity(self) -> u8 {
        match self {
            Self::BuiltinCompute
            | Self::BuitlinStore
            | Self::CoreAddF
            | Self::CoreAddI
            | Self::CoreAddU
            | Self::CoreDivF
            | Self::CoreDivI
            | Self::CoreDivU
            | Self::CoreMulF
            | Self::CoreMulI
            | Self::CoreMulU
            | Self::CoreSubF
            | Self::CoreSubI
            | Self::CoreSubU
            | Self::VecAdd
            | Self::VecDiv
            | Self::VecHAdd
            | Self::VecHDiv
            | Self::VecHMul
            | Self::VecHSub
            | Self::VecMul
            | Self::VecSub
            | Self::CoreCmpEqI
            | Self::CoreCmpGeI
            | Self::CoreCmpGtI
            | Self::CoreCmpLeI
            | Self::CoreCmpLtI
            | Self::CoreCmpNeI
            | Self::CoreCmpGeU
            | Self::CoreCmpGtU
            | Self::CoreCmpLeU
            | Self::CoreCmpLtU
            | Self::CoreCmpEqF
            | Self::CoreCmpGeF
            | Self::CoreCmpGtF
            | Self::CoreCmpLeF
            | Self::CoreCmpLtF
            | Self::CoreCmpNeF
            | Self::BitAnd
            | Self::BitNand
            | Self::BitNor
            | Self::BitOr
            | Self::BitShl
            | Self::BitShr
            | Self::BitXnor
            | Self::BitXor
            | Self::CoreAddP
            | Self::CoreSubP
            | Self::CoreMulP
            | Self::CoreDivP => 2,
            Self::BuiltinLoad
            | Self::MathExpF
            | Self::MathLog10F
            | Self::MathLog2F
            | Self::CoreCast
            | Self::BitCast => 1,
            Self::BuiltinSync
            | Self::BuiltinCall => 0,
        }
    }

    pub const fn has_output(self) -> bool {
        match self {
            Self::CoreAddF
            | Self::CoreAddI
            | Self::CoreAddU
            | Self::CoreDivF
            | Self::CoreDivI
            | Self::CoreDivU
            | Self::CoreMulF
            | Self::CoreMulI
            | Self::CoreMulU
            | Self::CoreSubF
            | Self::CoreSubI
            | Self::CoreSubU
            | Self::VecAdd
            | Self::VecDiv
            | Self::VecHAdd
            | Self::VecHDiv
            | Self::VecHMul
            | Self::VecHSub
            | Self::VecMul
            | Self::VecSub
            | Self::MathExpF
            | Self::MathLog10F
            | Self::MathLog2F
            | Self::BuiltinLoad
            | Self::CoreCmpEqI
            | Self::CoreCmpGeI
            | Self::CoreCmpGtI
            | Self::CoreCmpLeI
            | Self::CoreCmpLtI
            | Self::CoreCmpGeU
            | Self::CoreCmpGtU
            | Self::CoreCmpLeU
            | Self::CoreCmpLtU
            | Self::CoreCmpNeI
            | Self::CoreCmpEqF
            | Self::CoreCmpGeF
            | Self::CoreCmpGtF
            | Self::CoreCmpLeF
            | Self::CoreCmpLtF
            | Self::CoreCmpNeF
            | Self::BitAnd
            | Self::BitNand
            | Self::BitNor
            | Self::BitOr
            | Self::BitShl
            | Self::BitShr
            | Self::BitXnor
            | Self::BitXor
            | Self::CoreAddP
            | Self::CoreSubP
            | Self::CoreMulP
            | Self::CoreDivP
            | Self::CoreCast
            | Self::BitCast => true,
            Self::BuiltinSync
            | Self::BuiltinCompute
            | Self::BuitlinStore
            | Self::BuiltinCall => false,
        }
    }
}
