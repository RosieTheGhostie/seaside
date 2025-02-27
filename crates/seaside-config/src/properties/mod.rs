mod macros;

use macros::properties;

properties!([0x00, level: 0] {
    [0x00] VERSION;
    [0x01] ENDIAN;
    [0x02] PROJECT_DIRECTORY_IS_CWD;

    [0x01, level: 0] features {
        [0x00] KERNEL_SPACE_ACCESSIBLE;
        [0x01] SELF_MODIFYING_CODE;
        [0x02] DELAY_SLOT;
        [0x03] FREEABLE_HEAP_ALLOCATIONS;
        [0x04] SHOW_CRASH_HANDLER;
        [0x05] PSEUDO_INSTRUCTIONS;

        [0x01, level: 1] syscalls {
            [0x01, level: 2] mars_print {
                [0x00] INT;
                [0x01] UINT;
                [0x02] BIN;
                [0x03] HEX;
                [0x04] FLOAT;
                [0x05] DOUBLE;
                [0x06] CHAR;
                [0x07] STRING;
            }

            [0x02, level: 2] mars_read {
                [0x00] INT;
                [0x01] FLOAT;
                [0x02] DOUBLE;
                [0x03] CHAR;
                [0x04] STRING;
            }

            [0x03, level: 2] mars_file {
                [0x00] OPEN;
                [0x01] READ;
                [0x02] WRITE;
                [0x03] CLOSE;
            }

            [0x04, level: 2] mars_input_dialog {
                [0x00] CONFIRM;
                [0x01] INT;
                [0x02] FLOAT;
                [0x03] DOUBLE;
                [0x04] STRING;
            }

            [0x05, level: 2] mars_message_dialog {
                [0x00] GENERAL;
                [0x01] INT;
                [0x02] FLOAT;
                [0x03] DOUBLE;
                [0x04] STRING;
            }

            [0x06, level: 2] mars_system {
                [0x00] SBRK;
                [0x01] EXIT;
                [0x02] EXIT_2;
                [0x03] TIME;
                [0x04] SLEEP;
                [0x05] MIDI_OUT;
                [0x06] MIDI_OUT_SYNC;
            }

            [0x07, level: 2] mars_random {
                [0x00] SET_SEED;
                [0x01] INT;
                [0x02] INT_RANGE;
                [0x03] FLOAT;
                [0x04] DOUBLE;
            }
        }
    }

    [0x02, level: 0] memory_map {
        [0x00] EXCEPTION_HANDLER;

        [0x01, level: 1] user_space {
            [0x00] BASE;
            [0x01] LIMIT;
        }

        [0x02, level: 1] kernel_space {
            [0x00] BASE;
            [0x01] LIMIT;
        }

        [0x03, level: 1] segments {
            [0x01, level: 2] text {
                [0x00] BASE;
                [0x01] LIMIT;
                [0x02] ALLOCATE;
            }

            [0x02, level: 2] r#extern {
                [0x00] BASE;
                [0x01] LIMIT;
                [0x02] ALLOCATE;
            }

            [0x03, level: 2] data {
                [0x00] BASE;
                [0x01] LIMIT;
                [0x02] ALLOCATE;
            }

            [0x04, level: 2] runtime_data {
                [0x00] BASE;
                [0x01] LIMIT;
                [0x02] HEAP_SIZE;
                [0x03] STACK_SIZE;
            }

            [0x05, level: 2] ktext {
                [0x00] BASE;
                [0x01] LIMIT;
                [0x02] ALLOCATE;
            }

            [0x06, level: 2] kdata {
                [0x00] BASE;
                [0x01] LIMIT;
                [0x02] ALLOCATE;
            }

            [0x07, level: 2] mmio {
                [0x00] BASE;
                [0x01] LIMIT;
                [0x02] ALLOCATE;
            }
        }
    }

    [0x03, level: 0] register_defaults {
        [0x00] HI;
        [0x01] LO;

        [0x01, level: 1] general_purpose {
            // [0x00] ZERO;
            [0x01] AT;
            [0x02] V0;
            [0x03] V1;
            [0x04] A0;
            [0x05] A1;
            [0x06] A2;
            [0x07] A3;
            [0x08] T0;
            [0x09] T1;
            [0x0a] T2;
            [0x0b] T3;
            [0x0c] T4;
            [0x0d] T5;
            [0x0e] T6;
            [0x0f] T7;
            [0x10] S0;
            [0x11] S1;
            [0x12] S2;
            [0x13] S3;
            [0x14] S4;
            [0x15] S5;
            [0x16] S6;
            [0x17] S7;
            [0x18] T8;
            [0x19] T9;
            [0x1a] K0;
            [0x1b] K1;
            [0x1c] GP;
            [0x1d] SP;
            [0x1e] FP;
            [0x1f] RA;
        }

        [0x02, level: 1] coprocessor_0 {
            [0x08] VADDR;
            [0x0c] STATUS;
            [0x0d] CAUSE;
            [0x0e] EPC;
        }

        [0x03, level: 1] coprocessor_1 {
            [0x00] F0;
            [0x01] F1;
            [0x02] F2;
            [0x03] F3;
            [0x04] F4;
            [0x05] F5;
            [0x06] F6;
            [0x07] F7;
            [0x08] F8;
            [0x09] F9;
            [0x0a] F10;
            [0x0b] F11;
            [0x0c] F12;
            [0x0d] F13;
            [0x0e] F14;
            [0x0f] F15;
            [0x10] F16;
            [0x11] F17;
            [0x12] F18;
            [0x13] F19;
            [0x14] F20;
            [0x15] F21;
            [0x16] F22;
            [0x17] F23;
            [0x18] F24;
            [0x19] F25;
            [0x1a] F26;
            [0x1b] F27;
            [0x1c] F28;
            [0x1d] F29;
            [0x1e] F30;
            [0x1f] F31;
        }
    }
});
