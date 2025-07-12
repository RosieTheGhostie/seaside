pub mod directives;
pub mod error;
pub mod instruction;
pub mod parser;
pub mod segment;
pub mod token;

mod string_builder;

use crate::{
    directives::{SegmentDirective, ValueDirective},
    error::AssembleError,
    instruction::{ProcessedInstruction, UnresolvedInstruction, process_instruction},
    parser::Expr,
    segment::SegmentBuildInfo,
};
use core::iter::zip;
use seaside_config::Config;
use seaside_error::rich::{RichError, RichResult, Span};
use seaside_int_utils::Endian;
use seaside_type_aliases::Address;
use std::{
    collections::{HashMap, VecDeque},
    path::Path,
};

pub struct Assembler<'src> {
    /// A deque of [spanned](Span) [expressions](Expr).
    exprs: VecDeque<(Expr<'src>, Span)>,
    /// The current state of each segment in the build.
    segments: [SegmentBuildInfo; 5],
    /// Which segment is currently being built.
    current_segment: SegmentDirective,
    /// A record of all the labels defined so far.
    symbol_table: HashMap<&'src str, Address>,
    /// Instructions that have yet to be resolved due to having a [label](parser::Operand::Label) as
    /// an [operand](parser::Operand).
    unresolved: VecDeque<(Address, (UnresolvedInstruction<'src>, Span))>,
    /// The target build endianness.
    endian: Endian,
}

impl<'src> Assembler<'src> {
    pub fn new(config: &Config, exprs: VecDeque<(Expr<'src>, Span)>) -> Self {
        let segments = &config.memory_map.segments;

        Self {
            exprs,
            segments: [
                SegmentBuildInfo::new(segments.data.range.base),
                SegmentBuildInfo::new(segments.r#extern.range.base),
                SegmentBuildInfo::new(segments.kdata.range.base),
                SegmentBuildInfo::new(segments.ktext.range.base),
                SegmentBuildInfo::new(segments.text.range.base),
            ],
            current_segment: SegmentDirective::Text,
            unresolved: VecDeque::new(),
            symbol_table: HashMap::new(),
            endian: config.endian,
        }
    }

    pub fn build(mut self) -> RichResult<Build> {
        while self.build_next()? {}
        self.resolve_all()?;
        Ok(Build::new(self.segments))
    }

    fn this_segment(&self) -> &SegmentBuildInfo {
        &self.segments[self.current_segment as usize]
    }

    fn this_segment_mut(&mut self) -> &mut SegmentBuildInfo {
        &mut self.segments[self.current_segment as usize]
    }

    fn next_address(&self) -> Address {
        self.this_segment().next
    }

    const INSTRUCTION_IN_DATA_SEGMENT: &'static str = "instructions only allowed in text segments";

    fn build_next(&mut self) -> RichResult<bool> {
        let (expr, span) = match self.exprs.pop_front() {
            Some(spanned_expr) => spanned_expr,
            None => return Ok(false),
        };
        match expr {
            Expr::SegmentHeader { directive, address } => {
                self.current_segment = directive;
                if let Some(address) = address {
                    self.this_segment_mut().jump_ahead_to(span, address)?;
                }
            }
            Expr::AlignCommand { alignment } => {
                if self.current_segment.is_data_segment() {
                    self.this_segment_mut().align(alignment);
                } else {
                    return Err(RichError::new(AssembleError::WrongSegment, span)
                        .with_note(".align only supported in data segments"));
                }
            }
            Expr::SpaceCommand { n_bytes } => {
                if self.current_segment.is_data_segment() {
                    self.this_segment_mut().jump_ahead_by(n_bytes);
                } else {
                    return Err(RichError::new(AssembleError::WrongSegment, span)
                        .with_note(".space only supported in data segments"));
                }
            }
            Expr::IncludeCommand { file_path: _ } => {
                return Err(RichError::new(AssembleError::UnsupportedDirective, span)
                    .with_note("support for .include not yet planned"));
            }
            Expr::EqvMacro { name: _, expr: _ } => {
                return Err(RichError::new(AssembleError::UnsupportedDirective, span)
                    .with_note("support for .eqv planned for seaside v1.4.0"));
            }
            Expr::SetCommand { command: _ } => {
                return Err(RichError::new(AssembleError::UnsupportedDirective, span)
                    .with_note("support for .set planned for seaside v1.4.0"));
            }
            Expr::ValueArray { directive, values } => {
                if !self.current_segment.is_data_segment() {
                    return Err(RichError::new(AssembleError::WrongSegment, span)
                        .with_note("value arrays only supported in data segments"));
                }
                let endian = self.endian;
                let this_segment = self.this_segment_mut();
                match directive {
                    ValueDirective::Byte => this_segment.append_i8(span, values),
                    ValueDirective::Half => this_segment.append_i16(span, values, endian),
                    ValueDirective::Word => this_segment.append_i32(span, values, endian),
                    ValueDirective::Float => this_segment.append_f32(span, values, endian),
                    ValueDirective::Double => this_segment.append_f64(span, values, endian),
                }?
            }
            Expr::String { directive, value } => {
                if !self.current_segment.is_data_segment() {
                    return Err(RichError::new(AssembleError::WrongSegment, span)
                        .with_note("strings only supported in data segments"));
                }
                self.this_segment_mut()
                    .build_string(directive, value, span)?;
            }
            Expr::LabelDef { ident } => self.add_symbol(span, ident)?,
            Expr::Instruction { operator, operands } => {
                if !self.current_segment.is_text_segment() {
                    return Err(RichError::new(AssembleError::WrongSegment, span)
                        .with_note(Self::INSTRUCTION_IN_DATA_SEGMENT));
                }
                let pc = self.next_address();
                let mut bytes = match process_instruction(operator, operands, &span, pc)? {
                    ProcessedInstruction::MachineCode(machine_code) => match self.endian {
                        Endian::Little => machine_code.to_le_bytes(),
                        Endian::Big => machine_code.to_be_bytes(),
                    },
                    ProcessedInstruction::Unresolved(unresolved) => {
                        self.unresolved.push_back((pc, (unresolved, span)));
                        self.this_segment_mut().jump_ahead_by(4);
                        return Ok(true);
                    }
                }
                .to_vec();
                self.this_segment_mut().append(&mut bytes);
            }
        }
        Ok(true)
    }

    fn resolve_all(&mut self) -> RichResult<()> {
        for (pc, (unresolved, span)) in self.unresolved.drain(..) {
            let (label, label_span) = unresolved.spanned_label();
            let (address, machine_code) = match self.symbol_table.get(label) {
                Some(&address) => (address, unresolved.resolve(&span, address, pc)?),
                None => {
                    return Err(RichError::new(AssembleError::UndefinedSymbol, span)
                        .with_narrow_span(label_span.clone()));
                }
            };
            let text_diff =
                address.checked_sub(self.segments[SegmentDirective::Text as usize].base);
            let ktext_diff =
                address.checked_sub(self.segments[SegmentDirective::KText as usize].base);
            let segment = match (text_diff, ktext_diff) {
                (Some(text_diff), Some(ktext_diff)) => {
                    if text_diff < ktext_diff {
                        SegmentDirective::Text
                    } else {
                        SegmentDirective::KText
                    }
                }
                (Some(_), None) => SegmentDirective::Text,
                (None, Some(_)) => SegmentDirective::KText,
                (None, None) => {
                    return Err(RichError::new(AssembleError::WrongSegment, span)
                        .with_note(Self::INSTRUCTION_IN_DATA_SEGMENT));
                }
            };
            self.segments[segment as usize].overwrite_u32(pc, machine_code, self.endian);
        }
        Ok(())
    }

    fn add_symbol(&mut self, expr_span: Span, label: &'src str) -> RichResult<()> {
        if self
            .symbol_table
            .insert(label, self.next_address())
            .is_none()
        {
            Ok(())
        } else {
            Err(RichError::new(
                AssembleError::MultipleDefinitions,
                expr_span,
            ))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Build {
    segments: [SegmentBuildInfo; 5],
}

impl Build {
    pub const fn new(segments: [SegmentBuildInfo; 5]) -> Self {
        Self { segments }
    }

    pub fn export(self, directory: &Path) -> std::io::Result<()> {
        for (segment, name) in
            zip(self.segments, SegmentDirective::names()).filter(|(segment, _)| !segment.is_empty())
        {
            segment.export(directory.join(name))?;
        }
        Ok(())
    }
}
