pub use crate::instruction::Opcode;
pub use coprocessor_0::Coprocessor0Fn;
pub use coprocessor_1::{Coprocessor1Fn, Coprocessor1RegisterImmediateFn};
pub use coprocessor_1x::Coprocessor1XFn;
pub use coprocessor_2::Coprocessor2RegisterImmediateFn;
pub use register_immediate::RegisterImmediateFn;
pub use special::SpecialFn;
pub use special_2::Special2Fn;

mod coprocessor_0;
mod coprocessor_1;
mod coprocessor_1x;
mod coprocessor_2;
mod register_immediate;
mod special;
mod special_2;
