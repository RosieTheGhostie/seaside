pub use data::DataSegment;
pub use text::TextSegment;

mod data;
mod text;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Segments {
    pub text: TextSegment,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ktext: Option<TextSegment>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#extern: Option<DataSegment>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<DataSegment>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kdata: Option<DataSegment>,
}
