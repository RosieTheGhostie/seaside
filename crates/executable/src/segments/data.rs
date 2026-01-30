use core::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct DataSegment(pub Vec<u8>);

impl Deref for DataSegment {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DataSegment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DataSegment {
    pub fn overwrite(&mut self, bytes: Vec<u8>) {
        self.0 = bytes;
    }
}
