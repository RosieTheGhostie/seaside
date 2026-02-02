use std::io;

use serde::{Serialize, ser::SerializeStructVariant};

use super::Serializer;

pub struct StructVariantSerializer<'ser, W> {
    serializer: &'ser mut Serializer<W>,
}

impl<'ser, W> SerializeStructVariant for StructVariantSerializer<'ser, W>
where
    W: io::Write,
{
    type Ok = ();

    type Error = crate::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
