//! Wraps the [`seaside_assembler`] crate.
//!
//! Provides the wrapper function [`assemble`], which runs the assembler routine.

use core::fmt::Debug;
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::{Error, Result};
use minimal_logging::macros::grayln;
use seaside_assembler::{
    Assembler,
    parser::{Expr, Parser},
};
use seaside_config::Config;
use seaside_core::EngineError;
use seaside_rich_error::{RichError, Span};

/// Assembles `source` into a format usable by the seaside interpreter.
///
/// If `output_path` is [`None`], it defaults to the source path with the extension changed to
/// "seax".
pub fn assemble<P>(config: Config, source_path: P, output_path: Option<PathBuf>) -> Result<()>
where
    P: AsRef<Path> + Debug,
{
    let start_time = Instant::now();

    let source_path = source_path.as_ref();
    let output_path = output_path.unwrap_or_else(|| source_path.with_extension("seax"));
    let source = std::fs::read_to_string(source_path)?;
    let exprs = parse(source_path, &source)?;
    match Assembler::new(&config, exprs).build() {
        Ok((build, warnings)) => {
            report_all(warnings, source_path, &source);
            let executable = build.export();
            let output_file = std::fs::File::create(output_path)?;
            executable.to_writer(output_file)?;
        }
        Err(errors) => {
            report_all(errors, source_path, &source);
            return Err(Error::new(EngineError::AssemblyFailure));
        }
    }

    let elapsed = start_time.elapsed();
    grayln!("assembled {source_path:?} in {elapsed:#?}");

    Ok(())
}

/// Parses `source` into a sequence of [expressions](Expr).
fn parse<'src>(source_path: &Path, source: &'src str) -> Result<VecDeque<(Expr<'src>, Span)>> {
    match Parser::new(source).parse_all() {
        Ok((exprs, warnings)) => {
            report_all(warnings, source_path, source);
            Ok(exprs)
        }
        Err(errors) => {
            report_all(errors, source_path, source);
            Err(Error::new(EngineError::ParsingFailure))
        }
    }
}

fn report_all(errors: impl IntoIterator<Item = RichError>, source_path: &Path, source: &str) {
    for err in errors.into_iter() {
        let _ = err.report(source, source_path);
    }
}
