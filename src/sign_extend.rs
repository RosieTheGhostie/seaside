#![allow(dead_code)]
pub trait SignExtend<T>
where
    Self: Sized,
    T: Sized + From<<Self as SignExtend<T>>::SignedSelf>,
{
    type SignedSelf: Sized;

    fn sign_extend(&self) -> T;
}

macro_rules! impl_sign_extend {
    ($uint:ty as $int:ty => $extended:ty) => {
        impl SignExtend<$extended> for $uint {
            type SignedSelf = $int;

            fn sign_extend(&self) -> $extended {
                *self as $int as $extended
            }
        }
    };
}

impl_sign_extend!(u8 as i8 => i16);
impl_sign_extend!(u8 as i8 => i32);
impl_sign_extend!(u8 as i8 => i64);
impl_sign_extend!(u8 as i8 => i128);

impl_sign_extend!(u16 as i16 => i32);
impl_sign_extend!(u16 as i16 => i64);
impl_sign_extend!(u16 as i16 => i128);

impl_sign_extend!(u32 as i32 => i64);
impl_sign_extend!(u32 as i32 => i128);

impl_sign_extend!(u64 as i64 => i128);

#[cfg(test)]
mod tests {
    use super::SignExtend;

    #[test]
    fn sign_extend_u8() {
        let x: u8 = 0xe0;
        let x16: i16 = x.sign_extend();
        let x32: i32 = x.sign_extend();
        let x64: i64 = x.sign_extend();
        let x128: i128 = x.sign_extend();
        assert_eq!(x16, -32);
        assert_eq!(x32, -32);
        assert_eq!(x64, -32);
        assert_eq!(x128, -32);
    }

    #[test]
    fn sign_extend_u16() {
        let x: u16 = 0xfe5c;
        let x32: i32 = x.sign_extend();
        let x64: i64 = x.sign_extend();
        let x128: i128 = x.sign_extend();
        assert_eq!(x32, -420);
        assert_eq!(x64, -420);
        assert_eq!(x128, -420);
    }

    #[test]
    fn sign_extend_u32() {
        let x: u32 = 0xfffef0d4;
        let x64: i64 = x.sign_extend();
        let x128: i128 = x.sign_extend();
        assert_eq!(x64, -69420);
        assert_eq!(x128, -69420);
    }

    #[test]
    fn sign_extend_u64() {
        let x: u64 = 0xffff8fb779f0c5c8;
        let x128: i128 = x.sign_extend();
        assert_eq!(x128, -123456789101112);
    }
}
