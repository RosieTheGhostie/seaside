use seaside_core::{Services, consts::services::Service, prelude::*, types::ServiceCode};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{DebugInfo, Error, Flags, MemoryMap, Segment, Segments};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct Body {
    pub flags: Flags,

    pub memory_map: MemoryMap,
    pub services: Services,

    #[serde(default)]
    pub debug_info: Option<DebugInfo>,

    pub segments: Segments,
}

impl Body {
    pub fn new(flags: Flags, memory_map: MemoryMap) -> Self {
        Self {
            flags,
            memory_map,
            services: Services::default(),
            debug_info: None,
            segments: Segments::default(),
        }
    }

    pub fn add_service(&mut self, code: ServiceCode, service: Service) -> Result<(), Error> {
        if self.services.insert(code, service).is_none() {
            Ok(())
        } else {
            Err(Error::ServiceCodeInUse(code))
        }
    }

    pub fn add_services(&mut self, mapping: &[(ServiceCode, Service)]) -> Result<(), Error> {
        for &(code, service) in mapping {
            self.add_service(code, service)?;
        }

        Ok(())
    }

    pub fn set_text_segment(&mut self, instructions: &[Instruction]) {
        self.segments.text = self.make_text_segment(instructions);
    }

    pub fn add_ktext_segment(&mut self, instructions: &[Instruction]) -> Option<Segment> {
        self.segments
            .ktext
            .replace(self.make_text_segment(instructions))
    }

    pub fn add_extern_segment(&mut self, bytes: &[u8]) -> Option<Segment> {
        self.segments.r#extern.replace(Segment::new(bytes))
    }

    pub fn add_data_segment(&mut self, bytes: &[u8]) -> Option<Segment> {
        self.segments.data.replace(Segment::new(bytes))
    }

    pub fn add_kdata_segment(&mut self, bytes: &[u8]) -> Option<Segment> {
        self.segments.kdata.replace(Segment::new(bytes))
    }

    fn make_text_segment(&self, instructions: &[Instruction]) -> Segment {
        Segment::new_text(instructions, self.flags.endian())
    }
}
