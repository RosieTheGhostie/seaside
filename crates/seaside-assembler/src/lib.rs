#![recursion_limit = "256"]
pub mod error;
pub mod operation;
pub mod parser;

mod assemble;
mod directives;
mod macros;
mod segment;
mod string_literal;
mod token;

pub use assemble::assemble_instruction;
pub use error::AssembleError;
pub use operation::{BasicOperator, Operand};
pub use parser::{Node, ParseError, Parser};
pub use token::Token;

use anyhow::{Error, Result};
use directives::SegmentDirective;
use logos::Logos;
use operation::convert_address;
use seaside_config::Config;
use seaside_int_utils::Endian;
use seaside_type_aliases::Address;
use segment::SegmentBuildInfo;
use std::{
    collections::{HashMap, VecDeque},
    iter::zip,
    path::Path,
};

pub struct Assembler<'source> {
    parser: Parser<'source>,
    segments: [SegmentBuildInfo; 5],
    current_segment: SegmentDirective,
    unresolved: VecDeque<(Address, Node)>,
    symbol_table: HashMap<String, Address>,
    endian: Endian,
}

impl<'source> Assembler<'source> {
    pub fn init(config: &Config, source: &'source str) -> Self {
        let endian = config.endian;
        let directives = config.features.assembler.directives;
        let segments = &config.memory_map.segments;
        Self {
            parser: Parser::new(Token::lexer(source), directives),
            segments: [
                SegmentBuildInfo::new(segments.data.address_range.base),
                SegmentBuildInfo::new(segments.r#extern.address_range.base),
                SegmentBuildInfo::new(segments.kdata.address_range.base),
                SegmentBuildInfo::new(segments.ktext.address_range.base),
                SegmentBuildInfo::new(segments.text.address_range.base),
            ],
            current_segment: SegmentDirective::Text,
            unresolved: VecDeque::new(),
            symbol_table: HashMap::new(),
            endian,
        }
    }

    pub fn build(&mut self) -> Result<()> {
        while self.build_next()? {}
        while self.resolve_next()? {}
        Ok(())
    }

    pub fn export(&self, directory: &Path) -> Result<()> {
        for (segment, name) in zip(&self.segments, SegmentDirective::names())
            .filter(|(segment, _)| !segment.is_empty())
        {
            segment.export(directory.join(name))?;
        }
        Ok(())
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

    fn build_next(&mut self) -> Result<bool> {
        let node = match self.parser.next() {
            Some(maybe_node) => maybe_node?,
            None => return Ok(false),
        };
        if !node.can_resolve() {
            self.unresolved.push_back((self.next_address(), node));
            self.this_segment_mut().jump_ahead_by(4);
            return Ok(true);
        }
        let endian = self.endian;
        match node {
            Node::SegmentHeader(directive, address) => {
                self.current_segment = directive;
                if let Some(address) = address {
                    self.this_segment_mut().jump_ahead_to(address)?;
                }
            }
            Node::LabelDefinition(label) => self.add_symbol(label)?,
            Node::Instruction(operator, operands) if self.current_segment.is_text_segment() => {
                let machine_code = assemble_instruction(operator, operands)?;
                let mut bytes = match self.endian {
                    Endian::Little => machine_code.to_le_bytes(),
                    Endian::Big => machine_code.to_be_bytes(),
                }
                .to_vec();
                self.this_segment_mut().append(&mut bytes);
            }
            Node::AlignCommand(alignment) if self.current_segment.is_data_segment() => {
                self.this_segment_mut().align(alignment)
            }
            Node::ByteArray(bytes) if self.current_segment.is_data_segment() => {
                self.this_segment_mut().append_i8(bytes);
            }
            Node::HalfArray(halves) if self.current_segment.is_data_segment() => {
                self.this_segment_mut().append_i16(halves, endian);
            }
            Node::WordArray(words) if self.current_segment.is_data_segment() => {
                self.this_segment_mut().append_i32(words, endian);
            }
            Node::FloatArray(floats) if self.current_segment.is_data_segment() => {
                self.this_segment_mut().append_f32(floats, endian);
            }
            Node::DoubleArray(doubles) if self.current_segment.is_data_segment() => {
                self.this_segment_mut().append_f64(doubles, endian);
            }
            Node::String(string) if self.current_segment.is_data_segment() => {
                self.this_segment_mut().append(&mut string.into_bytes());
            }
            _ => return Err(Error::new(AssembleError::WrongSegment)),
        }
        Ok(true)
    }

    fn resolve_next(&mut self) -> Result<bool> {
        let (pc, operator, mut operands) = match self.unresolved.pop_front() {
            Some((pc, Node::Instruction(operator, operands))) => (pc, operator, operands),
            Some((_, _)) => return Err(Error::new(AssembleError::InternalLogicIssue)),
            None => return Ok(false),
        };
        for operand in &mut operands {
            if let Some(Operand::Label(label)) = operand {
                match self.symbol_table.get(label) {
                    Some(address) => *operand = Some(convert_address(operator, *address, pc)?),
                    None => return Err(Error::new(AssembleError::UndefinedSymbol)),
                }
            }
        }
        let machine_code = assemble_instruction(operator, operands)?;
        self.replace_instruction(pc, machine_code)?;
        Ok(true)
    }

    fn add_symbol(&mut self, label: String) -> Result<()> {
        match self.symbol_table.insert(label, self.next_address()) {
            Some(_) => Err(Error::new(AssembleError::MultipleDefinitions)),
            None => Ok(()),
        }
    }

    fn replace_instruction(&mut self, address: Address, instruction: u32) -> Result<()> {
        let text_diff = address.checked_sub(self.segments[SegmentDirective::Text as usize].base);
        let ktext_diff = address.checked_sub(self.segments[SegmentDirective::KText as usize].base);
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
            (None, None) => return Err(Error::new(AssembleError::WrongSegment)),
        };
        self.segments[segment as usize].overwrite_u32(address, instruction, self.endian)
    }
}
