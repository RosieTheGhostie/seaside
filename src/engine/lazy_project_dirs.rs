use std::{path::Path, sync::OnceLock};

use anyhow::{Error, Result};
use directories::ProjectDirs;
use seaside_error::EngineError;

/// A wrapper around [`ProjectDirs`] that only computes the project directories once.
pub struct LazyProjectDirs(OnceLock<ProjectDirs>);

impl LazyProjectDirs {
    /// Creates a new wrapper around an uninitialized [`ProjectDirs`] instance.
    pub const fn new() -> Self {
        Self(OnceLock::new())
    }

    /// Gets the project directories if they have already been computed.
    pub fn get(&self) -> Option<&ProjectDirs> {
        self.0.get()
    }

    /// Gets the project directories if they have already been computed, or attempts to compute them
    /// if they haven't been.
    ///
    /// # Errors
    ///
    /// This operation will fail if and only if [`ProjectDirs::from`] returns [`None`]. As of
    /// writing this, that will only happen if it is unable to locate the "home" directory.
    ///
    /// If the project directories have already been computed, this will never fail.
    pub fn get_or_init(&self) -> Result<&ProjectDirs> {
        if let Some(project_dirs) = self.get() {
            Ok(project_dirs)
        } else {
            let project_dirs = ProjectDirs::from("", "", "seaside").ok_or_else(|| {
                Error::new(EngineError::NotFound)
                    .context("couldn't find seaside's project directories")
            })?;

            // Since there can be at most one mutable reference to `self` at a time, there is no way
            // for the contents of the `OnceLock` to change between getting `None` and calling
            // `OnceLock::set`. Thus, we don't need to care about the result.
            let _ = self.0.set(project_dirs);

            // SAFETY: `OnceLock::set` guarantees that the contents will be initialized by the time
            // it returns.
            Ok(unsafe { self.get().unwrap_unchecked() })
        }
    }

    /// Gets the path to the project's configuration directory.
    ///
    /// # Errors
    ///
    /// This has the same error semantics as [`get_or_init`](Self::get_or_init).
    pub fn config_dir(&self) -> Result<&Path> {
        self.get_or_init().map(ProjectDirs::config_dir)
    }
}

pub static PROJECT_DIRS: LazyProjectDirs = LazyProjectDirs::new();
