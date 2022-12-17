use crate::{Type, Category};

/// わざ
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Move {
    pub name: String,
    pub r#type: Type,
    pub category: Category,
    pub power: u32,
    pub description: String
}