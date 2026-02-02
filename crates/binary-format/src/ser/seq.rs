use std::io;

use serde::ser::SerializeSeq;

use super::Serializer;

pub struct SeqSerializer<'ser, W> {
    serializer: &'ser mut Serializer<W>,
}

impl<'ser, W> SerializeSeq for SeqSerializer<'ser, W>
where
    W: io::Write,
{
    type Ok = ();

    type Error = crate::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
