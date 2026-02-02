pub mod body;
pub mod debug_info;
pub mod error;
pub mod flags;
pub mod header;
pub mod location;
pub mod memory_map;
pub mod prelude;
pub mod segments;

pub use body::Body;
pub use debug_info::DebugInfo;
pub use error::Error;
pub use flags::Flags;
pub use header::Header;
pub use location::Location;
pub use memory_map::MemoryMap;
pub use segments::{Segment, Segments};

use std::io;

use validator::Validate;

#[derive(Clone, Debug, Eq, PartialEq, Validate)]
pub struct Executable {
    #[validate(nested)]
    pub header: Header,

    #[validate(nested)]
    pub body: Body,
}

impl Executable {
    pub fn new(header: Header, body: Body) -> Self {
        Self { header, body }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        Self::from_reader(bytes)
    }

    pub fn from_reader<R>(mut reader: R) -> Result<Self, Error>
    where
        R: io::Read,
    {
        let header = {
            let mut buffer = [0; 8];
            reader.read_exact(&mut buffer)?;
            Header::from_bytes(buffer)
        };
        header.validate()?;

        let body = rmp_serde::from_read(reader)?;
        Ok(Self::new(header, body))
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::new();
        self.to_writer(&mut bytes).map(|()| bytes)
    }

    pub fn to_writer<W>(&self, mut writer: W) -> Result<(), Error>
    where
        W: io::Write,
    {
        writer.write_all(&self.header.as_bytes())?;
        let serialized_body = rmp_serde::to_vec(&self.body)?;
        writer.write_all(&serialized_body)?;

        Ok(())
    }
}
