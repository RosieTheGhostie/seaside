use seaside_address_range::{AddressRange, sized::SizedAddressRange};
use seaside_type_aliases::{Address, Size};

pub trait FixedSerializationSize: Sized {
    const SERIALIZED_SIZE: Size = size_of::<Self>() as _;
}

impl FixedSerializationSize for u8 {}
impl FixedSerializationSize for Option<u8> {}
impl FixedSerializationSize for u16 {}
impl FixedSerializationSize for Option<u16> {}
impl FixedSerializationSize for u32 {}
impl FixedSerializationSize for Option<u32> {}
impl FixedSerializationSize for u64 {}
impl FixedSerializationSize for Option<u64> {}
impl FixedSerializationSize for u128 {}
impl FixedSerializationSize for Option<u128> {}

impl FixedSerializationSize for i8 {}
impl FixedSerializationSize for Option<i8> {}
impl FixedSerializationSize for i16 {}
impl FixedSerializationSize for Option<i16> {}
impl FixedSerializationSize for i32 {}
impl FixedSerializationSize for Option<i32> {}
impl FixedSerializationSize for i64 {}
impl FixedSerializationSize for Option<i64> {}
impl FixedSerializationSize for i128 {}
impl FixedSerializationSize for Option<i128> {}

macro_rules! r#impl {
    (for $impl_ty:ty; $($($n:literal *)? $ty:ty),* $(,)?) => {
        impl $crate::ser::FixedSerializationSize for $impl_ty {
            const SERIALIZED_SIZE: ::seaside_type_aliases::Size =
                0 $(+ $crate::ser::fixed_size::r#impl!(@internal $($n *)? $ty))*;
        }
    };
    (@internal $ty:ty) => {
        <$ty as $crate::ser::FixedSerializationSize>::SERIALIZED_SIZE
    };
    (@internal $n:literal * $ty:ty) => {
        $n * $crate::ser::fixed_size::r#impl!(@internal $ty)
    };
}
pub(crate) use r#impl;

r#impl!(for AddressRange; 2 * Address);
r#impl!(for SizedAddressRange<false>; Address, Size);
r#impl!(for SizedAddressRange<true>; Address, Size);
