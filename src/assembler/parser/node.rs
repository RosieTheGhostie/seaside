use super::super::{directives::SegmentDirective, BasicOperator, Operand};

#[derive(Debug, PartialEq)]
pub enum Node {
    SegmentHeader(SegmentDirective, Option<u32>),
    LabelDefinition(String),
    Instruction(BasicOperator, [Option<Operand>; 4]),
    ByteArray(Vec<i8>),
    HalfArray(Vec<i16>),
    WordArray(Vec<i32>),
    FloatArray(Vec<f32>),
    DoubleArray(Vec<f64>),
    String(String),
}

impl Node {
    pub fn size(&self) -> usize {
        match self {
            Self::SegmentHeader(_, _) => todo!(),
            Self::LabelDefinition(_) => 0,
            Self::Instruction(_, _) => 4,
            Self::ByteArray(array) => array.len(),
            Self::HalfArray(array) => array.len() << 1,
            Self::WordArray(array) => array.len() << 2,
            Self::FloatArray(array) => array.len() << 2,
            Self::DoubleArray(array) => array.len() << 3,
            Self::String(string) => string.len(),
        }
    }

    pub fn can_resolve(&self) -> bool {
        if let Self::Instruction(_, operands) = self {
            let mut it = operands.iter();
            while let Some(Some(operand)) = it.next() {
                if operand.is_label() {
                    return false;
                }
            }
        }
        true
    }
}
