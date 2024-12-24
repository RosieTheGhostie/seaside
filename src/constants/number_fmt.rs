use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, PartialEq)]
pub enum NumberFormat {
    Single = 0,
    Double = 1,
    Word = 4,
    // Long = 5,
}
