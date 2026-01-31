pub mod label;
pub mod procedure;
pub mod source;
pub mod r#static;
pub mod variable;

pub use label::Label;
pub use procedure::Procedure;
pub use source::Source;
pub use r#static::Static;
pub use variable::Variable;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DebugInfo {
    pub sources: Vec<Source>,
    pub labels: Vec<Label>,
    pub procedures: BTreeMap<String, Procedure>,
    pub statics: BTreeMap<String, Static>,
}
