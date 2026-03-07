use core::{
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut},
};

use crate::u3;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ConditionCode(pub u3);

impl Deref for ConditionCode {
    type Target = u3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConditionCode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for ConditionCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
