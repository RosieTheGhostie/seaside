use seaside_type_aliases::Instruction;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Segments {
    pub text: TextSegment,
    pub ktext: TextSegment,
    pub r#extern: DataSegment,
    pub data: DataSegment,
    pub kdata: DataSegment,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TextSegment {
    pub instructions: Vec<Instruction>,
}

impl TextSegment {
    pub const fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DataSegment {
    pub bytes: Vec<u8>,
}

impl DataSegment {
    pub const fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}
