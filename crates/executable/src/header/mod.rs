pub mod flags;
pub mod table_of_contents;

pub use flags::Flags;
pub use table_of_contents::TableOfContents;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Header {
    pub version: u32,
    pub n_coprocessors: u32,
    pub flags: Flags,
    pub table_of_contents: TableOfContents,
}
