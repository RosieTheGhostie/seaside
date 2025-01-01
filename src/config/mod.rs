#![allow(dead_code)]
pub mod endian;
pub mod features;
pub mod memory_map;
pub mod register_defaults;
pub mod validate;

mod bitflags_addons;
mod presets;
mod version;

pub use endian::Endian;
pub use features::Features;
pub use memory_map::MemoryMap;
pub use register_defaults::RegisterDefaults;
pub use validate::Validate;

use crate::engine::{Error, ErrorKind};
use clap::crate_version;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use version::VersionComparison;

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(with = "version")]
    pub version: Version,
    #[serde(alias = "byte_order")]
    pub endian: Endian,
    pub project_directory_is_cwd: bool,
    pub features: Features,
    pub memory_map: MemoryMap,
    pub register_defaults: RegisterDefaults,
}

impl Validate for Config {
    fn validate(&self) -> Result<(), Error> {
        use VersionComparison::*;
        let seaside_version = Version::from_str(crate_version!()).map_err(|_| {
            Error::new(
                ErrorKind::InternalLogicIssue,
                "failed to fetch version of seaside",
            )
        })?;
        match VersionComparison::compare(&seaside_version, &self.version) {
            Compatible { patch_available: _ } => Ok(()),
            AIsAheadOfB => Err(Error::new(
                ErrorKind::OutdatedVersion,
                format!(
                    "consider updating config (v{}) to match seaside (v{seaside_version})",
                    self.version
                ),
            )),
            BIsAheadOfA => Err(Error::new(
                ErrorKind::OutdatedVersion,
                format!(
                    "consider updating seaside (v{seaside_version}) to match config (v{})",
                    self.version,
                ),
            )),
        }?;
        self.features.syscalls.validate()?;
        self.memory_map.validate()
    }
}
