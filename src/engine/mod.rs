//! Easy-to-use utilities for driving parts of the seaside engine.
//!
//! Provides wrapper functions for each major module in seaside. These wrappers set up the necessary
//! state and drive the relevant routines.

pub mod assembler;
pub mod config;
pub mod disassembler;
pub mod error;
pub mod interpreter;

pub use assembler::assemble;
pub use config::get_config;
pub use disassembler::{disassemble_instruction, disassemble_segment};
pub use error::{Error, ErrorKind};
pub use interpreter::{init_interpreter, run};

use std::path::{Path, PathBuf};

/// Tries to resolve the relative path `name` from the given `directory`.
///
/// This essentially amounts to appending `name` to `directory`, then checking if that file exists.
/// If it does, the previously-computed file path is returned.
fn resolve_if_exists<P>(directory: &Path, name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    let path = directory.join(name);
    if path.exists() {
        Some(path)
    } else {
        None
    }
}
