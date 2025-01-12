#![allow(dead_code)]
pub mod fields;

mod destructure;

use crate::type_aliases::instruction::Instruction;
use destructure::destructure;

pub fn disassemble_all(instructions: &[Instruction]) -> Option<Vec<String>> {
    instructions
        .iter()
        .map(|&instruction| disassemble(instruction))
        .collect()
}

pub fn disassemble(instruction: Instruction) -> Option<String> {
    Some(destructure(instruction)?.to_string())
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
