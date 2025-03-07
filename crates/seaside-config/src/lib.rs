pub mod features;
pub mod memory_map;
pub mod properties;
pub mod register_defaults;
pub mod validate;

mod binary;
mod version;

pub use binary::{EditFromBinary, FromBinary, ToBinary};
pub use features::Features;
pub use memory_map::MemoryMap;
pub use register_defaults::RegisterDefaults;
pub use validate::Validate;

use anyhow::{anyhow, Context, Error, Result};
use clap::crate_version;
use seaside_error::EngineError;
use seaside_int_utils::{AllZeroes, Endian};
use semver::Version;
use std::{
    io::{Read, Write},
    str::FromStr,
};
use version::VersionComparison;

pub struct Config {
    pub version: Version,
    pub endian: Endian,
    pub project_directory_is_cwd: bool,
    pub features: Features,
    pub memory_map: MemoryMap,
    pub register_defaults: RegisterDefaults,
}

impl Validate for Config {
    fn validate(&self) -> Result<()> {
        use VersionComparison::*;
        let seaside_version = Version::from_str(crate_version!())?;
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
        self.features.syscalls.validate()?;
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

impl FromBinary for Config {
    fn from_binary<R: Read>(stream: &mut R) -> Result<Self> {
        let mut magic = [0u8; 8];
        stream.read_exact(&mut magic)?;
        if &magic != b"seaside\0" {
            return Err(anyhow!("incorrect magic value (expected 'seaside\\0')"));
        }

        let mut version_buffer = [0u8; 4];
        stream.read_exact(&mut version_buffer)?;
        match u32::from_le_bytes(version_buffer) {
            1 => Self::from_binary_v1(stream),
            version => Err(anyhow!("unsupported config version: {version}")),
        }
    }
}

impl EditFromBinary<1> for Config {
    fn edit_from_binary<R: Read>(&mut self, ids: [u8; 4], stream: &mut R) -> Result<()> {
        use crate::properties::{
            features, memory_map, register_defaults, ENDIAN, PROJECT_DIRECTORY_IS_CWD, VERSION,
        };

        match (ids[0], ids[3]) {
            (0x00, VERSION) => self.version = Version::from_binary(stream)?,
            (0x00, ENDIAN) => self.endian = Endian::from_binary(stream)?,
            (0x00, PROJECT_DIRECTORY_IS_CWD) => {
                self.project_directory_is_cwd = bool::from_binary(stream)?
            }
            (features::ID, _) => {
                <Features as EditFromBinary<1>>::edit_from_binary(&mut self.features, ids, stream)?
            }
            (memory_map::ID, _) => <MemoryMap as EditFromBinary<1>>::edit_from_binary(
                &mut self.memory_map,
                ids,
                stream,
            )?,
            (register_defaults::ID, _) => {
                <RegisterDefaults as EditFromBinary<1>>::edit_from_binary(
                    &mut self.register_defaults,
                    ids,
                    stream,
                )?
            }
            _ => return Err(anyhow!("unknown property id: {}", u32::from_be_bytes(ids))),
        }
        Ok(())
    }
}

impl ToBinary<1> for Config {
    fn to_binary<W: Write>(&self, stream: &mut W) -> Result<()> {
        stream.write(&[
            b's', b'e', b'a', b's', b'i', b'd', b'e', 0, // magic
            1, 0, 0, 0, // version
        ])?;
        stream.write(&prefixed!(_[VERSION]).to_le_bytes())?;
        self.version.to_binary(stream)?;
        stream.write(&prefixed!(_[ENDIAN]).to_le_bytes())?;
        self.endian.to_binary(stream)?;
        stream.write(&prefixed!(_[PROJECT_DIRECTORY_IS_CWD]).to_le_bytes())?;
        self.project_directory_is_cwd.to_binary(stream)?;
        self.features.to_binary(stream)?;
        self.memory_map.to_binary(stream)?;
        self.register_defaults.to_binary(stream)
    }
}

impl Config {
    fn from_binary_v1<R: Read>(stream: &mut R) -> Result<Self> {
        let mut config = Config::all_zeroes();
        let mut id_buffer = [0u8; 4];

        loop {
            if let Err(error) = stream.read_exact(&mut id_buffer) {
                return if error.kind() == std::io::ErrorKind::UnexpectedEof {
                    Ok(config)
                } else {
                    Err(error.into())
                };
            }
            <Self as EditFromBinary<1>>::edit_from_binary(
                &mut config,
                [id_buffer[3], id_buffer[2], id_buffer[1], id_buffer[0]],
                stream,
            )?;
        }
    }
}
