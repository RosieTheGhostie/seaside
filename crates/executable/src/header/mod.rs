pub mod flags;

pub use flags::Flags;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct Header {
    #[validate(custom(function = validate_magic))]
    magic: [u8; 4],

    pub flags: Flags,
}

impl Header {
    pub const MAGIC: [u8; 4] = *b"seaX";

    pub const fn new(flags: Flags) -> Self {
        Self {
            magic: Self::MAGIC,
            flags,
        }
    }
}

impl Default for Header {
    fn default() -> Self {
        Self::new(Flags::default())
    }
}

fn validate_magic(magic: &[u8; 4]) -> Result<(), ValidationError> {
    if magic == &Header::MAGIC {
        Ok(())
    } else {
        Err(ValidationError::new(
            "bad magic value for seaside executable",
        ))
    }
}
