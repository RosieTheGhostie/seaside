pub mod _registers;
pub mod coprocessor_0;
pub mod floating_point;
pub mod general_purpose;
pub mod register_defaults;
pub mod register_set;

pub use _registers::Registers;
pub use coprocessor_0::Coprocessor0Register;
pub use floating_point::FloatingPointRegister;
pub use general_purpose::GeneralPurposeRegister;
pub use register_defaults::RegisterDefaults;
pub use register_set::RegisterSet;
