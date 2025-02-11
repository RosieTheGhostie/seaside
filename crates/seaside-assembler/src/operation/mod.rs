pub mod basic_operator;
pub mod macros;
pub mod operand;

pub use basic_operator::BasicOperator;
pub use operand::Operand;

use super::Error;
use macros::register_immediate;

pub fn convert_address(operator: BasicOperator, address: u32, pc: u32) -> Result<Operand, Error> {
    use BasicOperator::*;
    match operator {
        Jump | JumpAndLink => Ok(Operand::JumpIndex(
            address_to_index(address, pc).ok_or(Error::BranchTooLarge)?,
        )),
        register_immediate![
            BranchLessThanZero,
            BranchGreaterEqualZero,
            BranchLessThanZeroAndLink,
            BranchGreaterEqualZeroAndLink
        ]
        | BranchEqual
        | BranchNotEqual
        | BranchLessEqualZero
        | BranchGreaterThanZero
        | LoadByte
        | LoadHalf
        | LoadWordLeft
        | LoadWord
        | LoadByteUnsigned
        | LoadHalfUnsigned
        | LoadWordRight
        | StoreByte
        | StoreHalf
        | StoreWordLeft
        | StoreWord
        | StoreConditional
        | StoreWordRight
        | LoadLinked
        | LoadWordCoprocessor1
        | LoadDoubleCoprocessor1
        | StoreWordCoprocessor1
        | StoreDoubleCoprocessor1 => Ok(Operand::I16(
            address_to_offset(address, pc).ok_or(Error::BranchTooLarge)?,
        )),
        _ => Err(Error::InternalLogicIssue),
    }
}

fn address_to_offset(address: u32, pc: u32) -> Option<i16> {
    let offset = (address as i32 - pc as i32) / 4 - 1;
    <i32 as TryInto<i16>>::try_into(offset).ok()
}

fn address_to_index(address: u32, pc: u32) -> Option<u32> {
    if (address ^ pc.checked_add(4)?) & 0xf0000000 == 0 {
        Some((address & 0xf0000000) | ((address & 0x0fffffff) >> 2))
    } else {
        None
    }
}
