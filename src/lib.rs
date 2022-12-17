mod data;
mod pokemon;
mod raid;
mod error;

pub use raid::Raid;
pub use pokemon::{Type, Category, Ability, Move};

use error::Error;
use itertools::Itertools;

/// テラレイドに出現するポケモンの情報を返す。
/// ポケモンの名前`name`と難易度`star`を指定する。
pub fn raid(name: &str, star: u8) -> Result<Raid, Error> {
    let mv = data::mv();
    let abl = data::abl();
    
    // 全レイドのうち、指定したポケモンが出現するものを取得する
    let Some(v) = data::raid().get(&star).map(|r| r.clone()) else {
        return Err(Error::RankNotFound);
    };

    let Some(raid_info) = v.iter().find(|r| r.name == name) else {
        return Err(Error::NameNotFound);
    };

    let types = raid_info.types.iter().map(|s| {
        Type::try_from(s.as_str()).unwrap()
    }).collect_vec();

    let moves = raid_info.moves.iter().map(|s| {
        mv.iter().find(|m| &m.name == s).unwrap().clone()
    }).collect_vec();

    let abilities = raid_info.abilities.iter().map(|s| {
        abl.iter().find(|a| &a.name == s).unwrap().clone()
    }).collect_vec();

    Ok(Raid {
        name: name.to_string(),
        star, types, abilities, moves,
        actions: raid_info.actions.clone(),
        hp: raid_info.hp,
        atk: raid_info.atk,
        def: raid_info.def,
        sp_atk: raid_info.sp_atk,
        sp_def: raid_info.sp_def,
        speed: raid_info.speed,
    })
}