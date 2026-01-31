use core::ops::Range;

use seaside_type_aliases::Address;
use serde::{Deserialize, Serialize};

use super::variable::DisplayMode;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Static {
    pub display_mode: DisplayMode,
    pub address: Address,
    pub source_index: u64,
    pub lines: Range<u64>,
}
