#![allow(dead_code)]
mod bitflags_addons;
pub mod features;
pub mod memory_map;
pub mod presets;
pub mod red_flag_behavior;
pub mod registers;

use features::Features;
use memory_map::MemoryMap;
use registers::RegisterDefaults;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub features: Features,
    pub memory_map: MemoryMap,
    pub register_defaults: RegisterDefaults,
}
