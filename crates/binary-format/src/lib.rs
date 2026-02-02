pub mod de;
pub mod error;
pub mod ser;

pub use de::Deserializer;
pub use error::{Error, Result};
pub use ser::Serializer;

#[cfg(test)]
mod tests {
    use super::*;
}
