pub mod de;
pub mod header;
pub mod sections;
pub mod segments;
pub mod ser;

pub use header::Header;
pub use sections::Sections;
pub use segments::Segments;

pub struct Executable {
    pub header: Header,
    pub sections: Sections,
    pub segments: Segments,
}
