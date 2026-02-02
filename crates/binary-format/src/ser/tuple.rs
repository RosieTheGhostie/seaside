use std::io;

use serde::{Serialize, ser::SerializeTuple};

use super::Serializer;

pub struct TupleSerializer<'ser, W> {
    serializer: &'ser mut Serializer<W>,
}

impl<'ser, W> SerializeTuple for TupleSerializer<'ser, W>
where
    W: io::Write,
{
    type Ok = ();

    type Error = crate::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
