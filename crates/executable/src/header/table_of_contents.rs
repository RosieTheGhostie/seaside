use std::collections::HashMap;

use seaside_type_aliases::{Size, UnsignedOffset};

use crate::{Location, prelude::Error, tag::Tag};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TableOfContents {
    table: HashMap<Tag, Location>,
}

impl TableOfContents {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn plan(&mut self, tag: Tag, size: Option<Size>) -> Result<(), Error> {
        if self
            .table
            .insert(tag, Location::plan(size.unwrap_or_default()))
            .is_none()
        {
            Ok(())
        } else {
            Err(Error::SectionAlreadyExists(tag))
        }
    }

    pub fn set_section_offset(&mut self, tag: &Tag, offset: UnsignedOffset) -> Result<(), Error> {
        self.table
            .get_mut(tag)
            .ok_or(Error::SectionAlreadyExists(*tag))?
            .offset = offset;
        Ok(())
    }

    pub fn insert(&mut self, tag: Tag, location: Location) {
        if self.table.insert(tag, location).is_some() {
            panic!(
                "cannot overrwite existing section {:?}",
                String::from_utf8_lossy(&tag),
            );
        }
    }
}
