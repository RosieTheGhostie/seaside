pub mod features;

mod primitive_defaults;

pub use features::Features;

use anyhow::{Error, Result};
use seaside_error::EngineError;
use seaside_executable::MemoryMap;
use seaside_int_utils::Endian;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Config {
    #[serde(default, alias = "byte_order")]
    pub endian: Endian,

    #[serde(default = "primitive_defaults::r#true")]
    pub executable_parent_is_cwd: bool,

    #[validate(nested)]
    pub features: Features,

    #[validate(nested)]
    pub memory_map: MemoryMap,
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        <Self as Validate>::validate(self)
            .map_err(|errs| Error::new(EngineError::InvalidConfig).context(errs))
    }
}
