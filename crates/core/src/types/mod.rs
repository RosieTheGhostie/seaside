pub mod address;
pub mod register;
pub mod size;

pub use address::Address;
pub use size::{SignedSize, Size};

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
