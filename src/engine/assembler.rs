//! Wraps the [`seaside_assembler`] crate.
//!
//! Provides the wrapper function [`assemble`], which runs the assembler routine.

use anyhow::{Error, Result};
use core::str::FromStr;
use minimal_logging::macros::grayln;
use seaside_assembler::{
    Assembler,
    parser::{Expr, Parser},
};
use seaside_config::Config;
use seaside_error::{EngineError, rich::Span};
use std::{
    collections::VecDeque,
    fs::read_to_string,
    path::{Path, PathBuf},
    time::Instant,
};

/// Assembles `source` into a format usable by the seaside interpreter.
///
/// If `output_directory` is [`None`], it defaults to the current working directory.
pub fn assemble<P>(config: Config, source_path: P, output_directory: Option<PathBuf>) -> Result<()>
where
    P: AsRef<Path> + core::fmt::Debug,
{
    let start_time = Instant::now();
    let output_directory = output_directory.unwrap_or_else(|| PathBuf::from_str(".").unwrap());
    let source = read_to_string(&source_path)?;
    let exprs = parse(&source_path, &source)?;
    match Assembler::new(&config, exprs).build() {
        Ok(build) => build.export(&output_directory)?,
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
