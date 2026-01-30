use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DebugInfo;

impl DebugInfo {
    pub fn is_empty(&self) -> bool {
        // TODO
        true
    }
}
