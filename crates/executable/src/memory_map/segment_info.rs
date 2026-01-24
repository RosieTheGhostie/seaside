use seaside_address_range::AddressRange;
use seaside_type_aliases::Size;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SegmentInfo {
    pub range: AddressRange,
    pub allocate: Size,
}

crate::ser::fixed_size::r#impl!(for SegmentInfo; AddressRange, Size);
