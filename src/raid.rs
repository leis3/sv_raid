use crate::{Type, Ability, Move};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Raid {
    /// 名前
    pub name: String,
    /// 難易度
    pub star: u8,
    /// タイプ
    pub types: Vec<Type>,
    /// とくせい
    pub abilities: Vec<Ability>,
    /// わざ
    pub moves: Vec<Move>,
    /// 全体行動
    pub actions: Vec<String>,
    /// HP
    pub hp: u32,
    /// こうげき
    pub atk: u32,
    /// ぼうぎょ
    pub def: u32,
    /// とくこう
    pub sp_atk: u32,
    /// とくぼう
    pub sp_def: u32,
    /// すばやさ
    pub speed: u32
}