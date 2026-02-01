use core::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Segments {
    pub text: Segment,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ktext: Option<Segment>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#extern: Option<Segment>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Segment>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kdata: Option<Segment>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Segment(pub ByteBuf);

impl Deref for Segment {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Segment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Segment {
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self(ByteBuf::from(bytes))
    }

    pub fn overwrite(&mut self, bytes: impl Into<Vec<u8>>) {
        self.0 = ByteBuf::from(bytes);
    }
}

impl<T> From<T> for Segment
where
    T: Into<Vec<u8>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
