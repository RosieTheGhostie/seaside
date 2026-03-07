use seaside_config::{Config, features::AssemblerOptions};
use seaside_core::{Endian, Services};
use seaside_executable::{Executable, MemoryMap};

use crate::segments::Segments;

#[derive(Clone, Debug)]
pub struct Build<'config> {
    segments: Segments,
    endian: Endian,
    options: &'config AssemblerOptions,
    memory_map: &'config MemoryMap,
    services: &'config Services,
}

impl<'config> Build<'config> {
    pub(crate) const fn new(segments: Segments, config: &'config Config) -> Self {
        Self {
            segments,
            endian: config.endian,
            options: &config.features.assembler,
            memory_map: &config.memory_map,
            services: &config.features.services,
        }
    }

    pub fn export(self) -> Executable {
        use seaside_executable::{Body, Header};

        let mut body = Body::new(self.executable_flags(), *self.memory_map);

        body.services = self.services.clone();
        self.segments
            .export_into_executable_segments(&mut body.segments);

        Executable::new(Header::default(), body)
    }

    fn executable_flags(&self) -> seaside_executable::Flags {
        use seaside_executable::Flags;

        let mut flags = Flags::empty();
        if self.endian == Endian::Big {
            flags |= Flags::BIG_ENDIAN;
        }

        if self.options.self_modifying_code {
            flags |= Flags::SELF_MODIFYING_CODE;
        }

        if self.options.delay_slot {
            flags |= Flags::DELAY_SLOT;
        }

        if self.options.freeable_heap_allocations {
            flags |= Flags::FREEABLE_HEAP_ALLOCATIONS;
        }

        flags
    }
}
