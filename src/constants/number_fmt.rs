use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, PartialEq)]
pub enum NumberFormat {
    Single = 16,
    Double = 17,
    Word = 20,
    // Long = 21,
}
