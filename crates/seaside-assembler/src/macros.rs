macro_rules! assemble_field {
    ($value:literal ($size:literal bits) -> $machine_code:expr) => {
        $machine_code = ($machine_code << $size) | $value as seaside_type_aliases::Instruction;
    };
    ($field:ident ($size:literal bits) -> $machine_code:expr) => {
        $machine_code = ($machine_code << $size) | $field as seaside_type_aliases::Instruction;
    };
    ($field:ident #(8 bits) -> $machine_code:expr) => {
        $machine_code = ($machine_code << 8) | $field as u8 as seaside_type_aliases::Instruction;
    };
    ($field:ident #(16 bits) -> $machine_code:expr) => {
        $machine_code = ($machine_code << 16) | $field as u16 as seaside_type_aliases::Instruction;
    };
    ($x:expr; ($size:literal bits) -> $machine_code:expr) => {
        $machine_code = ($machine_code << $size) | { $x } as seaside_type_aliases::Instruction;
    };
}
pub(super) use assemble_field;
