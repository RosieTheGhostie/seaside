use std::io;

use serde::{Serialize, ser::SerializeTupleVariant};

use super::Serializer;

pub struct TupleVariantSerializer<'ser, W> {
    serializer: &'ser mut Serializer<W>,
}

impl<'ser, W> SerializeTupleVariant for TupleVariantSerializer<'ser, W>
where
    W: io::Write,
{
    type Ok = ();

    type Error = crate::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
