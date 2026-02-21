pub use coprocessor_0::Coprocessor0Fn;
pub use coprocessor_1::{Coprocessor1Fn, Coprocessor1RegisterImmediateFn};
pub use coprocessor_1x::Coprocessor1XFn;
pub use opcode::Opcode;
pub use register_immediate::RegisterImmediateFn;
pub use special::SpecialFn;
pub use special_2::Special2Fn;

mod coprocessor_0;
mod coprocessor_1;
mod coprocessor_1x;
mod opcode;
mod register_immediate;
mod special;
mod special_2;
