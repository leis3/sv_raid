mod raid;
mod pokemon;

pub use raid::Raid;
pub use pokemon::{Type, Category, Ability, Move};


/// テラレイドに出現するポケモンの情報を返す。
/// ポケモンの名前`name`と難易度`star`を指定する。
pub fn raid(name: &str, star: u8) -> Option<Raid> {
    raid_filter(Some(name.to_string()), Some(star)).get(0).cloned()
}

/// 指定した`name`や`star`で一致するテラレイドのリストを返す。
pub fn raid_filter(name: Option<String>, star: Option<u8>) -> Vec<Raid> {
    raid::raid_data().into_iter().filter(|r| {
        name.as_ref().map_or(true, |s| r.name == s.as_ref()) &&
        star.map_or(true, |s| r.star == s)
    }).collect()
}
