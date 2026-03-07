pub mod address;
pub mod consts;
pub mod instruction;
pub mod prelude;
pub mod register;
pub mod size;
pub mod traits;

pub use address::Address;
pub use address_range::AddressRange;
pub use byte_stream::ByteStream;
pub use endian::Endian;
pub use error::EngineError;
pub use instruction::{Instruction, UnpackedInstruction};
pub use services::Services;
pub use tiny_uints::{u2, u3, u5};

mod address_range;
mod byte_stream;
mod endian;
mod error;
mod services;
mod tiny_uints;

/// A signed offset, usually to a [memory address](address::Address).
pub type Offset = i32;

/// The unsigned counterpart to [`Offset`].
///
/// Mostly useless, but it can come up sometimes.
pub type UnsignedOffset = u32;

/// A code used to specify a system service.
pub type ServiceCode = u32;
