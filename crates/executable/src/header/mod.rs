pub mod basic_metadata;
pub mod flags;
pub mod table_of_contents;

pub use basic_metadata::BasicMetadata;
pub use flags::Flags;
pub use table_of_contents::TableOfContents;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Header {
    // pub version: u32, <-- currently a constant value of 1
    pub basic_metadata: BasicMetadata,
    pub table_of_contents: TableOfContents,
}

impl From<BasicMetadata> for Header {
    fn from(basic_metadata: BasicMetadata) -> Self {
        Self {
            basic_metadata,
            table_of_contents: TableOfContents::default(),
        }
    }
}
