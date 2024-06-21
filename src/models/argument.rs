use serde::{Deserialize, Serialize};

/// Represents an argument.
#[derive(Serialize, Deserialize, Clone)]
pub struct Argument {
    pub summarized_info: SummarizedInfo,
    /// The user-generated web content from which the argument was extracted
    /// as a raw string.
    pub raw: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SummarizedInfo {
    pub title: String,
    /// The extracted argument summarized as a string.
    pub summary: String,
}

impl Argument {
    pub fn new(summarized_info: SummarizedInfo, raw: String) -> Self {
        Self {
            summarized_info: summarized_info,
            raw: raw,
        }
    }
}