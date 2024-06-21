use serde::{Deserialize, Serialize};

/// Represents an argument.
#[derive(Serialize, Deserialize, Clone)]
pub struct Argument {
    /// The extracted argument summarized as a string.
    pub summary: String,
    /// The user-generated web content from which the argument was extracted
    /// as a raw string.
    pub raw: String,
}

impl Argument {
    pub fn new(summary: String, raw: String) -> Self {
        Self {
            summary: summary,
            raw: raw,
        }
    }
}