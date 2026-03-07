use seaside_core::prelude::*;
use serde::{Deserialize, Serialize};

use super::Variable;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Procedure {
    pub address: Address,
    pub len: Size,
    pub stack_size: Size,
    pub parameters: Vec<Variable>,
    pub locals: Vec<Variable>,
}
