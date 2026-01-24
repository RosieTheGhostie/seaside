pub mod de;
pub mod debug_info;
pub mod error;
pub mod header;
pub mod location;
pub mod memory_map;
pub mod prelude;
pub mod segments;
pub mod ser;
pub mod services;
pub mod tag;

pub(crate) use location::Location;
use prelude::*;
use segments::Segments;
use ser::builder::ExecutableBuilder;
pub(crate) use tag::{Tag, Tagged};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Executable {
    pub header: Header,
    pub memory_map: MemoryMap,
    pub services: Services,
    pub string_table: String,
    pub debug_info: Option<DebugInfo>,
    pub segments: Segments,
}

impl Executable {
    pub fn new_builder(basic_metadata: BasicMetadata, memory_map: MemoryMap) -> ExecutableBuilder {
        ExecutableBuilder::new(basic_metadata, memory_map)
    }
}

#[cfg(test)]
mod tests {
    use seaside_address_range::{address_range, sized::SizedAddressRange};
    use seaside_constants::StaticSegment;
    use seaside_type_aliases::size;

    use crate::{memory_map, prelude::*};

    #[test]
    fn concept() -> Result<(), Error> {
        let mut builder = Executable::new_builder(
            BasicMetadata {
                n_coprocessors: 2,
                flags: Flags::default(),
            },
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
        builder.add_services(&[
            (1, "spim.print.int"),
            (2, "spim.print.float"),
            (3, "spim.print.double"),
            (4, "spim.print.string"),
            (5, "spim.read.int"),
            (6, "spim.read.float"),
            (7, "spim.read.double"),
            (8, "spim.read.string"),
            (9, "spim.system.sbrk"),
            (10, "spim.system.exit"),
            (11, "spim.print.char"),
            (12, "spim.read.char"),
            (13, "spim.file.open"),
            (14, "spim.file.read"),
            (15, "spim.file.write"),
            (16, "spim.file.close"),
            (17, "spim.system.exit2"),
            (30, "mars.system.time"),
            (31, "mars.system.midi_out"),
            (32, "mars.system.sleep"),
            (33, "mars.system.midi_out_sync"),
            (34, "mars.print.hex"),
            (35, "mars.print.bin"),
            (36, "mars.print.uint"),
            (40, "mars.random.set_seed"),
            (41, "mars.random.rand_int"),
            (42, "mars.random.rand_int_range"),
            (43, "mars.random.rand_float"),
            (44, "mars.random.rand_double"),
            (50, "mars.dialog.input.confirm"),
            (51, "mars.dialog.input.int"),
            (52, "mars.dialog.input.float"),
            (53, "mars.dialog.input.double"),
            (54, "mars.dialog.input.string"),
            (55, "mars.dialog.message.general"),
            (56, "mars.dialog.message.int"),
            (57, "mars.dialog.message.float"),
            (58, "mars.dialog.message.double"),
            (59, "mars.dialog.message.string"),
        ])?;
        builder.add_text_segment(StaticSegment::Text, vec![])?;
        builder.add_data_segment(StaticSegment::Data, vec![])?;
        let executable = builder.build()?;

        // TODO: Look at `executable`.

        Ok(())
    }
}
