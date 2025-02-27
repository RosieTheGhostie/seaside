use crate::SeasideVersion;
use seaside_int_utils::Endian;

#[derive(Clone, Debug)]
pub struct EditableConfig {
    pub version: SeasideVersion,
    pub endian: Endian,
    pub project_directory_is_cwd: bool,
}

impl Default for EditableConfig {
    fn default() -> Self {
        Self {
            version: SeasideVersion::default(),
            endian: Endian::default(),
            project_directory_is_cwd: true,
        }
    }
}
