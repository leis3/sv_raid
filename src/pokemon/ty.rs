/// ポケモンのタイプ
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy
}

impl TryFrom<&str> for Type {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Type::*;
        match value {
            "ノーマル" => Ok(Normal),
            "かくとう" => Ok(Fighting),
            "ひこう" => Ok(Flying),
            "どく" => Ok(Poison),
            "じめん" => Ok(Ground),
            "いわ" => Ok(Rock),
            "むし" => Ok(Bug),
            "ゴースト" => Ok(Ghost),
            "はがね" => Ok(Steel),
            "ほのお" => Ok(Fire),
            "みず" => Ok(Water),
            "くさ" => Ok(Grass),
            "でんき" => Ok(Electric),
            "エスパー" => Ok(Psychic),
            "こおり" => Ok(Ice),
            "ドラゴン" => Ok(Dragon),
            "あく" => Ok(Dark),
            "フェアリー" => Ok(Fairy),
            _ => Err("unexpected Type")
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Type::*;
        let s = match self {
            Normal => "ノーマル",
            Fighting => "かくとう",
            Flying => "ひこう",
            Poison => "どく",
            Ground => "じめん",
            Rock => "いわ",
            Bug => "むし",
            Ghost => "ゴースト",
            Steel => "はがね",
            Fire => "ほのお",
            Water => "みず",
            Grass => "くさ",
            Electric => "でんき",
            Psychic => "エスパー",
            Ice => "こおり",
            Dragon => "ドラゴン",
            Dark => "あく",
            Fairy => "フェアリー"
        };
        write!(f, "{}", s)
    }
}