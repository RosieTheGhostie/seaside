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
use seaside_rich_error::Span;

/// Assembles `source` into a format usable by the seaside interpreter.
///
/// If `output_directory` is [`None`], it defaults to the current working directory.
pub fn assemble<P>(config: Config, source_path: P, output_path: Option<PathBuf>) -> Result<()>
where
    P: AsRef<Path> + Debug,
{
    let start_time = Instant::now();

    let output_path = output_path.unwrap_or_else(|| source_path.as_ref().with_extension("seax"));
    let source = std::fs::read_to_string(&source_path)?;
    let exprs = parse(&source_path, &source)?;
    match Assembler::new(&config, exprs).build() {
        Ok(build) => {
            let executable = build.export();
            let output_file = std::fs::File::create(output_path)?;
            executable.to_writer(output_file)?;
        }
        Err(err) => {
            let _ = err.report(&source, source_path);
            return Err(Error::new(EngineError::AssemblyFailure));
        }
    }

    let elapsed = start_time.elapsed();
    grayln!("assembled {source_path:?} in {elapsed:#?}");

    Ok(())
}

/// Parses `source` into a sequence of [expressions](Expr).
fn parse<P>(source_path: P, source: &str) -> Result<VecDeque<(Expr<'_>, Span)>>
where
    P: AsRef<Path>,
{
    let mut exprs = VecDeque::new();
    let mut n_errors: usize = 0;
    for expr_or_err in Parser::new(source) {
        match expr_or_err {
            Ok(spanned_expr) => exprs.push_back(spanned_expr),
            Err(err) => {
                n_errors += 1;
                let _ = err.report(source, &source_path);
            }
        }
    }

    if n_errors == 0 {
        Ok(exprs)
    } else {
        Err(Error::new(EngineError::ParsingFailure))
    }
}
