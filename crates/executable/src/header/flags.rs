use crate::ser::FixedSerializationSize;

/// TODO
#[derive(Clone, Copy, Debug, /* temporary */ Default, Eq, PartialEq)]
pub struct Flags(u64);

impl FixedSerializationSize for Flags {}
