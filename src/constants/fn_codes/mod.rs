#![allow(dead_code)]
#![allow(unused_imports)]
pub mod coprocessor_0;
pub mod coprocessor_1;
pub mod register_immediate;
pub mod special;
pub mod special2;

pub use coprocessor_0::Coprocessor0Fn;
pub use coprocessor_1::Coprocessor1Fn;
pub use register_immediate::RegisterImmediateFn;
pub use special::SpecialFn;
pub use special2::Special2Fn;
