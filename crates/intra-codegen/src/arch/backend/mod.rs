macro_rules! lower_error {
    (@RegisterExhaustion $reg:literal) => {
        $crate::error::Error::new(concat!("attempt to allocate ", $reg, " but they are exhausted"), $crate::error::ErrorCode::RegisterExhaustion)
    };

    (@UnsupportedRegister $reg:literal) => {
        $crate::error::Error::new(concat!("attempt to use unsupported register of class ", $reg), $crate::error::ErrorCode::UnsupportedRegister)
    };

    (@InvalidRegister $reg:literal $size:literal) => {
        $crate::error::Error::new(concat!("use of invalid register of class ", $reg, " with size of ", $size), $crate::error::ErrorCode::InvalidRegister)
    };

    (@UnsupportedOpcode $op:literal) => {
        $crate::error::Error::new(concat!("attempt to use unsupported opcode ", $op), $crate::error::ErrorCode::UnsupportedOpcode)
    };

    (@MissingOutput) => {
        $crate::error::Error::new(concat!("expected output for instruction"), $crate::error::ErrorCode::MissingOutput)
    };

    (@InvalidOperands) => {
        $crate::error::Error::new(concat!("found instruction with invalid operands"), $crate::error::ErrorCode::InvalidOperands)
    };

    (@InvalidOpcode) => {
        $crate::error::Error::new(concat!("found instruction with invalid opcode"), $crate::error::ErrorCode::InvalidInstruction)
    };
}

pub mod x86;
