use serde::{Deserialize, Serialize};

/// Represents an argument.
#[derive(Serialize, Deserialize, Clone)]
pub struct Argument {
    pub id: Option<u32>,
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
            id: None,
            summarized_info: summarized_info,
            raw: raw,
        }
    }

    pub fn with_id(id: u32, summarized_info: SummarizedInfo, raw: String) -> Self {
        Self {
            id: Some(id),
            summarized_info: summarized_info,
            raw: raw,
        }
    }
}