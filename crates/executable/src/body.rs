use seaside_constants::{Service, Services};
use seaside_type_aliases::ServiceCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{DebugInfo, Error, Flags, MemoryMap, Segments};

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
}
