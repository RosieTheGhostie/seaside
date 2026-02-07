pub mod directives;
pub mod error;
pub mod instruction;
pub mod parser;
pub mod token;

mod segments;
mod string_builder;

use std::collections::{HashMap, VecDeque};

use seaside_config::{Config, features::AssemblerOptions};
use seaside_constants::{Services, StaticSegment};
use seaside_error::rich::{RichError, RichResult, Span};
use seaside_executable::{Executable, MemoryMap};
use seaside_int_utils::Endian;
use seaside_type_aliases::Address;

use directives::ValueDirective;
use error::AssembleError;
use instruction::{ProcessedInstruction, UnresolvedInstruction, process_instruction};
use parser::Expr;
use segments::{SegmentBuildInfo, Segments};

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
            config,
        }
    }

    pub fn build(mut self) -> RichResult<Build<'config>> {
        while self.build_next()? {}
        self.resolve_all()?;
        Ok(Build::new(self.segments, self.config))
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

    fn build_next(&mut self) -> RichResult<bool> {
        let Some((expr, span)) = self.exprs.pop_front() else {
            return Ok(false);
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
            Expr::IncludeCommand { .. } | Expr::GlobalCommand { .. } => {
                return Err(RichError::new(AssembleError::UnsupportedDirective, span)
                    .with_note("multiple file support not yet planned"));
            }
            Expr::EqvMacro { .. } => {
                return Err(RichError::new(AssembleError::UnsupportedDirective, span)
                    .with_note("support for .eqv planned for seaside v1.4.0"));
            }
            Expr::SetCommand { .. } => {
                return Err(RichError::new(AssembleError::UnsupportedDirective, span)
                    .with_note("support for .set planned for seaside v1.4.0"));
            }
            Expr::ValueArray { directive, values } => {
                if !self.current_segment.is_data_segment() {
                    return Err(RichError::new(AssembleError::WrongSegment, span)
                        .with_note("value arrays only supported in data segments"));
                }

                let endian = self.config.endian;
                let this_segment = self.this_segment_mut();
                match directive {
                    ValueDirective::Byte => this_segment.append_i8(span, values),
                    ValueDirective::Half => this_segment.append_i16(span, values, endian),
                    ValueDirective::Word => this_segment.append_i32(span, values, endian),
                    ValueDirective::Float => this_segment.append_f32(span, values, endian),
                    ValueDirective::Double => this_segment.append_f64(span, values, endian),
                }?;
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
                    ProcessedInstruction::MachineCode(machine_code) => match self.config.endian {
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
                    return Err(RichError::new(AssembleError::WrongSegment, span)
                        .with_note(Self::INSTRUCTION_IN_DATA_SEGMENT));
                }
            };
            self.segments
                .get_mut(segment)
                .overwrite_u32(pc, machine_code, self.config.endian);
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

#[derive(Clone, Debug)]
pub struct Build<'config> {
    segments: Segments,
    endian: Endian,
    options: &'config AssemblerOptions,
    memory_map: &'config MemoryMap,
    services: &'config Services,
}

impl<'config> Build<'config> {
    pub(crate) const fn new(segments: Segments, config: &'config Config) -> Self {
        Self {
            segments,
            endian: config.endian,
            options: &config.features.assembler,
            memory_map: &config.memory_map,
            services: &config.features.services,
        }
    }

    pub fn export(self) -> Executable {
        use seaside_executable::{Body, Header};

        let mut body = Body::new(self.executable_flags(), self.memory_map.clone());

        body.services = self.services.clone();
        self.segments
            .export_into_executable_segments(&mut body.segments);

        Executable::new(Header::default(), body)
    }

    fn executable_flags(&self) -> seaside_executable::Flags {
        use seaside_executable::Flags;

        let mut flags = Flags::empty();
        if self.endian == Endian::Big {
            flags |= Flags::BIG_ENDIAN;
        }

        if self.options.self_modifying_code {
            flags |= Flags::SELF_MODIFYING_CODE;
        }

        if self.options.delay_slot {
            flags |= Flags::DELAY_SLOT;
        }

        if self.options.freeable_heap_allocations {
            flags |= Flags::FREEABLE_HEAP_ALLOCATIONS;
        }

        flags
    }
}
