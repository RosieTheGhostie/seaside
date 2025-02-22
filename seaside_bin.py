from enum import IntEnum


class ServiceSubId(IntEnum):
    PRINT_INT = 0x0100
    PRINT_UINT = 0x0101
    PRINT_BIN = 0x0102
    PRINT_HEX = 0x0103
    PRINT_FLOAT = 0x0104
    PRINT_DOUBLE = 0x0105
    PRINT_CHAR = 0x0106
    PRINT_STRING = 0x0107
    READ_INT = 0x0200
    READ_FLOAT = 0x0201
    READ_DOUBLE = 0x0202
    READ_CHAR = 0x0203
    READ_STRING = 0x0204
    FILE_OPEN = 0x0300
    FILE_READ = 0x0301
    FILE_WRITE = 0x0302
    FILE_CLOSE = 0x0303
    INPUT_DIALOG_CONFIRM = 0x0400
    INPUT_DIALOG_INT = 0x0401
    INPUT_DIALOG_FLOAT = 0x0402
    INPUT_DIALOG_DOUBLE = 0x0403
    INPUT_DIALOG_STRING = 0x0404
    MESSAGE_DIALOG_GENERAL = 0x0500
    MESSAGE_DIALOG_INT = 0x0501
    MESSAGE_DIALOG_FLOAT = 0x0502
    MESSAGE_DIALOG_DOUBLE = 0x0503
    MESSAGE_DIALOG_STRING = 0x0504
    SBRK = 0x0600
    EXIT = 0x0601
    EXIT_2 = 0x0602
    TIME = 0x0603
    SLEEP = 0x0604
    MIDI_OUT = 0x0605
    MIDI_OUT_SYNC = 0x0606
    SET_SEED = 0x0700
    RAND_INT = 0x0701
    RAND_INT_RANGE = 0x0702
    RAND_FLOAT = 0x0703
    RAND_DOUBLE = 0x0704


def version_as_bytes(major: int, minor: int, patch: int) -> bytes:
    patch_bytes: bytes = patch.to_bytes(2, "little")
    return bytes((patch_bytes[0], patch_bytes[1], minor, major))


def u16(x: int, /) -> bytes:
    return x.to_bytes(2, "little")


def u32(x: int, /) -> bytes:
    return x.to_bytes(4, "little")


KIB: int = 1024
MIB: int = KIB * KIB


