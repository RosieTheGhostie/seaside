pub use segment::SegmentBuildInfo;

mod segment;

use seaside_core::consts::StaticSegment;
use seaside_executable::memory_map;
use strum::EnumCount;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Segments {
    pub text: SegmentBuildInfo,
    pub ktext: SegmentBuildInfo,
    pub r#extern: SegmentBuildInfo,
    pub data: SegmentBuildInfo,
    pub kdata: SegmentBuildInfo,
}

impl Segments {
    pub const N: usize = StaticSegment::COUNT;

    pub const fn from_memory_map_segments(segments: &memory_map::Segments) -> Self {
        Self {
            text: SegmentBuildInfo::new(segments.text.range.base()),
            ktext: SegmentBuildInfo::new(segments.ktext.range.base()),
            r#extern: SegmentBuildInfo::new(segments.r#extern.range.base()),
            data: SegmentBuildInfo::new(segments.data.range.base()),
            kdata: SegmentBuildInfo::new(segments.kdata.range.base()),
        }
    }

    pub const fn get(&self, segment: StaticSegment) -> &SegmentBuildInfo {
        &self.as_slice()[segment as usize]
    }

    pub const fn get_mut(&mut self, segment: StaticSegment) -> &mut SegmentBuildInfo {
        &mut self.as_slice_mut()[segment as usize]
    }

    pub fn export_into_executable_segments(self, segments: &mut seaside_executable::Segments) {
        segments.text.overwrite(self.text.take_bytes());
        if !self.ktext.is_empty() {
            segments.ktext = Some(self.ktext.export());
        }

        if !self.r#extern.is_empty() {
            segments.r#extern = Some(self.r#extern.export());
        }

        if !self.data.is_empty() {
            segments.data = Some(self.data.export());
        }

        if !self.kdata.is_empty() {
            segments.kdata = Some(self.kdata.export());
        }
    }

    const fn as_slice(&self) -> &[SegmentBuildInfo] {
        let ptr: *const SegmentBuildInfo = core::ptr::from_ref(self) as _;
        unsafe { core::slice::from_raw_parts(ptr, Self::N) }
    }

    const fn as_slice_mut(&mut self) -> &mut [SegmentBuildInfo] {
        let ptr: *mut SegmentBuildInfo = core::ptr::from_mut(self) as _;
        unsafe { core::slice::from_raw_parts_mut(ptr, Self::N) }
    }
}

const _: () = assert!(Segments::N == size_of::<Segments>() / size_of::<SegmentBuildInfo>());
