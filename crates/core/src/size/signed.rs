/// The signed counterpart to [`Size`](super::unsigned::Size).
///
/// Mostly useless, but it can come up sometimes.
pub type SignedSize = i32;

#[allow(non_upper_case_globals)]
pub const kB: SignedSize = 1_000;

#[allow(non_upper_case_globals)]
pub const KiB: SignedSize = 1 << 10;

pub const MB: SignedSize = 1_000_000;

#[allow(non_upper_case_globals)]
pub const MiB: SignedSize = 1 << 20;

pub const GB: SignedSize = 1_000_000_000;

#[allow(non_upper_case_globals)]
pub const GiB: SignedSize = 1 << 30;
