pub const VERSION: u8 = 0x00;
pub const ENDIAN: u8 = 0x01;
pub const PROJECT_DIRECTORY_IS_CWD: u8 = 0x02;

pub mod features {
    pub const ID: u8 = 0x01;
    pub const PREFIX: u32 = (ID as u32) << 24;

    pub const KERNEL_SPACE_ACCESSIBLE: u8 = 0x00;
    pub const SELF_MODIFYING_CODE: u8 = 0x01;
    pub const DELAY_SLOT: u8 = 0x02;
    pub const FREEABLE_HEAP_ALLOCATIONS: u8 = 0x03;
    pub const SHOW_CRASH_HANDLER: u8 = 0x04;

    pub mod assembler {
        pub const ID: u8 = 0x01;
        pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 16);

        pub const PSEUDO_INSTRUCTIONS: u8 = 0x00;

        pub mod directives {
            pub const ID: u8 = 0x01;
            pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 8);

            pub const ASCIIZ: u8 = 0x00;
            pub const EQV: u8 = 0x01;
            pub const GLOBAL: u8 = 0x02;
            pub const INCLUDE: u8 = 0x03;
            pub const MACROS: u8 = 0x04;
            pub const SET: u8 = 0x05;
        }
    }

    pub mod syscalls {
        pub const ID: u8 = 0x02;
        pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 16);

        pub mod mars_print {
            pub const ID: u8 = 0x01;
            pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 8);

            pub const INT: u8 = 0x00;
            pub const UINT: u8 = 0x01;
            pub const BIN: u8 = 0x02;
            pub const HEX: u8 = 0x03;
            pub const FLOAT: u8 = 0x04;
            pub const DOUBLE: u8 = 0x05;
            pub const CHAR: u8 = 0x06;
            pub const STRING: u8 = 0x07;
        }

        pub mod mars_read {
            pub const ID: u8 = 0x02;
            pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 8);

            pub const INT: u8 = 0x00;
            pub const FLOAT: u8 = 0x01;
            pub const DOUBLE: u8 = 0x02;
            pub const CHAR: u8 = 0x03;
            pub const STRING: u8 = 0x04;
        }

        pub mod mars_file {
            pub const ID: u8 = 0x03;
            pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 8);

            pub const OPEN: u8 = 0x00;
            pub const READ: u8 = 0x01;
            pub const WRITE: u8 = 0x02;
            pub const CLOSE: u8 = 0x03;
        }

        pub mod mars_input_dialog {
            pub const ID: u8 = 0x04;
            pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 8);

            pub const CONFIRM: u8 = 0x00;
            pub const INT: u8 = 0x01;
            pub const FLOAT: u8 = 0x02;
            pub const DOUBLE: u8 = 0x03;
            pub const STRING: u8 = 0x04;
        }

        pub mod mars_message_dialog {
            pub const ID: u8 = 0x05;
            pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 8);

            pub const GENERAL: u8 = 0x00;
            pub const INT: u8 = 0x01;
            pub const FLOAT: u8 = 0x02;
            pub const DOUBLE: u8 = 0x03;
            pub const STRING: u8 = 0x04;
        }

        pub mod mars_system {
            pub const ID: u8 = 0x06;
            pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 8);

            pub const SBRK: u8 = 0x00;
            pub const EXIT: u8 = 0x01;
            pub const EXIT_2: u8 = 0x02;
            pub const TIME: u8 = 0x03;
            pub const SLEEP: u8 = 0x04;
            pub const MIDI_OUT: u8 = 0x05;
            pub const MIDI_OUT_SYNC: u8 = 0x06;
        }

        pub mod mars_random {
            pub const ID: u8 = 0x07;
            pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 8);

            pub const SET_SEED: u8 = 0x00;
            pub const INT: u8 = 0x01;
            pub const INT_RANGE: u8 = 0x02;
            pub const FLOAT: u8 = 0x03;
            pub const DOUBLE: u8 = 0x04;
        }
    }
}

pub mod memory_map {
    pub const ID: u8 = 0x02;
    pub const PREFIX: u32 = (ID as u32) << 24;

    pub const EXCEPTION_HANDLER: u8 = 0x00;

    pub mod user_space {
        pub const ID: u8 = 0x01;
        pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 16);

