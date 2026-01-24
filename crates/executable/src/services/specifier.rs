use crate::ser::FixedSerializationSize;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ServiceSpecifier {
    pub provider: u32,
    pub group: u32,
    pub subgroup: Option<u32>,
    pub member: u32,
}

impl FixedSerializationSize for ServiceSpecifier {}
