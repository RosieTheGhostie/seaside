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

use std::str::FromStr;

use crate::engine::{Error, ErrorKind};
use clap::crate_version;
pub use endian::Endian;
pub use features::Features;
pub use memory_map::MemoryMap;
use minimal_logging::attributes::wip;
pub use register_defaults::RegisterDefaults;
use semver::Version;
use serde::{Deserialize, Serialize};
pub use validate::Validate;

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(with = "version")]
    pub seaside_version: Version,
    #[serde(alias = "byte_order")]
    pub endian: Endian,
    pub features: Features,
    pub memory_map: MemoryMap,
    pub register_defaults: RegisterDefaults,
}

impl Validate for Config {
    #[wip]
    fn validate(&self) -> Result<(), Error> {
        match Version::from_str(crate_version!()) {
            Ok(version) if self.seaside_version > version => Err(Error::new(
                ErrorKind::OutdatedVersion,
                format!(
                    "consider upgrading seaside (v{version}) to match config (v{})",
                    self.seaside_version
                ),
            )),
            Ok(version) if self.seaside_version < version => Err(Error::new(
                ErrorKind::OutdatedVersion,
                format!(
                    "consider upgrade config (v{}) to match seaside (v{version})",
                    self.seaside_version
                ),
            )),
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(
                ErrorKind::InternalLogicIssue,
                "failed to fetch version of seaside",
            )),
        }?;
        self.features.syscalls.validate()?;
        self.memory_map.validate()
    }
}
