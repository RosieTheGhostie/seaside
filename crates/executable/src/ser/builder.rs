use std::collections::BTreeMap;

use seaside_constants::StaticSegment;
use seaside_type_aliases::{Instruction, ServiceCode, Size};

use super::{FixedSerializationSize, StringTable};
use crate::{Tag, Tagged, prelude::*, segments::Segments};

pub struct ExecutableBuilder {
    header: Header,

    memory_map: MemoryMap,
    services: BTreeMap<ServiceCode, Box<str>>,
    string_table: StringTable,
    debug_info: DebugInfo,
    segments: Segments,
}

impl ExecutableBuilder {
    pub fn new(basic_metadata: BasicMetadata, memory_map: MemoryMap) -> Self {
        let mut builder = Self {
            header: basic_metadata.into(),

            memory_map,
            services: BTreeMap::default(),
            string_table: StringTable::default(),
            debug_info: DebugInfo::default(),
            segments: Segments::default(),
        };

        let _ = builder.plan_section(MemoryMap::TAG, Some(MemoryMap::SERIALIZED_SIZE));
        builder
    }

    pub fn build(self) -> Result<Executable, Error> {
        // TODO: Validate shit.

        let string_table = self.string_table.compile();
        let services = compile_services(self.services, &string_table)?;
        let debug_info = (!self.debug_info.is_empty()).then_some(self.debug_info);

        Ok(Executable {
            header: self.header,
            memory_map: self.memory_map,
            services,
            string_table,
            debug_info,
            segments: self.segments,
        })
    }

    pub fn add_text_segment(
        &mut self,
        segment: StaticSegment,
        instructions: Vec<Instruction>,
    ) -> Result<(), Error> {
        match segment {
            StaticSegment::KText => self.segments.ktext.instructions = instructions,
            StaticSegment::Text => self.segments.text.instructions = instructions,
            _ => return Err(Error::NotATextSegment(segment)),
        }

        Ok(())
    }

    pub fn add_data_segment(
        &mut self,
        segment: StaticSegment,
        bytes: Vec<u8>,
    ) -> Result<(), Error> {
        match segment {
            StaticSegment::Data => self.segments.data.bytes = bytes,
            StaticSegment::Extern => self.segments.r#extern.bytes = bytes,
            StaticSegment::KData => self.segments.kdata.bytes = bytes,
            _ => return Err(Error::NotADataSegment(segment)),
        }

        Ok(())
    }

    pub fn add_services(&mut self, mapping: &[(ServiceCode, &str)]) -> Result<(), Error> {
        for &(code, identifier) in mapping {
            self.add_service(code, identifier)?;
        }

        Ok(())
    }

    pub fn add_service(&mut self, code: ServiceCode, identifier: &str) -> Result<(), Error> {
        // FIXME: This should attempt to parse `identifier` and add then add the information to
        //        `self.string_table`.
        if self
            .services
            .insert(code, identifier.to_string().into_boxed_str())
            .is_none()
        {
            Ok(())
        } else {
            Err(Error::ServiceCodeInUse(code))
        }
    }

    fn plan_section(&mut self, tag: Tag, size: Option<Size>) -> Result<(), Error> {
        self.header.table_of_contents.plan(tag, size)
    }
}

fn compile_services(
    services: BTreeMap<ServiceCode, Box<str>>,
    string_table: &str,
) -> Result<Services, Error> {
    todo!()
}
