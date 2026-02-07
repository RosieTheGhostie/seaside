#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Product<T> {
    pub upper_half: T,
    pub lower_half: T,
}

impl<T> Product<T>
where
    T: Sized,
{
    pub const BITS_IN_HALF: usize = 8 * size_of::<T>();
}

impl Product<i8> {
    pub const fn from_i16(value: i16) -> Self {
        let value = value.cast_unsigned();
        Self {
            upper_half: (value >> Self::BITS_IN_HALF) as _,
            lower_half: (value & u8::MAX as u16) as _,
        }
    }
}

impl From<i16> for Product<i8> {
    fn from(value: i16) -> Self {
        Self::from_i16(value)
    }
}

impl Product<i16> {
    pub const fn from_i32(value: i32) -> Self {
        let value = value.cast_unsigned();
        Self {
            upper_half: (value >> Self::BITS_IN_HALF) as _,
            lower_half: (value & u16::MAX as u32) as _,
        }
    }
}

impl From<i32> for Product<i16> {
    fn from(value: i32) -> Self {
        Self::from_i32(value)
    }
}

impl Product<i32> {
    pub const fn from_i64(value: i64) -> Self {
        let value = value.cast_unsigned();
        Self {
            upper_half: (value >> Self::BITS_IN_HALF) as _,
            lower_half: (value & u32::MAX as u64) as _,
        }
    }
}

impl From<i64> for Product<i32> {
    fn from(value: i64) -> Self {
        Self::from_i64(value)
    }
}

impl Product<u8> {
    pub const fn from_u16(value: u16) -> Self {
        Self {
            upper_half: (value >> Self::BITS_IN_HALF) as _,
            lower_half: (value & u8::MAX as u16) as _,
        }
    }
}

impl From<u16> for Product<u8> {
    fn from(value: u16) -> Self {
        Self::from_u16(value)
    }
}

impl Product<u16> {
    pub const fn from_u32(value: u32) -> Self {
        Self {
            upper_half: (value >> Self::BITS_IN_HALF) as _,
            lower_half: (value & u16::MAX as u32) as _,
        }
    }
}

impl From<u32> for Product<u16> {
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl Product<u32> {
    pub const fn from_u64(value: u64) -> Self {
        Self {
            upper_half: (value >> Self::BITS_IN_HALF) as _,
            lower_half: (value & u32::MAX as u64) as _,
        }
    }
}

impl From<u64> for Product<u32> {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}
