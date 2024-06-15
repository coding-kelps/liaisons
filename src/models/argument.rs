use serde::{Deserialize, Serialize};

/// Represents an argument.
#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Argument {
    /// The extracted argument summarized as a string.
    pub summary: String,
    /// The user-generated web content from which the argument was extracted
    /// as a raw string.
    pub raw: String,
}
