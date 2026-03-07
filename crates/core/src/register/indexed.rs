use core::{
    fmt::{self, Display, Formatter, Write},
    num::IntErrorKind,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use super::ParseError;
use crate::u5;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
pub struct RegisterIndex(pub u5);

impl Deref for RegisterIndex {
    type Target = u5;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RegisterIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for RegisterIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_char('$')?;
        }

        write!(f, "{}", self.0)
    }
}

impl FromStr for RegisterIndex {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix('$').unwrap_or(s).parse() {
            Ok(index) => Ok(Self(index)),
            Err(err) if *err.kind() == IntErrorKind::Empty => Err(ParseError::Empty),
            Err(_) => Err(ParseError::BadValue),
        }
    }
}
