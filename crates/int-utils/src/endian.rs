//! A minimalistic way to serialize/deserialize endian metadata.
//!
//! Provides the enum [`Endian`], which indicates the intended byte order/endianness. This
//! information can be seamlessly processed via [`serde`] if desired.

use core::fmt::{self, Display, Formatter};

/// Indicates the intended byte order/endianness.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(into = "String")
)]
pub enum Endian {
    /// Bytes are stored in ascending order of significance.
    #[default]
    #[cfg_attr(feature = "serde", serde(alias = "little", alias = "lsb"))]
    Little,

    /// Bytes are stored in descending order of significance.
    #[cfg_attr(feature = "serde", serde(alias = "big", alias = "msb"))]
    Big,
}

impl Display for Endian {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Endian::Little => "little",
            Endian::Big => "big",
        })
    }
}

// This is needed to derive the `Serialize` trait for some reason.
#[cfg(feature = "serde")]
impl From<Endian> for String {
    fn from(endianness: Endian) -> Self {
        endianness.to_string()
    }
}

impl Endian {
    /// Returns true if the intended byte order does not match the current platform's byte order.
    #[cfg(target_endian = "little")]
    pub fn should_swap_bytes(&self) -> bool {
        *self == Self::Big
    }

    /// Returns true if the intended byte order does not match the current platform's byte order.
    #[cfg(target_endian = "big")]
    pub fn should_swap_bytes(&self) -> bool {
        *self == Self::Little
    }
}
