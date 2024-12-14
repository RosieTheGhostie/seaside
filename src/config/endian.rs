use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Deserialize, Eq, Serialize, PartialEq)]
#[serde(into = "String")]
pub enum Endian {
    #[default]
    #[serde(alias = "little", alias = "lsb")]
    Little,
    #[serde(alias = "big", alias = "msb")]
    Big,
}

impl From<Endian> for String {
    fn from(value: Endian) -> Self {
        match value {
            Endian::Little => "little",
            Endian::Big => "big",
        }
        .to_string()
    }
}

impl Endian {
    #[cfg(target_endian = "big")]
    pub fn should_swap_bytes(&self) -> bool {
        *self == Self::Little
    }

    #[cfg(target_endian = "little")]
    pub fn should_swap_bytes(&self) -> bool {
        *self == Self::Big
    }
}
