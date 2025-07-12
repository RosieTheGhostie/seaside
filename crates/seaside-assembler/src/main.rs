/* mod directives;
mod error;
mod expr;
mod parser;
mod segment;
mod string_builder;
mod token;

use assembler::Assembler;
use logos::Logos;
use parser::Parser;
use seaside_int_utils::Endian;
use std::{collections::VecDeque, path::Path};
use token::Token;

// const SRC: &str = include_str!("D:/rose/04-temp/assembly-tests/perf/main.asm");
// const SRC_NAME: &str = "perf/main.asm";
const SRC: &str = include_str!("../res/sample.asm");
const SRC_NAME: &str = "res/sample.asm";
const BUILD_DIR: &str = "res/out";

fn main() -> std::io::Result<()> {
    let mut exprs = VecDeque::new();
    let mut n_errors: usize = 0;
    for expr_or_err in Parser::new(Token::lexer(SRC)) {
        match expr_or_err {
            Ok(spanned_expr) => exprs.push_back(spanned_expr),
            Err(err) => {
                n_errors += 1;
                let _ = err.report(SRC, SRC_NAME);
            }
        }
    }
    if n_errors != 0 {
        return Err(std::io::Error::other(format!(
            "failed with {n_errors} error(s)"
        )));
    }
    match Assembler::_new(
        seaside_config::memory_map::Segments::default(),
        Endian::Little,
        exprs,
    )
    .build()
    {
        Ok(build) => build
            .export(Path::new(BUILD_DIR))
            .inspect_err(|err| eprintln!("{err}")),
        Err(err) => err.report(SRC, SRC_NAME),
    }
}
 */

fn main() {}
