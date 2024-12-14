#![allow(dead_code)]
mod bitflags_addons;
pub mod endian;
pub mod features;
pub mod memory_map;
mod presets;
mod red_flag_behavior;
pub mod register_defaults;
mod version;

pub use endian::Endian;
pub use features::Features;
pub use memory_map::MemoryMap;
pub use register_defaults::RegisterDefaults;
use semver::Version;
use serde::{Deserialize, Serialize};

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
