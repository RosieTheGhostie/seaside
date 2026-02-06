use strum::{Display, EnumString};

pub type SegmentDirective = seaside_constants::StaticSegment;

#[derive(Clone, Copy, Debug, Display, EnumString, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[strum(serialize_all = "lowercase")]
pub enum ValueDirective {
    Byte,
    Half,
    Word,
    Float,
    Double,
}

#[derive(Clone, Copy, Debug, Display, EnumString, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[strum(serialize_all = "lowercase")]
pub enum StringDirective {
    Ascii,
    Asciiz,
}
