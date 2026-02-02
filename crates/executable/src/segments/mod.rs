pub use segment::Segment;

mod segment;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Segments {
    pub text: Segment,

    #[serde(default)]
    pub ktext: Option<Segment>,

    #[serde(default)]
    pub r#extern: Option<Segment>,

    #[serde(default)]
    pub data: Option<Segment>,

    #[serde(default)]
    pub kdata: Option<Segment>,
}
