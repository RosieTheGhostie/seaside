pub mod address;
pub mod register;
pub mod size;

pub use address::Address;
pub use size::{SignedSize, Size};
#[doc(inline)]
pub use u2_mod::u2;
#[doc(inline)]
pub use u3_mod::u3;
#[doc(inline)]
pub use u5_mod::u5;

mod u2_mod;
mod u3_mod;
mod u5_mod;

/// A raw MIPS machine code instruction.
pub type Instruction = u32;

/// A signed offset, usually to a [memory address](address::Address).
pub type Offset = i32;

/// The unsigned counterpart to [`Offset`].
///
/// Mostly useless, but it can come up sometimes.
pub type UnsignedOffset = u32;

/// A code used to specify a system service.
pub type ServiceCode = u32;

mod tiny_uint_panic_messages {
    pub const CONVERT_WITH_OVERFLOW: &str = "attempt to convert with overflow";
    pub const ADD_WITH_OVERFLOW: &str = "attempt to add with overflow";
    pub const SUBTRACT_WITH_OVERFLOW: &str = "attempt to subtract with overflow";
    pub const MULTIPLY_WITH_OVERFLOW: &str = "attempt to multiply with overflow";
    pub const DIVIDE_BY_ZERO: &str = "attempt to divide by zero";
    pub const REMAINDER_WITH_ZERO_DIVISOR: &str =
        "attempt to calculate the remainder with a divisor of zero";
    pub const NON_POSITIVE_LOGARITHM_ARGUMENT: &str =
        "argument of integer logarithm must be positive";
}
