pub mod debug_info;
pub mod error;
pub mod header;
pub mod location;
pub mod memory_map;
pub mod prelude;
pub mod segments;

use seaside_constants::{Service, Services};
use seaside_type_aliases::ServiceCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

use prelude::*;
use segments::Segments;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct Executable {
    #[validate(nested)]
    pub header: Header,

    pub memory_map: MemoryMap,
    pub services: Services,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug_info: Option<DebugInfo>,

    pub segments: Segments,
}

impl Executable {
    pub fn new(header: Header, memory_map: MemoryMap) -> Self {
        Self {
            header,
            memory_map,
            services: Services::default(),
            debug_info: None,
            segments: Segments::default(),
        }
    }

    pub fn add_services(&mut self, mapping: &[(ServiceCode, Service)]) -> Result<(), Error> {
        for &(code, service) in mapping {
            self.add_service(code, service)?;
        }

        Ok(())
    }

    pub fn add_service(&mut self, code: ServiceCode, service: Service) -> Result<(), Error> {
        if self.services.insert(code, service).is_none() {
            Ok(())
        } else {
            Err(Error::ServiceCodeInUse(code))
        }
    }
}

#[cfg(test)]
mod tests {
    use seaside_address_range::{address_range, sized::SizedAddressRange};
    use seaside_constants::{
        Service,
        services::{
            mars::{self, Mars},
            spim::{self, Spim},
        },
    };
    use seaside_type_aliases::size;
    use validator::Validate;

    use crate::{memory_map, prelude::*};

    #[test]
    fn concept() -> Result<(), Error> {
        let mut executable = Executable::new(
            Header::default(),
            MemoryMap {
                exception_handler: Some(0x8000_0180),
                user_space: address_range![0x0000_0000..=0x7fff_ffff],
                kernel_space: address_range![0x8000_0000..=0xffff_ffff],
                segments: memory_map::Segments {
                    text: SegmentInfo {
                        range: address_range![0x0040_0000..=0x0fff_fffc],
                        allocate: 8 * size::unsigned::MiB,
                    },
                    ktext: SegmentInfo {
                        range: address_range![0x8000_0000..=0x8fff_ffff],
                        allocate: 1 * size::unsigned::MiB,
                    },
                    r#extern: SegmentInfo {
                        range: address_range![0x1000_0000..=0x1000_ffff],
                        allocate: 64 * size::unsigned::KiB,
                    },
                    data: SegmentInfo {
                        range: address_range![0x1001_0000..=0x1003_ffff],
                        allocate: 192 * size::unsigned::KiB,
                    },
                    kdata: SegmentInfo {
                        range: address_range![0x9000_0000..=0xfffe_ffff],
                        allocate: 1 * size::unsigned::MiB,
                    },
                    stack: SizedAddressRange::new(0x7fff_ffff, 4 * size::unsigned::MiB),
                    heap: SizedAddressRange::new(0x1004_0000, 128 * size::unsigned::KiB),
                    mmio: SegmentInfo {
                        range: address_range![0xffff_0000..=0xffff_ffff],
                        allocate: 4 * size::unsigned::KiB,
                    },
                },
            },
        );
        executable.add_services(&[
            (1, Service::Spim(Spim::Print(spim::Print::Int))),
            (2, Service::Spim(Spim::Print(spim::Print::Float))),
            (3, Service::Spim(Spim::Print(spim::Print::Double))),
            (4, Service::Spim(Spim::Print(spim::Print::String))),
            (5, Service::Spim(Spim::Read(spim::Read::Int))),
            (6, Service::Spim(Spim::Read(spim::Read::Float))),
            (7, Service::Spim(Spim::Read(spim::Read::Double))),
            (8, Service::Spim(Spim::Read(spim::Read::String))),
            (9, Service::Spim(Spim::System(spim::System::Sbrk))),
            (10, Service::Spim(Spim::System(spim::System::Exit))),
            (11, Service::Spim(Spim::Print(spim::Print::Char))),
            (12, Service::Spim(Spim::Read(spim::Read::Char))),
            (13, Service::Spim(Spim::File(spim::File::Open))),
            (14, Service::Spim(Spim::File(spim::File::Read))),
            (15, Service::Spim(Spim::File(spim::File::Write))),
            (16, Service::Spim(Spim::File(spim::File::Close))),
            (17, Service::Spim(Spim::System(spim::System::Exit2))),
            (30, Service::Mars(Mars::System(mars::System::Time))),
            (31, Service::Mars(Mars::System(mars::System::MidiOut))),
            (32, Service::Mars(Mars::System(mars::System::Sleep))),
            (33, Service::Mars(Mars::System(mars::System::MidiOutSync))),
            (34, Service::Mars(Mars::Print(mars::Print::Hex))),
            (35, Service::Mars(Mars::Print(mars::Print::Bin))),
            (36, Service::Mars(Mars::Print(mars::Print::Uint))),
            (40, Service::Mars(Mars::Random(mars::Random::SetSeed))),
            (41, Service::Mars(Mars::Random(mars::Random::RandInt))),
            (42, Service::Mars(Mars::Random(mars::Random::RandIntRange))),
            (43, Service::Mars(Mars::Random(mars::Random::RandFloat))),
            (44, Service::Mars(Mars::Random(mars::Random::RandDouble))),
            (
                50,
                Service::Mars(Mars::Dialog(mars::Dialog::Input(
                    mars::InputDialog::Confirm,
                ))),
            ),
            (
                51,
                Service::Mars(Mars::Dialog(mars::Dialog::Input(mars::InputDialog::Int))),
            ),
            (
                52,
                Service::Mars(Mars::Dialog(mars::Dialog::Input(mars::InputDialog::Float))),
            ),
            (
                53,
                Service::Mars(Mars::Dialog(mars::Dialog::Input(mars::InputDialog::Double))),
            ),
            (
                54,
                Service::Mars(Mars::Dialog(mars::Dialog::Input(mars::InputDialog::String))),
            ),
            (
                55,
                Service::Mars(Mars::Dialog(mars::Dialog::Message(
                    mars::MessageDialog::General,
                ))),
            ),
            (
                56,
                Service::Mars(Mars::Dialog(mars::Dialog::Message(
                    mars::MessageDialog::Int,
                ))),
            ),
            (
                57,
                Service::Mars(Mars::Dialog(mars::Dialog::Message(
                    mars::MessageDialog::Float,
                ))),
            ),
            (
                58,
                Service::Mars(Mars::Dialog(mars::Dialog::Message(
                    mars::MessageDialog::Double,
                ))),
            ),
            (
                59,
                Service::Mars(Mars::Dialog(mars::Dialog::Message(
                    mars::MessageDialog::String,
                ))),
            ),
        ])?;
        executable.segments.text = TextSegment(vec![]);
        executable.segments.data = Some(DataSegment(vec![]));

        executable.validate().map_err(Error::from)
    }
}
