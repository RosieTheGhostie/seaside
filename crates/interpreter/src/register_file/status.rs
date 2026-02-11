use bitfields::bitfield;

/// Represents the value of coprocessor 0's `$status` register.
#[bitfield(u32)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Status {
    /// Indicates that interrupts are enabled.
    #[bits(default = true)]
    pub interrupts_enabled: bool,

    /// Indicates that an exception is currently active.
    pub exception_level: bool,

    #[bits(2)]
    _padding_a: u8,

    /// Indicates that execution is currently in "user mode".
    #[bits(default = true)]
    pub user_mode: bool,

    #[bits(3)]
    _padding_b: u8,

    /// TODO: What is this?
    #[bits(default = 0xff)]
    pub interrupt_level_mask: u8,

    /// The upper 16 bits of the status register.
    ///
    /// It is unclear whether or not these are used, but they have been made public just in case.
    pub upper_half: u16,
}
