//! Wraps the [`seaside_config`] crate.
//!
//! Provides the following wrapper functions:
//! - [`get_config`]: Finds and parses a configuration file.
//! - [`find_global_config`]: Finds the global configuration file.

use std::path::PathBuf;

use anyhow::{Context, Error, Result};
use seaside_config::Config;
use seaside_error::EngineError;

use super::{lazy_project_dirs::PROJECT_DIRS, resolve};
use crate::CmdArgs;

/// Tries to find and parse a seaside configuration file.
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

    let config: Config = toml::from_str(&std::fs::read_to_string(config_path)?)?;
    config.validate().map(|_| config)
}

/// Tries to find the global seaside configuration file.
pub fn find_global_config(ensure_exists: bool) -> Result<PathBuf> {
    _find_global_config(None, ensure_exists)
}

/// The standard name for a seaside configuration file.
const SEASIDE_TOML: &str = "Seaside.toml";

/// Tries to find 'Seaside.toml'.
///
/// This first searches the current working directory, but if it cannot find it there, it will move
/// on to the directory designated by the operating system for seaside's configuration files.
fn find_seaside_toml() -> Result<PathBuf> {
    let path = PathBuf::from(SEASIDE_TOML);
    if path.exists() {
        Ok(path)
    } else {
        _find_global_config(Some(path), true)
    }
}

/// Tries to find the global seaside configuration file.
///
/// The driving force making it worth splitting this implementation into its own function is being
/// able to reuse a 'Seaside.toml' [`PathBuf`] if the caller already happens to have one. Such is
/// the case in [`find_seaside_toml`].
fn _find_global_config(seaside_toml: Option<PathBuf>, ensure_exists: bool) -> Result<PathBuf> {
    let seaside_toml = seaside_toml.unwrap_or_else(|| PathBuf::from(SEASIDE_TOML));
    resolve(PROJECT_DIRS.config_dir()?, seaside_toml, ensure_exists)
        .ok_or_else(|| Error::new(EngineError::NotFound))
        .with_context(|| format!("couldn't find '{SEASIDE_TOML}'"))
}
