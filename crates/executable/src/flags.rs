use bitflags::bitflags;
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

impl Default for Flags {
    fn default() -> Self {
        Self::FREEABLE_HEAP_ALLOCATIONS
    }
}
