#![allow(dead_code)]
mod bitflags_addons;
pub mod features;
pub mod memory_map;
mod presets;
mod red_flag_behavior;
pub mod register_defaults;

pub use features::Features;
pub use memory_map::MemoryMap;
pub use register_defaults::RegisterDefaults;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub features: Features,
    pub memory_map: MemoryMap,
    pub register_defaults: RegisterDefaults,
}
