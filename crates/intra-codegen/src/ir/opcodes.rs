use core::fmt::{self, Debug, Display, Formatter};

use crate::ir::data::{ExecutionScope, MemoryEffects, MemoryOrdering, OperandHint, ScalarHint, SyncScope, TypeInference};

staticly::static_enum! {
    /// Primitive enum containing all instructions used by the compiler
    #[repr(u16)]
    #[non_exhaustive]
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Opcode {
        /// Loop over multiple axes simultaneously via `compute(Shape, block)`
        BuiltinCompute = "compute",
        /// Reads a value from an address `dst = load [src]`
        BuiltinLoad = "load",
        /// Writes the value to an address `store [dst], src`
        BuitlinStore = "store",
        /// Synchronizes the current [`SyncScope`]
        BuiltinSync = "sync",
        /// Calls a function `call sym`
        BuiltinCall = "call",
        /// Moves the value of one virtual register to another `mov dst, src`
        BuiltinMove = "mov",

        /// Add unsigned integers `dst = addu a, b`
        CoreAddU = "addu",
        /// Add signed integers `dst = addi a, b`
        CoreAddI = "addi",
        /// Add floating point values `dst = addf a, b`
        CoreAddF = "addf",
        /// Add an unsigned integer to a pointer `dst = addp a, b`
        CoreAddP = "addp",
        /// Multiply unsigned integers `dst = mulu a, b`
        CoreMulU = "mulu",
        /// Multiply signed integer `dst = muli a, b`
        CoreMulI = "muli",
        /// Multiply floating point values `dst = mulf a, b`
        CoreMulF = "mulf",
        /// Multiply an unsigned integer and a pointer `dst = mulp a, b`
        CoreMulP = "mulp",
        /// Subtract unsigned integers `dst = subu a, b`
        CoreSubU = "subu",
        /// Subtract signed integers `dst = subi a, b`
        CoreSubI = "subi",
        /// Subtract floating point values `dst = subf a, b`
        CoreSubF = "subf",
        /// Subtract an unsigned integer from a pointer `dst = subp a, b`
        CoreSubP = "subp",
        /// Divide unsigned integers `dst = divu a, b`
        CoreDivU = "divu",
        /// Divide signed integers `dst = divi a, b`
        CoreDivI = "divi",
        /// Divide floating point values `dst = divf a, b`
        CoreDivF = "divf",
        /// Divide a pointer by an unsigned integer `dst = divu a, b`
        CoreDivP = "divp",
        /// Semantically cast types; emits no instruction
        CoreCast = "cast",

        /// Reinterpret bits as a new type; emits no instruction
        BitCast = "bitcast",
        /// Bitwise eXclusive OR of `a ^ b`, `dst = xor a, b`
        BitXor = "xor",
        /// Bitwise OR of `a | b`, `dst = or a, b`
        BitOr = "or",
        /// Bitwise AND of `a & b`, `dst = and a, b`
        BitAnd = "and",
        /// Bitwise eXclusive NOR of `!(a ^ b)`, `dst = xnor a, b`
        BitXnor = "xnor",
        /// Bitwise NAND of `!(a & b)`, `dst = nand a, b`
        BitNand = "nand",
        /// Bitwise NOR of `!(a | b)`, `dst = nor a, b`
        BitNor = "nor",
        /// Shift integer left by runtime bit count, `dst = shl a, b`
        BitShl = "shl",
        /// Shift integer right by runtime bit count, `dst = shr a, b`
        BitShr = "shr",

        /// Mathematical exponentiation of floating point values `dst = expf a`
        MathExpF = "expf",
        /// Mathematical base-2 logarithm of floating point values `dst = log2f a`
        MathLog2F = "log2f",
        /// Mathematical base-10 logarithm of floating point values `dst = log10f a`
        MathLog10F = "log10f",

        /// Equality comparison of two integers `cond = cmpeqi a, b`
        CoreCmpEqI = "cmpeqi",
        /// Inequality comparison of two integers `cond = cmpnei a, b`
        CoreCmpNeI = "cmpnei",
        /// Less-than comparison of two unsigned integers `cond = cmpltu a, b`
        CoreCmpLtU = "cmpltu",
        /// Less-than-or-equal comparison of two unsigned integers `cond = cmpleu a, b`
        CoreCmpLeU = "cmpleu",
        /// Greater-than comparison of two unsigned integers `cond = cmpgtu a, b`
        CoreCmpGtU = "cmpgtu",
        /// Greater-than-or-equal comparison of two unsigned integers `cond = cmpgeu a, b`
        CoreCmpGeU = "cmpgeu",
        /// Less-than comparison of two signed integers `cond = cmplti a, b`
        CoreCmpLtI = "cmplti",
        /// Less-than-or-equal comparison of two signed integers `cond = cmplei a, b`
        CoreCmpLeI = "cmplei",
        /// Greater-than comparison of two signed integers `cond = cmpgti a, b`
        CoreCmpGtI = "cmpgti",
        /// Greater-than-or-equal comparison of two signed integers `cond = cmpgei a, b`
        CoreCmpGeI = "cmpgei",
        /// Equality comparison of two floating point values `cond = cmpeqf a, b`
        CoreCmpEqF = "cmpeqf",
        /// Inequality comparison of two floating point values `cond = cmpnef a, b`
        CoreCmpNeF = "cmpnef",
        /// Less-than comparison of two floating point values `cond = cmpltf a, b`
        CoreCmpLtF = "cmpltf",
        /// Less-than-or-equal comparison of two floating point values `cond = cmplef a, b`
        CoreCmpLeF = "cmplef",
        /// Greater-than comparison of two floating point values `cond = cmpgtf a, b`
        CoreCmpGtF = "cmpgtf",
        /// Greater-than-or-equal comparison of two floating point values `cond = cmpgef a, b`
        CoreCmpGeF = "cmpgef",

        /// Vectorized addition of two SIMD registers `dst = vadd a, b`
        VecAdd = "vadd",
        /// Vectorized multiplication of two SIMD registers `dst = vmul a, b`
        VecMul = "vmul",
        /// Vectorized subtraction of two SIMD registers `dst = vsub a, b`
        VecSub = "vsub",
        /// Vectorized division of two SIMD registers `dst = vdiv a, b`
        VecDiv = "vdiv",
        /// Vectorized horizontal addition of two SIMD registers, packed into one `dst = vhadd a, b`
        VecHAdd = "vhadd",
        /// Vectorized horizontal multiplication of two SIMD registers, packed into one `dst = vhmul a, b`
        VecHMul = "vhmul",
        /// Vectorized horizontal subtraction of two SIMD registers, packed into one `dst = vhsub a, b`
        VecHSub = "vhsub",
        /// Vectorized horizontal division of two SIMD registers, packed into one `dst = vhdiv a, b`
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
            Self::BuiltinMove => &[OperandHint::Unknown, OperandHint::Unknown],
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
            | Self::CoreSubI
            | Self::CoreCmpGeI
            | Self::CoreCmpLtI
            | Self::CoreCmpLeI
            | Self::CoreCmpGtI => &[OperandHint::Scalar(ScalarHint::SignedInteger), OperandHint::Scalar(ScalarHint::SignedInteger)],
            Self::CoreAddU
            | Self::CoreDivU
            | Self::CoreMulU
            | Self::CoreSubU
            | Self::CoreCmpGeU
            | Self::CoreCmpGtU
            | Self::CoreCmpLeU
            | Self::CoreCmpLtU => &[OperandHint::Scalar(ScalarHint::UnsignedInteger), OperandHint::Scalar(ScalarHint::UnsignedInteger)],
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
            | Self::BuiltinCall
            | Self::BuiltinMove => None,
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
            | Self::CoreMulP
            | Self::BuiltinMove => MemoryEffects::None,
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
            | Self::CoreDivP
            | Self::BuiltinMove => 2,
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
            | Self::BuiltinCall
            | Self::BuiltinMove => false,
        }
    }
}
