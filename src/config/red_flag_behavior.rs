use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Deserialize, Serialize)]
#[serde(into = "String")]
pub enum RedFlagBehavior {
    #[serde(alias = "allow")]
    Allow,
    #[default]
    #[serde(alias = "warn")]
    Warn,
    #[serde(alias = "error")]
    Error,
}

impl From<RedFlagBehavior> for String {
    fn from(value: RedFlagBehavior) -> Self {
        match value {
            RedFlagBehavior::Allow => "allow",
            RedFlagBehavior::Warn => "warn",
            RedFlagBehavior::Error => "error",
        }
        .to_string()
    }
}
