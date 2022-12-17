/// わざの種類
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Category {
    Physical,
    Special,
    Status
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Category::*;
        let c = match self {
            Physical => "物理",
            Special => "特殊",
            Status => "変化"
        };
        write!(f, "{}", c)
    }
}