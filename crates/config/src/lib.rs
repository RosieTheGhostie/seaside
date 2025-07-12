pub mod features;
pub mod memory_map;
pub mod register_defaults;
pub mod validate;

mod primitive_defaults;
mod version;

pub use features::Features;
pub use memory_map::MemoryMap;
pub use register_defaults::RegisterDefaults;
pub use validate::Validate;

use anyhow::{Context, Error, Result};
use core::str::FromStr;
use seaside_error::EngineError;
use seaside_int_utils::{AllZeroes, Endian};
use semver::Version;
use serde::{Deserialize, Serialize};
use version::VersionComparison;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(with = "version")]
    pub version: Version,
    #[serde(default, alias = "byte_order")]
    pub endian: Endian,
    #[serde(default = "primitive_defaults::r#true")]
    pub project_directory_is_cwd: bool,
    pub features: Features,
    pub memory_map: MemoryMap,
    pub register_defaults: RegisterDefaults,
}

impl Validate for Config {
    fn validate(&self) -> Result<()> {
        use VersionComparison::*;
        let seaside_version = Version::from_str(env!("CARGO_PKG_VERSION"))?;
        match VersionComparison::compare(&seaside_version, &self.version) {
            Compatible => Ok(()),
            AIsAheadOfB => Err(Error::new(EngineError::OutdatedVersion)).with_context(|| {
                format!(
                    "consider updating config (v{}) to match seaside (v{seaside_version})",
                    self.version,
                )
            }),
            BIsAheadOfA => Err(Error::new(EngineError::OutdatedVersion)).with_context(|| {
                format!(
                    "consider updating seaside (v{seaside_version}) to match config (v{})",
                    self.version,
                )
            }),
        }?;
        self.features.services.validate()?;
        self.memory_map.validate()
    }
}

impl AllZeroes for Config {
    fn all_zeroes() -> Self {
        Self {
            version: Version::new(0, 0, 0),
            endian: Endian::all_zeroes(),
            project_directory_is_cwd: false,
            features: Features::all_zeroes(),
            memory_map: MemoryMap::all_zeroes(),
            register_defaults: RegisterDefaults::default(),
        }
    }
}
