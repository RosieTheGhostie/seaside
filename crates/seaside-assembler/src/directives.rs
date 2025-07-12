use core::str::FromStr;
use thiserror::Error; // these aren't errors, but i wanna convert them to strings

#[derive(Clone, Copy, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum SegmentDirective {
    #[error("data")]
    Data = 0,
    #[error("extern")]
    Extern = 1,
    #[error("kdata")]
    KData = 2,
    #[error("ktext")]
    KText = 3,
    #[error("text")]
    Text = 4,
}

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

impl SegmentDirective {
    pub const fn names() -> [&'static str; 5] {
        ["data", "extern", "kdata", "ktext", "text"]
    }

    pub const fn is_data_segment(&self) -> bool {
        matches!(self, Self::Data | Self::Extern | Self::KData)
    }

    pub const fn is_text_segment(&self) -> bool {
        matches!(self, Self::KText | Self::Text)
    }
}

impl FromStr for SegmentDirective {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "data" => Self::Data,
            "extern" => Self::Extern,
            "kdata" => Self::KData,
            "ktext" => Self::KText,
            "text" => Self::Text,
            _ => return Err("not a valid segment directive"),
        })
    }
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
