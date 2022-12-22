use serde::{Serialize, Deserialize};

/// とくせい
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Ability {
    pub name: String,
    pub description: String
}