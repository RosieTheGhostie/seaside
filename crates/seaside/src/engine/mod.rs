//! Easy-to-use utilities for driving parts of the seaside engine.
//!
//! Provides wrapper functions for each major crate in seaside. These wrappers set up the necessary
//! state and drive the relevant routines.

pub mod assembler;
pub mod config;
pub mod disassembler;
pub mod interpreter;

pub use assembler::assemble;
pub use config::{find_global_config, get_config};
pub use disassembler::{disassemble_instruction, disassemble_segment};
pub use interpreter::{init_interpreter, run};

mod lazy_project_dirs;

use std::path::{Path, PathBuf};

/// Tries to resolve the relative path `name` from the given `directory`.
///
/// If `ensure_exists` is set, this will check if the file in question actually exists before
/// returning the computed path to it.
fn resolve<P>(directory: &Path, name: P, ensure_exists: bool) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    let path = directory.join(name);
    if ensure_exists {
        path.exists().then_some(path)
    } else {
        Some(path)
    }
}
