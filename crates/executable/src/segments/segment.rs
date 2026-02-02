use core::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

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
