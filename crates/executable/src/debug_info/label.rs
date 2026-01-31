use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Label {
    pub name: String,
    pub source_index: u64,
    pub line: u64,
}
