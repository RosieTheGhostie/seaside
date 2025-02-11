#[cfg(feature = "fields")]
pub mod fields;

use attribute_pseudoblock::pseudoblock;

pseudoblock! {
    #![cfg(feature = "disassembler")]
    mod destructure;

    use destructure::destructure;
    use seaside_type_aliases::{Address, Instruction};

    pub fn disassemble_advanced(
        instruction: Instruction,
        address: Address,
        prefix_with_address: bool,
    ) -> Option<String> {
        let destructured = destructure(instruction, address)?;
        Some(if prefix_with_address {
            format!("{address:08x} | {destructured}")
        } else {
            destructured.to_string()
        })
    }

    pub fn disassemble(instruction: Instruction) -> Option<String> {
        Some(destructure(instruction, 0x00000000)?.to_string())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn sll() {
            let disassembled = disassemble(0x000a4a80).expect("disassembly failed");
            assert_eq!(disassembled, "sll $t1, $t2, 10");
        }

        #[test]
        fn add() {
            let disassembled = disassemble(0x012a4020).expect("disassembly failed");
            assert_eq!(disassembled, "add $t0, $t1, $t2");
        }

        #[test]
        fn bgez() {
            let disassembled = disassemble(0x06010045).expect("disassembly failed");
            assert_eq!(disassembled, "bgez $s0, 0x0045");
        }

        #[test]
        fn addiu() {
            let disassembled = disassemble(0x24020004).expect("disassembly failed");
            assert_eq!(disassembled, "addiu $v0, $zero, 4");
        }

        #[test]
        fn add_s() {
            let disassembled = disassemble(0x46051840).expect("disassembly failed");
            assert_eq!(disassembled, "add.s $f1, $f3, $f5");
        }

        #[test]
        fn bc1t() {
            let disassembled = disassemble(0x45190013).expect("disassembly failed");
            assert_eq!(disassembled, "bc1t 6, 0x0013");
        }

        #[test]
        fn movt_s() {
            let disassembled = disassemble(0x46190811).expect("disassembly failed");
            assert_eq!(disassembled, "movt.s $f0, $f1, 6");
        }

        #[test]
        fn lb() {
            let disassembled = disassemble(0x83a80004).expect("disassembly failed");
            assert_eq!(disassembled, "lb $t0, 4($sp)");
        }
    }
}