        pub const BASE: u8 = 0x00;
        pub const LIMIT: u8 = 0x01;
    }

    pub mod kernel_space {
        pub const ID: u8 = 0x02;
        pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 16);

        pub const BASE: u8 = 0x00;
        pub const LIMIT: u8 = 0x01;
    }

    pub mod segments {
        pub const ID: u8 = 0x03;
        pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 16);

        pub mod text {
            pub const ID: u8 = 0x01;
            pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 8);

            pub const BASE: u8 = 0x00;
            pub const LIMIT: u8 = 0x01;
            pub const ALLOCATE: u8 = 0x02;
        }

        pub mod r#extern {
            pub const ID: u8 = 0x02;
            pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 8);

            pub const BASE: u8 = 0x00;
            pub const LIMIT: u8 = 0x01;
            pub const ALLOCATE: u8 = 0x02;
        }

        pub mod data {
            pub const ID: u8 = 0x03;
            pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 8);

            pub const BASE: u8 = 0x00;
            pub const LIMIT: u8 = 0x01;
            pub const ALLOCATE: u8 = 0x02;
        }

        pub mod runtime_data {
            pub const ID: u8 = 0x04;
            pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 8);

            pub const BASE: u8 = 0x00;
            pub const LIMIT: u8 = 0x01;
            pub const ALLOCATE: u8 = 0x02;
        }

        pub mod ktext {
            pub const ID: u8 = 0x05;
            pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 8);

            pub const BASE: u8 = 0x00;
            pub const LIMIT: u8 = 0x01;
            pub const ALLOCATE: u8 = 0x02;
        }

        pub mod kdata {
            pub const ID: u8 = 0x06;
            pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 8);

            pub const BASE: u8 = 0x00;
            pub const LIMIT: u8 = 0x01;
            pub const ALLOCATE: u8 = 0x02;
        }

        pub mod mmio {
            pub const ID: u8 = 0x07;
            pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 8);

            pub const BASE: u8 = 0x00;
            pub const LIMIT: u8 = 0x01;
            pub const ALLOCATE: u8 = 0x02;
        }
    }
}

pub mod register_defaults {
    pub const ID: u8 = 0x03;
    pub const PREFIX: u32 = (ID as u32) << 24;

    pub mod general_purpose {
        pub const ID: u8 = 0x01;
        pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 16);

        // pub const ZERO: u8 = 0x00;
        pub const AT: u8 = 0x01;
        pub const V0: u8 = 0x02;
        pub const V1: u8 = 0x03;
        pub const A0: u8 = 0x04;
        pub const A1: u8 = 0x05;
        pub const A2: u8 = 0x06;
        pub const A3: u8 = 0x07;
        pub const T0: u8 = 0x08;
        pub const T1: u8 = 0x09;
        pub const T2: u8 = 0x0a;
        pub const T3: u8 = 0x0b;
        pub const T4: u8 = 0x0c;
        pub const T5: u8 = 0x0d;
        pub const T6: u8 = 0x0e;
        pub const T7: u8 = 0x0f;
        pub const S0: u8 = 0x10;
        pub const S1: u8 = 0x11;
        pub const S2: u8 = 0x12;
        pub const S3: u8 = 0x13;
        pub const S4: u8 = 0x14;
        pub const S5: u8 = 0x15;
        pub const S6: u8 = 0x16;
        pub const S7: u8 = 0x17;
        pub const T8: u8 = 0x18;
        pub const T9: u8 = 0x19;
        pub const K0: u8 = 0x1a;
        pub const K1: u8 = 0x1b;
        pub const GP: u8 = 0x1c;
        pub const SP: u8 = 0x1d;
        pub const FP: u8 = 0x1e;
        pub const RA: u8 = 0x1f;
    }

    pub mod coprocessor_0 {
        pub const ID: u8 = 0x02;
        pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 16);

        pub const VADDR: u8 = 0x08;
        pub const STATUS: u8 = 0x0c;
        pub const CAUSE: u8 = 0x0d;
        pub const EPC: u8 = 0x0e;
    }

    pub mod coprocessor_1 {
        pub const ID: u8 = 0x03;
        pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << 16);

        pub const F0: u8 = 0x00;
        pub const F1: u8 = 0x01;
        pub const F2: u8 = 0x02;
        pub const F3: u8 = 0x03;
        pub const F4: u8 = 0x04;
        pub const F5: u8 = 0x05;
        pub const F6: u8 = 0x06;
        pub const F7: u8 = 0x07;
        pub const F8: u8 = 0x08;
        pub const F9: u8 = 0x09;
        pub const F10: u8 = 0x0a;
        pub const F11: u8 = 0x0b;
        pub const F12: u8 = 0x0c;
        pub const F13: u8 = 0x0d;
        pub const F14: u8 = 0x0e;
        pub const F15: u8 = 0x0f;
        pub const F16: u8 = 0x10;
        pub const F17: u8 = 0x11;
        pub const F18: u8 = 0x12;
        pub const F19: u8 = 0x13;
        pub const F20: u8 = 0x14;
        pub const F21: u8 = 0x15;
        pub const F22: u8 = 0x16;
        pub const F23: u8 = 0x17;
        pub const F24: u8 = 0x18;
        pub const F25: u8 = 0x19;
        pub const F26: u8 = 0x1a;
        pub const F27: u8 = 0x1b;
        pub const F28: u8 = 0x1c;
        pub const F29: u8 = 0x1d;
        pub const F30: u8 = 0x1e;
        pub const F31: u8 = 0x1f;
    }
}
