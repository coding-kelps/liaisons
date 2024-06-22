use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Relation {
    pub arg_a_id: u32,
    pub arg_b_id: u32,
    pub relation_type: RelationType,
    pub confidence: f32,
    pub explanation: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum RelationType {
    Support,
    Attack,
    NotRelated,
}

impl RelationType {
    pub fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "support" => RelationType::Support,
            "attack" => RelationType::Attack,
            "not related" => RelationType::NotRelated,
            _ => RelationType::NotRelated,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            RelationType::Support => "Support",
            RelationType::Attack => "Attack",
            RelationType::NotRelated => "NotRelated",
        }
    }
}
