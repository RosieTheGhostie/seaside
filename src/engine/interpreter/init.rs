use super::{memory::Memory, register_file::RegisterFile, syscalls::Syscalls, Interpreter};
use crate::{
    config::{Config, RegisterDefaults},
    engine::Error,
};
use std::path::PathBuf;

impl Interpreter {
    pub fn init(
        config: &Config,
        text: PathBuf,
        r#extern: Option<PathBuf>,
        data: Option<PathBuf>,
        ktext: Option<PathBuf>,
        kdata: Option<PathBuf>,
    ) -> Result<Self, Error> {
        let memory = Memory::init(config, text, r#extern, data, ktext, kdata)?;
        let pc = memory.initial_pc();
        let syscalls = Syscalls::from(&config.features.syscalls);
        let registers = init_register_file(&config.register_defaults);
        Ok(Self {
            memory,
            registers,
            pc,
            syscalls,
        })
    }
}

fn init_register_file(register_defaults: &RegisterDefaults) -> RegisterFile {
    let mut register_file = RegisterFile::default();
    // can't iterate directly due to borrow checker stuff
    for i in 0..32 {
        let default_value = register_defaults.general_purpose[i];
        let _ = register_file.write_u32_to_cpu(i as u8, default_value);
    }
    register_file.hi = register_defaults.hi;
    register_file.lo = register_defaults.lo;
    for i in 0..32 {
        let x = f32::from_bits(register_defaults.floating_point[i]);
        let _ = register_file.write_f32_to_fpu(i as u8, x);
    }
    register_file.vaddr = register_defaults.coprocessor_0[0];
    register_file.status = register_defaults.coprocessor_0[1];
    register_file.cause = register_defaults.coprocessor_0[2];
    register_file.epc = register_defaults.coprocessor_0[3];
    register_file
}
