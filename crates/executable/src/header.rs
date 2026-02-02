use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct Header {
    #[validate(custom(function = validate_magic))]
    magic: [u8; 4],

    #[validate(range(
        min = Self::OLDEST_SUPPORTED_VERSION,
        max = Self::CURRENT_VERSION,
        message = "unsupported version of seaX format",
    ))]
    pub version: u32,
}

impl Header {
    pub const MAGIC: [u8; 4] = *b"seaX";
    pub const OLDEST_SUPPORTED_VERSION: u32 = 1;
    pub const CURRENT_VERSION: u32 = 1;

    pub const fn new(version: u32) -> Self {
        Self {
            magic: Self::MAGIC,
            version,
        }
    }

    pub const fn from_bytes(bytes: [u8; 8]) -> Self {
        Self {
            magic: [bytes[0], bytes[1], bytes[2], bytes[3]],
            version: u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
        }
    }

    pub const fn as_bytes(&self) -> [u8; 8] {
        let version_bytes = self.version.to_le_bytes();
        [
            self.magic[0],
            self.magic[1],
            self.magic[2],
            self.magic[3],
            version_bytes[0],
            version_bytes[1],
            version_bytes[2],
            version_bytes[3],
        ]
    }
}

impl Default for Header {
    fn default() -> Self {
        Self::new(Self::CURRENT_VERSION)
    }
}

impl From<[u8; 8]> for Header {
    fn from(bytes: [u8; 8]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl From<Header> for [u8; 8] {
    fn from(header: Header) -> Self {
        header.as_bytes()
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
