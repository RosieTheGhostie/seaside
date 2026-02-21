use std::{collections::BTreeMap, path::PathBuf};

use seaside_core::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Source {
    pub path: PathBuf,
    pub address_table: BTreeMap<u64, Address>,
}
