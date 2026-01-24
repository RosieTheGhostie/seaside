use super::Flags;
use crate::ser::FixedSerializationSize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasicMetadata {
    pub n_coprocessors: u32,
    pub flags: Flags,
}

impl FixedSerializationSize for BasicMetadata {}
