use num_traits::{FromPrimitive, ToPrimitive};
use std::str::FromStr;

/// A set of related registers.
pub trait RegisterSet: FromStr + FromPrimitive + ToPrimitive {
    /// The number of registers in this set.
    const NUM_REGISTERS: usize;
    /// The names of each register. Should be in the same order as the register array.
    const REGISTER_NAMES: &'static [&'static str];
}
