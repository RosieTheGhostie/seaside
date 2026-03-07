/// Configuration settings accessible from assembly code via the `.set` directive.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct Options {
    /// Allow explicit references to the assembler's temporary register ([`$at`]).
    ///
    /// Configurable via the "at"/"noat" commands.
    ///
    /// [`$at`]: seaside_core::register::CpuRegister::AsmTemp
    pub explicit_asm_temp: bool,
}

impl Options {
    pub const fn new() -> Self {
        Self {
            explicit_asm_temp: false,
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct OptionsStack {
    pub active: Options,
    stack: Vec<Options>,
}

impl OptionsStack {
    pub const fn new() -> Self {
        Self {
            active: Options::new(),
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self) {
        self.stack.push(self.active);
    }

    pub fn pop(&mut self) {
        if let Some(new_active) = self.stack.pop() {
            self.active = new_active;
        }
    }
}
