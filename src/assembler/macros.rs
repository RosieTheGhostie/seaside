macro_rules! assemble_field {
    ($value:literal ($size:literal bits) -> $machine_code:expr) => {
        $machine_code = ($machine_code << $size) | $value as u32;
    };
    ($field:ident ($size:literal bits) -> $machine_code:expr) => {
        $machine_code = ($machine_code << $size) | $field as u32;
    };
    ($field:ident #(8 bits) -> $machine_code:expr) => {
        $machine_code = ($machine_code << 8) | $field as u8 as u32;
    };
    ($field:ident #(16 bits) -> $machine_code:expr) => {
        $machine_code = ($machine_code << 16) | $field as u16 as u32;
    };
    ($x:expr; ($size:literal bits) -> $machine_code:expr) => {
        $machine_code = ($machine_code << $size) | { $x } as u32;
    };
}
pub(super) use assemble_field;
