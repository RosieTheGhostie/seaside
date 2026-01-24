use strum::{Display, EnumString, IntoStaticStr, VariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    EnumString,
    Eq,
    Hash,
    IntoStaticStr,
    Ord,
    PartialEq,
    PartialOrd,
    VariantNames,
)]
#[strum(serialize_all = "lowercase")]
#[repr(u8)]
pub enum StaticSegment {
    Data,
    Extern,
    KData,
    KText,
    Text,
}

impl StaticSegment {
    pub const fn names() -> &'static [&'static str] {
        Self::VARIANTS
    }

    pub const fn is_data_segment(&self) -> bool {
        matches!(self, Self::Data | Self::Extern | Self::KData)
    }

    pub const fn is_text_segment(&self) -> bool {
        matches!(self, Self::KText | Self::Text)
    }
}
