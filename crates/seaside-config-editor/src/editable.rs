use crate::SeasideVersion;
use anyhow::Error;
use cursive::Cursive;
use seaside_config::{Config as SeasideConfig, Features, MemoryMap, RegisterDefaults};
use seaside_int_utils::Endian;

pub trait Editable<const S: char>: Clone + Default {
    fn menu(siv: &mut Cursive);
}

#[derive(Clone, Debug)]
pub struct Config {
    pub version: SeasideVersion,
    pub endian: Endian,
    pub project_directory_is_cwd: bool,
    pub features: Features,
    pub memory_map: MemoryMap,
    pub register_defaults: RegisterDefaults,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: SeasideVersion::default(),
            endian: Endian::default(),
            project_directory_is_cwd: true,
            features: Features::default(),
            memory_map: MemoryMap::default(),
            register_defaults: RegisterDefaults::default(),
        }
    }
}

impl TryFrom<SeasideConfig> for Config {
    type Error = Error;

    fn try_from(value: SeasideConfig) -> Result<Self, Self::Error> {
        Ok(Config {
            version: value.version.try_into()?,
            endian: value.endian,
            project_directory_is_cwd: value.project_directory_is_cwd,
            features: value.features,
            memory_map: value.memory_map,
            register_defaults: value.register_defaults,
        })
    }
}

impl From<Config> for SeasideConfig {
    fn from(value: Config) -> Self {
        SeasideConfig {
            version: value.version.into(),
            endian: value.endian,
            project_directory_is_cwd: value.project_directory_is_cwd,
            features: value.features,
            memory_map: value.memory_map,
            register_defaults: value.register_defaults,
        }
    }
}
