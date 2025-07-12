//! A set of string literals used to represent what any given parser might expect from the token
//! stream.
//!
//! These strings can be used on their own, but it's often necessary to combine them. I chose to use
//! the [`formatcp`](const_format::formatcp) macro from the [`const_format`] crate for this, as it
//! ensures I can't mess up and introduce inconsistencies while allowing me to continue using
//! static [string slices](str) instead of [`String`]s created by the builtin [`format`] macro.

#![allow(dead_code, reason = "these may or may not come up later")]

pub const NEWLINE: &str = r"'\n'";
pub const COMMA: &str = "','";
pub const COLON: &str = "':'";
pub const L_PAREN: &str = "'('";
pub const R_PAREN: &str = "')'";

pub const INT_LIT: &str = "integer";
pub const FLOAT_LIT: &str = "float";
pub const STRING_LIT: &str = "string literal";

pub const REGISTER: &str = "register";
pub const DIRECTIVE: &str = "directive";
pub const IDENT: &str = "identifier";

pub const EXPR: &str = "expression";
pub const COMMAND: &str = "command";
pub const OPERAND: &str = "operand";
pub const WRAPPED_REGISTER: &str = "wrapped register";
