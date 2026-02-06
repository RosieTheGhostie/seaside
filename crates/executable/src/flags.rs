use bitflags::bitflags;
use seaside_int_utils::Endian;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
    pub struct Flags: u64 {
        const BIG_ENDIAN = 1 << 0;
        const SELF_MODIFYING_CODE = 1 << 1;
        const DELAY_SLOT = 1 << 2;
        const FREEABLE_HEAP_ALLOCATIONS = 1 << 3;
    }
}

impl Flags {
    pub const fn endian(&self) -> Endian {
        if self.contains(Self::BIG_ENDIAN) {
            Endian::Big
        } else {
            Endian::Little
        }
    }

    pub const fn self_modifying_code(&self) -> bool {
        self.contains(Self::SELF_MODIFYING_CODE)
    }

    pub const fn delay_slot(&self) -> bool {
        self.contains(Self::DELAY_SLOT)
    }

    pub const fn freeable_heap_allocations(&self) -> bool {
        self.contains(Self::FREEABLE_HEAP_ALLOCATIONS)
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self::FREEABLE_HEAP_ALLOCATIONS
    }
}
