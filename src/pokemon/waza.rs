use crate::{Type, Category};
use serde::{Serialize, Deserialize};

/// わざ
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Move {
    pub name: String,
    pub r#type: Type,
    pub category: Category,
    pub power: u32,
    pub description: String
}