use bitflags::Flags;
use std::marker::PhantomData;

pub struct BitFlagVisitor<F>
where
    F: Flags,
{
    marker: PhantomData<fn() -> F>,
}

impl<F> BitFlagVisitor<F>
where
    F: Flags,
{
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}
