use super::{get_file, Error, ErrorKind};
use crate::{
    config::{Config, Validate},
    CmdArgs,
};
use std::{env::current_exe, fs::read_to_string, path::PathBuf};

pub fn get_config(args: &CmdArgs) -> Result<Config, Error> {
    let config_path: &PathBuf;
    let stupid_binding: PathBuf;
    if let Some(path) = &args.config {
        config_path = path;
    } else {
        stupid_binding = find_seaside_toml()?;
        config_path = &stupid_binding;
    }
    let file_contents = read_to_string(config_path)
        .map_err(|_| Error::new(ErrorKind::ExternalFailure, "failed to read config file"))?;
    let config: Config = toml::from_str(&file_contents)
        .map_err(|error| Error::new(ErrorKind::InvalidConfig, error))?;
    config.validate().map(|_| config)
}

fn find_seaside_toml() -> Result<PathBuf, Error> {
    let path = PathBuf::from("Seaside.toml");
    if path.exists() {
        return Ok(path);
    }
    match current_exe()
        .map_err(|_| Error::new(ErrorKind::ExternalFailure, "'std::env::current_exe' failed"))?
        .ancestors()
        .nth(3)
    {
        Some(seaside_root) => get_file(seaside_root, path)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "couldn't find 'Seaside.toml'")),
        None => Err(Error::new(
            ErrorKind::NotFound,
            "couldn't find seaside's root directory",
        )),
    }
}
