//! A minimalistic way to serialize/deserialize endian metadata.
//!
//! Provides the enum [`Endian`], which indicates the intended byte order/endianness. This
//! information can be seamlessly processed via [`serde`] if desired.

#[cfg(feature = "all_zeroes")]
use crate::AllZeroes;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Indicates the intended byte order/endianness.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
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
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(match self {
            Endian::Little => "little",
            Endian::Big => "big",
        })
    }
}

// This is needed to derive the `Serialize` trait for some reason.
#[cfg(feature = "serde")]
impl From<Endian> for String {
    fn from(value: Endian) -> Self {
        value.to_string()
    }
}

#[cfg(feature = "all_zeroes")]
impl AllZeroes for Endian {
    fn all_zeroes() -> Self {
        Self::Little
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
