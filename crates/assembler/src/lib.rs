pub mod directives;
pub mod error;
pub mod instruction;
pub mod parser;
pub mod token;

mod build;
mod options;
mod segments;
mod string_builder;

use std::collections::{HashMap, VecDeque};

use seaside_config::Config;
use seaside_core::{Endian, consts::StaticSegment, instruction::Packable, prelude::*};
use seaside_rich_error::{RichError, RichResult, RichResultBuilder, Span, map_err, result::Bailed};

use build::Build;
use directives::ValueDirective;
use error::AssembleError;
use instruction::{ProcessedInstruction, UnresolvedInstruction, process_instruction};
use options::OptionsStack;
use parser::Expr;
use segments::{SegmentBuildInfo, Segments};

use crate::{
    directives::{SegmentDirective, StringDirective},
    parser::{Operand, Value},
};

pub struct Assembler<'src, 'config> {
    /// A deque of [spanned](Span) [expressions](Expr).
    exprs: VecDeque<(Expr<'src>, Span)>,

    /// The current state of each segment in the build.
    segments: Segments,

    /// Which segment is currently being built.
    current_segment: StaticSegment,

    /// A record of all the labels defined so far.
    symbol_table: HashMap<&'src str, Address>,

    /// Instructions that have yet to be resolved due to having a [label](parser::Operand::Label) as
    /// an [operand](parser::Operand).
    unresolved: VecDeque<(Address, (UnresolvedInstruction<'src>, Span))>,

    result_builder: RichResultBuilder,
    options: OptionsStack,

    config: &'config Config,
}

impl<'src, 'config> Assembler<'src, 'config> {
    pub fn new(config: &'config Config, exprs: VecDeque<(Expr<'src>, Span)>) -> Self {
        Self {
            exprs,
            segments: Segments::from_memory_map_segments(&config.memory_map.segments),
            current_segment: StaticSegment::Text,
            unresolved: VecDeque::new(),
            symbol_table: HashMap::new(),
            result_builder: RichResultBuilder::new(),
            options: OptionsStack::new(),
            config,
        }
    }

    pub fn build(mut self) -> RichResult<Build<'config>> {
        while matches!(self.build_next(), Ok(true) | Err(Bailed)) {}
        self.resolve_all();

        self.result_builder
            .finish_with(|| Build::new(self.segments, self.config))
    }

    const fn this_segment(&self) -> &SegmentBuildInfo {
        self.segments.get(self.current_segment)
    }

    const fn this_segment_mut(&mut self) -> &mut SegmentBuildInfo {
        self.segments.get_mut(self.current_segment)
    }

    const fn next_address(&self) -> Address {
        self.this_segment().next
    }

    const INSTRUCTION_IN_DATA_SEGMENT: &'static str = "instructions only allowed in text segments";

    fn build_next(&mut self) -> Result<bool, Bailed> {
        let Some((expr, span)) = self.exprs.pop_front() else {
            return Ok(false);
        };

        match expr {
            Expr::SegmentHeader { directive, address } => {
                self.build_segment_header(span, directive, address)
            }
            Expr::AlignCommand { alignment } => self.build_align_command(span, alignment),
            Expr::SpaceCommand { n_bytes } => self.build_space_command(span, n_bytes),
            Expr::IncludeCommand { file_path } => self.build_include_command(span, file_path),
            Expr::GlobalCommand { labels } => self.build_global_command(span, labels),
            Expr::EqvMacro { name, expr } => self.build_eqv_macro(span, name, expr),
            Expr::SetCommand { command } => self.build_set_command(span, command),
            Expr::ValueArray { directive, values } => {
                self.build_value_array(span, directive, values)
            }
            Expr::String { directive, value } => self.build_string(span, directive, value),
            Expr::LabelDef { ident } => self.add_symbol(span, ident),
            Expr::Instruction { operator, operands } => {
                self.build_instruction(span, operator, operands)
            }
        }
        .map(|()| true)
    }

    fn build_segment_header(
        &mut self,
        span: Span,
        directive: SegmentDirective,
        address: Option<Address>,
    ) -> Result<(), Bailed> {
        self.current_segment = directive;
        if let Some(address) = address {
            self.this_segment_mut()
                .jump_ahead_to(span, address)
                .map_err(map_err!(err -> self.result_builder))
        } else {
            Ok(())
        }
    }

    fn build_align_command(&mut self, span: Span, alignment: u8) -> Result<(), Bailed> {
        if !self.current_segment.is_data_segment() {
            return self.result_builder.bail(
                RichError::new(AssembleError::WrongSegment, span)
                    .with_note(".align only supported in data segments"),
            );
        }

        self.this_segment_mut().align(alignment);
        Ok(())
    }

    fn build_space_command(&mut self, span: Span, n_bytes: Size) -> Result<(), Bailed> {
        if !self.current_segment.is_data_segment() {
            return self.result_builder.bail(
                RichError::new(AssembleError::WrongSegment, span)
                    .with_note(".space only supported in data segments"),
            );
        }

        self.this_segment_mut().jump_ahead_by(n_bytes);
        Ok(())
    }

    fn build_include_command(&mut self, span: Span, _file_path: &'src str) -> Result<(), Bailed> {
        self.result_builder.bail(
            RichError::new(AssembleError::UnsupportedDirective, span)
                .with_note("multiple file support not yet planned"),
        )
    }

    fn build_global_command(
        &mut self,
        span: Span,
        _labels: Vec<(&'src str, Span)>,
    ) -> Result<(), Bailed> {
        self.result_builder.bail(
            RichError::new(AssembleError::UnsupportedDirective, span)
                .with_note("multiple file support not yet planned"),
        )
    }

    fn build_eqv_macro(
        &mut self,
        span: Span,
        _name: &'src str,
        _expr: Box<Expr<'src>>,
    ) -> Result<(), Bailed> {
        self.result_builder.bail(
            RichError::new(AssembleError::UnsupportedDirective, span)
                .with_note("support for .eqv planned for seaside v2.0.0"),
        )
    }

    fn build_set_command(&mut self, span: Span, command: &'src str) -> Result<(), Bailed> {
        match command {
            "push" => self.options.push(),
            "pop" => self.options.pop(),
            "at" => self.options.active.explicit_asm_temp = false,
            "noat" => self.options.active.explicit_asm_temp = true,
            _ => self
                .result_builder
                .bail(RichError::new_warning(AssembleError::ExplicitAsmTemp, span))?,
        }

        Ok(())
    }

    fn build_value_array(
        &mut self,
        span: Span,
        directive: ValueDirective,
        values: Vec<(Value, Span)>,
    ) -> Result<(), Bailed> {
        if !self.current_segment.is_data_segment() {
            return self.result_builder.bail(
                RichError::new(AssembleError::WrongSegment, span)
                    .with_note("value arrays only supported in data segments"),
            );
        }

        let endian = self.config.endian;
        let this_segment = self.this_segment_mut();
        match directive {
            ValueDirective::Byte => this_segment.append_i8(span, values),
            ValueDirective::Half => this_segment.append_i16(span, values, endian),
            ValueDirective::Word => this_segment.append_i32(span, values, endian),
            ValueDirective::Float => this_segment.append_f32(span, values, endian),
            ValueDirective::Double => this_segment.append_f64(span, values, endian),
        }
        .map_err(map_err!(err -> self.result_builder))
    }

    fn build_string(
        &mut self,
        span: Span,
        directive: StringDirective,
        value: &'src str,
    ) -> Result<(), Bailed> {
        if !self.current_segment.is_data_segment() {
            return self.result_builder.bail(
                RichError::new(AssembleError::WrongSegment, span)
                    .with_note("strings only supported in data segments"),
            );
        }

        self.this_segment_mut()
            .build_string(directive, value, span)
            .map_err(map_err!(err -> self.result_builder))
    }

    fn build_instruction(
        &mut self,
        span: Span,
        operator: &'src str,
        operands: Vec<(Operand<'src>, Span)>,
    ) -> Result<(), Bailed> {
        if !self.current_segment.is_text_segment() {
            return self.result_builder.bail(
                RichError::new(AssembleError::WrongSegment, span)
                    .with_note(Self::INSTRUCTION_IN_DATA_SEGMENT),
            );
        }

        let pc = self.next_address();
        match process_instruction(
            &mut self.result_builder,
            &self.options.active,
            operator,
            operands,
            &span,
            pc,
        )? {
            ProcessedInstruction::Resolved(instruction) => {
                let machine_code = instruction.pack();
                let bytes = match self.config.endian {
                    Endian::Little => machine_code.to_le_bytes(),
                    Endian::Big => machine_code.to_be_bytes(),
                };
                self.this_segment_mut().append(&mut bytes.to_vec());
            }
            ProcessedInstruction::Unresolved(unresolved) => {
                self.unresolved.push_back((pc, (unresolved, span)));
                self.this_segment_mut().jump_ahead_by(4);
            }
        }

        Ok(())
    }

    fn resolve_all(&mut self) {
        for (pc, (unresolved, span)) in self.unresolved.drain(..) {
            let (label, label_span) = unresolved.spanned_label();
            let Some(&address) = self.symbol_table.get(label) else {
                self.result_builder.add_error(
                    RichError::new(AssembleError::UndefinedSymbol, span)
                        .with_narrow_span(label_span.clone()),
                );
                continue;
            };
            let Ok(instruction) = unresolved.resolve(&mut self.result_builder, &span, address, pc)
            else {
                continue;
            };

            let text_diff = address.checked_sub(self.segments.text.base);
            let ktext_diff = address.checked_sub(self.segments.ktext.base);
            let segment = match (text_diff, ktext_diff) {
                (Some(text_diff), Some(ktext_diff)) => {
                    if text_diff < ktext_diff {
                        StaticSegment::Text
                    } else {
                        StaticSegment::KText
                    }
                }
                (Some(_), None) => StaticSegment::Text,
                (None, Some(_)) => StaticSegment::KText,
                (None, None) => {
                    self.result_builder.add_error(
                        RichError::new(AssembleError::WrongSegment, span)
                            .with_note(Self::INSTRUCTION_IN_DATA_SEGMENT),
                    );
                    continue;
                }
            };
            self.segments.get_mut(segment).overwrite_u32(
                pc,
                instruction.pack(),
                self.config.endian,
            );
        }
    }

    fn add_symbol(&mut self, expr_span: Span, label: &'src str) -> Result<(), Bailed> {
        if self
            .symbol_table
            .insert(label, self.next_address())
            .is_none()
        {
            Ok(())
        } else {
            self.result_builder.bail(RichError::new(
                AssembleError::MultipleDefinitions,
                expr_span,
            ))
        }
    }
}
