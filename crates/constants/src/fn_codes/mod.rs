pub mod coprocessor_0;
pub mod coprocessor_1;
#[cfg(feature = "unimplemented")]
pub mod coprocessor_1x;
pub mod register_immediate;
pub mod special;
pub mod special_2;

pub use coprocessor_0::Coprocessor0Fn;
pub use coprocessor_1::{Coprocessor1Fn, Coprocessor1RegisterImmediateFn};
#[cfg(feature = "unimplemented")]
pub use coprocessor_1x::Coprocessor1XFn;
pub use register_immediate::RegisterImmediateFn;
pub use special::SpecialFn;
pub use special_2::Special2Fn;
