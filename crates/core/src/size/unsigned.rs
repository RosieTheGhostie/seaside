/// A non-negative number of bytes.
///
/// This is commonly used for communicating the size of an array or other object.
pub type Size = u32;

#[allow(non_upper_case_globals)]
pub const kB: Size = 1_000;

#[allow(non_upper_case_globals)]
pub const KiB: Size = 1 << 10;

pub const MB: Size = 1_000_000;

#[allow(non_upper_case_globals)]
pub const MiB: Size = 1 << 20;

pub const GB: Size = 1_000_000_000;

#[allow(non_upper_case_globals)]
pub const GiB: Size = 1 << 30;
