#![allow(dead_code)]
mod bitflags_addons;
pub mod endian;
pub mod features;
pub mod memory_map;
mod presets;
mod red_flag_behavior;
pub mod register_defaults;
pub mod validate;
mod version;

use crate::engine::{Error, ErrorKind};
use clap::crate_version;
pub use endian::Endian;
pub use features::Features;
pub use memory_map::MemoryMap;
use minimal_logging::attributes::wip;
pub use register_defaults::RegisterDefaults;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
pub use validate::Validate;
use version::VersionComparison;

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(with = "version")]
    pub version: Version,
    #[serde(alias = "byte_order")]
    pub endian: Endian,
    pub features: Features,
    pub memory_map: MemoryMap,
    pub register_defaults: RegisterDefaults,
}

impl Validate for Config {
    #[wip]
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
