#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SegmentDirective {
    Data = 0,
    Extern = 1,
    KData = 2,
    KText = 3,
    Text = 4,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DataTypeDirective {
    Align,
    Ascii,
    Asciiz,
    Byte,
    Double,
    Float,
    Half,
    Space,
    Word,
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
