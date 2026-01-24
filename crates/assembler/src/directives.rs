use core::str::FromStr;

use thiserror::Error; // these aren't errors, but i wanna convert them to strings

pub type SegmentDirective = seaside_constants::StaticSegment;

#[derive(Clone, Copy, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum ValueDirective {
    #[error("byte")]
    Byte,

    #[error("half")]
    Half,

    #[error("word")]
    Word,

    #[error("float")]
    Float,

    #[error("double")]
    Double,
}

#[derive(Clone, Copy, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum StringDirective {
    #[error("ascii")]
    Ascii,

    #[error("asciiz")]
    Asciiz,
}

impl FromStr for ValueDirective {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "byte" => Self::Byte,
            "double" => Self::Double,
            "float" => Self::Float,
            "half" => Self::Half,
            "word" => Self::Word,
            _ => return Err("not a valid value directive"),
        })
    }
}

impl FromStr for StringDirective {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "ascii" => Self::Ascii,
            "asciiz" => Self::Asciiz,
            _ => return Err("not a valid string directive"),
        })
    }
}
