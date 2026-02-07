use core::ops::{Deref, DerefMut};

use seaside_type_aliases::{Address, Size};

use super::Inner;

#[repr(transparent)]
pub struct DataRegion {
    pub inner: Inner,
}

impl Deref for DataRegion {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for DataRegion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl DataRegion {
    pub fn new(low_address: Address, bytes_to_allocate: Size) -> Self {
        Self {
            inner: Inner::new(low_address, bytes_to_allocate),
        }
    }
}
