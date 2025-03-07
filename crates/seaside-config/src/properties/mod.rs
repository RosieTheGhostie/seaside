mod macros;

use macros::properties;

properties! {
    #[0x00: _] {
        #[name = "version"]
        [0x00] VERSION,
        #[name = "endian"]
        [0x01] ENDIAN,
        #[name = "project directory is CWD"]
        [0x02] PROJECT_DIRECTORY_IS_CWD,
    }

    /// Customizes the features available to the seaside engine.
    #[0x01: features, name = "Features"] {
        #[0x00: _] {
            /// Allow users to provide code and/or data relating to kernel space.
            #[name = "kernel space accessible"]
            [0x00] KERNEL_SPACE_ACCESSIBLE,
            /// Enable run-time modification of text segments.
            #[name = "self-modifying code"]
            [0x01] SELF_MODIFYING_CODE,
            /// Simulate the delay slot.
            #[name = "delay slot"]
            [0x02] DELAY_SLOT,
            /// Allow `sbrk` to free memory when given a negative argument.
            #[name = "freeable heap allocations"]
            [0x03] FREEABLE_HEAP_ALLOCATIONS,
            /// Enables displaying a crash handler when an unhandled exception is thrown.
            #[name = "show crash handler"]
            [0x04] SHOW_CRASH_HANDLER,
            /// Allow use of pseudo-instructions and formats.
            #[name = "pseudo-instructions"]
            [0x05] PSEUDO_INSTRUCTIONS,
        }

        /// Controls which system services are available to the seaside engine.
        #[0x01: syscalls, name = "Syscalls"] {
            #[0x00: _] {}

            #[extra_fns(service_id, full_name_of, all_full_names_and_service_ids)]
            #[0x01: mars_print, name = "MARS Print"] {
                #[name = "int"]
                [0x00] INT,
                #[name = "uint"]
                [0x01] UINT,
                #[name = "bin"]
                [0x02] BIN,
                #[name = "hex"]
                [0x03] HEX,
                #[name = "float"]
                [0x04] FLOAT,
                #[name = "double"]
                [0x05] DOUBLE,
                #[name = "char"]
                [0x06] CHAR,
                #[name = "string"]
                [0x07] STRING,
            }

            #[extra_fns(service_id, full_name_of, all_full_names_and_service_ids)]
            #[0x02: mars_read, name = "MARS Read"] {
                #[name = "int"]
                [0x00] INT,
                #[name = "float"]
                [0x01] FLOAT,
                #[name = "double"]
                [0x02] DOUBLE,
                #[name = "char"]
                [0x03] CHAR,
                #[name = "string"]
                [0x04] STRING,
            }

            #[extra_fns(service_id, full_name_of, all_full_names_and_service_ids)]
            #[0x03: mars_file, name = "MARS File"] {
                #[name = "open"]
                [0x00] OPEN,
                #[name = "read"]
                [0x01] READ,
                #[name = "write"]
                [0x02] WRITE,
                #[name = "close"]
                [0x03] CLOSE,
            }

            #[extra_fns(service_id, full_name_of, all_full_names_and_service_ids)]
            #[0x04: mars_input_dialog, name = "MARS Input Dialog"] {
                #[name = "confirm"]
                [0x00] CONFIRM,
                #[name = "int"]
                [0x01] INT,
                #[name = "float"]
                [0x02] FLOAT,
                #[name = "double"]
                [0x03] DOUBLE,
                #[name = "string"]
                [0x04] STRING,
            }

            #[extra_fns(service_id, full_name_of, all_full_names_and_service_ids)]
            #[0x05: mars_message_dialog, name = "MARS Message Dialog"] {
                #[name = "general"]
                [0x00] GENERAL,
                #[name = "int"]
                [0x01] INT,
                #[name = "float"]
                [0x02] FLOAT,
                #[name = "double"]
                [0x03] DOUBLE,
                #[name = "string"]
                [0x04] STRING,
            }

            #[extra_fns(service_id, full_name_of, all_full_names_and_service_ids)]
            #[0x06: mars_system, name = "MARS System"] {
                #[name = "sbrk"]
                [0x00] SBRK,
                #[name = "exit"]
                [0x01] EXIT,
                #[name = "exit 2"]
                [0x02] EXIT_2,
                #[name = "time"]
                [0x03] TIME,
                #[name = "sleep"]
                [0x04] SLEEP,
                #[name = "MIDI out"]
                [0x05] MIDI_OUT,
                #[name = "MIDI out (sync)"]
                [0x06] MIDI_OUT_SYNC,
            }

            #[extra_fns(service_id, full_name_of, all_full_names_and_service_ids)]
            #[0x07: mars_random, name = "MARS Random"] {
                #[name = "set seed"]
                [0x00] SET_SEED,
                #[name = "int"]
                [0x01] INT,
                #[name = "int (range)"]
                [0x02] INT_RANGE,
                #[name = "float"]
                [0x03] FLOAT,
                #[name = "double"]
                [0x04] DOUBLE,
            }

            ---

            pub fn all_full_names_and_service_ids() -> impl Iterator<Item = (String, u16)> {
                mars_print::all_full_names_and_service_ids()
                    .chain(mars_read::all_full_names_and_service_ids())
                    .chain(mars_file::all_full_names_and_service_ids())
                    .chain(mars_input_dialog::all_full_names_and_service_ids())
                    .chain(mars_message_dialog::all_full_names_and_service_ids())
                    .chain(mars_system::all_full_names_and_service_ids())
                    .chain(mars_random::all_full_names_and_service_ids())
            }
        }
    }

    #[0x02: memory_map, name = "Memory Map"] {
        #[0x00: _] {
            #[name = "exception handler"]
            [0x00] EXCEPTION_HANDLER,
        }

        #[0x01: user_space, name = "User Space"] {
            #[name = "base"]
            [0x00] BASE,
            #[name = "limit"]
            [0x01] LIMIT,
        }

        #[0x02: kernel_space, name = "Kernel Space"] {
            #[name = "base"]
            [0x00] BASE,
            #[name = "limit"]
            [0x01] LIMIT,
        }

        #[0x03: segments, name = "Segments"] {
            #[0x00: _] {}

            #[0x01: text, name = ".text"] {
                #[name = "base"]
                [0x00] BASE,
                #[name = "limit"]
                [0x01] LIMIT,
                #[name = "allocate"]
                [0x02] ALLOCATE,
            }

            #[0x02: r#extern, name = ".extern"] {
                #[name = "base"]
                [0x00] BASE,
                #[name = "limit"]
                [0x01] LIMIT,
                #[name = "allocate"]
                [0x02] ALLOCATE,
            }

            #[0x03: data, name = ".data"] {
                #[name = "base"]
                [0x00] BASE,
                #[name = "limit"]
                [0x01] LIMIT,
                #[name = "allocate"]
                [0x02] ALLOCATE,
            }

            #[0x04: runtime_data, name = "Runtime Data"] {
                #[name = "base"]
                [0x00] BASE,
                #[name = "limit"]
                [0x01] LIMIT,
                #[name = "heap size"]
                [0x02] HEAP_SIZE,
                #[name = "stack size"]
                [0x03] STACK_SIZE,
            }

            #[0x05: ktext, name = ".ktext"] {
                #[name = "base"]
                [0x00] BASE,
                #[name = "limit"]
                [0x01] LIMIT,
                #[name = "allocate"]
                [0x02] ALLOCATE,
            }

            #[0x06: kdata, name = ".kdata"] {
                #[name = "base"]
                [0x00] BASE,
                #[name = "limit"]
                [0x01] LIMIT,
                #[name = "allocate"]
                [0x02] ALLOCATE,
            }

            #[0x07: mmio, name = "MMIO"] {
                #[name = "base"]
                [0x00] BASE,
                #[name = "limit"]
                [0x01] LIMIT,
                #[name = "allocate"]
                [0x02] ALLOCATE,
            }
        }
    }

    #[0x03: register_defaults, name = "Register Defaults"] {
        #[0x00: _] {
            #[name = "hi"]
            [0x00] HI,
            #[name = "lo"]
            [0x01] LO,
        }

        #[0x01: general_purpose, name = "General Purpose"] {
            // #[name = "$zero"]
            // [0x00] ZERO,
            #[name = "$at"]
            [0x01] AT,
            #[name = "$v0"]
            [0x02] V0,
            #[name = "$v1"]
            [0x03] V1,
            #[name = "$a0"]
            [0x04] A0,
            #[name = "$a1"]
            [0x05] A1,
            #[name = "$a2"]
            [0x06] A2,
            #[name = "$a3"]
            [0x07] A3,
            #[name = "$t0"]
            [0x08] T0,
            #[name = "$t1"]
            [0x09] T1,
            #[name = "$t2"]
            [0x0a] T2,
            #[name = "$t3"]
            [0x0b] T3,
            #[name = "$t4"]
            [0x0c] T4,
            #[name = "$t5"]
            [0x0d] T5,
            #[name = "$t6"]
            [0x0e] T6,
            #[name = "$t7"]
            [0x0f] T7,
            #[name = "$s0"]
            [0x10] S0,
            #[name = "$s1"]
            [0x11] S1,
            #[name = "$s2"]
            [0x12] S2,
            #[name = "$s3"]
            [0x13] S3,
            #[name = "$s4"]
            [0x14] S4,
            #[name = "$s5"]
            [0x15] S5,
            #[name = "$s6"]
            [0x16] S6,
            #[name = "$s7"]
            [0x17] S7,
            #[name = "$t8"]
            [0x18] T8,
            #[name = "$t9"]
            [0x19] T9,
            #[name = "$k0"]
            [0x1a] K0,
            #[name = "$k1"]
            [0x1b] K1,
            #[name = "$gp"]
            [0x1c] GP,
            #[name = "$sp"]
            [0x1d] SP,
            #[name = "$fp"]
            [0x1e] FP,
            #[name = "$ra"]
            [0x1f] RA,
        }

        #[0x02: coprocessor_0, name = "Coprocessor 0"] {
            #[name = "$vaddr"]
            [0x08] VADDR,
            #[name = "$status"]
            [0x0c] STATUS,
            #[name = "$cause"]
            [0x0d] CAUSE,
            #[name = "$epc"]
            [0x0e] EPC,
        }

        #[0x03: coprocessor_1, name = "Coprocessor 1"] {
            #[name = "f0"]
            [0x00] F0,
            #[name = "f1"]
            [0x01] F1,
            #[name = "f2"]
            [0x02] F2,
            #[name = "f3"]
            [0x03] F3,
            #[name = "f4"]
            [0x04] F4,
            #[name = "f5"]
            [0x05] F5,
            #[name = "f6"]
            [0x06] F6,
            #[name = "f7"]
            [0x07] F7,
            #[name = "f8"]
            [0x08] F8,
            #[name = "f9"]
            [0x09] F9,
            #[name = "f10"]
            [0x0a] F10,
            #[name = "f11"]
            [0x0b] F11,
            #[name = "f12"]
            [0x0c] F12,
            #[name = "f13"]
            [0x0d] F13,
            #[name = "f14"]
            [0x0e] F14,
            #[name = "f15"]
            [0x0f] F15,
            #[name = "f16"]
            [0x10] F16,
            #[name = "f17"]
            [0x11] F17,
            #[name = "f18"]
            [0x12] F18,
            #[name = "f19"]
            [0x13] F19,
            #[name = "f20"]
            [0x14] F20,
            #[name = "f21"]
            [0x15] F21,
            #[name = "f22"]
            [0x16] F22,
            #[name = "f23"]
            [0x17] F23,
            #[name = "f24"]
            [0x18] F24,
            #[name = "f25"]
            [0x19] F25,
            #[name = "f26"]
            [0x1a] F26,
            #[name = "f27"]
            [0x1b] F27,
            #[name = "f28"]
            [0x1c] F28,
            #[name = "f29"]
            [0x1d] F29,
            #[name = "f30"]
            [0x1e] F30,
            #[name = "f31"]
            [0x1f] F31,
        }
    }
}