PROPERTIES: dict[int, bytes] = {
    # --- / ---
    0x00000000: version_as_bytes(1, 2, 0),  # version: Version
    0x00000001: b"\x00",  # endian: Endian
    0x00000002: b"\x01",  # project_directory_is_cwd: bool
    # --- /features/ ---
    0x01000000: b"\x01",  # kernel_space_accessible: bool
    0x01000001: b"\x00",  # self_modifying_code: bool
    0x01000002: b"\x00",  # delay_slot: bool
    0x01000003: b"\x01",  # freeable_heap_allocations: bool
    0x01000004: b"\x01",  # show_crash_handler: bool
    # --- /features/assembler/ ---
    0x01010000: b"\x00",  # pseudo_instructions: bool
    # --- /features/assembler/directives/ ---
    0x01010100: b"\x01",  # asciiz: bool
    0x01010101: b"\x00",  # eqv: bool
    0x01010102: b"\x01",  # global: bool
    0x01010103: b"\x01",  # include: bool
    0x01010104: b"\x00",  # macros: bool
    0x01010105: b"\x00",  # set: bool
    # --- /features/syscalls/mars_print/ ---
    0x01020100: u32(1),   # int: u32
    0x01020101: u32(36),  # uint: u32
    0x01020102: u32(35),  # bin: u32
    0x01020103: u32(34),  # hex: u32
    0x01020104: u32(2),   # float: u32
    0x01020105: u32(3),   # double: u32
    0x01020106: u32(11),  # char: u32
    0x01020107: u32(4),   # string: u32
    # --- /features/syscalls/mars_read/ ---
    0x01020200: u32(5),   # int: u32
    0x01020201: u32(6),   # float: u32
    0x01020202: u32(7),   # double: u32
    0x01020203: u32(12),  # char: u32
    0x01020204: u32(8),   # string: u32
    # --- /features/syscalls/mars_file/ ---
    0x01020300: u32(13),  # open: u32
    0x01020301: u32(14),  # read: u32
    0x01020302: u32(15),  # write: u32
    0x01020303: u32(16),  # close: u32
    # --- /features/syscalls/mars_input_dialog/ ---
    0x01020400: u32(50),  # confirm: u32
    0x01020401: u32(51),  # int: u32
    0x01020402: u32(52),  # float: u32
    0x01020403: u32(53),  # double: u32
    0x01020404: u32(54),  # string: u32
    # --- /features/syscalls/mars_message_dialog/ ---
    0x01020500: u32(55),  # general: u32
    0x01020501: u32(56),  # int: u32
    0x01020502: u32(57),  # float: u32
    0x01020503: u32(58),  # double: u32
    0x01020504: u32(59),  # string: u32
    # --- /features/syscalls/mars_system/ ---
    0x01020600: u32(9),   # sbrk: u32
    0x01020601: u32(10),  # exit: u32
    0x01020602: u32(17),  # exit_2: u32
    0x01020603: u32(30),  # time: u32
    0x01020604: u32(32),  # sleep: u32
    0x01020605: u32(31),  # midi_out: u32
    0x01020606: u32(33),  # midi_out_sync: u32
    # --- /features/syscalls/mars_random/ ---
    0x01020700: u32(40),  # set_seed: u32
    0x01020701: u32(41),  # rand_int: u32
    0x01020702: u32(42),  # rand_int_range: u32
    0x01020703: u32(43),  # rand_float: u32
    0x01020704: u32(44),  # rand_double: u32
    # --- /memory_map/ ---
    0x02000000: u32(0x80000180),  # exception_handler: Address
    # --- /memory_map/user_space/ ---
    0x02010000: u32(0x00000000),  # base: Address
    0x02010001: u32(0x7fffffff),  # limit: Address
    # --- /memory_map/kernel_space/ ---
    0x02020000: u32(0x80000000),  # base: Address
    0x02020001: u32(0xffffffff),  # limit: Address
    # --- /memory_map/segments/text/ ---
    0x02030100: u32(0x00400000),  # base: Address
    0x02030101: u32(0x0ffffffc),  # limit: Address
    0x02030102: u32(8 * MIB),     # allocate: u32
    # --- /memory_map/segments/extern/ ---
    0x02030200: u32(0x10000000),  # base: Address
    0x02030201: u32(0x1000ffff),  # limit: Address
    0x02030202: u32(64 * KIB),    # allocate: u32
    # --- /memory_map/segments/data/ ---
    0x02030300: u32(0x10010000),  # base: Address
    0x02030301: u32(0x1003ffff),  # limit: Address
    0x02030302: u32(192 * KIB),   # allocate: u32
    # --- /memory_map/segments/runtime_data/ ---
    0x02030400: u32(0x10040000),  # base: Address
    0x02030401: u32(0x7fffffff),  # limit: Address
    0x02030402: u32(128 * KIB),   # heap_size: u32
    0x02030403: u32(4 * MIB),     # stack_size: u32
    # --- /memory_map/segments/ktext/ ---
    0x02030500: u32(0x80000000),  # base: Address
    0x02030501: u32(0x8fffffff),  # limit: Address
    0x02030502: u32(1 * MIB),     # allocate: u32
    # --- /memory_map/segments/kdata/ ---
    0x02030600: u32(0x90000000),  # base: Address
    0x02030601: u32(0xfffeffff),  # limit: Address
    0x02030602: u32(1 * MIB),     # allocate: u32
    # --- /memory_map/segments/mmio/ ---
    0x02030700: u32(0xffff0000),  # base: Address
    0x02030701: u32(0xffffffff),  # limit: Address
    0x02030702: u32(4 * KIB),     # allocate: u32
    # --- /register_defaults/general_purpose/ ---
    0x03010001: u32(0x00000000),  # at: u32
    0x03010002: u32(0x00000000),  # v0: u32
    0x03010003: u32(0x00000000),  # v1: u32
    0x03010004: u32(0x00000000),  # a0: u32
    0x03010005: u32(0x00000000),  # a1: u32
    0x03010006: u32(0x00000000),  # a2: u32
    0x03010007: u32(0x00000000),  # a3: u32
    0x03010008: u32(0x00000000),  # t0: u32
    0x03010009: u32(0x00000000),  # t1: u32
    0x0301000a: u32(0x00000000),  # t2: u32
    0x0301000b: u32(0x00000000),  # t3: u32
    0x0301000c: u32(0x00000000),  # t4: u32
    0x0301000d: u32(0x00000000),  # t5: u32
    0x0301000e: u32(0x00000000),  # t6: u32
    0x0301000f: u32(0x00000000),  # t7: u32
    0x03010010: u32(0x00000000),  # s0: u32
    0x03010011: u32(0x00000000),  # s1: u32
    0x03010012: u32(0x00000000),  # s2: u32
    0x03010013: u32(0x00000000),  # s3: u32
    0x03010014: u32(0x00000000),  # s4: u32
    0x03010015: u32(0x00000000),  # s5: u32
    0x03010016: u32(0x00000000),  # s6: u32
    0x03010017: u32(0x00000000),  # s7: u32
    0x03010018: u32(0x00000000),  # t8: u32
    0x03010019: u32(0x00000000),  # t9: u32
    0x0301001a: u32(0x00000000),  # k0: u32
    0x0301001b: u32(0x00000000),  # k1: u32
    0x0301001c: u32(0x10008000),  # gp: u32
    0x0301001d: u32(0x7fffeffc),  # sp: u32
    0x0301001e: u32(0x00000000),  # fp: u32
    0x0301001f: u32(0x00000000),  # ra: u32
    # --- /register_defaults/coprocessor_0/ ---
    0x03020008: u32(0x00000000),  # vaddr: u32
    0x0302000c: u32(0x0000ff11),  # status: u32
    0x0302000d: u32(0x00000000),  # cause: u32
    0x0302000e: u32(0x00000000),  # epc: u32
    # --- /register_defaults/coprocessor_1/ ---
    0x03030000: u32(0x00000000),  # f0: u32
    0x03030001: u32(0x00000000),  # f1: u32
    0x03030002: u32(0x00000000),  # f2: u32
    0x03030003: u32(0x00000000),  # f3: u32
    0x03030004: u32(0x00000000),  # f4: u32
    0x03030005: u32(0x00000000),  # f5: u32
    0x03030006: u32(0x00000000),  # f6: u32
    0x03030007: u32(0x00000000),  # f7: u32
    0x03030008: u32(0x00000000),  # f8: u32
    0x03030009: u32(0x00000000),  # f9: u32
    0x0303000a: u32(0x00000000),  # f10: u32
    0x0303000b: u32(0x00000000),  # f11: u32
    0x0303000c: u32(0x00000000),  # f12: u32
    0x0303000d: u32(0x00000000),  # f13: u32
    0x0303000e: u32(0x00000000),  # f14: u32
    0x0303000f: u32(0x00000000),  # f15: u32
    0x03030010: u32(0x00000000),  # f16: u32
    0x03030011: u32(0x00000000),  # f17: u32
    0x03030012: u32(0x00000000),  # f18: u32
    0x03030013: u32(0x00000000),  # f19: u32
    0x03030014: u32(0x00000000),  # f20: u32
    0x03030015: u32(0x00000000),  # f21: u32
    0x03030016: u32(0x00000000),  # f22: u32
    0x03030017: u32(0x00000000),  # f23: u32
    0x03030018: u32(0x00000000),  # f24: u32
    0x03030019: u32(0x00000000),  # f25: u32
    0x0303001a: u32(0x00000000),  # f26: u32
    0x0303001b: u32(0x00000000),  # f27: u32
    0x0303001c: u32(0x00000000),  # f28: u32
    0x0303001d: u32(0x00000000),  # f29: u32
    0x0303001e: u32(0x00000000),  # f30: u32
    0x0303001f: u32(0x00000000),  # f31: u32
}


def main() -> None:
    with open("res/NewSeaside.bin", 'xb') as file:
        file.write(b"seaside\x00\x01\x00\x00\x00")
        for property_id, value in PROPERTIES.items():
            file.write(property_id.to_bytes(length=4, byteorder='little'))
            file.write(value)


if __name__ == "__main__":
    main()
