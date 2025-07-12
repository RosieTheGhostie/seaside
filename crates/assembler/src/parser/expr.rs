use super::{Operand, Value};
use crate::directives::{SegmentDirective, StringDirective, ValueDirective};
use core::fmt::{Display, Formatter, Result as FmtResult};
use seaside_error::rich::Span;

/// A standalone "expression".
///
/// This name doesn't make perfect sense in the context of MIPS Assembly, but it's what I've decided
/// to call it for now.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr<'src> {
    /// A segment directive, optionally followed by an address.
    SegmentHeader {
        /// The directive used to define this segment.
        directive: SegmentDirective,
        /// The memory address to start at.
        ///
        /// If omitted, the assembler will attempt to infer the address to use.
        address: Option<u32>,
    },
    /// An alignment command using the `.align` directive.
    AlignCommand {
        /// The power of two to align the bytes to.
        alignment: u8,
    },
    /// A spacing command using the `.space` directive.
    SpaceCommand {
        /// The number of padding bytes to insert.
        n_bytes: u32,
    },
    /// A C-style include command using the `.include` directive.
    IncludeCommand {
        /// The path of the file to include.
        file_path: &'src str,
    },
    /// A command to make labels visible from other files.
    GlobalCommand {
        /// The labels to make global.
        labels: Vec<(&'src str, Span)>,
    },
    /// A C-style macro using the `.eqv` directive.
    EqvMacro {
        /// The name of the macro to define.
        name: &'src str,
        /// The expression to insert wherever this macro is invoked.
        expr: Box<Expr<'src>>,
    },
    /// A command to dynamically set assembler settings.
    SetCommand {
        /// The command to run.
        command: &'src str,
    },
    /// A series of scalar values, all of which must be the same type.
    ValueArray {
        /// The directive defining the data type to store.
        directive: ValueDirective,
        /// The array of values.
        ///
        /// Their types are only checked during the assembly phase.
        values: Vec<(Value, Span)>,
    },
    /// A string literal preceded by an appropriate directive.
    String {
        /// The directive defining the type of string literal to store.
        directive: StringDirective,
        /// The string literal to store.
        ///
        /// Escape sequences aren't processed until the assembly phase.
        value: &'src str,
    },
    /// A human-readable name for a memory address.
    LabelDef {
        /// The name of this label.
        ident: &'src str,
    },
    /// An assembly instruction that will eventually become machine code.
    Instruction {
        /// The name of the operation to perform.
        operator: &'src str,
        /// A series of [operand](Operand)s to use in the operation.
        operands: Vec<(Operand<'src>, Span)>,
    },
}

impl Display for Expr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::SegmentHeader {
                directive: name,
                address: Some(address),
            } => write!(f, ".{name} {address:#010x}"),
            Self::SegmentHeader {
                directive: name,
                address: None,
            } => write!(f, ".{name}"),
            Self::AlignCommand { alignment } => write!(f, ".align {alignment}"),
            Self::SpaceCommand { n_bytes } => write!(f, ".space {n_bytes}"),
            Self::IncludeCommand { file_path } => write!(f, ".include {file_path:?}"),
            Self::GlobalCommand { labels } => {
                write!(f, ".global ")?;
                let mut iter = labels.iter();
                if let Some((first, _)) = iter.next() {
                    write!(f, "{first}")?;
                    for (label, _) in iter {
                        write!(f, ", {label}")?;
                    }
                }
                Ok(())
            }
            Self::EqvMacro { name, expr } => write!(f, ".eqv {name}, {expr}"),
            Self::SetCommand { command } => write!(f, ".set {command}"),
            Self::ValueArray { directive, values } => {
                write!(f, ".{directive} ")?;
                let mut iter = values.iter();
                if let Some((first, _)) = iter.next() {
                    write!(f, "{first}")?;
                    for (value, _) in iter {
                        write!(f, ", {value}")?;
                    }
                }
                Ok(())
            }
            Self::String { directive, value } => write!(f, ".{directive} \"{value}\""),
            Self::LabelDef { ident } => write!(f, "{ident}:"),
            Self::Instruction { operator, operands } => {
                write!(f, "{operator}")?;
                let mut iter = operands.iter();
                if let Some((first, _)) = iter.next() {
                    write!(f, " {first}")?;
                    for (operand, _) in iter {
                        if matches!(operand, Operand::WrappedRegister(_)) {
                            write!(f, "{operand}")
                        } else {
                            write!(f, ", {operand}")
                        }?;
                    }
                }
                Ok(())
            }
        }
    }
}
