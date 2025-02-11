//! Wraps the [`seaside_config`] crate.
//!
//! Provides the wrapper function [`get_config`], which finds and parses a configuration file.

use super::resolve_if_exists;
use crate::CmdArgs;
use anyhow::{Error, Result};
use directories::ProjectDirs;
use seaside_config::{Config, Validate};
use seaside_error::EngineError;
use std::{fs::read_to_string, path::PathBuf};

/// Finds and parses a seaside configuration file.
///
/// If the user does not specify an explicit path to the config file, it will search for a file
/// called 'Seaside.toml' in the current working directory or seaside's config directory. The latter
/// depends on the operating system.
pub fn get_config(args: &CmdArgs) -> Result<Config> {
    // The borrow checker requires an explicit binding to the temporary produced by
    // `find_seaside_toml` to make a reference to it.
    let stupid_binding: PathBuf;
    let config_path: &PathBuf = if let Some(path) = &args.config {
        path
    } else {
        stupid_binding = find_seaside_toml()?;
        &stupid_binding
    };

    let file_contents = read_to_string(config_path)?;
    let config: Config = toml::from_str(&file_contents)?;
    config.validate().map(|_| config)
}

/// Tries to find 'Seaside.toml'.
///
/// This first searches the current working directory, but if it cannot find it there, it will move
/// on to the directory designated by the operating system for seaside's configuration files.
fn find_seaside_toml() -> Result<PathBuf, Error> {
    let path = PathBuf::from("Seaside.toml");
    if path.exists() {
        return Ok(path);
    }
    let project_directories = ProjectDirs::from("", "", "seaside").ok_or_else(|| {
        Error::new(EngineError::NotFound).context("couldn't find seaside's project directories")
    })?;
    resolve_if_exists(project_directories.config_dir(), path)
        .ok_or_else(|| Error::new(EngineError::NotFound).context("couldn't find 'Seaside.toml'"))
}
