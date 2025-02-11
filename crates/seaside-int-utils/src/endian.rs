//! A minimalistic way to serialize/deserialize endian metadata.
//!
//! Provides the enum [`Endian`], which indicates the intended byte order/endianness. This
//! information can be seamlessly processed via [`serde`].

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Indicates the intended byte order/endianness.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Serialize, PartialEq)]
#[serde(into = "String")]
pub enum Endian {
    /// Bytes are stored in ascending order of significance.
    #[default]
    #[serde(alias = "little", alias = "lsb")]
    Little,
    /// Bytes are stored in descending order of significance.
    #[serde(alias = "big", alias = "msb")]
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

// This is needed to derive the [`Serialize`] trait for some reason.
impl From<Endian> for String {
    fn from(value: Endian) -> Self {
        value.to_string()
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
