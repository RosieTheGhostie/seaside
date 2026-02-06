use strum::{Display, EnumCount, EnumString, IntoStaticStr, VariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    EnumCount,
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
    Text,
    KText,
    Extern,
    Data,
    KData,
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
