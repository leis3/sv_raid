use crate::{Type, Ability, Move};
use serde::{Serialize, Deserialize};

/// テラレイドバトルの情報を表す。
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

/// レイドの情報を返す
pub(crate) fn raid_data() -> Vec<Raid> {
    serde_json::from_str(
    r#"[
        {
          "name": "アマカジ",
          "star": 1,
          "types": [
            "Grass"
          ],
          "abilities": [
            {
              "name": "リーフガード",
              "description": "天気が晴れのときは状態異常にならない。"
            },
            {
              "name": "どんかん",
              "description": "鈍感なのでメロメロやちょうはつ状態にならない。いかくにも動じない。"
            },
            {
              "name": "スイートベール",
              "description": "自分と味方のポケモンは眠らなくなる。"
            }
          ],
          "moves": [
            {
              "name": "こうそくスピン",
              "type": "Normal",
              "category": "Physical",
              "power": 50,
              "description": "回転して相手を攻撃する。しめつけるまきつくやどりぎのタネなど吹きとばす。自分の素早さもあがる。"
            },
            {
              "name": "はっぱカッター",
              "type": "Grass",
              "category": "Physical",
              "power": 55,
              "description": "はっぱをとばして相手を切りつけて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "じたばた",
              "type": "Normal",
              "category": "Physical",
              "power": 1,
              "description": "じたばた暴れて攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            },
            {
              "name": "なかよくする",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手となかよくなって戦う気力を失わせ相手の攻撃をさげる。"
            }
          ],
          "actions": [],
          "hp": 160,
          "atk": 12,
          "def": 14,
          "sp_atk": 12,
          "sp_def": 14,
          "speed": 12
        },
        {
          "name": "アメタマ",
          "star": 1,
          "types": [
            "Bug",
            "Water"
          ],
          "abilities": [
            {
              "name": "すいすい",
              "description": "天気が雨のとき素早さが上がる。"
            },
            {
              "name": "あめうけざら",
              "description": "天気が雨のとき少しずつＨＰを回復する。"
            }
          ],
          "moves": [
            {
              "name": "バブルこうせん",
              "type": "Water",
              "category": "Special",
              "power": 65,
              "description": "泡を勢いよく相手に発射して攻撃する。素早さをさげることがある。"
            },
            {
              "name": "むしのていこう",
              "type": "Bug",
              "category": "Special",
              "power": 50,
              "description": "抵抗して相手を攻撃する。相手の特攻をさげる。"
            },
            {
              "name": "でんこうせっか",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "サイケこうせん",
              "type": "Psychic",
              "category": "Special",
              "power": 65,
              "description": "不思議な光線を相手に発射して攻撃する。混乱させることがある。"
            }
          ],
          "actions": [],
          "hp": 155,
          "atk": 12,
          "def": 12,
          "sp_atk": 17,
          "sp_def": 17,
          "speed": 20
        },
        {
          "name": "イワンコ",
          "star": 1,
          "types": [
            "Rock"
          ],
          "abilities": [
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            },
            {
              "name": "やるき",
              "description": "やる気をだすことによってねむり状態にならない。"
            },
            {
              "name": "ふくつのこころ",
              "description": "ひるむたびに不屈の心を燃やして素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "いわおとし",
              "type": "Rock",
              "category": "Physical",
              "power": 50,
              "description": "小さな岩を持ちあげて相手に投げつけて攻撃する。"
            },
            {
              "name": "すなかけ",
              "type": "Ground",
              "category": "Status",
              "power": 0,
              "description": "相手の顔に砂をかけて命中率をさげる。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "たいあたり",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "相手にむかって全身でぶつかっていき攻撃する。"
            }
          ],
          "actions": [],
          "hp": 160,
          "atk": 20,
          "def": 14,
          "sp_atk": 12,
          "sp_def": 14,
          "speed": 19
        },
        {
          "name": "ウソハチ",
          "star": 1,
          "types": [
            "Rock"
          ],
          "abilities": [
            {
              "name": "がんじょう",
              "description": "ＨＰが満タンのとき技を受けても一撃で倒されることがない。一撃必殺技も効かない。"
            },
            {
              "name": "いしあたま",
              "description": "反動を受ける技を出してもＨＰが減らない。"
            },
            {
              "name": "びびり",
              "description": "あくゴーストむしタイプの攻撃を受けたりいかくをされるとびびって素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "いわおとし",
              "type": "Rock",
              "category": "Physical",
              "power": 50,
              "description": "小さな岩を持ちあげて相手に投げつけて攻撃する。"
            },
            {
              "name": "じたばた",
              "type": "Normal",
              "category": "Physical",
              "power": 1,
              "description": "じたばた暴れて攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            },
            {
              "name": "まねっこ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "直前にでた技をまねして同じ技をだす。技がでていないと失敗する。"
            },
            {
              "name": "けたぐり",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "足を強くけり相手を転ばせて攻撃する。相手が重いほど威力があがる。"
            }
          ],
          "actions": [],
          "hp": 170,
          "atk": 24,
          "def": 27,
          "sp_atk": 7,
          "sp_def": 15,
          "speed": 7
        },
        {
          "name": "ウパー",
          "star": 1,
          "types": [
            "Water",
            "Ground"
          ],
          "abilities": [
            {
              "name": "どくのトゲ",
              "description": "自分に触った相手をどく状態にすることがある。"
            },
            {
              "name": "ちょすい",
              "description": "みずタイプの技を受けるとダメージを受けずに回復する。"
            },
            {
              "name": "てんねん",
              "description": "相手の能力の変化を無視して攻撃ができる。"
            }
          ],
          "moves": [
            {
              "name": "たいあたり",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "相手にむかって全身でぶつかっていき攻撃する。"
            },
            {
              "name": "マッドショット",
              "type": "Ground",
              "category": "Special",
              "power": 55,
              "description": "泥の塊を相手に投げつけて攻撃する。同時に相手の素早さをさげる。"
            },
            {
              "name": "たたきつける",
              "type": "Normal",
              "category": "Physical",
              "power": 80,
              "description": "長いしっぽやつるなどを使い相手をたたきつけて攻撃する。"
            },
            {
              "name": "ポイズンテール",
              "type": "Poison",
              "category": "Physical",
              "power": 50,
              "description": "しっぽでたたく。毒状態にすることがあり急所にも当たりやすい。"
            }
          ],
          "actions": [],
          "hp": 175,
          "atk": 15,
          "def": 15,
          "sp_atk": 11,
          "sp_def": 11,
          "speed": 8
        },
        {
          "name": "ウミディグダ",
          "star": 1,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "ぬめぬめ",
              "description": "攻撃で自分に触れた相手の素早さを下げる。"
            },
            {
              "name": "びびり",
              "description": "あくゴーストむしタイプの攻撃を受けたりいかくをされるとびびって素早さが上がる。"
            },
            {
              "name": "すながくれ",
              "description": "砂あらしのとき回避率が上がる。"
            }
          ],
          "moves": [
            {
              "name": "まきつく",
              "type": "Normal",
              "category": "Physical",
              "power": 15,
              "description": "長い体やつるなどを使って４ー５ターンの間相手にまきついて攻撃する。"
            },
            {
              "name": "アクアジェット",
              "type": "Water",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "すなかけ",
              "type": "Ground",
              "category": "Status",
              "power": 0,
              "description": "相手の顔に砂をかけて命中率をさげる。"
            },
            {
              "name": "みずでっぽう",
              "type": "Water",
              "category": "Special",
              "power": 40,
              "description": "水を勢いよく相手に発射して攻撃する。"
            }
          ],
          "actions": [],
          "hp": 120,
          "atk": 18,
          "def": 11,
          "sp_atk": 13,
          "sp_def": 11,
          "speed": 27
        },
        {
          "name": "エレズン",
          "star": 1,
          "types": [
            "Electric",
            "Poison"
          ],
          "abilities": [
            {
              "name": "びびり",
              "description": "あくゴーストむしタイプの攻撃を受けたりいかくをされるとびびって素早さが上がる。"
            },
            {
              "name": "せいでんき",
              "description": "静電気を体にまとい触った相手をまひさせることがある。"
            },
            {
              "name": "ぶきよう",
              "description": "持っている道具を使うことができない。"
            }
          ],
          "moves": [
            {
              "name": "ほっぺすりすり",
              "type": "Electric",
              "category": "Physical",
              "power": 20,
              "description": "電気を帯びたほっぺをすりつけて攻撃。相手をまひ状態にする。"
            },
            {
              "name": "なみだめ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "なみだめになって相手の戦力を喪失させる。相手の攻撃と特攻がさがる。"
            },
            {
              "name": "ようかいえき",
              "type": "Poison",
              "category": "Special",
              "power": 40,
              "description": "強い酸を相手にかけて攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "じたばた",
              "type": "Normal",
              "category": "Physical",
              "power": 1,
              "description": "じたばた暴れて攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            }
          ],
          "actions": [],
          "hp": 155,
          "atk": 14,
          "def": 13,
          "sp_atk": 17,
          "sp_def": 13,
          "speed": 14
        },
        {
          "name": "オラチフ",
          "star": 1,
          "types": [
            "Dark"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "にげあし",
              "description": "野生のポケモンから必ず逃げられる。"
            },
            {
              "name": "はりこみ",
              "description": "交代で出てきた相手に２倍のダメージで攻撃できる。"
            }
          ],
          "moves": [
            {
              "name": "バークアウト",
              "type": "Dark",
              "category": "Special",
              "power": 55,
              "description": "まくしたてるように怒鳴りつけて相手の特攻をさげる。"
            },
            {
              "name": "つめとぎ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "ツメを磨いて鋭くする。自分の攻撃と命中率をあげる。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "したでなめる",
              "type": "Ghost",
              "category": "Physical",
              "power": 30,
              "description": "長い舌で相手をなめまわして攻撃する。まひ状態にすることがある。"
            }
          ],
          "actions": [],
          "hp": 180,
          "atk": 23,
          "def": 19,
          "sp_atk": 14,
          "sp_def": 17,
          "speed": 17
        },
        {
          "name": "カイデン",
          "star": 1,
          "types": [
            "Electric",
            "Flying"
          ],
          "abilities": [
            {
              "name": "ふうりょくでんき",
              "description": "風技を受けるとじゅうでん状態になる。"
            },
            {
              "name": "ちくでん",
              "description": "でんきタイプの技を受けるとダメージを受けずに回復する。"
            },
            {
              "name": "かちき",
              "description": "相手に能力を下げられると特攻がぐーんと上がる。"
            }
          ],
          "moves": [
            {
              "name": "スパーク",
              "type": "Electric",
              "category": "Physical",
              "power": 65,
              "description": "電気をまとい相手に突進して攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            },
            {
              "name": "でんこうせっか",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "ついばむ",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "くちばしで攻撃。相手がきのみを持っているとき食べてきのみの効果を受けられる。"
            }
          ],
          "actions": [],
          "hp": 155,
          "atk": 14,
          "def": 13,
          "sp_atk": 18,
          "sp_def": 14,
          "speed": 21
        },
        {
          "name": "カジッチュ",
          "star": 1,
          "types": [
            "Grass",
            "Dragon"
          ],
          "abilities": [
            {
              "name": "じゅくせい",
              "description": "熟成させることできのみの効果が倍になる。"
            },
            {
              "name": "くいしんぼう",
              "description": "ＨＰが少なくなったら食べるきのみをＨＰ半分の時に食べてしまう。"
            },
            {
              "name": "ぼうだん",
              "description": "相手の弾や爆弾などの技を防ぐことができる。"
            }
          ],
          "moves": [
            {
              "name": "ふいうち",
              "type": "Dark",
              "category": "Physical",
              "power": 70,
              "description": "相手より先に攻撃できる。相手がだす技が攻撃技でないと失敗する。"
            },
            {
              "name": "おどろかす",
              "type": "Ghost",
              "category": "Physical",
              "power": 30,
              "description": "大きな声などで不意に驚かして攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ころがる",
              "type": "Rock",
              "category": "Physical",
              "power": 30,
              "description": "５ターンの間転がり続けて攻撃する。技が当たるたびに威力があがる。"
            },
            {
              "name": "まるくなる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体をまるめてちぢこまり自分の防御をあげる。"
            }
          ],
          "actions": [],
          "hp": 155,
          "atk": 14,
          "def": 24,
          "sp_atk": 14,
          "sp_def": 14,
          "speed": 9
        },
        {
          "name": "カムカメ",
          "star": 1,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "がんじょうあご",
              "description": "あごが頑丈で噛む技の威力が高くなる。"
            },
            {
              "name": "シェルアーマー",
              "description": "硬い殻に守られ相手の攻撃が急所に当たらない。"
            },
            {
              "name": "すいすい",
              "description": "天気が雨のとき素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            },
            {
              "name": "みずでっぽう",
              "type": "Water",
              "category": "Special",
              "power": 40,
              "description": "水を勢いよく相手に発射して攻撃する。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "カウンター",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "相手から受けた物理攻撃のダメージを２倍にして同じ相手に返す。"
            }
          ],
          "actions": [],
          "hp": 170,
          "atk": 20,
          "def": 17,
          "sp_atk": 14,
          "sp_def": 14,
          "speed": 15
        },
        {
          "name": "カリキリ",
          "star": 1,
          "types": [
            "Grass"
          ],
          "abilities": [
            {
              "name": "リーフガード",
              "description": "天気が晴れのときは状態異常にならない。"
            },
            {
              "name": "あまのじゃく",
              "description": "能力の変化が逆転して上がるときに下がり下がるときに上がる。"
            }
          ],
          "moves": [
            {
              "name": "このは",
              "type": "Grass",
              "category": "Physical",
              "power": 40,
              "description": "はっぱを相手に当てて攻撃する。"
            },
            {
              "name": "れんぞくぎり",
              "type": "Bug",
              "category": "Physical",
              "power": 40,
              "description": "カマやツメなどで相手を切りつけて攻撃する。連続で当てると威力があがる。"
            },
            {
              "name": "せいちょう",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体を一気に大きく生長させて攻撃と特攻をあげる。"
            },
            {
              "name": "マジカルリーフ",
              "type": "Grass",
              "category": "Special",
              "power": 60,
              "description": "相手を追跡する不思議なはっぱをまきちらす。攻撃は必ず命中する。"
            }
          ],
          "actions": [],
          "hp": 155,
          "atk": 18,
          "def": 13,
          "sp_atk": 17,
          "sp_def": 13,
          "speed": 13
        },
        {
          "name": "カルボウ",
          "star": 1,
          "types": [
            "Fire"
          ],
          "abilities": [
            {
              "name": "もらいび",
              "description": "ほのおタイプの技を受けると炎をもらい自分が出すほのおタイプの技が強くなる。"
            },
            {
              "name": "ほのおのからだ",
              "description": "自分に触った相手をやけど状態にすることがある。"
            }
          ],
          "moves": [
            {
              "name": "ひのこ",
              "type": "Fire",
              "category": "Special",
              "power": 40,
              "description": "小さな炎を相手に発射して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "おにび",
              "type": "Fire",
              "category": "Status",
              "power": 0,
              "description": "不気味で怪しい炎を放って相手をやけどの状態にする。"
            },
            {
              "name": "かなしばり",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手の動きをとめて直前にだしていた技を４ターンの間使えなくする。"
            },
            {
              "name": "からげんき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "自分が毒まひやけど状態のとき相手にくりだすと技の威力が２倍になる。"
            }
          ],
          "actions": [],
          "hp": 155,
          "atk": 17,
          "def": 14,
          "sp_atk": 17,
          "sp_def": 14,
          "speed": 13
        },
        {
          "name": "キノココ",
          "star": 1,
          "types": [
            "Grass"
          ],
          "abilities": [
            {
              "name": "ほうし",
              "description": "攻撃で自分に触れた相手をどくやまひやねむり状態にすることがある。"
            },
            {
              "name": "ポイズンヒール",
              "description": "どく状態になるとＨＰが減らずに増えていく。"
            },
            {
              "name": "はやあし",
              "description": "状態異常になると素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            },
            {
              "name": "タネマシンガン",
              "type": "Grass",
              "category": "Physical",
              "power": 25,
              "description": "タネを勢いよく相手に発射して攻撃する。２ー５回の間連続でだす。"
            },
            {
              "name": "キノコのほうし",
              "type": "Grass",
              "category": "Status",
              "power": 0,
              "description": "催眠効果のある胞子をパラパラとふりまき相手を眠り状態にする。"
            },
            {
              "name": "すいとる",
              "type": "Grass",
              "category": "Special",
              "power": 20,
              "description": "養分を吸い取り攻撃する。相手に与えたダメージの半分のＨＰを回復できる。"
            }
          ],
          "actions": [],
          "hp": 180,
          "atk": 14,
          "def": 19,
          "sp_atk": 14,
          "sp_def": 19,
          "speed": 13
        },
        {
          "name": "キャモメ",
          "star": 1,
          "types": [
            "Water",
            "Flying"
          ],
          "abilities": [
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            },
            {
              "name": "うるおいボディ",
              "description": "天気が雨のとき状態異常が治る。"
            },
            {
              "name": "あめうけざら",
              "description": "天気が雨のとき少しずつＨＰを回復する。"
            }
          ],
          "moves": [
            {
              "name": "みずでっぽう",
              "type": "Water",
              "category": "Special",
              "power": 40,
              "description": "水を勢いよく相手に発射して攻撃する。"
            },
            {
              "name": "つばさでうつ",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "大きくひろげたりっぱな翼を相手にぶつけて攻撃する。"
            },
            {
              "name": "なきごえ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "かわいいなきごえを聞かせて気をひき油断をさせて相手の攻撃をさげる。"
            },
            {
              "name": "どろぼう",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "攻撃と同時に道具を盗む。自分が道具を持っている場合は盗めない。"
            }
          ],
          "actions": [],
          "hp": 155,
          "atk": 12,
          "def": 12,
          "sp_atk": 18,
          "sp_def": 12,
          "speed": 25
        },
        {
          "name": "グルトン",
          "star": 1,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "アロマベール",
              "description": "自分と味方へのメンタル攻撃を防ぐことができる。"
            },
            {
              "name": "くいしんぼう",
              "description": "ＨＰが少なくなったら食べるきのみをＨＰ半分の時に食べてしまう。"
            },
            {
              "name": "あついしぼう",
              "description": "厚い脂肪で守られているのでほのおタイプとこおりタイプの技のダメージを半減させる。"
            }
          ],
          "moves": [
            {
              "name": "たいあたり",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "相手にむかって全身でぶつかっていき攻撃する。"
            },
            {
              "name": "エコーボイス",
              "type": "Normal",
              "category": "Special",
              "power": 40,
              "description": "響く声で相手を攻撃する。毎ターンだれかが技を使い続けると威力があがる。"
            },
            {
              "name": "ほしがる",
              "type": "Normal",
              "category": "Physical",
              "power": 60,
              "description": "かわいくあまえながら相手にちかづき持っている道具をうばう。"
            },
            {
              "name": "しっぽをふる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "しっぽを左右にかわいくふって油断を誘う。相手の防御をさげる。"
            }
          ],
          "actions": [],
          "hp": 170,
          "atk": 15,
          "def": 14,
          "sp_atk": 13,
          "sp_def": 15,
          "speed": 13
        },
        {
          "name": "ケイコウオ",
          "star": 1,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "すいすい",
              "description": "天気が雨のとき素早さが上がる。"
            },
            {
              "name": "よびみず",
              "description": "みずタイプの技を自分によせつけダメージは受けずに特攻が上がる。"
            },
            {
              "name": "みずのベール",
              "description": "水のベールを身にまといやけど状態にならない。"
            }
          ],
          "moves": [
            {
              "name": "サイケこうせん",
              "type": "Psychic",
              "category": "Special",
              "power": 65,
              "description": "不思議な光線を相手に発射して攻撃する。混乱させることがある。"
            },
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "かぜおこし",
              "type": "Flying",
              "category": "Special",
              "power": 40,
              "description": "翼でおこした激しい風を相手にぶつけて攻撃する。"
            },
            {
              "name": "はたく",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "長いしっぽや手などを使って相手をはたいて攻撃する。"
            }
          ],
          "actions": [],
          "hp": 165,
          "atk": 16,
          "def": 18,
          "sp_atk": 16,
          "sp_def": 19,
          "speed": 20
        },
        {
          "name": "ココガラ",
          "star": 1,
          "types": [
            "Flying"
          ],
          "abilities": [
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            },
            {
              "name": "はとむね",
              "description": "防御を下げる効果を受けない。"
            }
          ],
          "moves": [
            {
              "name": "ついばむ",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "くちばしで攻撃。相手がきのみを持っているとき食べてきのみの効果を受けられる。"
            },
            {
              "name": "つめとぎ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "ツメを磨いて鋭くする。自分の攻撃と命中率をあげる。"
            },
            {
              "name": "みだれづき",
              "type": "Normal",
              "category": "Physical",
              "power": 15,
              "description": "つのやくちばしで相手をつついて攻撃する。２ー５回の間連続でだす。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            }
          ],
          "actions": [],
          "hp": 155,
          "atk": 16,
          "def": 13,
          "sp_atk": 12,
          "sp_def": 13,
          "speed": 18
        },
        {
          "name": "コジオ",
          "star": 1,
          "types": [
            "Rock"
          ],
          "abilities": [
            {
              "name": "きよめのしお",
              "description": "清らかな塩で状態異常にならない。ゴーストタイプの技のダメージを半減させる。"
            },
            {
              "name": "がんじょう",
              "description": "ＨＰが満タンのとき技を受けても一撃で倒されることがない。一撃必殺技も効かない。"
            },
            {
              "name": "クリアボディ",
              "description": "相手の技や特性で能力を下げられない。"
            }
          ],
          "moves": [
            {
              "name": "ロックカット",
              "type": "Rock",
              "category": "Status",
              "power": 0,
              "description": "自分の体を磨いて空気の抵抗を少なくする。素早さをぐーんとあげることができる。"
            },
            {
              "name": "いわおとし",
              "type": "Rock",
              "category": "Physical",
              "power": 50,
              "description": "小さな岩を持ちあげて相手に投げつけて攻撃する。"
            },
            {
              "name": "マッドショット",
              "type": "Ground",
              "category": "Special",
              "power": 55,
              "description": "泥の塊を相手に投げつけて攻撃する。同時に相手の素早さをさげる。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [],
          "hp": 175,
          "atk": 18,
          "def": 23,
          "sp_atk": 13,
          "sp_def": 13,
          "speed": 11
        },
        {
          "name": "コロボーシ",
          "star": 1,
          "types": [
            "Bug"
          ],
          "abilities": [
            {
              "name": "だっぴ",
              "description": "体の皮を脱ぎ捨てることで状態異常を治すことがある。"
            },
            {
              "name": "にげあし",
              "description": "野生のポケモンから必ず逃げられる。"
            }
          ],
          "moves": [
            {
              "name": "なきごえ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "かわいいなきごえを聞かせて気をひき油断をさせて相手の攻撃をさげる。"
            },
            {
              "name": "むしのていこう",
              "type": "Bug",
              "category": "Special",
              "power": 50,
              "description": "抵抗して相手を攻撃する。相手の特攻をさげる。"
            },
            {
              "name": "むしくい",
              "type": "Bug",
              "category": "Physical",
              "power": 60,
              "description": "かみついて攻撃する。相手がきのみを持っているとき食べてきのみの効果を受けられる。"
            }
          ],
          "actions": [],
          "hp": 150,
          "atk": 11,
          "def": 14,
          "sp_atk": 11,
          "sp_def": 14,
          "speed": 11
        },
        {
          "name": "コンパン",
          "star": 1,
          "types": [
            "Bug",
            "Poison"
          ],
          "abilities": [
            {
              "name": "ふくがん",
              "description": "複眼を持っているため技の命中率が上がる。"
            },
            {
              "name": "いろめがね",
              "description": "効果がいまひとつの技を通常の威力で出すことができる。"
            },
            {
              "name": "にげあし",
              "description": "野生のポケモンから必ず逃げられる。"
            }
          ],
          "moves": [
            {
              "name": "サイケこうせん",
              "type": "Psychic",
              "category": "Special",
              "power": 65,
              "description": "不思議な光線を相手に発射して攻撃する。混乱させることがある。"
            },
            {
              "name": "むしのていこう",
              "type": "Bug",
              "category": "Special",
              "power": 50,
              "description": "抵抗して相手を攻撃する。相手の特攻をさげる。"
            },
            {
              "name": "かなしばり",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手の動きをとめて直前にだしていた技を４ターンの間使えなくする。"
            },
            {
              "name": "むしくい",
              "type": "Bug",
              "category": "Physical",
              "power": 60,
              "description": "かみついて攻撃する。相手がきのみを持っているとき食べてきのみの効果を受けられる。"
            }
          ],
          "actions": [],
          "hp": 180,
          "atk": 18,
          "def": 17,
          "sp_atk": 14,
          "sp_def": 18,
          "speed": 15
        },
        {
          "name": "シェルダー",
          "star": 1,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "シェルアーマー",
              "description": "硬い殻に守られ相手の攻撃が急所に当たらない。"
            },
            {
              "name": "スキルリンク",
              "description": "連続技を使うといつも最高回数出すことができる。"
            },
            {
              "name": "ぼうじん",
              "description": "すなあらしのダメージを受けない。粉や胞子の影響も受けない。"
            }
          ],
          "moves": [
            {
              "name": "みずでっぽう",
              "type": "Water",
              "category": "Special",
              "power": 40,
              "description": "水を勢いよく相手に発射して攻撃する。"
            },
            {
              "name": "こおりのつぶて",
              "type": "Ice",
              "category": "Physical",
              "power": 40,
              "description": "氷の塊を一瞬でつくり相手に素早く放つ。必ず先制攻撃できる。"
            },
            {
              "name": "ちょうおんぱ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "特殊な音波を体から発して相手を混乱させる。"
            },
            {
              "name": "オーロラビーム",
              "type": "Ice",
              "category": "Special",
              "power": 65,
              "description": "にじいろのビームを相手に発射して攻撃する。攻撃をさげることがある。"
            }
          ],
          "actions": [],
          "hp": 145,
          "atk": 20,
          "def": 29,
          "sp_atk": 15,
          "sp_def": 11,
          "speed": 14
        },
        {
          "name": "シガロコ",
          "star": 1,
          "types": [
            "Bug"
          ],
          "abilities": [
            {
              "name": "ふくがん",
              "description": "複眼を持っているため技の命中率が上がる。"
            },
            {
              "name": "だっぴ",
              "description": "体の皮を脱ぎ捨てることで状態異常を治すことがある。"
            }
          ],
          "moves": [
            {
              "name": "むしのていこう",
              "type": "Bug",
              "category": "Special",
              "power": 50,
              "description": "抵抗して相手を攻撃する。相手の特攻をさげる。"
            },
            {
              "name": "ころがる",
              "type": "Rock",
              "category": "Physical",
              "power": 30,
              "description": "５ターンの間転がり続けて攻撃する。技が当たるたびに威力があがる。"
            },
            {
              "name": "マッドショット",
              "type": "Ground",
              "category": "Special",
              "power": 55,
              "description": "泥の塊を相手に投げつけて攻撃する。同時に相手の素早さをさげる。"
            },
            {
              "name": "まるくなる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体をまるめてちぢこまり自分の防御をあげる。"
            }
          ],
          "actions": [],
          "hp": 155,
          "atk": 17,
          "def": 19,
          "sp_atk": 12,
          "sp_def": 18,
          "speed": 12
        },
        {
          "name": "シキジカ",
          "star": 1,
          "types": [
            "Normal",
            "Grass"
          ],
          "abilities": [
            {
              "name": "ようりょくそ",
              "description": "天気が晴れのとき素早さが上がる。"
            },
            {
              "name": "そうしょく",
              "description": "くさタイプの技を受けるとダメージを受けずに攻撃が上がる。"
            },
            {
              "name": "てんのめぐみ",
              "description": "天の恵みのおかげで技の追加効果がでやすい。"
            }
          ],
          "moves": [
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            },
            {
              "name": "にどげり",
              "type": "Fighting",
              "category": "Physical",
              "power": 30,
              "description": "２本の足で相手をけとばして攻撃する。２回連続でダメージを与える。"
            },
            {
              "name": "すなかけ",
              "type": "Ground",
              "category": "Status",
              "power": 0,
              "description": "相手の顔に砂をかけて命中率をさげる。"
            },
            {
              "name": "なきごえ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "かわいいなきごえを聞かせて気をひき油断をさせて相手の攻撃をさげる。"
            }
          ],
          "actions": [],
          "hp": 180,
          "atk": 19,
          "def": 17,
          "sp_atk": 14,
          "sp_def": 17,
          "speed": 23
        },
        {
          "name": "シシコ",
          "star": 1,
          "types": [
            "Fire",
            "Normal"
          ],
          "abilities": [
            {
              "name": "とうそうしん",
              "description": "性別が同じだと闘争心を燃やして強くなる。性別が違うと弱くなる。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            },
            {
              "name": "じしんかじょう",
              "description": "相手を倒すと自信がついて攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ひのこ",
              "type": "Fire",
              "category": "Special",
              "power": 40,
              "description": "小さな炎を相手に発射して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "ふるいたてる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "自分を奮いたてて攻撃と特攻をあげる。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            },
            {
              "name": "にらみつける",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "鋭い目つきでおびえさせて相手の防御をさげる。"
            }
          ],
          "actions": [],
          "hp": 180,
          "atk": 17,
          "def": 18,
          "sp_atk": 22,
          "sp_def": 17,
          "speed": 22
        },
        {
          "name": "シビシラス",
          "star": 1,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "ふゆう",
              "description": "地面から浮くことによってじめんタイプの技を受けない。"
            }
          ],
          "moves": [
            {
              "name": "たいあたり",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "相手にむかって全身でぶつかっていき攻撃する。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            },
            {
              "name": "スパーク",
              "type": "Electric",
              "category": "Physical",
              "power": 65,
              "description": "電気をまとい相手に突進して攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "チャージビーム",
              "type": "Electric",
              "category": "Special",
              "power": 50,
              "description": "電撃の束を相手に発射する。電気をためて自分の特攻をあげることがある。"
            }
          ],
          "actions": [],
          "hp": 150,
          "atk": 18,
          "def": 14,
          "sp_atk": 15,
          "sp_def": 14,
          "speed": 19
        },
        {
          "name": "シルシュルー",
          "star": 1,
          "types": [
            "Poison",
            "Normal"
          ],
          "abilities": [
            {
              "name": "かるわざ",
              "description": "持っていた道具がなくなると素早さが上がる。"
            },
            {
              "name": "わるいてぐせ",
              "description": "触られた相手の道具を盗んでしまう。"
            },
            {
              "name": "いたずらごころ",
              "description": "変化技を先制で出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "どくどくのキバ",
              "type": "Poison",
              "category": "Physical",
              "power": 50,
              "description": "毒のあるキバで相手にかみついて攻撃する。猛毒をおわせることがある。"
            },
            {
              "name": "おだてる",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手をおだてて混乱させる。同時に相手の特攻もあげてしまう。"
            },
            {
              "name": "みだれひっかき",
              "type": "Normal",
              "category": "Physical",
              "power": 18,
              "description": "ツメやカマなどで相手をひっかいて攻撃する。２ー５回の間連続でだす。"
            },
            {
              "name": "にらみつける",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "鋭い目つきでおびえさせて相手の防御をさげる。"
            }
          ],
          "actions": [],
          "hp": 155,
          "atk": 20,
          "def": 13,
          "sp_atk": 14,
          "sp_def": 13,
          "speed": 23
        },
        {
          "name": "スナヘビ",
          "star": 1,
          "types": [
            "Ground"
          ],
          "abilities": [
            {
              "name": "すなはき",
              "description": "攻撃を受けると砂あらしを起こす。"
            },
            {
              "name": "だっぴ",
              "description": "体の皮を脱ぎ捨てることで状態異常を治すことがある。"
            },
            {
              "name": "すながくれ",
              "description": "砂あらしのとき回避率が上がる。"
            }
          ],
          "moves": [
            {
              "name": "じならし",
              "type": "Ground",
              "category": "Physical",
              "power": 60,
              "description": "地面を踏みならして自分の周りにいるものを攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "ぶんまわす",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "自分の体をぶんまわして相手にダメージを与える。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            },
            {
              "name": "とぐろをまく",
              "type": "Poison",
              "category": "Status",
              "power": 0,
              "description": "とぐろをまいて集中する。自分の攻撃と防御と命中率をあげる。"
            }
          ],
          "actions": [],
          "hp": 170,
          "atk": 18,
          "def": 23,
          "sp_atk": 13,
          "sp_def": 17,
          "speed": 16
        },
        {
          "name": "スリープ",
          "star": 1,
          "types": [
            "Psychic"
          ],
          "abilities": [
            {
              "name": "ふみん",
              "description": "眠れない体質なのでねむり状態にならない。"
            },
            {
              "name": "よちむ",
              "description": "登場したとき相手の持つ技をひとつだけ読み取る。"
            },
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            }
          ],
          "moves": [
            {
              "name": "かなしばり",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手の動きをとめて直前にだしていた技を４ターンの間使えなくする。"
            },
            {
              "name": "さいみんじゅつ",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "眠気を誘う暗示をかけて相手を眠り状態にする。"
            },
            {
              "name": "ねんりき",
              "type": "Psychic",
              "category": "Special",
              "power": 50,
              "description": "弱い念力を相手に送って攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [],
          "hp": 180,
          "atk": 16,
          "def": 15,
          "sp_atk": 15,
          "sp_def": 26,
          "speed": 15
        },
        {
          "name": "ズピカ",
          "star": 1,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "せいでんき",
              "description": "静電気を体にまとい触った相手をまひさせることがある。"
            },
            {
              "name": "しめりけ",
              "description": "あたりを湿らせることによってじばくなどの爆発する技をだれも使えなくなる。"
            }
          ],
          "moves": [
            {
              "name": "でんきショック",
              "type": "Electric",
              "category": "Special",
              "power": 40,
              "description": "電気の刺激を相手に浴びせて攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "みずでっぽう",
              "type": "Water",
              "category": "Special",
              "power": 40,
              "description": "水を勢いよく相手に発射して攻撃する。"
            },
            {
              "name": "じゅうでん",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "じゅうでん状態になり次にだすでんきタイプの技の威力をあげる。自分の特防もあがる。"
            },
            {
              "name": "マッドショット",
              "type": "Ground",
              "category": "Special",
              "power": 55,
              "description": "泥の塊を相手に投げつけて攻撃する。同時に相手の素早さをさげる。"
            }
          ],
          "actions": [],
          "hp": 180,
          "atk": 12,
          "def": 14,
          "sp_atk": 19,
          "sp_def": 13,
          "speed": 15
        },
        {
          "name": "タマンチュラ",
          "star": 1,
          "types": [
            "Bug"
          ],
          "abilities": [
            {
              "name": "ふみん",
              "description": "眠れない体質なのでねむり状態にならない。"
            },
            {
              "name": "はりこみ",
              "description": "交代で出てきた相手に２倍のダメージで攻撃できる。"
            }
          ],
          "moves": [
            {
              "name": "むしのていこう",
              "type": "Bug",
              "category": "Special",
              "power": 50,
              "description": "抵抗して相手を攻撃する。相手の特攻をさげる。"
            },
            {
              "name": "とびつく",
              "type": "Bug",
              "category": "Physical",
              "power": 50,
              "description": "相手に飛びついて攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "ダメおし",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "そのターンに相手がすでにダメージを受けていたら技の威力は２倍になる。"
            },
            {
              "name": "いとをはく",
              "type": "Bug",
              "category": "Status",
              "power": 0,
              "description": "口から吹きだした糸をまきつけて相手の素早さをがくっとさげる。"
            }
          ],
          "actions": [],
          "hp": 150,
          "atk": 14,
          "def": 15,
          "sp_atk": 11,
          "sp_def": 14,
          "speed": 9
        },
        {
          "name": "チュリネ",
          "star": 1,
          "types": [
            "Grass"
          ],
          "abilities": [
            {
              "name": "ようりょくそ",
              "description": "天気が晴れのとき素早さが上がる。"
            },
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "リーフガード",
              "description": "天気が晴れのときは状態異常にならない。"
            }
          ],
          "moves": [
            {
              "name": "せいちょう",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体を一気に大きく生長させて攻撃と特攻をあげる。"
            },
            {
              "name": "ねむりごな",
              "type": "Grass",
              "category": "Status",
              "power": 0,
              "description": "眠くなる粉をたくさんふりまいて相手を眠り状態にする。"
            },
            {
              "name": "すいとる",
              "type": "Grass",
              "category": "Special",
              "power": 20,
              "description": "養分を吸い取り攻撃する。相手に与えたダメージの半分のＨＰを回復できる。"
            },
            {
              "name": "メガドレイン",
              "type": "Grass",
              "category": "Special",
              "power": 40,
              "description": "養分を吸い取り攻撃する。相手に与えたダメージの半分のＨＰを回復できる。"
            }
          ],
          "actions": [],
          "hp": 160,
          "atk": 13,
          "def": 17,
          "sp_atk": 21,
          "sp_def": 17,
          "speed": 12
        },
        {
          "name": "チルット",
          "star": 1,
          "types": [
            "Normal",
            "Flying"
          ],
          "abilities": [
            {
              "name": "しぜんかいふく",
              "description": "手持ちにひっこむと状態異常が治る。"
            },
            {
              "name": "ノーてんき",
              "description": "あらゆる天気の影響がなくなってしまう。"
            }
          ],
          "moves": [
            {
              "name": "つつく",
              "type": "Flying",
              "category": "Physical",
              "power": 35,
              "description": "鋭くとがったくちばしやつので相手を突いて攻撃する。"
            },
            {
              "name": "りんしょう",
              "type": "Normal",
              "category": "Special",
              "power": 60,
              "description": "歌で相手を攻撃する。みんなで輪唱すると続けてだすことができ威力もあがる。"
            },
            {
              "name": "りゅうのいぶき",
              "type": "Dragon",
              "category": "Special",
              "power": 60,
              "description": "ものすごい息を相手に吹きつけて攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "チャームボイス",
              "type": "Fairy",
              "category": "Special",
              "power": 40,
              "description": "魅惑の鳴き声をだして相手に精神的なダメージを与える。攻撃は必ず命中する。"
            }
          ],
          "actions": [],
          "hp": 160,
          "atk": 14,
          "def": 19,
          "sp_atk": 14,
          "sp_def": 23,
          "speed": 17
        },
        {
          "name": "ドジョッチ",
          "star": 1,
          "types": [
            "Water",
            "Ground"
          ],
          "abilities": [
            {
              "name": "どんかん",
              "description": "鈍感なのでメロメロやちょうはつ状態にならない。いかくにも動じない。"
            },
            {
              "name": "きけんよち",
              "description": "相手の持つ危険な技を察知することができる。"
            },
            {
              "name": "うるおいボディ",
              "description": "天気が雨のとき状態異常が治る。"
            }
          ],
          "moves": [
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "マッドショット",
              "type": "Ground",
              "category": "Special",
              "power": 55,
              "description": "泥の塊を相手に投げつけて攻撃する。同時に相手の素早さをさげる。"
            },
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "スパーク",
              "type": "Electric",
              "category": "Physical",
              "power": 65,
              "description": "電気をまとい相手に突進して攻撃する。まひ状態にすることがある。"
            }
          ],
          "actions": [],
          "hp": 170,
          "atk": 16,
          "def": 15,
          "sp_atk": 16,
          "sp_def": 14,
          "speed": 19
        },
        {
          "name": "ノコッチ",
          "star": 1,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "てんのめぐみ",
              "description": "天の恵みのおかげで技の追加効果がでやすい。"
            },
            {
              "name": "にげあし",
              "description": "野生のポケモンから必ず逃げられる。"
            },
            {
              "name": "びびり",
              "description": "あくゴーストむしタイプの攻撃を受けたりいかくをされるとびびって素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "どろかけ",
              "type": "Ground",
              "category": "Special",
              "power": 20,
              "description": "相手の顔などに泥を投げつけて攻撃する。命中率をさげる。"
            },
            {
              "name": "ころがる",
              "type": "Rock",
              "category": "Physical",
              "power": 30,
              "description": "５ターンの間転がり続けて攻撃する。技が当たるたびに威力があがる。"
            },
            {
              "name": "まるくなる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体をまるめてちぢこまり自分の防御をあげる。"
            },
            {
              "name": "じたばた",
              "type": "Normal",
              "category": "Physical",
              "power": 1,
              "description": "じたばた暴れて攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            }
          ],
          "actions": [],
          "hp": 230,
          "atk": 21,
          "def": 21,
          "sp_atk": 20,
          "sp_def": 20,
          "speed": 15
        },
        {
          "name": "ノノクラゲ",
          "star": 1,
          "types": [
            "Ground",
            "Grass"
          ],
          "abilities": [
            {
              "name": "きんしのちから",
              "description": "変化技を出すとき必ず行動が遅くなるが相手の特性にジャマされない。"
            }
          ],
          "moves": [
            {
              "name": "メガドレイン",
              "type": "Grass",
              "category": "Special",
              "power": 40,
              "description": "養分を吸い取り攻撃する。相手に与えたダメージの半分のＨＰを回復できる。"
            },
            {
              "name": "しびれごな",
              "type": "Grass",
              "category": "Status",
              "power": 0,
              "description": "しびれる粉をたくさんふりまいて相手をまひ状態にする。"
            },
            {
              "name": "どろかけ",
              "type": "Ground",
              "category": "Special",
              "power": 20,
              "description": "相手の顔などに泥を投げつけて攻撃する。命中率をさげる。"
            },
            {
              "name": "マッドショット",
              "type": "Ground",
              "category": "Special",
              "power": 55,
              "description": "泥の塊を相手に投げつけて攻撃する。同時に相手の素早さをさげる。"
            }
          ],
          "actions": [],
          "hp": 155,
          "atk": 14,
          "def": 13,
          "sp_atk": 17,
          "sp_def": 29,
          "speed": 21
        },
        {
          "name": "ハネッコ",
          "star": 1,
          "types": [
            "Grass",
            "Flying"
          ],
          "abilities": [
            {
              "name": "ようりょくそ",
              "description": "天気が晴れのとき素早さが上がる。"
            },
            {
              "name": "リーフガード",
              "description": "天気が晴れのときは状態異常にならない。"
            },
            {
              "name": "すりぬけ",
              "description": "相手の壁や身代わりをすりぬけて攻撃できる。"
            }
          ],
          "moves": [
            {
              "name": "すいとる",
              "type": "Grass",
              "category": "Special",
              "power": 20,
              "description": "養分を吸い取り攻撃する。相手に与えたダメージの半分のＨＰを回復できる。"
            },
            {
              "name": "ようせいのかぜ",
              "type": "Fairy",
              "category": "Special",
              "power": 40,
              "description": "ようせいのかぜを起こし相手に吹きつけて攻撃する。"
            },
            {
              "name": "たいあたり",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "相手にむかって全身でぶつかっていき攻撃する。"
            }
          ],
          "actions": [],
          "hp": 150,
          "atk": 13,
          "def": 14,
          "sp_atk": 13,
          "sp_def": 18,
          "speed": 17
        },
        {
          "name": "パピモッチ",
          "star": 1,
          "types": [
            "Fairy"
          ],
          "abilities": [
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "ぶきよう",
              "description": "持っている道具を使うことができない。"
            }
          ],
          "moves": [
            {
              "name": "たいあたり",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "相手にむかって全身でぶつかっていき攻撃する。"
            },
            {
              "name": "したでなめる",
              "type": "Ghost",
              "category": "Physical",
              "power": 30,
              "description": "長い舌で相手をなめまわして攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ほしがる",
              "type": "Normal",
              "category": "Physical",
              "power": 60,
              "description": "かわいくあまえながら相手にちかづき持っている道具をうばう。"
            }
          ],
          "actions": [],
          "hp": 150,
          "atk": 18,
          "def": 21,
          "sp_atk": 12,
          "sp_def": 18,
          "speed": 20
        },
        {
          "name": "パモ",
          "star": 1,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "せいでんき",
              "description": "静電気を体にまとい触った相手をまひさせることがある。"
            },
            {
              "name": "しぜんかいふく",
              "description": "手持ちにひっこむと状態異常が治る。"
            },
            {
              "name": "てつのこぶし",
              "description": "パンチを使う技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ほっぺすりすり",
              "type": "Electric",
              "category": "Physical",
              "power": 20,
              "description": "電気を帯びたほっぺをすりつけて攻撃。相手をまひ状態にする。"
            },
            {
              "name": "でんこうせっか",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "じゅうでん",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "じゅうでん状態になり次にだすでんきタイプの技の威力をあげる。自分の特防もあがる。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [],
          "hp": 160,
          "atk": 17,
          "def": 9,
          "sp_atk": 14,
          "sp_def": 11,
          "speed": 19
        },
        {
          "name": "ヒマナッツ",
          "star": 1,
          "types": [
            "Grass"
          ],
          "abilities": [
            {
              "name": "ようりょくそ",
              "description": "天気が晴れのとき素早さが上がる。"
            },
            {
              "name": "サンパワー",
              "description": "天気が晴れると特攻が上がるが毎ターンＨＰが減る。"
            },
            {
              "name": "はやおき",
              "description": "ねむり状態になっても２倍の早さで目覚めることができる。"
            }
          ],
          "moves": [
            {
              "name": "すいとる",
              "type": "Grass",
              "category": "Special",
              "power": 20,
              "description": "養分を吸い取り攻撃する。相手に与えたダメージの半分のＨＰを回復できる。"
            },
            {
              "name": "たいあたり",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "相手にむかって全身でぶつかっていき攻撃する。"
            },
            {
              "name": "ひかりのかべ",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "５ターンの間不思議なかべで相手から受ける特殊攻撃のダメージを弱める。"
            },
            {
              "name": "メガドレイン",
              "type": "Grass",
              "category": "Special",
              "power": 40,
              "description": "養分を吸い取り攻撃する。相手に与えたダメージの半分のＨＰを回復できる。"
            }
          ],
          "actions": [],
          "hp": 145,
          "atk": 12,
          "def": 12,
          "sp_atk": 12,
          "sp_def": 12,
          "speed": 12
        },
        {
          "name": "ヒラヒナ",
          "star": 1,
          "types": [
            "Psychic"
          ],
          "abilities": [
            {
              "name": "きけんよち",
              "description": "相手の持つ危険な技を察知することができる。"
            },
            {
              "name": "おみとおし",
              "description": "登場したとき相手の持ち物を見通すことができる。"
            },
            {
              "name": "かそく",
              "description": "毎ターン素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "サイケこうせん",
              "type": "Psychic",
              "category": "Special",
              "power": 65,
              "description": "不思議な光線を相手に発射して攻撃する。混乱させることがある。"
            },
            {
              "name": "つぶらなひとみ",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "つぶらなひとみで相手をみつめて攻撃をさげる。必ず先制攻撃できる。"
            },
            {
              "name": "チャームボイス",
              "type": "Fairy",
              "category": "Special",
              "power": 40,
              "description": "魅惑の鳴き声をだして相手に精神的なダメージを与える。攻撃は必ず命中する。"
            },
            {
              "name": "つつく",
              "type": "Flying",
              "category": "Physical",
              "power": 35,
              "description": "鋭くとがったくちばしやつので相手を突いて攻撃する。"
            }
          ],
          "actions": [],
          "hp": 145,
          "atk": 13,
          "def": 12,
          "sp_atk": 18,
          "sp_def": 12,
          "speed": 23
        },
        {
          "name": "ピチュー",
          "star": 1,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "せいでんき",
              "description": "静電気を体にまとい触った相手をまひさせることがある。"
            },
            {
              "name": "ひらいしん",
              "description": "でんきタイプの技を自分に寄せつけダメージを受けずに特攻が上がる。"
            }
          ],
          "moves": [
            {
              "name": "でんきショック",
              "type": "Electric",
              "category": "Special",
              "power": 40,
              "description": "電気の刺激を相手に浴びせて攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "ほっぺすりすり",
              "type": "Electric",
              "category": "Physical",
              "power": 20,
              "description": "電気を帯びたほっぺをすりつけて攻撃。相手をまひ状態にする。"
            },
            {
              "name": "わるだくみ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "悪いことを考えて頭を活性化させる。自分の特攻をぐーんとあげる。"
            },
            {
              "name": "スピードスター",
              "type": "Normal",
              "category": "Special",
              "power": 60,
              "description": "星型の光を発射して相手を攻撃する。攻撃は必ず命中する。"
            }
          ],
          "actions": [],
          "hp": 130,
          "atk": 14,
          "def": 8,
          "sp_atk": 13,
          "sp_def": 13,
          "speed": 19
        },
        {
          "name": "フラベベ",
          "star": 1,
          "types": [
            "Fairy"
          ],
          "abilities": [
            {
              "name": "フラワーベール",
              "description": "味方の草ポケモンは能力が下がらず状態異常にもならない。"
            },
            {
              "name": "きょうせい",
              "description": "味方が道具を使うと自分の持っている道具を味方に渡す。"
            }
          ],
          "moves": [
            {
              "name": "ようせいのかぜ",
              "type": "Fairy",
              "category": "Special",
              "power": 40,
              "description": "ようせいのかぜを起こし相手に吹きつけて攻撃する。"
            },
            {
              "name": "なみだめ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "なみだめになって相手の戦力を喪失させる。相手の攻撃と特攻がさがる。"
            },
            {
              "name": "つるのムチ",
              "type": "Grass",
              "category": "Physical",
              "power": 45,
              "description": "ムチのようにしなる細長いつるで相手をたたきつけて攻撃する。"
            },
            {
              "name": "たいあたり",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "相手にむかって全身でぶつかっていき攻撃する。"
            }
          ],
          "actions": [],
          "hp": 160,
          "atk": 14,
          "def": 14,
          "sp_atk": 19,
          "sp_def": 23,
          "speed": 15
        },
        {
          "name": "ププリン",
          "star": 1,
          "types": [
            "Normal",
            "Fairy"
          ],
          "abilities": [
            {
              "name": "メロメロボディ",
              "description": "自分に触った相手をメロメロにすることがある。"
            },
            {
              "name": "かちき",
              "description": "相手に能力を下げられると特攻がぐーんと上がる。"
            },
            {
              "name": "フレンドガード",
              "description": "味方のダメージを減らすことができる。"
            }
          ],
          "moves": [
            {
              "name": "チャームボイス",
              "type": "Fairy",
              "category": "Special",
              "power": 40,
              "description": "魅惑の鳴き声をだして相手に精神的なダメージを与える。攻撃は必ず命中する。"
            },
            {
              "name": "てんしのキッス",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "天使のようにかわいくキスして相手を混乱させる。"
            },
            {
              "name": "ドレインキッス",
              "type": "Fairy",
              "category": "Special",
              "power": 50,
              "description": "キッスによって相手からＨＰを吸い取る。与えたダメージの半分以上ＨＰを回復する。"
            },
            {
              "name": "こごえるかぜ",
              "type": "Ice",
              "category": "Special",
              "power": 55,
              "description": "凍てつく冷気を相手に吹きつけて攻撃する。相手の素早さをさげる。"
            }
          ],
          "actions": [],
          "hp": 215,
          "atk": 12,
          "def": 8,
          "sp_atk": 14,
          "sp_def": 9,
          "speed": 8
        },
        {
          "name": "ホシガリス",
          "star": 1,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "ほおぶくろ",
              "description": "どんなきのみでも食べるとＨＰも回復する。"
            },
            {
              "name": "くいしんぼう",
              "description": "ＨＰが少なくなったら食べるきのみをＨＰ半分の時に食べてしまう。"
            }
          ],
          "moves": [
            {
              "name": "たいあたり",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "相手にむかって全身でぶつかっていき攻撃する。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "しっぽをふる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "しっぽを左右にかわいくふって油断を誘う。相手の防御をさげる。"
            },
            {
              "name": "カウンター",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "相手から受けた物理攻撃のダメージを２倍にして同じ相手に返す。"
            }
          ],
          "actions": [],
          "hp": 190,
          "atk": 18,
          "def": 18,
          "sp_atk": 13,
          "sp_def": 13,
          "speed": 11
        },
        {
          "name": "ボチ",
          "star": 1,
          "types": [
            "Ghost"
          ],
          "abilities": [
            {
              "name": "ものひろい",
              "description": "相手の使った道具を拾ってくることがある。冒険中も拾ってくる。"
            },
            {
              "name": "もふもふ",
              "description": "相手から受けた接触する技のダメージを半減するがほのおタイプの技のダメージは２倍になる。"
            }
          ],
          "moves": [
            {
              "name": "したでなめる",
              "type": "Ghost",
              "category": "Physical",
              "power": 30,
              "description": "長い舌で相手をなめまわして攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            },
            {
              "name": "なきごえ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "かわいいなきごえを聞かせて気をひき油断をさせて相手の攻撃をさげる。"
            }
          ],
          "actions": [],
          "hp": 170,
          "atk": 19,
          "def": 19,
          "sp_atk": 12,
          "sp_def": 18,
          "speed": 13
        },
        {
          "name": "マクノシタ",
          "star": 1,
          "types": [
            "Fighting"
          ],
          "abilities": [
            {
              "name": "あついしぼう",
              "description": "厚い脂肪で守られているのでほのおタイプとこおりタイプの技のダメージを半減させる。"
            },
            {
              "name": "こんじょう",
              "description": "状態異常になると根性をだして攻撃が上がる。"
            },
            {
              "name": "ちからずく",
              "description": "技の追加効果はなくなるがそのぶん高い威力で技を出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "つっぱり",
              "type": "Fighting",
              "category": "Physical",
              "power": 15,
              "description": "ひらいた両手で相手をつっぱって攻撃する。２ー５回の間連続でだす。"
            },
            {
              "name": "たいあたり",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "相手にむかって全身でぶつかっていき攻撃する。"
            },
            {
              "name": "きあいだめ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "深く息を吸い気合をこめる。自分の攻撃が急所に当たりやすくなる。"
            },
            {
              "name": "すなかけ",
              "type": "Ground",
              "category": "Status",
              "power": 0,
              "description": "相手の顔に砂をかけて命中率をさげる。"
            }
          ],
          "actions": [],
          "hp": 195,
          "atk": 19,
          "def": 12,
          "sp_atk": 9,
          "sp_def": 12,
          "speed": 11
        },
        {
          "name": "マケンカニ",
          "star": 1,
          "types": [
            "Fighting"
          ],
          "abilities": [
            {
              "name": "かいりきバサミ",
              "description": "力自慢のハサミを持っているので相手に攻撃を下げられない。"
            },
            {
              "name": "てつのこぶし",
              "description": "パンチを使う技の威力が上がる。"
            },
            {
              "name": "いかりのつぼ",
              "description": "急所に攻撃が当たると怒りくるって攻撃力が最大になる。"
            }
          ],
          "moves": [
            {
              "name": "はさむ",
              "type": "Normal",
              "category": "Physical",
              "power": 55,
              "description": "相手を両側からはさんでダメージをあたえる。"
            },
            {
              "name": "いわくだき",
              "type": "Fighting",
              "category": "Physical",
              "power": 40,
              "description": "パンチで攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "にらみつける",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "鋭い目つきでおびえさせて相手の防御をさげる。"
            },
            {
              "name": "しっぺがえし",
              "type": "Dark",
              "category": "Physical",
              "power": 50,
              "description": "ためこんで攻撃する。相手よりあとに攻撃できると技の威力は２倍になる。"
            }
          ],
          "actions": [],
          "hp": 165,
          "atk": 24,
          "def": 18,
          "sp_atk": 15,
          "sp_def": 16,
          "speed": 20
        },
        {
          "name": "マメバッタ",
          "star": 1,
          "types": [
            "Bug"
          ],
          "abilities": [
            {
              "name": "むしのしらせ",
              "description": "ＨＰが減ったときむしタイプの技の威力が上がる。"
            },
            {
              "name": "いろめがね",
              "description": "効果がいまひとつの技を通常の威力で出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "ダメおし",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "そのターンに相手がすでにダメージを受けていたら技の威力は２倍になる。"
            },
            {
              "name": "にどげり",
              "type": "Fighting",
              "category": "Physical",
              "power": 30,
              "description": "２本の足で相手をけとばして攻撃する。２回連続でダメージを与える。"
            },
            {
              "name": "とびつく",
              "type": "Bug",
              "category": "Physical",
              "power": 50,
              "description": "相手に飛びついて攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "にらみつける",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "鋭い目つきでおびえさせて相手の防御をさげる。"
            }
          ],
          "actions": [],
          "hp": 145,
          "atk": 16,
          "def": 14,
          "sp_atk": 10,
          "sp_def": 11,
          "speed": 15
        },
        {
          "name": "ミツハニー",
          "star": 1,
          "types": [
            "Bug",
            "Flying"
          ],
          "abilities": [
            {
              "name": "みつあつめ",
              "description": "戦闘が終わったときあまいミツを拾うことがある。"
            },
            {
              "name": "はりきり",
              "description": "自分の攻撃が高くなるが命中率が下がる。"
            }
          ],
          "moves": [
            {
              "name": "あまいかおり",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "香りで相手の回避率をがくっとさげる。"
            },
            {
              "name": "むしのていこう",
              "type": "Bug",
              "category": "Special",
              "power": 50,
              "description": "抵抗して相手を攻撃する。相手の特攻をさげる。"
            },
            {
              "name": "かぜおこし",
              "type": "Flying",
              "category": "Special",
              "power": 40,
              "description": "翼でおこした激しい風を相手にぶつけて攻撃する。"
            },
            {
              "name": "むしくい",
              "type": "Bug",
              "category": "Physical",
              "power": 60,
              "description": "かみついて攻撃する。相手がきのみを持っているとき食べてきのみの効果を受けられる。"
            }
          ],
          "actions": [],
          "hp": 145,
          "atk": 12,
          "def": 15,
          "sp_atk": 12,
          "sp_def": 15,
          "speed": 21
        },
        {
          "name": "ミニーブ",
          "star": 1,
          "types": [
            "Grass",
            "Normal"
          ],
          "abilities": [
            {
              "name": "はやおき",
              "description": "ねむり状態になっても２倍の早さで目覚めることができる。"
            },
            {
              "name": "しゅうかく",
              "description": "使ったきのみを何回も作りだす。"
            }
          ],
          "moves": [
            {
              "name": "たいあたり",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "相手にむかって全身でぶつかっていき攻撃する。"
            },
            {
              "name": "すいとる",
              "type": "Grass",
              "category": "Special",
              "power": 20,
              "description": "養分を吸い取り攻撃する。相手に与えたダメージの半分のＨＰを回復できる。"
            },
            {
              "name": "せいちょう",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体を一気に大きく生長させて攻撃と特攻をあげる。"
            },
            {
              "name": "はっぱカッター",
              "type": "Grass",
              "category": "Physical",
              "power": 55,
              "description": "はっぱをとばして相手を切りつけて攻撃する。急所に当たりやすい。"
            }
          ],
          "actions": [],
          "hp": 155,
          "atk": 13,
          "def": 15,
          "sp_atk": 18,
          "sp_def": 17,
          "speed": 12
        },
        {
          "name": "ミブリム",
          "star": 1,
          "types": [
            "Psychic"
          ],
          "abilities": [
            {
              "name": "いやしのこころ",
              "description": "状態異常の味方をたまに治してあげる。"
            },
            {
              "name": "きけんよち",
              "description": "相手の持つ危険な技を察知することができる。"
            },
            {
              "name": "マジックミラー",
              "description": "相手にだされた変化技を受けずにそのまま返すことができる。"
            }
          ],
          "moves": [
            {
              "name": "ねんりき",
              "type": "Psychic",
              "category": "Special",
              "power": 50,
              "description": "弱い念力を相手に送って攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "チャームボイス",
              "type": "Fairy",
              "category": "Special",
              "power": 40,
              "description": "魅惑の鳴き声をだして相手に精神的なダメージを与える。攻撃は必ず命中する。"
            },
            {
              "name": "マジカルリーフ",
              "type": "Grass",
              "category": "Special",
              "power": 60,
              "description": "相手を追跡する不思議なはっぱをまきちらす。攻撃は必ず命中する。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            }
          ],
          "actions": [],
          "hp": 160,
          "atk": 12,
          "def": 15,
          "sp_atk": 18,
          "sp_def": 17,
          "speed": 14
        },
        {
          "name": "ムックル",
          "star": 1,
          "types": [
            "Normal",
            "Flying"
          ],
          "abilities": [
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            },
            {
              "name": "すてみ",
              "description": "反動でダメージを受ける技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "つばめがえし",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "素早い動きで相手をほんろうして切りつける。攻撃は必ず命中する。"
            },
            {
              "name": "でんこうせっか",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "かげぶんしん",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "素早い動きで分身をつくり相手をまどわせて回避率をあげる。"
            },
            {
              "name": "なきごえ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "かわいいなきごえを聞かせて気をひき油断をさせて相手の攻撃をさげる。"
            }
          ],
          "actions": [],
          "hp": 155,
          "atk": 18,
          "def": 12,
          "sp_atk": 12,
          "sp_def": 12,
          "speed": 19
        },
        {
          "name": "メェークル",
          "star": 1,
          "types": [
            "Grass"
          ],
          "abilities": [
            {
              "name": "そうしょく",
              "description": "くさタイプの技を受けるとダメージを受けずに攻撃が上がる。"
            },
            {
              "name": "くさのけがわ",
              "description": "グラスフィールドのとき防御が上がる。"
            }
          ],
          "moves": [
            {
              "name": "たいあたり",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "相手にむかって全身でぶつかっていき攻撃する。"
            },
            {
              "name": "せいちょう",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体を一気に大きく生長させて攻撃と特攻をあげる。"
            },
            {
              "name": "つるのムチ",
              "type": "Grass",
              "category": "Physical",
              "power": 45,
              "description": "ムチのようにしなる細長いつるで相手をたたきつけて攻撃する。"
            },
            {
              "name": "はっぱカッター",
              "type": "Grass",
              "category": "Physical",
              "power": 55,
              "description": "はっぱをとばして相手を切りつけて攻撃する。急所に当たりやすい。"
            }
          ],
          "actions": [],
          "hp": 185,
          "atk": 20,
          "def": 16,
          "sp_atk": 19,
          "sp_def": 18,
          "speed": 17
        },
        {
          "name": "ヤングース",
          "star": 1,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "はりこみ",
              "description": "交代で出てきた相手に２倍のダメージで攻撃できる。"
            },
            {
              "name": "がんじょうあご",
              "description": "あごが頑丈で噛む技の威力が高くなる。"
            },
            {
              "name": "てきおうりょく",
              "description": "自分とおなじタイプの技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "たいあたり",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "相手にむかって全身でぶつかっていき攻撃する。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "すなかけ",
              "type": "Ground",
              "category": "Status",
              "power": 0,
              "description": "相手の顔に砂をかけて命中率をさげる。"
            },
            {
              "name": "ふるいたてる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "自分を奮いたてて攻撃と特攻をあげる。"
            }
          ],
          "actions": [],
          "hp": 165,
          "atk": 21,
          "def": 12,
          "sp_atk": 12,
          "sp_def": 12,
          "speed": 15
        },
        {
          "name": "ユキハミ",
          "star": 1,
          "types": [
            "Ice",
            "Bug"
          ],
          "abilities": [
            {
              "name": "りんぷん",
              "description": "りんぷんに守られて技の追加効果を受けなくなる。"
            },
            {
              "name": "こおりのりんぷん",
              "description": "こおりのりんぷんに守られて特殊攻撃で受けるダメージが半減する。"
            }
          ],
          "moves": [
            {
              "name": "こなゆき",
              "type": "Ice",
              "category": "Special",
              "power": 40,
              "description": "冷たい粉雪を相手に吹きつけて攻撃する。こおり状態にすることがある。"
            },
            {
              "name": "むしのていこう",
              "type": "Bug",
              "category": "Special",
              "power": 50,
              "description": "抵抗して相手を攻撃する。相手の特攻をさげる。"
            },
            {
              "name": "こごえるかぜ",
              "type": "Ice",
              "category": "Special",
              "power": 55,
              "description": "凍てつく冷気を相手に吹きつけて攻撃する。相手の素早さをさげる。"
            }
          ],
          "actions": [],
          "hp": 145,
          "atk": 11,
          "def": 13,
          "sp_atk": 15,
          "sp_def": 12,
          "speed": 9
        },
        {
          "name": "ラルトス",
          "star": 1,
          "types": [
            "Psychic",
            "Fairy"
          ],
          "abilities": [
            {
              "name": "シンクロ",
              "description": "自分がなってしまったどくやまひややけどを相手にうつす。"
            },
            {
              "name": "トレース",
              "description": "登場したとき相手の特性をトレースして同じ特性になる。"
            },
            {
              "name": "テレパシー",
              "description": "味方の攻撃を読み取って技を回避する。"
            }
          ],
          "moves": [
            {
              "name": "チャームボイス",
              "type": "Fairy",
              "category": "Special",
              "power": 40,
              "description": "魅惑の鳴き声をだして相手に精神的なダメージを与える。攻撃は必ず命中する。"
            },
            {
              "name": "ねんりき",
              "type": "Psychic",
              "category": "Special",
              "power": 50,
              "description": "弱い念力を相手に送って攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "ドレインキッス",
              "type": "Fairy",
              "category": "Special",
              "power": 50,
              "description": "キッスによって相手からＨＰを吸い取る。与えたダメージの半分以上ＨＰを回復する。"
            },
            {
              "name": "さいみんじゅつ",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "眠気を誘う暗示をかけて相手を眠り状態にする。"
            }
          ],
          "actions": [],
          "hp": 140,
          "atk": 11,
          "def": 11,
          "sp_atk": 15,
          "sp_def": 13,
          "speed": 14
        },
        {
          "name": "ルリリ",
          "star": 1,
          "types": [
            "Normal",
            "Fairy"
          ],
          "abilities": [
            {
              "name": "あついしぼう",
              "description": "厚い脂肪で守られているのでほのおタイプとこおりタイプの技のダメージを半減させる。"
            },
            {
              "name": "ちからもち",
              "description": "物理攻撃の威力が２倍になる。"
            },
            {
              "name": "そうしょく",
              "description": "くさタイプの技を受けるとダメージを受けずに攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "みずでっぽう",
              "type": "Water",
              "category": "Special",
              "power": 40,
              "description": "水を勢いよく相手に発射して攻撃する。"
            },
            {
              "name": "あまえる",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。"
            },
            {
              "name": "しっぽをふる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "しっぽを左右にかわいくふって油断を誘う。相手の防御をさげる。"
            },
            {
              "name": "たたきつける",
              "type": "Normal",
              "category": "Physical",
              "power": 80,
              "description": "長いしっぽやつるなどを使い相手をたたきつけて攻撃する。"
            }
          ],
          "actions": [],
          "hp": 170,
          "atk": 9,
          "def": 14,
          "sp_atk": 9,
          "sp_def": 14,
          "speed": 9
        },
        {
          "name": "アサナン",
          "star": 2,
          "types": [
            "Fighting",
            "Psychic"
          ],
          "abilities": [
            {
              "name": "ヨガパワー",
              "description": "ヨガの力で物理攻撃の威力が２倍になる。"
            },
            {
              "name": "テレパシー",
              "description": "味方の攻撃を読み取って技を回避する。"
            }
          ],
          "moves": [
            {
              "name": "ねこだまし",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "先制攻撃で相手をひるませる。戦闘にでたらすぐにださないと成功しない。"
            },
            {
              "name": "ねんりき",
              "type": "Psychic",
              "category": "Special",
              "power": 50,
              "description": "弱い念力を相手に送って攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "ローキック",
              "type": "Fighting",
              "category": "Physical",
              "power": 65,
              "description": "素早い動きで相手の足をねらって攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "じこあんじ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "自分に暗示をかけることで能力変化の状態を相手と同じにする。"
            }
          ],
          "actions": [],
          "hp": 210,
          "atk": 21,
          "def": 27,
          "sp_atk": 21,
          "sp_def": 27,
          "speed": 29
        },
        {
          "name": "アノクサ",
          "star": 2,
          "types": [
            "Grass",
            "Ghost"
          ],
          "abilities": [
            {
              "name": "かぜのり",
              "description": "おいかぜが吹いたり風技を受けるとダメージを受けずに攻撃が上がる。"
            },
            {
              "name": "すりぬけ",
              "description": "相手の壁や身代わりをすりぬけて攻撃できる。"
            }
          ],
          "moves": [
            {
              "name": "ころがる",
              "type": "Rock",
              "category": "Physical",
              "power": 30,
              "description": "５ターンの間転がり続けて攻撃する。技が当たるたびに威力があがる。"
            },
            {
              "name": "タネマシンガン",
              "type": "Grass",
              "category": "Physical",
              "power": 25,
              "description": "タネを勢いよく相手に発射して攻撃する。２ー５回の間連続でだす。"
            },
            {
              "name": "まとわりつく",
              "type": "Bug",
              "category": "Special",
              "power": 20,
              "description": "４ー５ターンの間相手にまとわりついて攻撃する。そのあいだ相手は逃げられない。"
            },
            {
              "name": "かげうち",
              "type": "Ghost",
              "category": "Physical",
              "power": 40,
              "description": "影をのばして相手の背後から攻撃する。必ず先制攻撃できる。"
            }
          ],
          "actions": [],
          "hp": 230,
          "atk": 31,
          "def": 17,
          "sp_atk": 23,
          "sp_def": 19,
          "speed": 29
        },
        {
          "name": "ウデッポウ",
          "star": 2,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "メガランチャー",
              "description": "波動の技の威力が高くなる。"
            }
          ],
          "moves": [
            {
              "name": "アクアジェット",
              "type": "Water",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "うちおとす",
              "type": "Rock",
              "category": "Physical",
              "power": 50,
              "description": "石や弾を投げて飛んでいる相手を攻撃する。相手はうち落とされて地面に落ちる。"
            },
            {
              "name": "どろぼう",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "攻撃と同時に道具を盗む。自分が道具を持っている場合は盗めない。"
            },
            {
              "name": "じたばた",
              "type": "Normal",
              "category": "Physical",
              "power": 1,
              "description": "じたばた暴れて攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            }
          ],
          "actions": [],
          "hp": 250,
          "atk": 26,
          "def": 29,
          "sp_atk": 28,
          "sp_def": 30,
          "speed": 22
        },
        {
          "name": "カゲボウズ",
          "star": 2,
          "types": [
            "Ghost"
          ],
          "abilities": [
            {
              "name": "ふみん",
              "description": "眠れない体質なのでねむり状態にならない。"
            },
            {
              "name": "おみとおし",
              "description": "登場したとき相手の持ち物を見通すことができる。"
            },
            {
              "name": "のろわれボディ",
              "description": "攻撃を受けると相手の技をかなしばり状態にすることがある。"
            }
          ],
          "moves": [
            {
              "name": "かげうち",
              "type": "Ghost",
              "category": "Physical",
              "power": 40,
              "description": "影をのばして相手の背後から攻撃する。必ず先制攻撃できる。"
            },
            {
              "name": "いやなおと",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "おもわず耳をふさぎたくなるいやなおとをだして相手の防御をがくっとさげる。"
            },
            {
              "name": "おにび",
              "type": "Fire",
              "category": "Status",
              "power": 0,
              "description": "不気味で怪しい炎を放って相手をやけどの状態にする。"
            },
            {
              "name": "サイケこうせん",
              "type": "Psychic",
              "category": "Special",
              "power": 65,
              "description": "不思議な光線を相手に発射して攻撃する。混乱させることがある。"
            }
          ],
          "actions": [],
          "hp": 235,
          "atk": 35,
          "def": 19,
          "sp_atk": 30,
          "sp_def": 18,
          "speed": 23
        },
        {
          "name": "カヌチャン",
          "star": 2,
          "types": [
            "Fairy",
            "Steel"
          ],
          "abilities": [
            {
              "name": "かたやぶり",
              "description": "相手の特性にジャマされることなく相手に技を出すことができる。"
            },
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "わるいてぐせ",
              "description": "触られた相手の道具を盗んでしまう。"
            }
          ],
          "moves": [
            {
              "name": "うそなき",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "ないたふりをして涙を流す。こまらせることで相手の特防をがくっとさげる。"
            },
            {
              "name": "メタルクロー",
              "type": "Steel",
              "category": "Physical",
              "power": 50,
              "description": "鋼鉄のツメで相手を切り裂いて攻撃する。自分の攻撃があがることがある。"
            },
            {
              "name": "おどろかす",
              "type": "Ghost",
              "category": "Physical",
              "power": 30,
              "description": "大きな声などで不意に驚かして攻撃する。相手をひるませることがある。"
            },
            {
              "name": "いわくだき",
              "type": "Fighting",
              "category": "Physical",
              "power": 40,
              "description": "パンチで攻撃する。相手の防御をさげることがある。"
            }
          ],
          "actions": [],
          "hp": 250,
          "atk": 23,
          "def": 23,
          "sp_atk": 19,
          "sp_def": 30,
          "speed": 28
        },
        {
          "name": "カプサイジ",
          "star": 2,
          "types": [
            "Grass"
          ],
          "abilities": [
            {
              "name": "ようりょくそ",
              "description": "天気が晴れのとき素早さが上がる。"
            },
            {
              "name": "ふみん",
              "description": "眠れない体質なのでねむり状態にならない。"
            },
            {
              "name": "ぶきよう",
              "description": "持っている道具を使うことができない。"
            }
          ],
          "moves": [
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "はっぱカッター",
              "type": "Grass",
              "category": "Physical",
              "power": 55,
              "description": "はっぱをとばして相手を切りつけて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "タネマシンガン",
              "type": "Grass",
              "category": "Physical",
              "power": 25,
              "description": "タネを勢いよく相手に発射して攻撃する。２ー５回の間連続でだす。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [],
          "hp": 250,
          "atk": 29,
          "def": 21,
          "sp_atk": 29,
          "sp_def": 21,
          "speed": 25
        },
        {
          "name": "カラナクシ",
          "star": 2,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "ねんちゃく",
              "description": "粘着質の体に道具がくっついているため相手に道具を奪われない。"
            },
            {
              "name": "よびみず",
              "description": "みずタイプの技を自分によせつけダメージは受けずに特攻が上がる。"
            },
            {
              "name": "すなのちから",
              "description": "天気がすなあらしのときいわタイプとじめんタイプとはがねタイプの威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "マッドショット",
              "type": "Ground",
              "category": "Special",
              "power": 55,
              "description": "泥の塊を相手に投げつけて攻撃する。同時に相手の素早さをさげる。"
            },
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "げんしのちから",
              "type": "Rock",
              "category": "Special",
              "power": 60,
              "description": "原始の力で攻撃する。自分のすべての能力があがることがある。"
            },
            {
              "name": "クリアスモッグ",
              "type": "Poison",
              "category": "Special",
              "power": 50,
              "description": "特殊な泥の塊を相手に投げつけて攻撃する。能力変化をもとにもどす。"
            }
          ],
          "actions": [],
          "hp": 300,
          "atk": 24,
          "def": 24,
          "sp_atk": 27,
          "sp_def": 29,
          "speed": 18
        },
        {
          "name": "カラナクシ",
          "star": 2,
          "types": [],
          "abilities": [
            {
              "name": "ねんちゃく",
              "description": "粘着質の体に道具がくっついているため相手に道具を奪われない。"
            },
            {
              "name": "よびみず",
              "description": "みずタイプの技を自分によせつけダメージは受けずに特攻が上がる。"
            },
            {
              "name": "すなのちから",
              "description": "天気がすなあらしのときいわタイプとじめんタイプとはがねタイプの威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "マッドショット",
              "type": "Ground",
              "category": "Special",
              "power": 55,
              "description": "泥の塊を相手に投げつけて攻撃する。同時に相手の素早さをさげる。"
            },
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "げんしのちから",
              "type": "Rock",
              "category": "Special",
              "power": 60,
              "description": "原始の力で攻撃する。自分のすべての能力があがることがある。"
            },
            {
              "name": "クリアスモッグ",
              "type": "Poison",
              "category": "Special",
              "power": 50,
              "description": "特殊な泥の塊を相手に投げつけて攻撃する。能力変化をもとにもどす。"
            }
          ],
          "actions": [],
          "hp": 0,
          "atk": 0,
          "def": 0,
          "sp_atk": 0,
          "sp_def": 0,
          "speed": 0
        },
        {
          "name": "ガーディ",
          "star": 2,
          "types": [
            "Fire"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "もらいび",
              "description": "ほのおタイプの技を受けると炎をもらい自分が出すほのおタイプの技が強くなる。"
            },
            {
              "name": "せいぎのこころ",
              "description": "あくタイプの攻撃を受けると正義感で攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "かえんぐるま",
              "type": "Fire",
              "category": "Physical",
              "power": 60,
              "description": "炎をまとい相手に突進して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "とおぼえ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大声でほえて気合を高め自分と味方の攻撃をあげる。"
            },
            {
              "name": "かたきうち",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "倒れた味方のかたきを討つ。前のターンに味方が倒されていると威力があがる。"
            }
          ],
          "actions": [],
          "hp": 260,
          "atk": 33,
          "def": 23,
          "sp_atk": 33,
          "sp_def": 25,
          "speed": 29
        },
        {
          "name": "クズモー",
          "star": 2,
          "types": [
            "Poison",
            "Water"
          ],
          "abilities": [
            {
              "name": "どくのトゲ",
              "description": "自分に触った相手をどく状態にすることがある。"
            },
            {
              "name": "どくしゅ",
              "description": "触るだけで相手をどく状態にすることがある。"
            },
            {
              "name": "てきおうりょく",
              "description": "自分とおなじタイプの技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ようかいえき",
              "type": "Poison",
              "category": "Special",
              "power": 40,
              "description": "強い酸を相手にかけて攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "たつまき",
              "type": "Dragon",
              "category": "Special",
              "power": 40,
              "description": "竜巻をおこして相手をまきこみ攻撃する。相手をひるませることがある。"
            },
            {
              "name": "くろいきり",
              "type": "Ice",
              "category": "Status",
              "power": 0,
              "description": "黒い霧をだして戦闘にでているポケモン全員の能力変化をもとにもどす。"
            },
            {
              "name": "みずでっぽう",
              "type": "Water",
              "category": "Special",
              "power": 40,
              "description": "水を勢いよく相手に発射して攻撃する。"
            }
          ],
          "actions": [],
          "hp": 250,
          "atk": 29,
          "def": 29,
          "sp_atk": 29,
          "sp_def": 29,
          "speed": 17
        },
        {
          "name": "クヌギダマ",
          "star": 2,
          "types": [
            "Bug"
          ],
          "abilities": [
            {
              "name": "がんじょう",
              "description": "ＨＰが満タンのとき技を受けても一撃で倒されることがない。一撃必殺技も効かない。"
            },
            {
              "name": "ぼうじん",
              "description": "すなあらしのダメージを受けない。粉や胞子の影響も受けない。"
            }
          ],
          "moves": [
            {
              "name": "むしくい",
              "type": "Bug",
              "category": "Physical",
              "power": 60,
              "description": "かみついて攻撃する。相手がきのみを持っているとき食べてきのみの効果を受けられる。"
            },
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "ころがる",
              "type": "Rock",
              "category": "Physical",
              "power": 30,
              "description": "５ターンの間転がり続けて攻撃する。技が当たるたびに威力があがる。"
            },
            {
              "name": "むしのていこう",
              "type": "Bug",
              "category": "Special",
              "power": 50,
              "description": "抵抗して相手を攻撃する。相手の特攻をさげる。"
            }
          ],
          "actions": [],
          "hp": 250,
          "atk": 31,
          "def": 41,
          "sp_atk": 19,
          "sp_def": 19,
          "speed": 11
        },
        {
          "name": "グレッグル",
          "star": 2,
          "types": [
            "Poison",
            "Fighting"
          ],
          "abilities": [
            {
              "name": "きけんよち",
              "description": "相手の持つ危険な技を察知することができる。"
            },
            {
              "name": "かんそうはだ",
              "description": "天気が雨の時やみずタイプの技でＨＰが回復しはれの時やほのおタイプの技で減ってしまう。"
            },
            {
              "name": "どくしゅ",
              "description": "触るだけで相手をどく状態にすることがある。"
            }
          ],
          "moves": [
            {
              "name": "おどろかす",
              "type": "Ghost",
              "category": "Physical",
              "power": 30,
              "description": "大きな声などで不意に驚かして攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            },
            {
              "name": "けたぐり",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "足を強くけり相手を転ばせて攻撃する。相手が重いほど威力があがる。"
            },
            {
              "name": "ベノムショック",
              "type": "Poison",
              "category": "Special",
              "power": 65,
              "description": "特殊な毒液を浴びせかける。毒状態の相手には威力が２倍になる。"
            }
          ],
          "actions": [],
          "hp": 245,
          "atk": 29,
          "def": 21,
          "sp_atk": 29,
          "sp_def": 21,
          "speed": 25
        },
        {
          "name": "ケイコウオ",
          "star": 2,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "すいすい",
              "description": "天気が雨のとき素早さが上がる。"
            },
            {
              "name": "よびみず",
              "description": "みずタイプの技を自分によせつけダメージは受けずに特攻が上がる。"
            },
            {
              "name": "みずのベール",
              "description": "水のベールを身にまといやけど状態にならない。"
            }
          ],
          "moves": [
            {
              "name": "みずでっぽう",
              "type": "Water",
              "category": "Special",
              "power": 40,
              "description": "水を勢いよく相手に発射して攻撃する。"
            },
            {
              "name": "かぜおこし",
              "type": "Flying",
              "category": "Special",
              "power": 40,
              "description": "翼でおこした激しい風を相手にぶつけて攻撃する。"
            },
            {
              "name": "オーロラビーム",
              "type": "Ice",
              "category": "Special",
              "power": 65,
              "description": "にじいろのビームを相手に発射して攻撃する。攻撃をさげることがある。"
            },
            {
              "name": "あやしいひかり",
              "type": "Ghost",
              "category": "Status",
              "power": 0,
              "description": "怪しい光を相手にみせてまどわせる。相手を混乱させる。"
            }
          ],
          "actions": [],
          "hp": 245,
          "atk": 24,
          "def": 27,
          "sp_atk": 24,
          "sp_def": 29,
          "speed": 31
        },
        {
          "name": "コイキング",
          "star": 2,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "すいすい",
              "description": "天気が雨のとき素早さが上がる。"
            },
            {
              "name": "びびり",
              "description": "あくゴーストむしタイプの攻撃を受けたりいかくをされるとびびって素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "たいあたり",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "相手にむかって全身でぶつかっていき攻撃する。"
            },
            {
              "name": "じたばた",
              "type": "Normal",
              "category": "Physical",
              "power": 1,
              "description": "じたばた暴れて攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            },
            {
              "name": "はねる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "攻撃もせずにピョンピョンと跳ねるだけでなにもおこらない……。"
            }
          ],
          "actions": [],
          "hp": 190,
          "atk": 9,
          "def": 27,
          "sp_atk": 11,
          "sp_def": 13,
          "speed": 37
        },
        {
          "name": "コイル",
          "star": 2,
          "types": [
            "Electric",
            "Steel"
          ],
          "abilities": [
            {
              "name": "じりょく",
              "description": "はがねタイプのポケモンを磁力で引きつけて逃げられなくする。"
            },
            {
              "name": "がんじょう",
              "description": "ＨＰが満タンのとき技を受けても一撃で倒されることがない。一撃必殺技も効かない。"
            },
            {
              "name": "アナライズ",
              "description": "いちばん最後に技を出すと技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            },
            {
              "name": "スパーク",
              "type": "Electric",
              "category": "Physical",
              "power": 65,
              "description": "電気をまとい相手に突進して攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "いやなおと",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "おもわず耳をふさぎたくなるいやなおとをだして相手の防御をがくっとさげる。"
            },
            {
              "name": "エレキネット",
              "type": "Electric",
              "category": "Special",
              "power": 55,
              "description": "電気のネットで相手を捕まえて攻撃する。相手の素早さをさげる。"
            }
          ],
          "actions": [],
          "hp": 200,
          "atk": 19,
          "def": 33,
          "sp_atk": 43,
          "sp_def": 27,
          "speed": 23
        },
        {
          "name": "コダック",
          "star": 2,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "しめりけ",
              "description": "あたりを湿らせることによってじばくなどの爆発する技をだれも使えなくなる。"
            },
            {
              "name": "ノーてんき",
              "description": "あらゆる天気の影響がなくなってしまう。"
            },
            {
              "name": "すいすい",
              "description": "天気が雨のとき素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "しねんのずつき",
              "type": "Psychic",
              "category": "Physical",
              "power": 80,
              "description": "思念の力を額に集めて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "かなしばり",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手の動きをとめて直前にだしていた技を４ターンの間使えなくする。"
            },
            {
              "name": "さいみんじゅつ",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "眠気を誘う暗示をかけて相手を眠り状態にする。"
            }
          ],
          "actions": [],
          "hp": 250,
          "atk": 25,
          "def": 24,
          "sp_atk": 31,
          "sp_def": 25,
          "speed": 27
        },
        {
          "name": "コマタナ",
          "star": 2,
          "types": [
            "Dark",
            "Steel"
          ],
          "abilities": [
            {
              "name": "まけんき",
              "description": "相手に能力を下げられると攻撃がぐーんと上がる。"
            },
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            },
            {
              "name": "プレッシャー",
              "description": "プレッシャーをあたえて相手の使う技のＰＰを多く減らす。"
            }
          ],
          "moves": [
            {
              "name": "れんぞくぎり",
              "type": "Bug",
              "category": "Physical",
              "power": 40,
              "description": "カマやツメなどで相手を切りつけて攻撃する。連続で当てると威力があがる。"
            },
            {
              "name": "メタルクロー",
              "type": "Steel",
              "category": "Physical",
              "power": 50,
              "description": "鋼鉄のツメで相手を切り裂いて攻撃する。自分の攻撃があがることがある。"
            },
            {
              "name": "こわいかお",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "恐ろしい顔でにらみおびえさせて相手の素早さをがくっとさげる。"
            },
            {
              "name": "ダメおし",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "そのターンに相手がすでにダメージを受けていたら技の威力は２倍になる。"
            }
          ],
          "actions": [],
          "hp": 240,
          "atk": 39,
          "def": 33,
          "sp_atk": 21,
          "sp_def": 21,
          "speed": 29
        },
        {
          "name": "コリンク",
          "star": 2,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "とうそうしん",
              "description": "性別が同じだと闘争心を燃やして強くなる。性別が違うと弱くなる。"
            },
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "こんじょう",
              "description": "状態異常になると根性をだして攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "かみなりのキバ",
              "type": "Electric",
              "category": "Physical",
              "power": 65,
              "description": "電気をためたキバでかみつく。相手をひるませたりまひ状態にすることがある。"
            },
            {
              "name": "にどげり",
              "type": "Fighting",
              "category": "Physical",
              "power": 30,
              "description": "２本の足で相手をけとばして攻撃する。２回連続でダメージを与える。"
            },
            {
              "name": "つぶらなひとみ",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "つぶらなひとみで相手をみつめて攻撃をさげる。必ず先制攻撃できる。"
            }
          ],
          "actions": [],
          "hp": 240,
          "atk": 31,
          "def": 18,
          "sp_atk": 21,
          "sp_def": 18,
          "speed": 23
        },
        {
          "name": "ゴクリン",
          "star": 2,
          "types": [
            "Poison"
          ],
          "abilities": [
            {
              "name": "ヘドロえき",
              "description": "ヘドロ液を吸い取った相手は強烈な悪臭でダメージを受けてＨＰを減らす。"
            },
            {
              "name": "ねんちゃく",
              "description": "粘着質の体に道具がくっついているため相手に道具を奪われない。"
            },
            {
              "name": "くいしんぼう",
              "description": "ＨＰが少なくなったら食べるきのみをＨＰ半分の時に食べてしまう。"
            }
          ],
          "moves": [
            {
              "name": "ヘドロこうげき",
              "type": "Poison",
              "category": "Special",
              "power": 65,
              "description": "汚いヘドロを相手に投げつけて攻撃する。毒状態にすることがある。"
            },
            {
              "name": "アシッドボム",
              "type": "Poison",
              "category": "Special",
              "power": 40,
              "description": "相手をとかす液体を吐きだして攻撃する。相手の特防をがくっとさげる。"
            },
            {
              "name": "どろかけ",
              "type": "Ground",
              "category": "Special",
              "power": 20,
              "description": "相手の顔などに泥を投げつけて攻撃する。命中率をさげる。"
            },
            {
              "name": "のしかかり",
              "type": "Normal",
              "category": "Physical",
              "power": 85,
              "description": "全身で相手にのしかかり攻撃する。まひ状態にすることがある。"
            }
          ],
          "actions": [],
          "hp": 290,
          "atk": 22,
          "def": 26,
          "sp_atk": 22,
          "sp_def": 26,
          "speed": 21
        },
        {
          "name": "ゴチム",
          "star": 2,
          "types": [
            "Psychic"
          ],
          "abilities": [
            {
              "name": "おみとおし",
              "description": "登場したとき相手の持ち物を見通すことができる。"
            },
            {
              "name": "かちき",
              "description": "相手に能力を下げられると特攻がぐーんと上がる。"
            },
            {
              "name": "かげふみ",
              "description": "相手の影を踏み逃げたり交代できなくする。"
            }
          ],
          "moves": [
            {
              "name": "くすぐる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体をくすぐり笑わせることで相手の攻撃と防御をさげる。"
            },
            {
              "name": "サイケこうせん",
              "type": "Psychic",
              "category": "Special",
              "power": 65,
              "description": "不思議な光線を相手に発射して攻撃する。混乱させることがある。"
            },
            {
              "name": "いちゃもん",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手にいちゃもんをつけて同じ技を２回連続でだせなくする。"
            },
            {
              "name": "はたく",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "長いしっぽや手などを使って相手をはたいて攻撃する。"
            }
          ],
          "actions": [],
          "hp": 240,
          "atk": 17,
          "def": 25,
          "sp_atk": 27,
          "sp_def": 31,
          "speed": 23
        },
        {
          "name": "ゴマゾウ",
          "star": 2,
          "types": [
            "Ground"
          ],
          "abilities": [
            {
              "name": "ものひろい",
              "description": "相手の使った道具を拾ってくることがある。冒険中も拾ってくる。"
            },
            {
              "name": "すながくれ",
              "description": "砂あらしのとき回避率が上がる。"
            }
          ],
          "moves": [
            {
              "name": "まるくなる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体をまるめてちぢこまり自分の防御をあげる。"
            },
            {
              "name": "ころがる",
              "type": "Rock",
              "category": "Physical",
              "power": 30,
              "description": "５ターンの間転がり続けて攻撃する。技が当たるたびに威力があがる。"
            },
            {
              "name": "じならし",
              "type": "Ground",
              "category": "Physical",
              "power": 60,
              "description": "地面を踏みならして自分の周りにいるものを攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "たたきつける",
              "type": "Normal",
              "category": "Physical",
              "power": 80,
              "description": "長いしっぽやつるなどを使い相手をたたきつけて攻撃する。"
            }
          ],
          "actions": [],
          "hp": 330,
          "atk": 29,
          "def": 29,
          "sp_atk": 21,
          "sp_def": 21,
          "speed": 21
        },
        {
          "name": "ゴース",
          "star": 2,
          "types": [
            "Ghost",
            "Poison"
          ],
          "abilities": [
            {
              "name": "ふゆう",
              "description": "地面から浮くことによってじめんタイプの技を受けない。"
            }
          ],
          "moves": [
            {
              "name": "しっぺがえし",
              "type": "Dark",
              "category": "Physical",
              "power": 50,
              "description": "ためこんで攻撃する。相手よりあとに攻撃できると技の威力は２倍になる。"
            },
            {
              "name": "たたりめ",
              "type": "Ghost",
              "category": "Special",
              "power": 65,
              "description": "たたみかけるように攻撃する。状態異常の相手に大きなダメージを与える。"
            },
            {
              "name": "あやしいひかり",
              "type": "Ghost",
              "category": "Status",
              "power": 0,
              "description": "怪しい光を相手にみせてまどわせる。相手を混乱させる。"
            },
            {
              "name": "こごえるかぜ",
              "type": "Ice",
              "category": "Special",
              "power": 55,
              "description": "凍てつく冷気を相手に吹きつけて攻撃する。相手の素早さをさげる。"
            }
          ],
          "actions": [],
          "hp": 210,
          "atk": 19,
          "def": 17,
          "sp_atk": 45,
          "sp_def": 19,
          "speed": 37
        },
        {
          "name": "サシカマス",
          "star": 2,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "すいすい",
              "description": "天気が雨のとき素早さが上がる。"
            },
            {
              "name": "スクリューおびれ",
              "description": "相手の技を引き受ける特性や技の影響を無視できる。"
            }
          ],
          "moves": [
            {
              "name": "アクアジェット",
              "type": "Water",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "つつく",
              "type": "Flying",
              "category": "Physical",
              "power": 35,
              "description": "鋭くとがったくちばしやつので相手を突いて攻撃する。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "こうそくいどう",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "力をぬいて体を軽くして高速で動く。自分の素早さをぐーんとあげる。"
            }
          ],
          "actions": [],
          "hp": 230,
          "atk": 30,
          "def": 21,
          "sp_atk": 21,
          "sp_def": 17,
          "speed": 31
        },
        {
          "name": "サボネア",
          "star": 2,
          "types": [
            "Grass"
          ],
          "abilities": [
            {
              "name": "すながくれ",
              "description": "砂あらしのとき回避率が上がる。"
            },
            {
              "name": "ちょすい",
              "description": "みずタイプの技を受けるとダメージを受けずに回復する。"
            }
          ],
          "moves": [
            {
              "name": "すいとる",
              "type": "Grass",
              "category": "Special",
              "power": 20,
              "description": "養分を吸い取り攻撃する。相手に与えたダメージの半分のＨＰを回復できる。"
            },
            {
              "name": "タネマシンガン",
              "type": "Grass",
              "category": "Physical",
              "power": 25,
              "description": "タネを勢いよく相手に発射して攻撃する。２ー５回の間連続でだす。"
            },
            {
              "name": "すなかけ",
              "type": "Ground",
              "category": "Status",
              "power": 0,
              "description": "相手の顔に砂をかけて命中率をさげる。"
            },
            {
              "name": "しっぺがえし",
              "type": "Dark",
              "category": "Physical",
              "power": 50,
              "description": "ためこんで攻撃する。相手よりあとに攻撃できると技の威力は２倍になる。"
            }
          ],
          "actions": [],
          "hp": 250,
          "atk": 39,
          "def": 21,
          "sp_atk": 39,
          "sp_def": 21,
          "speed": 19
        },
        {
          "name": "シェルダー",
          "star": 2,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "シェルアーマー",
              "description": "硬い殻に守られ相手の攻撃が急所に当たらない。"
            },
            {
              "name": "スキルリンク",
              "description": "連続技を使うといつも最高回数出すことができる。"
            },
            {
              "name": "ぼうじん",
              "description": "すなあらしのダメージを受けない。粉や胞子の影響も受けない。"
            }
          ],
          "moves": [
            {
              "name": "シェルブレード",
              "type": "Water",
              "category": "Physical",
              "power": 75,
              "description": "鋭い貝殻で切りつけて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "こおりのつぶて",
              "type": "Ice",
              "category": "Physical",
              "power": 40,
              "description": "氷の塊を一瞬でつくり相手に素早く放つ。必ず先制攻撃できる。"
            },
            {
              "name": "にらみつける",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "鋭い目つきでおびえさせて相手の防御をさげる。"
            },
            {
              "name": "たいあたり",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "相手にむかって全身でぶつかっていき攻撃する。"
            }
          ],
          "actions": [],
          "hp": 210,
          "atk": 31,
          "def": 45,
          "sp_atk": 23,
          "sp_def": 15,
          "speed": 21
        },
        {
          "name": "スカンプー",
          "star": 2,
          "types": [
            "Poison",
            "Dark"
          ],
          "abilities": [
            {
              "name": "あくしゅう",
              "description": "臭いにおいを放つことによって攻撃したときに相手をひるませることがある。"
            },
            {
              "name": "ゆうばく",
              "description": "ひんしになったとき触った相手にダメージをあたえる。"
            },
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            }
          ],
          "moves": [
            {
              "name": "えんまく",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "煙や墨などを吹きかけて相手の命中率をさげる。"
            },
            {
              "name": "みだれひっかき",
              "type": "Normal",
              "category": "Physical",
              "power": 18,
              "description": "ツメやカマなどで相手をひっかいて攻撃する。２ー５回の間連続でだす。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ベノムショック",
              "type": "Poison",
              "category": "Special",
              "power": 65,
              "description": "特殊な毒液を浴びせかける。毒状態の相手には威力が２倍になる。"
            }
          ],
          "actions": [],
          "hp": 275,
          "atk": 30,
          "def": 23,
          "sp_atk": 21,
          "sp_def": 21,
          "speed": 34
        },
        {
          "name": "スナバァ",
          "star": 2,
          "types": [
            "Ghost",
            "Ground"
          ],
          "abilities": [
            {
              "name": "みずがため",
              "description": "みずタイプの技を受けると防御がぐーんと上がる。"
            },
            {
              "name": "すながくれ",
              "description": "砂あらしのとき回避率が上がる。"
            }
          ],
          "moves": [
            {
              "name": "じならし",
              "type": "Ground",
              "category": "Physical",
              "power": 60,
              "description": "地面を踏みならして自分の周りにいるものを攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "おどろかす",
              "type": "Ghost",
              "category": "Physical",
              "power": 30,
              "description": "大きな声などで不意に驚かして攻撃する。相手をひるませることがある。"
            },
            {
              "name": "すなじごく",
              "type": "Ground",
              "category": "Physical",
              "power": 35,
              "description": "激しく吹きあれる砂あらしの中に４ー５ターンの間相手を閉じこめて攻撃する。"
            },
            {
              "name": "すなかけ",
              "type": "Ground",
              "category": "Status",
              "power": 0,
              "description": "相手の顔に砂をかけて命中率をさげる。"
            }
          ],
          "actions": [],
          "hp": 260,
          "atk": 27,
          "def": 37,
          "sp_atk": 33,
          "sp_def": 23,
          "speed": 11
        },
        {
          "name": "ゾウドウ",
          "star": 2,
          "types": [
            "Steel"
          ],
          "abilities": [
            {
              "name": "ちからずく",
              "description": "技の追加効果はなくなるがそのぶん高い威力で技を出すことができる。"
            },
            {
              "name": "ヘヴィメタル",
              "description": "自分の重さが２倍になる。"
            }
          ],
          "moves": [
            {
              "name": "いわくだき",
              "type": "Fighting",
              "category": "Physical",
              "power": 40,
              "description": "パンチで攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "じならし",
              "type": "Ground",
              "category": "Physical",
              "power": 60,
              "description": "地面を踏みならして自分の周りにいるものを攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "ふみつけ",
              "type": "Normal",
              "category": "Physical",
              "power": 65,
              "description": "大きな足で相手を踏みつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "なきごえ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "かわいいなきごえを聞かせて気をひき油断をさせて相手の攻撃をさげる。"
            }
          ],
          "actions": [],
          "hp": 290,
          "atk": 37,
          "def": 24,
          "sp_atk": 21,
          "sp_def": 24,
          "speed": 21
        },
        {
          "name": "タマゲタケ",
          "star": 2,
          "types": [
            "Grass",
            "Poison"
          ],
          "abilities": [
            {
              "name": "ほうし",
              "description": "攻撃で自分に触れた相手をどくやまひやねむり状態にすることがある。"
            },
            {
              "name": "さいせいりょく",
              "description": "手持ちに引っ込むとＨＰが少し回復する。"
            }
          ],
          "moves": [
            {
              "name": "クリアスモッグ",
              "type": "Poison",
              "category": "Special",
              "power": 50,
              "description": "特殊な泥の塊を相手に投げつけて攻撃する。能力変化をもとにもどす。"
            },
            {
              "name": "おどろかす",
              "type": "Ghost",
              "category": "Physical",
              "power": 30,
              "description": "大きな声などで不意に驚かして攻撃する。相手をひるませることがある。"
            },
            {
              "name": "メガドレイン",
              "type": "Grass",
              "category": "Special",
              "power": 40,
              "description": "養分を吸い取り攻撃する。相手に与えたダメージの半分のＨＰを回復できる。"
            },
            {
              "name": "しびれごな",
              "type": "Grass",
              "category": "Status",
              "power": 0,
              "description": "しびれる粉をたくさんふりまいて相手をまひ状態にする。"
            }
          ],
          "actions": [],
          "hp": 285,
          "atk": 27,
          "def": 23,
          "sp_atk": 27,
          "sp_def": 27,
          "speed": 11
        },
        {
          "name": "タンドン",
          "star": 2,
          "types": [
            "Rock"
          ],
          "abilities": [
            {
              "name": "じょうききかん",
              "description": "みずタイプほのおタイプの技を受けると素早さがぐぐーんと上がる。"
            },
            {
              "name": "たいねつ",
              "description": "耐熱の体によってほのおタイプの技のダメージを半減させる。"
            },
            {
              "name": "もらいび",
              "description": "ほのおタイプの技を受けると炎をもらい自分が出すほのおタイプの技が強くなる。"
            }
          ],
          "moves": [
            {
              "name": "えんまく",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "煙や墨などを吹きかけて相手の命中率をさげる。"
            },
            {
              "name": "どろかけ",
              "type": "Ground",
              "category": "Special",
              "power": 20,
              "description": "相手の顔などに泥を投げつけて攻撃する。命中率をさげる。"
            },
            {
              "name": "やきつくす",
              "type": "Fire",
              "category": "Special",
              "power": 60,
              "description": "炎で相手を攻撃する。相手がきのみなどを持っているとき燃やして使えなくする。"
            },
            {
              "name": "うちおとす",
              "type": "Rock",
              "category": "Physical",
              "power": 50,
              "description": "石や弾を投げて飛んでいる相手を攻撃する。相手はうち落とされて地面に落ちる。"
            }
          ],
          "actions": [],
          "hp": 210,
          "atk": 21,
          "def": 25,
          "sp_atk": 21,
          "sp_def": 25,
          "speed": 17
        },
        {
          "name": "ディグダ",
          "star": 2,
          "types": [
            "Ground"
          ],
          "abilities": [
            {
              "name": "すながくれ",
              "description": "砂あらしのとき回避率が上がる。"
            },
            {
              "name": "ありじごく",
              "description": "戦闘で相手を逃げられなくする。"
            },
            {
              "name": "すなのちから",
              "description": "天気がすなあらしのときいわタイプとじめんタイプとはがねタイプの威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "じならし",
              "type": "Ground",
              "category": "Physical",
              "power": 60,
              "description": "地面を踏みならして自分の周りにいるものを攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "きりさく",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "つめとぎ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "ツメを磨いて鋭くする。自分の攻撃と命中率をあげる。"
            },
            {
              "name": "おどろかす",
              "type": "Ghost",
              "category": "Physical",
              "power": 30,
              "description": "大きな声などで不意に驚かして攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [],
          "hp": 170,
          "atk": 27,
          "def": 15,
          "sp_atk": 19,
          "sp_def": 23,
          "speed": 43
        },
        {
          "name": "デルビル",
          "star": 2,
          "types": [
            "Dark",
            "Fire"
          ],
          "abilities": [
            {
              "name": "はやおき",
              "description": "ねむり状態になっても２倍の早さで目覚めることができる。"
            },
            {
              "name": "もらいび",
              "description": "ほのおタイプの技を受けると炎をもらい自分が出すほのおタイプの技が強くなる。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            }
          ],
          "moves": [
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ひのこ",
              "type": "Fire",
              "category": "Special",
              "power": 40,
              "description": "小さな炎を相手に発射して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "ふくろだたき",
              "type": "Dark",
              "category": "Physical",
              "power": 1,
              "description": "味方全員で攻撃する。仲間のポケモンが多いほど技の攻撃回数が増える。"
            },
            {
              "name": "からげんき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "自分が毒まひやけど状態のとき相手にくりだすと技の威力が２倍になる。"
            }
          ],
          "actions": [],
          "hp": 240,
          "atk": 29,
          "def": 17,
          "sp_atk": 37,
          "sp_def": 25,
          "speed": 31
        },
        {
          "name": "ドロバンコ",
          "star": 2,
          "types": [
            "Ground"
          ],
          "abilities": [
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "じきゅうりょく",
              "description": "攻撃を受けると防御が上がる。"
            },
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            }
          ],
          "moves": [
            {
              "name": "にどげり",
              "type": "Fighting",
              "category": "Physical",
              "power": 30,
              "description": "２本の足で相手をけとばして攻撃する。２回連続でダメージを与える。"
            },
            {
              "name": "じならし",
              "type": "Ground",
              "category": "Physical",
              "power": 60,
              "description": "地面を踏みならして自分の周りにいるものを攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "ふみつけ",
              "type": "Normal",
              "category": "Physical",
              "power": 65,
              "description": "大きな足で相手を踏みつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "どろかけ",
              "type": "Ground",
              "category": "Special",
              "power": 20,
              "description": "相手の顔などに泥を投げつけて攻撃する。命中率をさげる。"
            }
          ],
          "actions": [],
          "hp": 290,
          "atk": 45,
          "def": 33,
          "sp_atk": 23,
          "sp_def": 27,
          "speed": 23
        },
        {
          "name": "ナマケロ",
          "star": 2,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "なまけ",
              "description": "技を出すと次のターンは休んでしまう。"
            }
          ],
          "moves": [
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            },
            {
              "name": "アンコール",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手にアンコールした技を３回続けて出させる。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            },
            {
              "name": "じゃれつく",
              "type": "Fairy",
              "category": "Physical",
              "power": 90,
              "description": "相手にじゃれついて攻撃する。相手の攻撃をさげることがある。"
            }
          ],
          "actions": [],
          "hp": 270,
          "atk": 29,
          "def": 29,
          "sp_atk": 19,
          "sp_def": 19,
          "speed": 17
        },
        {
          "name": "ニャース",
          "star": 2,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "ものひろい",
              "description": "相手の使った道具を拾ってくることがある。冒険中も拾ってくる。"
            },
            {
              "name": "テクニシャン",
              "description": "威力が低い技の威力を高くして攻撃できる。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            }
          ],
          "moves": [
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ネコにこばん",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "相手の体に小判を投げつけて攻撃する。戦闘のあとでお金がもらえる。"
            },
            {
              "name": "ダメおし",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "そのターンに相手がすでにダメージを受けていたら技の威力は２倍になる。"
            },
            {
              "name": "みだれひっかき",
              "type": "Normal",
              "category": "Physical",
              "power": 18,
              "description": "ツメやカマなどで相手をひっかいて攻撃する。２ー５回の間連続でだす。"
            }
          ],
          "actions": [],
          "hp": 230,
          "atk": 23,
          "def": 19,
          "sp_atk": 21,
          "sp_def": 21,
          "speed": 41
        },
        {
          "name": "バネブー",
          "star": 2,
          "types": [
            "Psychic"
          ],
          "abilities": [
            {
              "name": "あついしぼう",
              "description": "厚い脂肪で守られているのでほのおタイプとこおりタイプの技のダメージを半減させる。"
            },
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "くいしんぼう",
              "description": "ＨＰが少なくなったら食べるきのみをＨＰ半分の時に食べてしまう。"
            }
          ],
          "moves": [
            {
              "name": "サイケこうせん",
              "type": "Psychic",
              "category": "Special",
              "power": 65,
              "description": "不思議な光線を相手に発射して攻撃する。混乱させることがある。"
            },
            {
              "name": "あやしいひかり",
              "type": "Ghost",
              "category": "Status",
              "power": 0,
              "description": "怪しい光を相手にみせてまどわせる。相手を混乱させる。"
            },
            {
              "name": "くさわけ",
              "type": "Grass",
              "category": "Physical",
              "power": 50,
              "description": "草むらから飛びだすように攻撃する。軽快な足どりによって自分の素早さをあげる。"
            },
            {
              "name": "ねんりき",
              "type": "Psychic",
              "category": "Special",
              "power": 50,
              "description": "弱い念力を相手に送って攻撃する。相手を混乱させることがある。"
            }
          ],
          "actions": [],
          "hp": 270,
          "atk": 15,
          "def": 19,
          "sp_atk": 33,
          "sp_def": 37,
          "speed": 29
        },
        {
          "name": "ヒメグマ",
          "star": 2,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "ものひろい",
              "description": "相手の使った道具を拾ってくることがある。冒険中も拾ってくる。"
            },
            {
              "name": "はやあし",
              "description": "状態異常になると素早さが上がる。"
            },
            {
              "name": "みつあつめ",
              "description": "戦闘が終わったときあまいミツを拾うことがある。"
            }
          ],
          "moves": [
            {
              "name": "ほしがる",
              "type": "Normal",
              "category": "Physical",
              "power": 60,
              "description": "かわいくあまえながら相手にちかづき持っている道具をうばう。"
            },
            {
              "name": "しっぺがえし",
              "type": "Dark",
              "category": "Physical",
              "power": 50,
              "description": "ためこんで攻撃する。相手よりあとに攻撃できると技の威力は２倍になる。"
            },
            {
              "name": "つぶらなひとみ",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "つぶらなひとみで相手をみつめて攻撃をさげる。必ず先制攻撃できる。"
            },
            {
              "name": "メタルクロー",
              "type": "Steel",
              "category": "Physical",
              "power": 50,
              "description": "鋼鉄のツメで相手を切り裂いて攻撃する。自分の攻撃があがることがある。"
            }
          ],
          "actions": [],
          "hp": 270,
          "atk": 37,
          "def": 25,
          "sp_atk": 25,
          "sp_def": 25,
          "speed": 21
        },
        {
          "name": "ビリリダマ",
          "star": 2,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "ぼうおん",
              "description": "音を遮断することによって音の技を受けない。"
            },
            {
              "name": "せいでんき",
              "description": "静電気を体にまとい触った相手をまひさせることがある。"
            },
            {
              "name": "ゆうばく",
              "description": "ひんしになったとき触った相手にダメージをあたえる。"
            }
          ],
          "moves": [
            {
              "name": "かいでんぱ",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "体からかいでんぱを放ち相手に浴びせることによって特攻をがくっとさげる。"
            },
            {
              "name": "でんきショック",
              "type": "Electric",
              "category": "Special",
              "power": 40,
              "description": "電気の刺激を相手に浴びせて攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "スピードスター",
              "type": "Normal",
              "category": "Special",
              "power": 60,
              "description": "星型の光を発射して相手を攻撃する。攻撃は必ず命中する。"
            },
            {
              "name": "ころがる",
              "type": "Rock",
              "category": "Physical",
              "power": 30,
              "description": "５ターンの間転がり続けて攻撃する。技が当たるたびに威力があがる。"
            }
          ],
          "actions": [],
          "hp": 230,
          "atk": 17,
          "def": 25,
          "sp_atk": 27,
          "sp_def": 27,
          "speed": 45
        },
        {
          "name": "ピカチュウ",
          "star": 2,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "せいでんき",
              "description": "静電気を体にまとい触った相手をまひさせることがある。"
            },
            {
              "name": "ひらいしん",
              "description": "でんきタイプの技を自分に寄せつけダメージを受けずに特攻が上がる。"
            }
          ],
          "moves": [
            {
              "name": "スパーク",
              "type": "Electric",
              "category": "Physical",
              "power": 65,
              "description": "電気をまとい相手に突進して攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "ほっぺすりすり",
              "type": "Electric",
              "category": "Physical",
              "power": 20,
              "description": "電気を帯びたほっぺをすりつけて攻撃。相手をまひ状態にする。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            },
            {
              "name": "スピードスター",
              "type": "Normal",
              "category": "Special",
              "power": 60,
              "description": "星型の光を発射して相手を攻撃する。攻撃は必ず命中する。"
            }
          ],
          "actions": [],
          "hp": 220,
          "atk": 27,
          "def": 21,
          "sp_atk": 25,
          "sp_def": 25,
          "speed": 41
        },
        {
          "name": "フワンテ",
          "star": 2,
          "types": [
            "Ghost",
            "Flying"
          ],
          "abilities": [
            {
              "name": "ゆうばく",
              "description": "ひんしになったとき触った相手にダメージをあたえる。"
            },
            {
              "name": "かるわざ",
              "description": "持っていた道具がなくなると素早さが上がる。"
            },
            {
              "name": "ねつぼうそう",
              "description": "やけど状態になったとき特殊技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "かぜおこし",
              "type": "Flying",
              "category": "Special",
              "power": 40,
              "description": "翼でおこした激しい風を相手にぶつけて攻撃する。"
            },
            {
              "name": "しっぺがえし",
              "type": "Dark",
              "category": "Physical",
              "power": 50,
              "description": "ためこんで攻撃する。相手よりあとに攻撃できると技の威力は２倍になる。"
            },
            {
              "name": "おどろかす",
              "type": "Ghost",
              "category": "Physical",
              "power": 30,
              "description": "大きな声などで不意に驚かして攻撃する。相手をひるませることがある。"
            },
            {
              "name": "くろいきり",
              "type": "Ice",
              "category": "Status",
              "power": 0,
              "description": "黒い霧をだして戦闘にでているポケモン全員の能力変化をもとにもどす。"
            }
          ],
          "actions": [],
          "hp": 330,
          "atk": 25,
          "def": 18,
          "sp_atk": 29,
          "sp_def": 22,
          "speed": 33
        },
        {
          "name": "ブイゼル",
          "star": 2,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "すいすい",
              "description": "天気が雨のとき素早さが上がる。"
            },
            {
              "name": "みずのベール",
              "description": "水のベールを身にまといやけど状態にならない。"
            }
          ],
          "moves": [
            {
              "name": "アクアジェット",
              "type": "Water",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "みずでっぽう",
              "type": "Water",
              "category": "Special",
              "power": 40,
              "description": "水を勢いよく相手に発射して攻撃する。"
            },
            {
              "name": "なきごえ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "かわいいなきごえを聞かせて気をひき油断をさせて相手の攻撃をさげる。"
            },
            {
              "name": "れんぞくぎり",
              "type": "Bug",
              "category": "Physical",
              "power": 40,
              "description": "カマやツメなどで相手を切りつけて攻撃する。連続で当てると威力があがる。"
            }
          ],
          "actions": [],
          "hp": 260,
          "atk": 31,
          "def": 19,
          "sp_atk": 29,
          "sp_def": 17,
          "speed": 39
        },
        {
          "name": "ブロロン",
          "star": 2,
          "types": [
            "Steel",
            "Poison"
          ],
          "abilities": [
            {
              "name": "ぼうじん",
              "description": "すなあらしのダメージを受けない。粉や胞子の影響も受けない。"
            },
            {
              "name": "スロースタート",
              "description": "５ターンのあいだ攻撃と素早さが半分になる。"
            }
          ],
          "moves": [
            {
              "name": "ジャイロボール",
              "type": "Steel",
              "category": "Physical",
              "power": 1,
              "description": "体を高速に回転させて体当たりする。相手より素早さが低いほど強い。"
            },
            {
              "name": "ヘドロこうげき",
              "type": "Poison",
              "category": "Special",
              "power": 65,
              "description": "汚いヘドロを相手に投げつけて攻撃する。毒状態にすることがある。"
            },
            {
              "name": "ダメおし",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "そのターンに相手がすでにダメージを受けていたら技の威力は２倍になる。"
            },
            {
              "name": "どくガス",
              "type": "Poison",
              "category": "Status",
              "power": 0,
              "description": "毒ガスを相手の顔に吹きかけて毒の状態にする。"
            }
          ],
          "actions": [],
          "hp": 240,
          "atk": 33,
          "def": 30,
          "sp_atk": 17,
          "sp_def": 23,
          "speed": 23
        },
        {
          "name": "プリン",
          "star": 2,
          "types": [
            "Normal",
            "Fairy"
          ],
          "abilities": [
            {
              "name": "メロメロボディ",
              "description": "自分に触った相手をメロメロにすることがある。"
            },
            {
              "name": "かちき",
              "description": "相手に能力を下げられると特攻がぐーんと上がる。"
            },
            {
              "name": "フレンドガード",
              "description": "味方のダメージを減らすことができる。"
            }
          ],
          "moves": [
            {
              "name": "のしかかり",
              "type": "Normal",
              "category": "Physical",
              "power": 85,
              "description": "全身で相手にのしかかり攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "チャームボイス",
              "type": "Fairy",
              "category": "Special",
              "power": 40,
              "description": "魅惑の鳴き声をだして相手に精神的なダメージを与える。攻撃は必ず命中する。"
            },
            {
              "name": "ジャイロボール",
              "type": "Steel",
              "category": "Physical",
              "power": 1,
              "description": "体を高速に回転させて体当たりする。相手より素早さが低いほど強い。"
            },
            {
              "name": "うたう",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "心地好いきれいな歌声を聞かせて相手を眠り状態にする。"
            }
          ],
          "actions": [],
          "hp": 380,
          "atk": 23,
          "def": 13,
          "sp_atk": 23,
          "sp_def": 15,
          "speed": 13
        },
        {
          "name": "ベトベター",
          "star": 2,
          "types": [
            "Poison"
          ],
          "abilities": [
            {
              "name": "あくしゅう",
              "description": "臭いにおいを放つことによって攻撃したときに相手をひるませることがある。"
            },
            {
              "name": "ねんちゃく",
              "description": "粘着質の体に道具がくっついているため相手に道具を奪われない。"
            },
            {
              "name": "どくしゅ",
              "description": "触るだけで相手をどく状態にすることがある。"
            }
          ],
          "moves": [
            {
              "name": "ヘドロこうげき",
              "type": "Poison",
              "category": "Special",
              "power": 65,
              "description": "汚いヘドロを相手に投げつけて攻撃する。毒状態にすることがある。"
            },
            {
              "name": "マッドショット",
              "type": "Ground",
              "category": "Special",
              "power": 55,
              "description": "泥の塊を相手に投げつけて攻撃する。同時に相手の素早さをさげる。"
            },
            {
              "name": "かなしばり",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手の動きをとめて直前にだしていた技を４ターンの間使えなくする。"
            },
            {
              "name": "かげうち",
              "type": "Ghost",
              "category": "Physical",
              "power": 40,
              "description": "影をのばして相手の背後から攻撃する。必ず先制攻撃できる。"
            }
          ],
          "actions": [],
          "hp": 310,
          "atk": 37,
          "def": 25,
          "sp_atk": 21,
          "sp_def": 25,
          "speed": 15
        },
        {
          "name": "ベロバー",
          "star": 2,
          "types": [
            "Dark",
            "Fairy"
          ],
          "abilities": [
            {
              "name": "いたずらごころ",
              "description": "変化技を先制で出すことができる。"
            },
            {
              "name": "おみとおし",
              "description": "登場したとき相手の持ち物を見通すことができる。"
            },
            {
              "name": "わるいてぐせ",
              "description": "触られた相手の道具を盗んでしまう。"
            }
          ],
          "moves": [
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "おだてる",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手をおだてて混乱させる。同時に相手の特攻もあげてしまう。"
            },
            {
              "name": "けたぐり",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "足を強くけり相手を転ばせて攻撃する。相手が重いほど威力があがる。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            }
          ],
          "actions": [],
          "hp": 240,
          "atk": 23,
          "def": 17,
          "sp_atk": 27,
          "sp_def": 21,
          "speed": 25
        },
        {
          "name": "マンキー",
          "star": 2,
          "types": [
            "Fighting"
          ],
          "abilities": [
            {
              "name": "やるき",
              "description": "やる気をだすことによってねむり状態にならない。"
            },
            {
              "name": "いかりのつぼ",
              "description": "急所に攻撃が当たると怒りくるって攻撃力が最大になる。"
            },
            {
              "name": "まけんき",
              "description": "相手に能力を下げられると攻撃がぐーんと上がる。"
            }
          ],
          "moves": [
            {
              "name": "かわらわり",
              "type": "Fighting",
              "category": "Physical",
              "power": 75,
              "description": "手刀を勢いよく振りおろして相手を攻撃する。ひかりのかべやリフレクターなども破壊できる。"
            },
            {
              "name": "みだれひっかき",
              "type": "Normal",
              "category": "Physical",
              "power": 18,
              "description": "ツメやカマなどで相手をひっかいて攻撃する。２ー５回の間連続でだす。"
            },
            {
              "name": "にらみつける",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "鋭い目つきでおびえさせて相手の防御をさげる。"
            },
            {
              "name": "きあいだめ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "深く息を吸い気合をこめる。自分の攻撃が急所に当たりやすくなる。"
            }
          ],
          "actions": [],
          "hp": 230,
          "atk": 37,
          "def": 19,
          "sp_atk": 19,
          "sp_def": 23,
          "speed": 33
        },
        {
          "name": "ムウマ",
          "star": 2,
          "types": [
            "Ghost"
          ],
          "abilities": [
            {
              "name": "ふゆう",
              "description": "地面から浮くことによってじめんタイプの技を受けない。"
            }
          ],
          "moves": [
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            },
            {
              "name": "おどろかす",
              "type": "Ghost",
              "category": "Physical",
              "power": 30,
              "description": "大きな声などで不意に驚かして攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ねんりき",
              "type": "Psychic",
              "category": "Special",
              "power": 50,
              "description": "弱い念力を相手に送って攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "かげうち",
              "type": "Ghost",
              "category": "Physical",
              "power": 40,
              "description": "影をのばして相手の背後から攻撃する。必ず先制攻撃できる。"
            }
          ],
          "actions": [],
          "hp": 270,
          "atk": 29,
          "def": 29,
          "sp_atk": 39,
          "sp_def": 39,
          "speed": 39
        },
        {
          "name": "メグロコ",
          "star": 2,
          "types": [
            "Ground",
            "Dark"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "じしんかじょう",
              "description": "相手を倒すと自信がついて攻撃が上がる。"
            },
            {
              "name": "いかりのつぼ",
              "description": "急所に攻撃が当たると怒りくるって攻撃力が最大になる。"
            }
          ],
          "moves": [
            {
              "name": "じならし",
              "type": "Ground",
              "category": "Physical",
              "power": 60,
              "description": "地面を踏みならして自分の周りにいるものを攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "すなかけ",
              "type": "Ground",
              "category": "Status",
              "power": 0,
              "description": "相手の顔に砂をかけて命中率をさげる。"
            },
            {
              "name": "にらみつける",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "鋭い目つきでおびえさせて相手の防御をさげる。"
            }
          ],
          "actions": [],
          "hp": 250,
          "atk": 33,
          "def": 19,
          "sp_atk": 19,
          "sp_def": 19,
          "speed": 31
        },
        {
          "name": "メリープ",
          "star": 2,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "せいでんき",
              "description": "静電気を体にまとい触った相手をまひさせることがある。"
            },
            {
              "name": "プラス",
              "description": "プラスかマイナスの特性を持つポケモンが仲間にいると自分の特攻が上がる。"
            }
          ],
          "moves": [
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            },
            {
              "name": "でんきショック",
              "type": "Electric",
              "category": "Special",
              "power": 40,
              "description": "電気の刺激を相手に浴びせて攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "たいあたり",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "相手にむかって全身でぶつかっていき攻撃する。"
            },
            {
              "name": "パワージェム",
              "type": "Rock",
              "category": "Special",
              "power": 80,
              "description": "宝石のようにきらめく光を発射して相手を攻撃する。"
            }
          ],
          "actions": [],
          "hp": 260,
          "atk": 21,
          "def": 21,
          "sp_atk": 31,
          "sp_def": 23,
          "speed": 19
        },
        {
          "name": "ヤトウモリ",
          "star": 2,
          "types": [
            "Poison",
            "Fire"
          ],
          "abilities": [
            {
              "name": "ふしょく",
              "description": "はがねタイプやどくタイプもどく状態にすることができる。"
            },
            {
              "name": "どんかん",
              "description": "鈍感なのでメロメロやちょうはつ状態にならない。いかくにも動じない。"
            }
          ],
          "moves": [
            {
              "name": "ひのこ",
              "type": "Fire",
              "category": "Special",
              "power": 40,
              "description": "小さな炎を相手に発射して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "やきつくす",
              "type": "Fire",
              "category": "Special",
              "power": 60,
              "description": "炎で相手を攻撃する。相手がきのみなどを持っているとき燃やして使えなくする。"
            },
            {
              "name": "どろかけ",
              "type": "Ground",
              "category": "Special",
              "power": 20,
              "description": "相手の顔などに泥を投げつけて攻撃する。命中率をさげる。"
            },
            {
              "name": "ポイズンテール",
              "type": "Poison",
              "category": "Physical",
              "power": 50,
              "description": "しっぽでたたく。毒状態にすることがあり急所にも当たりやすい。"
            }
          ],
          "actions": [],
          "hp": 245,
          "atk": 22,
          "def": 21,
          "sp_atk": 33,
          "sp_def": 21,
          "speed": 35
        },
        {
          "name": "ヤドン",
          "star": 2,
          "types": [
            "Water",
            "Psychic"
          ],
          "abilities": [
            {
              "name": "どんかん",
              "description": "鈍感なのでメロメロやちょうはつ状態にならない。いかくにも動じない。"
            },
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "さいせいりょく",
              "description": "手持ちに引っ込むとＨＰが少し回復する。"
            }
          ],
          "moves": [
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "ねんりき",
              "type": "Psychic",
              "category": "Special",
              "power": 50,
              "description": "弱い念力を相手に送って攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            },
            {
              "name": "なきごえ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "かわいいなきごえを聞かせて気をひき油断をさせて相手の攻撃をさげる。"
            }
          ],
          "actions": [],
          "hp": 330,
          "atk": 31,
          "def": 31,
          "sp_atk": 21,
          "sp_def": 21,
          "speed": 11
        },
        {
          "name": "ヤバチャ",
          "star": 2,
          "types": [
            "Ghost"
          ],
          "abilities": [
            {
              "name": "くだけるよろい",
              "description": "物理技でダメージを受けると防御が下がり素早さがぐーんと上がる。"
            },
            {
              "name": "のろわれボディ",
              "description": "攻撃を受けると相手の技をかなしばり状態にすることがある。"
            }
          ],
          "moves": [
            {
              "name": "おどろかす",
              "type": "Ghost",
              "category": "Physical",
              "power": 30,
              "description": "大きな声などで不意に驚かして攻撃する。相手をひるませることがある。"
            },
            {
              "name": "メガドレイン",
              "type": "Grass",
              "category": "Special",
              "power": 40,
              "description": "養分を吸い取り攻撃する。相手に与えたダメージの半分のＨＰを回復できる。"
            },
            {
              "name": "からにこもる",
              "type": "Water",
              "category": "Status",
              "power": 0,
              "description": "殻に潜りこんで身を守り自分の防御をあげる。"
            },
            {
              "name": "サイケこうせん",
              "type": "Psychic",
              "category": "Special",
              "power": 65,
              "description": "不思議な光線を相手に発射して攻撃する。混乱させることがある。"
            }
          ],
          "actions": [],
          "hp": 230,
          "atk": 23,
          "def": 23,
          "sp_atk": 34,
          "sp_def": 26,
          "speed": 25
        },
        {
          "name": "ヤミカラス",
          "star": 2,
          "types": [
            "Dark",
            "Flying"
          ],
          "abilities": [
            {
              "name": "ふみん",
              "description": "眠れない体質なのでねむり状態にならない。"
            },
            {
              "name": "きょううん",
              "description": "強運を持っているため相手の急所に攻撃が当たりやすい。"
            },
            {
              "name": "いたずらごころ",
              "description": "変化技を先制で出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "つばさでうつ",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "大きくひろげたりっぱな翼を相手にぶつけて攻撃する。"
            },
            {
              "name": "おどろかす",
              "type": "Ghost",
              "category": "Physical",
              "power": 30,
              "description": "大きな声などで不意に驚かして攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ダメおし",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "そのターンに相手がすでにダメージを受けていたら技の威力は２倍になる。"
            },
            {
              "name": "くろいきり",
              "type": "Ice",
              "category": "Status",
              "power": 0,
              "description": "黒い霧をだして戦闘にでているポケモン全員の能力変化をもとにもどす。"
            }
          ],
          "actions": [],
          "hp": 270,
          "atk": 39,
          "def": 21,
          "sp_atk": 39,
          "sp_def": 21,
          "speed": 41
        },
        {
          "name": "ヤヤコマ",
          "star": 2,
          "types": [
            "Normal",
            "Flying"
          ],
          "abilities": [
            {
              "name": "はとむね",
              "description": "防御を下げる効果を受けない。"
            },
            {
              "name": "はやてのつばさ",
              "description": "ＨＰが満タンだとひこうタイプの技を先制で出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "ニトロチャージ",
              "type": "Fire",
              "category": "Physical",
              "power": 50,
              "description": "炎をまとい相手を攻撃する。力をためて自分の素早さをあげる。"
            },
            {
              "name": "でんこうせっか",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "じたばた",
              "type": "Normal",
              "category": "Physical",
              "power": 1,
              "description": "じたばた暴れて攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            },
            {
              "name": "つつく",
              "type": "Flying",
              "category": "Physical",
              "power": 35,
              "description": "鋭くとがったくちばしやつので相手を突いて攻撃する。"
            }
          ],
          "actions": [],
          "hp": 240,
          "atk": 25,
          "def": 22,
          "sp_atk": 21,
          "sp_def": 20,
          "speed": 29
        },
        {
          "name": "ユキワラシ",
          "star": 2,
          "types": [
            "Ice"
          ],
          "abilities": [
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            },
            {
              "name": "アイスボディ",
              "description": "天気がゆきのときＨＰを少しずつ回復する。"
            },
            {
              "name": "ムラっけ",
              "description": "毎ターン能力のどれかがぐーんと上がってどれかが下がる。"
            }
          ],
          "moves": [
            {
              "name": "こおりのつぶて",
              "type": "Ice",
              "category": "Physical",
              "power": 40,
              "description": "氷の塊を一瞬でつくり相手に素早く放つ。必ず先制攻撃できる。"
            },
            {
              "name": "こごえるかぜ",
              "type": "Ice",
              "category": "Special",
              "power": 55,
              "description": "凍てつく冷気を相手に吹きつけて攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "にらみつける",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "鋭い目つきでおびえさせて相手の防御をさげる。"
            },
            {
              "name": "ころがる",
              "type": "Rock",
              "category": "Physical",
              "power": 30,
              "description": "５ターンの間転がり続けて攻撃する。技が当たるたびに威力があがる。"
            }
          ],
          "actions": [],
          "hp": 250,
          "atk": 25,
          "def": 25,
          "sp_atk": 25,
          "sp_def": 25,
          "speed": 25
        },
        {
          "name": "ラブカス",
          "star": 2,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "すいすい",
              "description": "天気が雨のとき素早さが上がる。"
            },
            {
              "name": "うるおいボディ",
              "description": "天気が雨のとき状態異常が治る。"
            }
          ],
          "moves": [
            {
              "name": "あまえる",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。"
            },
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "ドレインキッス",
              "type": "Fairy",
              "category": "Special",
              "power": 50,
              "description": "キッスによって相手からＨＰを吸い取る。与えたダメージの半分以上ＨＰを回復する。"
            },
            {
              "name": "じたばた",
              "type": "Normal",
              "category": "Physical",
              "power": 1,
              "description": "じたばた暴れて攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            }
          ],
          "actions": [],
          "hp": 235,
          "atk": 17,
          "def": 27,
          "sp_atk": 21,
          "sp_def": 31,
          "speed": 43
        },
        {
          "name": "ワッカネズミ",
          "star": 2,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "にげあし",
              "description": "野生のポケモンから必ず逃げられる。"
            },
            {
              "name": "ものひろい",
              "description": "相手の使った道具を拾ってくることがある。冒険中も拾ってくる。"
            },
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            }
          ],
          "moves": [
            {
              "name": "エコーボイス",
              "type": "Normal",
              "category": "Special",
              "power": 40,
              "description": "響く声で相手を攻撃する。毎ターンだれかが技を使い続けると威力があがる。"
            },
            {
              "name": "ダブルアタック",
              "type": "Normal",
              "category": "Physical",
              "power": 35,
              "description": "しっぽなどを使い相手をたたいて攻撃する。２回連続でダメージを与える。"
            },
            {
              "name": "つぶらなひとみ",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "つぶらなひとみで相手をみつめて攻撃をさげる。必ず先制攻撃できる。"
            },
            {
              "name": "じゃれつく",
              "type": "Fairy",
              "category": "Physical",
              "power": 90,
              "description": "相手にじゃれついて攻撃する。相手の攻撃をさげることがある。"
            }
          ],
          "actions": [],
          "hp": 250,
          "atk": 25,
          "def": 23,
          "sp_atk": 21,
          "sp_def": 23,
          "speed": 35
        },
        {
          "name": "アオガラス",
          "star": 3,
          "types": [
            "Flying"
          ],
          "abilities": [
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            },
            {
              "name": "はとむね",
              "description": "防御を下げる効果を受けない。"
            }
          ],
          "moves": [
            {
              "name": "ドリルくちばし",
              "type": "Flying",
              "category": "Physical",
              "power": 80,
              "description": "回転しながらとがったくちばしを相手に突き刺して攻撃する。"
            },
            {
              "name": "つめとぎ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "ツメを磨いて鋭くする。自分の攻撃と命中率をあげる。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            },
            {
              "name": "からげんき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "自分が毒まひやけど状態のとき相手にくりだすと技の威力が２倍になる。"
            }
          ],
          "actions": [
            "時間75% つけあがる"
          ],
          "hp": 736,
          "atk": 51,
          "def": 43,
          "sp_atk": 35,
          "sp_def": 43,
          "speed": 58
        },
        {
          "name": "アママイコ",
          "star": 3,
          "types": [
            "Grass"
          ],
          "abilities": [
            {
              "name": "リーフガード",
              "description": "天気が晴れのときは状態異常にならない。"
            },
            {
              "name": "どんかん",
              "description": "鈍感なのでメロメロやちょうはつ状態にならない。いかくにも動じない。"
            },
            {
              "name": "スイートベール",
              "description": "自分と味方のポケモンは眠らなくなる。"
            }
          ],
          "moves": [
            {
              "name": "ふみつけ",
              "type": "Normal",
              "category": "Physical",
              "power": 65,
              "description": "大きな足で相手を踏みつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "フラフラダンス",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "フラフラとダンスをおどって自分の周りにいるものを混乱状態にさせる。"
            },
            {
              "name": "マジカルリーフ",
              "type": "Grass",
              "category": "Special",
              "power": 60,
              "description": "相手を追跡する不思議なはっぱをまきちらす。攻撃は必ず命中する。"
            },
            {
              "name": "ドレインキッス",
              "type": "Fairy",
              "category": "Special",
              "power": 50,
              "description": "キッスによって相手からＨＰを吸い取る。与えたダメージの半分以上ＨＰを回復する。"
            }
          ],
          "actions": [
            "時間90% グラスフィールド"
          ],
          "hp": 648,
          "atk": 33,
          "def": 38,
          "sp_atk": 33,
          "sp_def": 38,
          "speed": 48
        },
        {
          "name": "アルクジラ",
          "star": 3,
          "types": [
            "Ice"
          ],
          "abilities": [
            {
              "name": "あついしぼう",
              "description": "厚い脂肪で守られているのでほのおタイプとこおりタイプの技のダメージを半減させる。"
            },
            {
              "name": "ゆきがくれ",
              "description": "天気がゆきのとき回避率が上がる。"
            },
            {
              "name": "ちからずく",
              "description": "技の追加効果はなくなるがそのぶん高い威力で技を出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "ゆきなだれ",
              "type": "Ice",
              "category": "Physical",
              "power": 60,
              "description": "相手から技を受けているとその相手に対して技の威力が２倍になる。"
            },
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "じならし",
              "type": "Ground",
              "category": "Physical",
              "power": 60,
              "description": "地面を踏みならして自分の周りにいるものを攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "ゆきげしき",
              "type": "Ice",
              "category": "Status",
              "power": 0,
              "description": "５ターンの間ゆきを降らせる。こおりタイプの防御があがる。"
            }
          ],
          "actions": [
            "時間90% ゆきげしき"
          ],
          "hp": 960,
          "atk": 52,
          "def": 36,
          "sp_atk": 26,
          "sp_def": 33,
          "speed": 35
        },
        {
          "name": "イエッサン",
          "star": 3,
          "types": [
            "Psychic",
            "Normal"
          ],
          "abilities": [
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            },
            {
              "name": "シンクロ",
              "description": "自分がなってしまったどくやまひややけどを相手にうつす。"
            },
            {
              "name": "サイコメイカー",
              "description": "登場したときにサイコフィールドをはりめぐらせる。"
            }
          ],
          "moves": [
            {
              "name": "サイコキネシス",
              "type": "Psychic",
              "category": "Special",
              "power": 90,
              "description": "強い念力を相手に送って攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "アンコール",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手にアンコールした技を３回続けて出させる。"
            },
            {
              "name": "のしかかり",
              "type": "Normal",
              "category": "Physical",
              "power": 85,
              "description": "全身で相手にのしかかり攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            }
          ],
          "actions": [
            "体力75% サイコフィールド"
          ],
          "hp": 696,
          "atk": 50,
          "def": 43,
          "sp_atk": 78,
          "sp_def": 71,
          "speed": 71
        },
        {
          "name": "イエッサン",
          "star": 3,
          "types": [],
          "abilities": [
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            },
            {
              "name": "シンクロ",
              "description": "自分がなってしまったどくやまひややけどを相手にうつす。"
            },
            {
              "name": "サイコメイカー",
              "description": "登場したときにサイコフィールドをはりめぐらせる。"
            }
          ],
          "moves": [
            {
              "name": "サイコキネシス",
              "type": "Psychic",
              "category": "Special",
              "power": 90,
              "description": "強い念力を相手に送って攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "じこあんじ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "自分に暗示をかけることで能力変化の状態を相手と同じにする。"
            },
            {
              "name": "のしかかり",
              "type": "Normal",
              "category": "Physical",
              "power": 85,
              "description": "全身で相手にのしかかり攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            }
          ],
          "actions": [
            "体力75% サイコフィールド"
          ],
          "hp": 0,
          "atk": 0,
          "def": 0,
          "sp_atk": 0,
          "sp_def": 0,
          "speed": 0
        },
        {
          "name": "イーブイ",
          "star": 3,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "にげあし",
              "description": "野生のポケモンから必ず逃げられる。"
            },
            {
              "name": "てきおうりょく",
              "description": "自分とおなじタイプの技の威力が上がる。"
            },
            {
              "name": "きけんよち",
              "description": "相手の持つ危険な技を察知することができる。"
            }
          ],
          "moves": [
            {
              "name": "でんこうせっか",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "めいそう",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "静かに精神を統一し心を鎮めることで自分の特攻と特防をあげる。"
            },
            {
              "name": "スピードスター",
              "type": "Normal",
              "category": "Special",
              "power": 60,
              "description": "星型の光を発射して相手を攻撃する。攻撃は必ず命中する。"
            },
            {
              "name": "どろかけ",
              "type": "Ground",
              "category": "Special",
              "power": 20,
              "description": "相手の顔などに泥を投げつけて攻撃する。命中率をさげる。"
            }
          ],
          "actions": [
            "体力50% あまえる"
          ],
          "hp": 664,
          "atk": 43,
          "def": 40,
          "sp_atk": 36,
          "sp_def": 50,
          "speed": 43
        },
        {
          "name": "ウソッキー",
          "star": 3,
          "types": [
            "Rock"
          ],
          "abilities": [
            {
              "name": "がんじょう",
              "description": "ＨＰが満タンのとき技を受けても一撃で倒されることがない。一撃必殺技も効かない。"
            },
            {
              "name": "いしあたま",
              "description": "反動を受ける技を出してもＨＰが減らない。"
            },
            {
              "name": "びびり",
              "description": "あくゴーストむしタイプの攻撃を受けたりいかくをされるとびびって素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "ウッドハンマー",
              "type": "Grass",
              "category": "Physical",
              "power": 120,
              "description": "硬い胴体を相手にたたきつけて攻撃する。自分もかなりダメージを受ける。"
            },
            {
              "name": "がんせきふうじ",
              "type": "Rock",
              "category": "Physical",
              "power": 60,
              "description": "岩石を投げつけて攻撃する。相手の動きを封じることで素早さをさげる。"
            },
            {
              "name": "じたばた",
              "type": "Normal",
              "category": "Physical",
              "power": 1,
              "description": "じたばた暴れて攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            },
            {
              "name": "かたくなる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "全身に力をこめて体を硬くして自分の防御をあげる。"
            }
          ],
          "actions": [
            "時間50% 強化解除"
          ],
          "hp": 752,
          "atk": 75,
          "def": 85,
          "sp_atk": 26,
          "sp_def": 50,
          "speed": 26
        },
        {
          "name": "エクスレッグ",
          "star": 3,
          "types": [
            "Bug",
            "Dark"
          ],
          "abilities": [
            {
              "name": "むしのしらせ",
              "description": "ＨＰが減ったときむしタイプの技の威力が上がる。"
            },
            {
              "name": "いろめがね",
              "description": "効果がいまひとつの技を通常の威力で出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "じごくづき",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "この技を受けた相手は地獄の苦しみから２ターンの間音の技を出すことができなくなる。"
            },
            {
              "name": "とびかかる",
              "type": "Bug",
              "category": "Physical",
              "power": 80,
              "description": "全力で相手に飛びかかって攻撃。相手の攻撃をさげる。"
            },
            {
              "name": "つばめがえし",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "素早い動きで相手をほんろうして切りつける。攻撃は必ず命中する。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            }
          ],
          "actions": [
            "体力50% ローキック"
          ],
          "hp": 752,
          "atk": 76,
          "def": 59,
          "sp_atk": 41,
          "sp_def": 43,
          "speed": 69
        },
        {
          "name": "オドシシ",
          "star": 3,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "おみとおし",
              "description": "登場したとき相手の持ち物を見通すことができる。"
            },
            {
              "name": "そうしょく",
              "description": "くさタイプの技を受けるとダメージを受けずに攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ふみつけ",
              "type": "Normal",
              "category": "Physical",
              "power": 65,
              "description": "大きな足で相手を踏みつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "サイコショック",
              "type": "Psychic",
              "category": "Special",
              "power": 80,
              "description": "不思議な念波を実体化して相手を攻撃する。物理的なダメージを与える。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "にらみつける",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "鋭い目つきでおびえさせて相手の防御をさげる。"
            }
          ],
          "actions": [
            "体力75% リフレクター"
          ],
          "hp": 768,
          "atk": 71,
          "def": 48,
          "sp_atk": 64,
          "sp_def": 50,
          "speed": 64
        },
        {
          "name": "オリーニョ",
          "star": 3,
          "types": [
            "Grass",
            "Normal"
          ],
          "abilities": [
            {
              "name": "はやおき",
              "description": "ねむり状態になっても２倍の早さで目覚めることができる。"
            },
            {
              "name": "しゅうかく",
              "description": "使ったきのみを何回も作りだす。"
            }
          ],
          "moves": [
            {
              "name": "せいちょう",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体を一気に大きく生長させて攻撃と特攻をあげる。"
            },
            {
              "name": "タネばくだん",
              "type": "Grass",
              "category": "Physical",
              "power": 80,
              "description": "硬い殻をもつ大きなタネを上からたたきつけて相手を攻撃する。"
            },
            {
              "name": "メガドレイン",
              "type": "Grass",
              "category": "Special",
              "power": 40,
              "description": "養分を吸い取り攻撃する。相手に与えたダメージの半分のＨＰを回復できる。"
            },
            {
              "name": "からげんき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "自分が毒まひやけど状態のとき相手にくりだすと技の威力が２倍になる。"
            }
          ],
          "actions": [],
          "hp": 648,
          "atk": 42,
          "def": 47,
          "sp_atk": 59,
          "sp_def": 59,
          "speed": 28
        },
        {
          "name": "オンバット",
          "star": 3,
          "types": [
            "Flying",
            "Dragon"
          ],
          "abilities": [
            {
              "name": "おみとおし",
              "description": "登場したとき相手の持ち物を見通すことができる。"
            },
            {
              "name": "すりぬけ",
              "description": "相手の壁や身代わりをすりぬけて攻撃できる。"
            },
            {
              "name": "テレパシー",
              "description": "味方の攻撃を読み取って技を回避する。"
            }
          ],
          "moves": [
            {
              "name": "エアスラッシュ",
              "type": "Flying",
              "category": "Special",
              "power": 75,
              "description": "空をも切り裂く空気の刃で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ドラゴンダイブ",
              "type": "Dragon",
              "category": "Physical",
              "power": 100,
              "description": "すさまじい殺気で威圧しながら体当たりする。相手をひるませることがある。"
            },
            {
              "name": "いかりのまえば",
              "type": "Normal",
              "category": "Physical",
              "power": 1,
              "description": "鋭い前歯で激しくかみついて攻撃する。相手のＨＰは半分になる。"
            },
            {
              "name": "ハイパーボイス",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "うるさく響く大きな振動を相手に与えて攻撃する。"
            }
          ],
          "actions": [
            "体力50% おいかぜ"
          ],
          "hp": 584,
          "atk": 26,
          "def": 29,
          "sp_atk": 36,
          "sp_def": 33,
          "speed": 43
        },
        {
          "name": "カチコール",
          "star": 3,
          "types": [
            "Ice"
          ],
          "abilities": [
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "アイスボディ",
              "description": "天気がゆきのときＨＰを少しずつ回復する。"
            },
            {
              "name": "がんじょう",
              "description": "ＨＰが満タンのとき技を受けても一撃で倒されることがない。一撃必殺技も効かない。"
            }
          ],
          "moves": [
            {
              "name": "こおりのキバ",
              "type": "Ice",
              "category": "Physical",
              "power": 65,
              "description": "冷気をひめたキバでかみつく。相手をひるませたりこおり状態にすることがある。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "のろい",
              "type": "Ghost",
              "category": "Status",
              "power": 0,
              "description": "使うポケモンがゴーストタイプとそれ以外とでは効果が変わる。"
            }
          ],
          "actions": [
            "時間90% ゆきげしき"
          ],
          "hp": 664,
          "atk": 53,
          "def": 64,
          "sp_atk": 27,
          "sp_def": 29,
          "speed": 24
        },
        {
          "name": "カルボウ",
          "star": 3,
          "types": [
            "Fire"
          ],
          "abilities": [
            {
              "name": "もらいび",
              "description": "ほのおタイプの技を受けると炎をもらい自分が出すほのおタイプの技が強くなる。"
            },
            {
              "name": "ほのおのからだ",
              "description": "自分に触った相手をやけど状態にすることがある。"
            }
          ],
          "moves": [
            {
              "name": "ふんえん",
              "type": "Fire",
              "category": "Special",
              "power": 80,
              "description": "真っ赤な炎で自分の周りにいるものを攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "おにび",
              "type": "Fire",
              "category": "Status",
              "power": 0,
              "description": "不気味で怪しい炎を放って相手をやけどの状態にする。"
            },
            {
              "name": "ナイトヘッド",
              "type": "Ghost",
              "category": "Special",
              "power": 1,
              "description": "恐ろしい幻をみせて自分のレベルと同じだけのダメージを相手に与える。"
            },
            {
              "name": "クリアスモッグ",
              "type": "Poison",
              "category": "Special",
              "power": 50,
              "description": "特殊な泥の塊を相手に投げつけて攻撃する。能力変化をもとにもどす。"
            }
          ],
          "actions": [
            "体力50% 弱体解除"
          ],
          "hp": 584,
          "atk": 40,
          "def": 33,
          "sp_atk": 40,
          "sp_def": 33,
          "speed": 29
        },
        {
          "name": "キバゴ",
          "star": 3,
          "types": [
            "Dragon"
          ],
          "abilities": [
            {
              "name": "とうそうしん",
              "description": "性別が同じだと闘争心を燃やして強くなる。性別が違うと弱くなる。"
            },
            {
              "name": "かたやぶり",
              "description": "相手の特性にジャマされることなく相手に技を出すことができる。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            }
          ],
          "moves": [
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ドラゴンクロー",
              "type": "Dragon",
              "category": "Physical",
              "power": 80,
              "description": "鋭くとがった巨大なツメで相手を切り裂いて攻撃する。"
            },
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            }
          ],
          "actions": [
            "体力50% きあいだめ"
          ],
          "hp": 616,
          "atk": 65,
          "def": 47,
          "sp_atk": 26,
          "sp_def": 33,
          "speed": 44
        },
        {
          "name": "キマワリ",
          "star": 3,
          "types": [
            "Grass"
          ],
          "abilities": [
            {
              "name": "ようりょくそ",
              "description": "天気が晴れのとき素早さが上がる。"
            },
            {
              "name": "サンパワー",
              "description": "天気が晴れると特攻が上がるが毎ターンＨＰが減る。"
            },
            {
              "name": "はやおき",
              "description": "ねむり状態になっても２倍の早さで目覚めることができる。"
            }
          ],
          "moves": [
            {
              "name": "せいちょう",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体を一気に大きく生長させて攻撃と特攻をあげる。"
            },
            {
              "name": "ギガドレイン",
              "type": "Grass",
              "category": "Special",
              "power": 75,
              "description": "養分を吸い取り攻撃する。与えたダメージの半分のＨＰを回復できる。"
            },
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "だいちのちから",
              "type": "Ground",
              "category": "Special",
              "power": 90,
              "description": "相手の足下へ大地の力を放出する。相手の特防をさげることがある。"
            }
          ],
          "actions": [
            "時間90% にほんばれ"
          ],
          "hp": 776,
          "atk": 57,
          "def": 43,
          "sp_atk": 78,
          "sp_def": 64,
          "speed": 26
        },
        {
          "name": "キラーメ",
          "star": 3,
          "types": [
            "Rock",
            "Poison"
          ],
          "abilities": [
            {
              "name": "どくげしょう",
              "description": "物理技でダメージを受けると相手の足下にどくびしがちらばる。"
            },
            {
              "name": "ふしょく",
              "description": "はがねタイプやどくタイプもどく状態にすることができる。"
            }
          ],
          "moves": [
            {
              "name": "ベノムショック",
              "type": "Poison",
              "category": "Special",
              "power": 65,
              "description": "特殊な毒液を浴びせかける。毒状態の相手には威力が２倍になる。"
            },
            {
              "name": "パワージェム",
              "type": "Rock",
              "category": "Special",
              "power": 80,
              "description": "宝石のようにきらめく光を発射して相手を攻撃する。"
            },
            {
              "name": "どくどく",
              "type": "Poison",
              "category": "Status",
              "power": 0,
              "description": "相手を猛毒の状態にする。ターンがすすむほど毒のダメージが増えていく。"
            },
            {
              "name": "マッドショット",
              "type": "Ground",
              "category": "Special",
              "power": 55,
              "description": "泥の塊を相手に投げつけて攻撃する。同時に相手の素早さをさげる。"
            }
          ],
          "actions": [
            "時間75% どくどく"
          ],
          "hp": 624,
          "atk": 29,
          "def": 34,
          "sp_atk": 78,
          "sp_def": 47,
          "speed": 47
        },
        {
          "name": "キリンリキ",
          "star": 3,
          "types": [
            "Normal",
            "Psychic"
          ],
          "abilities": [
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            },
            {
              "name": "はやおき",
              "description": "ねむり状態になっても２倍の早さで目覚めることができる。"
            },
            {
              "name": "そうしょく",
              "description": "くさタイプの技を受けるとダメージを受けずに攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "サイケこうせん",
              "type": "Psychic",
              "category": "Special",
              "power": 65,
              "description": "不思議な光線を相手に発射して攻撃する。混乱させることがある。"
            },
            {
              "name": "サイコショック",
              "type": "Psychic",
              "category": "Special",
              "power": 80,
              "description": "不思議な念波を実体化して相手を攻撃する。物理的なダメージを与える。"
            },
            {
              "name": "ダメおし",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "そのターンに相手がすでにダメージを受けていたら技の威力は２倍になる。"
            }
          ],
          "actions": [
            "時間90% サイコフィールド"
          ],
          "hp": 752,
          "atk": 61,
          "def": 50,
          "sp_atk": 68,
          "sp_def": 50,
          "speed": 64
        },
        {
          "name": "キルリア",
          "star": 3,
          "types": [
            "Psychic",
            "Fairy"
          ],
          "abilities": [
            {
              "name": "シンクロ",
              "description": "自分がなってしまったどくやまひややけどを相手にうつす。"
            },
            {
              "name": "トレース",
              "description": "登場したとき相手の特性をトレースして同じ特性になる。"
            },
            {
              "name": "テレパシー",
              "description": "味方の攻撃を読み取って技を回避する。"
            }
          ],
          "moves": [
            {
              "name": "マジカルシャイン",
              "type": "Fairy",
              "category": "Special",
              "power": 80,
              "description": "強力な光を放ち相手にダメージを与える。"
            },
            {
              "name": "サイケこうせん",
              "type": "Psychic",
              "category": "Special",
              "power": 65,
              "description": "不思議な光線を相手に発射して攻撃する。混乱させることがある。"
            },
            {
              "name": "めいそう",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "静かに精神を統一し心を鎮めることで自分の特攻と特防をあげる。"
            },
            {
              "name": "あまえる",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。"
            }
          ],
          "actions": [
            "時間90% リフレクター"
          ],
          "hp": 568,
          "atk": 29,
          "def": 29,
          "sp_atk": 50,
          "sp_def": 43,
          "speed": 40
        },
        {
          "name": "ギモー",
          "star": 3,
          "types": [
            "Dark",
            "Fairy"
          ],
          "abilities": [
            {
              "name": "いたずらごころ",
              "description": "変化技を先制で出すことができる。"
            },
            {
              "name": "おみとおし",
              "description": "登場したとき相手の持ち物を見通すことができる。"
            },
            {
              "name": "わるいてぐせ",
              "description": "触られた相手の道具を盗んでしまう。"
            }
          ],
          "moves": [
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "あくのはどう",
              "type": "Dark",
              "category": "Special",
              "power": 80,
              "description": "体から悪意にみちた恐ろしいオーラを発する。相手をひるませることがある。"
            },
            {
              "name": "じゃれつく",
              "type": "Fairy",
              "category": "Physical",
              "power": 90,
              "description": "相手にじゃれついて攻撃する。相手の攻撃をさげることがある。"
            },
            {
              "name": "うそなき",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "ないたふりをして涙を流す。こまらせることで相手の特防をがくっとさげる。"
            }
          ],
          "actions": [
            "体力50% わるだくみ"
          ],
          "hp": 720,
          "atk": 47,
          "def": 36,
          "sp_atk": 57,
          "sp_def": 43,
          "speed": 54
        },
        {
          "name": "クマシュン",
          "star": 3,
          "types": [
            "Ice"
          ],
          "abilities": [
            {
              "name": "ゆきがくれ",
              "description": "天気がゆきのとき回避率が上がる。"
            },
            {
              "name": "ゆきかき",
              "description": "天気がゆきのとき素早さが上がる。"
            },
            {
              "name": "びびり",
              "description": "あくゴーストむしタイプの攻撃を受けたりいかくをされるとびびって素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "れいとうパンチ",
              "type": "Ice",
              "category": "Physical",
              "power": 75,
              "description": "冷気をこめたパンチで相手を攻撃する。こおり状態にすることがある。"
            },
            {
              "name": "つじぎり",
              "type": "Dark",
              "category": "Physical",
              "power": 70,
              "description": "一瞬のすきをついて相手を切りはらう。急所に当たりやすい。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            },
            {
              "name": "けたぐり",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "足を強くけり相手を転ばせて攻撃する。相手が重いほど威力があがる。"
            }
          ],
          "actions": [
            "時間90% ゆきげしき"
          ],
          "hp": 664,
          "atk": 54,
          "def": 33,
          "sp_atk": 47,
          "sp_def": 33,
          "speed": 33
        },
        {
          "name": "コイキング",
          "star": 3,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "すいすい",
              "description": "天気が雨のとき素早さが上がる。"
            },
            {
              "name": "びびり",
              "description": "あくゴーストむしタイプの攻撃を受けたりいかくをされるとびびって素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "たいあたり",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "相手にむかって全身でぶつかっていき攻撃する。"
            },
            {
              "name": "はねる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "攻撃もせずにピョンピョンと跳ねるだけでなにもおこらない……。"
            },
            {
              "name": "じたばた",
              "type": "Normal",
              "category": "Physical",
              "power": 1,
              "description": "じたばた暴れて攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            }
          ],
          "actions": [
            "時間50% 強化解除"
          ],
          "hp": 472,
          "atk": 12,
          "def": 43,
          "sp_atk": 15,
          "sp_def": 19,
          "speed": 61
        },
        {
          "name": "コロトック",
          "star": 3,
          "types": [
            "Bug"
          ],
          "abilities": [
            {
              "name": "むしのしらせ",
              "description": "ＨＰが減ったときむしタイプの技の威力が上がる。"
            },
            {
              "name": "テクニシャン",
              "description": "威力が低い技の威力を高くして攻撃できる。"
            }
          ],
          "moves": [
            {
              "name": "つばめがえし",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "素早い動きで相手をほんろうして切りつける。攻撃は必ず命中する。"
            },
            {
              "name": "きりさく",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "れんぞくぎり",
              "type": "Bug",
              "category": "Physical",
              "power": 40,
              "description": "カマやツメなどで相手を切りつけて攻撃する。連続で当てると威力があがる。"
            },
            {
              "name": "シザークロス",
              "type": "Bug",
              "category": "Physical",
              "power": 80,
              "description": "カマやツメをハサミのように交差させながら相手を切り裂く。"
            }
          ],
          "actions": [
            "体力50% 強化解除"
          ],
          "hp": 784,
          "atk": 64,
          "def": 40,
          "sp_atk": 43,
          "sp_def": 40,
          "speed": 50
        },
        {
          "name": "ゴチミル",
          "star": 3,
          "types": [
            "Psychic"
          ],
          "abilities": [
            {
              "name": "おみとおし",
              "description": "登場したとき相手の持ち物を見通すことができる。"
            },
            {
              "name": "かちき",
              "description": "相手に能力を下げられると特攻がぐーんと上がる。"
            },
            {
              "name": "かげふみ",
              "description": "相手の影を踏み逃げたり交代できなくする。"
            }
          ],
          "moves": [
            {
              "name": "サイコショック",
              "type": "Psychic",
              "category": "Special",
              "power": 80,
              "description": "不思議な念波を実体化して相手を攻撃する。物理的なダメージを与える。"
            },
            {
              "name": "さいみんじゅつ",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "眠気を誘う暗示をかけて相手を眠り状態にする。"
            },
            {
              "name": "うそなき",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "ないたふりをして涙を流す。こまらせることで相手の特防をがくっとさげる。"
            },
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            }
          ],
          "actions": [
            "時間90% リフレクター"
          ],
          "hp": 696,
          "atk": 36,
          "def": 54,
          "sp_atk": 57,
          "sp_def": 64,
          "speed": 43
        },
        {
          "name": "ゴースト",
          "star": 3,
          "types": [
            "Ghost",
            "Poison"
          ],
          "abilities": [
            {
              "name": "ふゆう",
              "description": "地面から浮くことによってじめんタイプの技を受けない。"
            }
          ],
          "moves": [
            {
              "name": "ナイトヘッド",
              "type": "Ghost",
              "category": "Special",
              "power": 1,
              "description": "恐ろしい幻をみせて自分のレベルと同じだけのダメージを相手に与える。"
            },
            {
              "name": "たたりめ",
              "type": "Ghost",
              "category": "Special",
              "power": 65,
              "description": "たたみかけるように攻撃する。状態異常の相手に大きなダメージを与える。"
            },
            {
              "name": "ふいうち",
              "type": "Dark",
              "category": "Physical",
              "power": 70,
              "description": "相手より先に攻撃できる。相手がだす技が攻撃技でないと失敗する。"
            },
            {
              "name": "クリアスモッグ",
              "type": "Poison",
              "category": "Special",
              "power": 50,
              "description": "特殊な泥の塊を相手に投げつけて攻撃する。能力変化をもとにもどす。"
            }
          ],
          "actions": [
            "体力50% おにび"
          ],
          "hp": 608,
          "atk": 40,
          "def": 36,
          "sp_atk": 85,
          "sp_def": 43,
          "speed": 71
        },
        {
          "name": "ザングース",
          "star": 3,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "めんえき",
              "description": "体内に免疫を持っているためどく状態にならない。"
            },
            {
              "name": "どくぼうそう",
              "description": "どく状態になったとき物理技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ブレイククロー",
              "type": "Normal",
              "category": "Physical",
              "power": 75,
              "description": "硬く鋭いツメで切り裂いて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "にらみつける",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "鋭い目つきでおびえさせて相手の防御をさげる。"
            },
            {
              "name": "つじぎり",
              "type": "Dark",
              "category": "Physical",
              "power": 70,
              "description": "一瞬のすきをついて相手を切りはらう。急所に当たりやすい。"
            },
            {
              "name": "でんこうせっか",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            }
          ],
          "actions": [
            "体力50% にらみつける"
          ],
          "hp": 768,
          "atk": 85,
          "def": 47,
          "sp_atk": 47,
          "sp_def": 47,
          "speed": 68
        },
        {
          "name": "ジオヅム",
          "star": 3,
          "types": [
            "Rock"
          ],
          "abilities": [
            {
              "name": "きよめのしお",
              "description": "清らかな塩で状態異常にならない。ゴーストタイプの技のダメージを半減させる。"
            },
            {
              "name": "がんじょう",
              "description": "ＨＰが満タンのとき技を受けても一撃で倒されることがない。一撃必殺技も効かない。"
            },
            {
              "name": "クリアボディ",
              "description": "相手の技や特性で能力を下げられない。"
            }
          ],
          "moves": [
            {
              "name": "しおづけ",
              "type": "Rock",
              "category": "Physical",
              "power": 40,
              "description": "相手をしおづけ状態にして毎ターンダメージを与える。はがねみずタイプはより苦しむ。"
            },
            {
              "name": "いわなだれ",
              "type": "Rock",
              "category": "Physical",
              "power": 75,
              "description": "大きな岩を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            },
            {
              "name": "てっぺき",
              "type": "Steel",
              "category": "Status",
              "power": 0,
              "description": "皮膚を鉄のように硬くすることで自分の防御をぐーんとあげる。"
            }
          ],
          "actions": [
            "体力50% 強化解除"
          ],
          "hp": 696,
          "atk": 47,
          "def": 75,
          "sp_atk": 29,
          "sp_def": 50,
          "speed": 29
        },
        {
          "name": "セビエ",
          "star": 3,
          "types": [
            "Dragon",
            "Ice"
          ],
          "abilities": [
            {
              "name": "ねつこうかん",
              "description": "ほのおタイプの技を受けると攻撃が上がる。やけど状態にならない。"
            },
            {
              "name": "アイスボディ",
              "description": "天気がゆきのときＨＰを少しずつ回復する。"
            }
          ],
          "moves": [
            {
              "name": "こおりのキバ",
              "type": "Ice",
              "category": "Physical",
              "power": 65,
              "description": "冷気をひめたキバでかみつく。相手をひるませたりこおり状態にすることがある。"
            },
            {
              "name": "ドラゴンクロー",
              "type": "Dragon",
              "category": "Physical",
              "power": 80,
              "description": "鋭くとがった巨大なツメで相手を切り裂いて攻撃する。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "にらみつける",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "鋭い目つきでおびえさせて相手の防御をさげる。"
            }
          ],
          "actions": [
            "時間90% ゆきげしき"
          ],
          "hp": 720,
          "atk": 57,
          "def": 36,
          "sp_atk": 29,
          "sp_def": 36,
          "speed": 43
        },
        {
          "name": "ゾロア",
          "star": 3,
          "types": [
            "Dark"
          ],
          "abilities": [
            {
              "name": "イリュージョン",
              "description": "手持ちのいちばんうしろにいるポケモンになりきって登場して相手を化かす。"
            }
          ],
          "moves": [
            {
              "name": "バークアウト",
              "type": "Dark",
              "category": "Special",
              "power": 55,
              "description": "まくしたてるように怒鳴りつけて相手の特攻をさげる。"
            },
            {
              "name": "はたきおとす",
              "type": "Dark",
              "category": "Physical",
              "power": 65,
              "description": "相手の持ち物をはたき落として戦闘が終わるまで使えなくする。物を持つ相手にはダメージが増す。"
            },
            {
              "name": "みだれひっかき",
              "type": "Normal",
              "category": "Physical",
              "power": 18,
              "description": "ツメやカマなどで相手をひっかいて攻撃する。２ー５回の間連続でだす。"
            },
            {
              "name": "じんつうりき",
              "type": "Psychic",
              "category": "Special",
              "power": 80,
              "description": "みえない不思議な力を送って攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [
            "体力50% わるだくみ"
          ],
          "hp": 584,
          "atk": 50,
          "def": 33,
          "sp_atk": 61,
          "sp_def": 33,
          "speed": 50
        },
        {
          "name": "タツベイ",
          "star": 3,
          "types": [
            "Dragon"
          ],
          "abilities": [
            {
              "name": "いしあたま",
              "description": "反動を受ける技を出してもＨＰが減らない。"
            },
            {
              "name": "ちからずく",
              "description": "技の追加効果はなくなるがそのぶん高い威力で技を出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "ドラゴンクロー",
              "type": "Dragon",
              "category": "Physical",
              "power": 80,
              "description": "鋭くとがった巨大なツメで相手を切り裂いて攻撃する。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "きあいだめ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "深く息を吸い気合をこめる。自分の攻撃が急所に当たりやすくなる。"
            }
          ],
          "actions": [
            "体力75% りゅうのまい"
          ],
          "hp": 608,
          "atk": 57,
          "def": 47,
          "sp_atk": 33,
          "sp_def": 26,
          "speed": 40
        },
        {
          "name": "テブリム",
          "star": 3,
          "types": [
            "Psychic"
          ],
          "abilities": [
            {
              "name": "いやしのこころ",
              "description": "状態異常の味方をたまに治してあげる。"
            },
            {
              "name": "きけんよち",
              "description": "相手の持つ危険な技を察知することができる。"
            },
            {
              "name": "マジックミラー",
              "description": "相手にだされた変化技を受けずにそのまま返すことができる。"
            }
          ],
          "moves": [
            {
              "name": "マジカルシャイン",
              "type": "Fairy",
              "category": "Special",
              "power": 80,
              "description": "強力な光を放ち相手にダメージを与える。"
            },
            {
              "name": "ぶんまわす",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "自分の体をぶんまわして相手にダメージを与える。"
            },
            {
              "name": "サイケこうせん",
              "type": "Psychic",
              "category": "Special",
              "power": 65,
              "description": "不思議な光線を相手に発射して攻撃する。混乱させることがある。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            }
          ],
          "actions": [
            "体力50% トリックルーム"
          ],
          "hp": 672,
          "atk": 33,
          "def": 50,
          "sp_atk": 65,
          "sp_def": 56,
          "speed": 39
        },
        {
          "name": "デカグース",
          "star": 3,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "はりこみ",
              "description": "交代で出てきた相手に２倍のダメージで攻撃できる。"
            },
            {
              "name": "がんじょうあご",
              "description": "あごが頑丈で噛む技の威力が高くなる。"
            },
            {
              "name": "てきおうりょく",
              "description": "自分とおなじタイプの技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "どろかけ",
              "type": "Ground",
              "category": "Special",
              "power": 20,
              "description": "相手の顔などに泥を投げつけて攻撃する。命中率をさげる。"
            },
            {
              "name": "くさわけ",
              "type": "Grass",
              "category": "Physical",
              "power": 50,
              "description": "草むらから飛びだすように攻撃する。軽快な足どりによって自分の素早さをあげる。"
            },
            {
              "name": "ほのおのキバ",
              "type": "Fire",
              "category": "Physical",
              "power": 65,
              "description": "炎をまとったキバでかみつく。相手をひるませたりやけど状態にすることがある。"
            }
          ],
          "actions": [
            "体力50% 強化解除"
          ],
          "hp": 848,
          "atk": 82,
          "def": 47,
          "sp_atk": 43,
          "sp_def": 47,
          "speed": 36
        },
        {
          "name": "デデンネ",
          "star": 3,
          "types": [
            "Electric",
            "Fairy"
          ],
          "abilities": [
            {
              "name": "ほおぶくろ",
              "description": "どんなきのみでも食べるとＨＰも回復する。"
            },
            {
              "name": "ものひろい",
              "description": "相手の使った道具を拾ってくることがある。冒険中も拾ってくる。"
            },
            {
              "name": "プラス",
              "description": "プラスかマイナスの特性を持つポケモンが仲間にいると自分の特攻が上がる。"
            }
          ],
          "moves": [
            {
              "name": "マジカルシャイン",
              "type": "Fairy",
              "category": "Special",
              "power": 80,
              "description": "強力な光を放ち相手にダメージを与える。"
            },
            {
              "name": "ほうでん",
              "type": "Electric",
              "category": "Special",
              "power": 80,
              "description": "まばゆい電撃で自分の周りにいるものを攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "ほっぺすりすり",
              "type": "Electric",
              "category": "Physical",
              "power": 20,
              "description": "電気を帯びたほっぺをすりつけて攻撃。相手をまひ状態にする。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            }
          ],
          "actions": [
            "体力50% エレキフィールド"
          ],
          "hp": 728,
          "atk": 45,
          "def": 44,
          "sp_atk": 61,
          "sp_def": 51,
          "speed": 75
        },
        {
          "name": "トロッゴン",
          "star": 3,
          "types": [
            "Rock",
            "Fire"
          ],
          "abilities": [
            {
              "name": "じょうききかん",
              "description": "みずタイプほのおタイプの技を受けると素早さがぐぐーんと上がる。"
            },
            {
              "name": "ほのおのからだ",
              "description": "自分に触った相手をやけど状態にすることがある。"
            },
            {
              "name": "もらいび",
              "description": "ほのおタイプの技を受けると炎をもらい自分が出すほのおタイプの技が強くなる。"
            }
          ],
          "moves": [
            {
              "name": "がんせきふうじ",
              "type": "Rock",
              "category": "Physical",
              "power": 60,
              "description": "岩石を投げつけて攻撃する。相手の動きを封じることで素早さをさげる。"
            },
            {
              "name": "やきつくす",
              "type": "Fire",
              "category": "Special",
              "power": 60,
              "description": "炎で相手を攻撃する。相手がきのみなどを持っているとき燃やして使えなくする。"
            },
            {
              "name": "ニトロチャージ",
              "type": "Fire",
              "category": "Physical",
              "power": 50,
              "description": "炎をまとい相手を攻撃する。力をためて自分の素早さをあげる。"
            },
            {
              "name": "いわなだれ",
              "type": "Rock",
              "category": "Physical",
              "power": 75,
              "description": "大きな岩を激しくぶつけて攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [
            "体力50% 強化解除"
          ],
          "hp": 808,
          "atk": 47,
          "def": 68,
          "sp_atk": 47,
          "sp_def": 54,
          "speed": 40
        },
        {
          "name": "ドラメシヤ",
          "star": 3,
          "types": [
            "Dragon",
            "Ghost"
          ],
          "abilities": [
            {
              "name": "クリアボディ",
              "description": "相手の技や特性で能力を下げられない。"
            },
            {
              "name": "すりぬけ",
              "description": "相手の壁や身代わりをすりぬけて攻撃できる。"
            },
            {
              "name": "のろわれボディ",
              "description": "攻撃を受けると相手の技をかなしばり状態にすることがある。"
            }
          ],
          "moves": [
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "でんこうせっか",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "おどろかす",
              "type": "Ghost",
              "category": "Physical",
              "power": 30,
              "description": "大きな声などで不意に驚かして攻撃する。相手をひるませることがある。"
            },
            {
              "name": "まとわりつく",
              "type": "Bug",
              "category": "Special",
              "power": 20,
              "description": "４ー５ターンの間相手にまとわりついて攻撃する。そのあいだ相手は逃げられない。"
            }
          ],
          "actions": [
            "時間90% リフレクター"
          ],
          "hp": 512,
          "atk": 47,
          "def": 26,
          "sp_atk": 33,
          "sp_def": 26,
          "speed": 62
        },
        {
          "name": "ドンメル",
          "star": 3,
          "types": [
            "Fire",
            "Ground"
          ],
          "abilities": [
            {
              "name": "どんかん",
              "description": "鈍感なのでメロメロやちょうはつ状態にならない。いかくにも動じない。"
            },
            {
              "name": "たんじゅん",
              "description": "能力変化がいつもの２倍になる。"
            },
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            }
          ],
          "moves": [
            {
              "name": "じならし",
              "type": "Ground",
              "category": "Physical",
              "power": 60,
              "description": "地面を踏みならして自分の周りにいるものを攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "ふんえん",
              "type": "Fire",
              "category": "Special",
              "power": 80,
              "description": "真っ赤な炎で自分の周りにいるものを攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            },
            {
              "name": "きあいだめ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "深く息を吸い気合をこめる。自分の攻撃が急所に当たりやすくなる。"
            }
          ],
          "actions": [
            "体力75% にほんばれ"
          ],
          "hp": 696,
          "atk": 47,
          "def": 33,
          "sp_atk": 50,
          "sp_def": 36,
          "speed": 29
        },
        {
          "name": "ドーミラー",
          "star": 3,
          "types": [
            "Steel",
            "Psychic"
          ],
          "abilities": [
            {
              "name": "ふゆう",
              "description": "地面から浮くことによってじめんタイプの技を受けない。"
            },
            {
              "name": "たいねつ",
              "description": "耐熱の体によってほのおタイプの技のダメージを半減させる。"
            },
            {
              "name": "ヘヴィメタル",
              "description": "自分の重さが２倍になる。"
            }
          ],
          "moves": [
            {
              "name": "ジャイロボール",
              "type": "Steel",
              "category": "Physical",
              "power": 1,
              "description": "体を高速に回転させて体当たりする。相手より素早さが低いほど強い。"
            },
            {
              "name": "さいみんじゅつ",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "眠気を誘う暗示をかけて相手を眠り状態にする。"
            },
            {
              "name": "ヘビーボンバー",
              "type": "Steel",
              "category": "Physical",
              "power": 1,
              "description": "重たい体で相手にぶつかって攻撃する。自分が相手より重いほど威力があがる。"
            },
            {
              "name": "サイコキネシス",
              "type": "Psychic",
              "category": "Special",
              "power": 90,
              "description": "強い念力を相手に送って攻撃する。相手の特防をさげることがある。"
            }
          ],
          "actions": [
            "体力50% トリックルーム"
          ],
          "hp": 672,
          "atk": 21,
          "def": 65,
          "sp_atk": 21,
          "sp_def": 65,
          "speed": 21
        },
        {
          "name": "ナカヌチャン",
          "star": 3,
          "types": [
            "Fairy",
            "Steel"
          ],
          "abilities": [
            {
              "name": "かたやぶり",
              "description": "相手の特性にジャマされることなく相手に技を出すことができる。"
            },
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "わるいてぐせ",
              "description": "触られた相手の道具を盗んでしまう。"
            }
          ],
          "moves": [
            {
              "name": "たたきつける",
              "type": "Normal",
              "category": "Physical",
              "power": 80,
              "description": "長いしっぽやつるなどを使い相手をたたきつけて攻撃する。"
            },
            {
              "name": "じゃれつく",
              "type": "Fairy",
              "category": "Physical",
              "power": 90,
              "description": "相手にじゃれついて攻撃する。相手の攻撃をさげることがある。"
            },
            {
              "name": "ラスターカノン",
              "type": "Steel",
              "category": "Special",
              "power": 80,
              "description": "体の光を一点に集めて力を放つ。相手の特防をさげることがある。"
            },
            {
              "name": "ぶんまわす",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "自分の体をぶんまわして相手にダメージを与える。"
            }
          ],
          "actions": [
            "体力50% 弱体解除"
          ],
          "hp": 720,
          "atk": 43,
          "def": 43,
          "sp_atk": 36,
          "sp_def": 62,
          "speed": 59
        },
        {
          "name": "ナゲツケサル",
          "star": 3,
          "types": [
            "Fighting"
          ],
          "abilities": [
            {
              "name": "レシーバー",
              "description": "倒された味方の特性を受け継いで同じ特性になる。"
            },
            {
              "name": "まけんき",
              "description": "相手に能力を下げられると攻撃がぐーんと上がる。"
            }
          ],
          "moves": [
            {
              "name": "かわらわり",
              "type": "Fighting",
              "category": "Physical",
              "power": 75,
              "description": "手刀を勢いよく振りおろして相手を攻撃する。ひかりのかべやリフレクターなども破壊できる。"
            },
            {
              "name": "こわいかお",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "恐ろしい顔でにらみおびえさせて相手の素早さをがくっとさげる。"
            },
            {
              "name": "いわくだき",
              "type": "Fighting",
              "category": "Physical",
              "power": 40,
              "description": "パンチで攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            }
          ],
          "actions": [
            "体力50% ビルドアップ"
          ],
          "hp": 920,
          "atk": 89,
          "def": 68,
          "sp_atk": 33,
          "sp_def": 47,
          "speed": 61
        },
        {
          "name": "ナミイルカ",
          "star": 3,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "みずのベール",
              "description": "水のベールを身にまといやけど状態にならない。"
            }
          ],
          "moves": [
            {
              "name": "ダブルアタック",
              "type": "Normal",
              "category": "Physical",
              "power": 35,
              "description": "しっぽなどを使い相手をたたいて攻撃する。２回連続でダメージを与える。"
            },
            {
              "name": "アクアテール",
              "type": "Water",
              "category": "Physical",
              "power": 90,
              "description": "激しくあれくるう荒波のように大きなしっぽをふって相手を攻撃する。"
            },
            {
              "name": "ちょうおんぱ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "特殊な音波を体から発して相手を混乱させる。"
            },
            {
              "name": "くろいきり",
              "type": "Ice",
              "category": "Status",
              "power": 0,
              "description": "黒い霧をだして戦闘にでているポケモン全員の能力変化をもとにもどす。"
            }
          ],
          "actions": [
            "体力50% あまえる"
          ],
          "hp": 752,
          "atk": 36,
          "def": 33,
          "sp_atk": 36,
          "sp_def": 33,
          "speed": 57
        },
        {
          "name": "ニューラ",
          "star": 3,
          "types": [
            "Dark",
            "Ice"
          ],
          "abilities": [
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            },
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            },
            {
              "name": "わるいてぐせ",
              "description": "触られた相手の道具を盗んでしまう。"
            }
          ],
          "moves": [
            {
              "name": "つららおとし",
              "type": "Ice",
              "category": "Physical",
              "power": 85,
              "description": "大きな氷柱を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "こおりのつぶて",
              "type": "Ice",
              "category": "Physical",
              "power": 40,
              "description": "氷の塊を一瞬でつくり相手に素早く放つ。必ず先制攻撃できる。"
            },
            {
              "name": "きりさく",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "にらみつける",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "鋭い目つきでおびえさせて相手の防御をさげる。"
            }
          ],
          "actions": [
            "時間50% 強化解除"
          ],
          "hp": 664,
          "atk": 71,
          "def": 43,
          "sp_atk": 29,
          "sp_def": 57,
          "speed": 85
        },
        {
          "name": "ヌメラ",
          "star": 3,
          "types": [
            "Dragon"
          ],
          "abilities": [
            {
              "name": "そうしょく",
              "description": "くさタイプの技を受けるとダメージを受けずに攻撃が上がる。"
            },
            {
              "name": "うるおいボディ",
              "description": "天気が雨のとき状態異常が治る。"
            },
            {
              "name": "ぬめぬめ",
              "description": "攻撃で自分に触れた相手の素早さを下げる。"
            }
          ],
          "moves": [
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "ヘドロばくだん",
              "type": "Poison",
              "category": "Special",
              "power": 90,
              "description": "汚いヘドロを相手に投げつけて攻撃する。毒状態にすることがある。"
            },
            {
              "name": "あまえる",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。"
            },
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            }
          ],
          "actions": [
            "時間50% 強化解除"
          ],
          "hp": 608,
          "atk": 40,
          "def": 29,
          "sp_atk": 43,
          "sp_def": 57,
          "speed": 33
        },
        {
          "name": "ノコッチ",
          "star": 3,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "てんのめぐみ",
              "description": "天の恵みのおかげで技の追加効果がでやすい。"
            },
            {
              "name": "にげあし",
              "description": "野生のポケモンから必ず逃げられる。"
            },
            {
              "name": "びびり",
              "description": "あくゴーストむしタイプの攻撃を受けたりいかくをされるとびびって素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "のしかかり",
              "type": "Normal",
              "category": "Physical",
              "power": 85,
              "description": "全身で相手にのしかかり攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "ドリルライナー",
              "type": "Ground",
              "category": "Physical",
              "power": 80,
              "description": "ドリルのように体を回転しながら相手に体当たりする。急所に当たりやすい。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            },
            {
              "name": "どろかけ",
              "type": "Ground",
              "category": "Special",
              "power": 20,
              "description": "相手の顔などに泥を投げつけて攻撃する。命中率をさげる。"
            }
          ],
          "actions": [
            "体力50% 弱体解除"
          ],
          "hp": 920,
          "atk": 54,
          "def": 54,
          "sp_atk": 50,
          "sp_def": 50,
          "speed": 36
        },
        {
          "name": "ハギギシリ",
          "star": 3,
          "types": [
            "Water",
            "Psychic"
          ],
          "abilities": [
            {
              "name": "ビビッドボディ",
              "description": "相手をびっくりさせてこちらにむかって先制技を出せないようにする。"
            },
            {
              "name": "がんじょうあご",
              "description": "あごが頑丈で噛む技の威力が高くなる。"
            },
            {
              "name": "ミラクルスキン",
              "description": "変化技を受けにくい体になっている。"
            }
          ],
          "moves": [
            {
              "name": "サイコファング",
              "type": "Psychic",
              "category": "Physical",
              "power": 85,
              "description": "サイコパワーでかみついて相手を攻撃する。ひかりのかべやリフレクターなども破壊できる。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "アクアジェット",
              "type": "Water",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "いやなおと",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "おもわず耳をふさぎたくなるいやなおとをだして相手の防御をがくっとさげる。"
            }
          ],
          "actions": [
            "体力75% あまごい"
          ],
          "hp": 736,
          "atk": 78,
          "def": 54,
          "sp_atk": 54,
          "sp_def": 54,
          "speed": 69
        },
        {
          "name": "ハブネーク",
          "star": 3,
          "types": [
            "Poison"
          ],
          "abilities": [
            {
              "name": "だっぴ",
              "description": "体の皮を脱ぎ捨てることで状態異常を治すことがある。"
            },
            {
              "name": "すりぬけ",
              "description": "相手の壁や身代わりをすりぬけて攻撃できる。"
            }
          ],
          "moves": [
            {
              "name": "ポイズンテール",
              "type": "Poison",
              "category": "Physical",
              "power": 50,
              "description": "しっぽでたたく。毒状態にすることがあり急所にも当たりやすい。"
            },
            {
              "name": "いやなおと",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "おもわず耳をふさぎたくなるいやなおとをだして相手の防御をがくっとさげる。"
            },
            {
              "name": "つじぎり",
              "type": "Dark",
              "category": "Physical",
              "power": 70,
              "description": "一瞬のすきをついて相手を切りはらう。急所に当たりやすい。"
            },
            {
              "name": "まきつく",
              "type": "Normal",
              "category": "Physical",
              "power": 15,
              "description": "長い体やつるなどを使って４ー５ターンの間相手にまきついて攻撃する。"
            }
          ],
          "actions": [
            "体力50% にらみつける"
          ],
          "hp": 768,
          "atk": 75,
          "def": 47,
          "sp_atk": 75,
          "sp_def": 47,
          "speed": 50
        },
        {
          "name": "ハリーセン",
          "star": 3,
          "types": [
            "Water",
            "Poison"
          ],
          "abilities": [
            {
              "name": "どくのトゲ",
              "description": "自分に触った相手をどく状態にすることがある。"
            },
            {
              "name": "すいすい",
              "description": "天気が雨のとき素早さが上がる。"
            },
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            }
          ],
          "moves": [
            {
              "name": "たきのぼり",
              "type": "Water",
              "category": "Physical",
              "power": 80,
              "description": "すごい勢いで相手につっこむ。相手をひるませることがある。"
            },
            {
              "name": "どくづき",
              "type": "Poison",
              "category": "Physical",
              "power": 80,
              "description": "毒にそまった触手や腕で相手を突き刺す。毒状態にすることがある。"
            },
            {
              "name": "ミサイルばり",
              "type": "Bug",
              "category": "Physical",
              "power": 25,
              "description": "鋭いハリを相手に発射して攻撃する。２ー５回の間連続でだす。"
            },
            {
              "name": "ちいさくなる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体をちぢめて小さくみせて自分の回避率をぐーんとあげる。"
            }
          ],
          "actions": [
            "時間50% 強化解除"
          ],
          "hp": 720,
          "atk": 71,
          "def": 64,
          "sp_atk": 43,
          "sp_def": 43,
          "speed": 64
        },
        {
          "name": "バスラオ",
          "star": 3,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "すてみ",
              "description": "反動でダメージを受ける技の威力が上がる。"
            },
            {
              "name": "てきおうりょく",
              "description": "自分とおなじタイプの技の威力が上がる。"
            },
            {
              "name": "かたやぶり",
              "description": "相手の特性にジャマされることなく相手に技を出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "アクアジェット",
              "type": "Water",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            },
            {
              "name": "じたばた",
              "type": "Normal",
              "category": "Physical",
              "power": 1,
              "description": "じたばた暴れて攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            }
          ],
          "actions": [
            "時間50% あまごい"
          ],
          "hp": 752,
          "atk": 69,
          "def": 50,
          "sp_atk": 61,
          "sp_def": 43,
          "speed": 73
        },
        {
          "name": "バチンウニ",
          "star": 3,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "ひらいしん",
              "description": "でんきタイプの技を自分に寄せつけダメージを受けずに特攻が上がる。"
            },
            {
              "name": "エレキメイカー",
              "description": "登場したときにエレキフィールドをはりめぐらせる。"
            }
          ],
          "moves": [
            {
              "name": "スパーク",
              "type": "Electric",
              "category": "Physical",
              "power": 65,
              "description": "電気をまとい相手に突進して攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "バブルこうせん",
              "type": "Water",
              "category": "Special",
              "power": 65,
              "description": "泡を勢いよく相手に発射して攻撃する。素早さをさげることがある。"
            },
            {
              "name": "どくづき",
              "type": "Poison",
              "category": "Physical",
              "power": 80,
              "description": "毒にそまった触手や腕で相手を突き刺す。毒状態にすることがある。"
            },
            {
              "name": "のろい",
              "type": "Ghost",
              "category": "Status",
              "power": 0,
              "description": "使うポケモンがゴーストタイプとそれ以外とでは効果が変わる。"
            }
          ],
          "actions": [
            "体力50% 強化解除"
          ],
          "hp": 624,
          "atk": 75,
          "def": 71,
          "sp_atk": 68,
          "sp_def": 64,
          "speed": 15
        },
        {
          "name": "パチリス",
          "star": 3,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "にげあし",
              "description": "野生のポケモンから必ず逃げられる。"
            },
            {
              "name": "ものひろい",
              "description": "相手の使った道具を拾ってくることがある。冒険中も拾ってくる。"
            },
            {
              "name": "ちくでん",
              "description": "でんきタイプの技を受けるとダメージを受けずに回復する。"
            }
          ],
          "moves": [
            {
              "name": "スパーク",
              "type": "Electric",
              "category": "Physical",
              "power": 65,
              "description": "電気をまとい相手に突進して攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "でんこうせっか",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "てんしのキッス",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "天使のようにかわいくキスして相手を混乱させる。"
            },
            {
              "name": "タネばくだん",
              "type": "Grass",
              "category": "Physical",
              "power": 80,
              "description": "硬い殻をもつ大きなタネを上からたたきつけて相手を攻撃する。"
            }
          ],
          "actions": [
            "体力75% エレキフィールド"
          ],
          "hp": 696,
          "atk": 36,
          "def": 54,
          "sp_atk": 36,
          "sp_def": 68,
          "speed": 71
        },
        {
          "name": "パフュートン",
          "star": 3,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "とれないにおい",
              "description": "相手に触られるととれないにおいが相手にうつってしまう。"
            },
            {
              "name": "くいしんぼう",
              "description": "ＨＰが少なくなったら食べるきのみをＨＰ半分の時に食べてしまう。"
            },
            {
              "name": "あついしぼう",
              "description": "厚い脂肪で守られているのでほのおタイプとこおりタイプの技のダメージを半減させる。"
            }
          ],
          "moves": [
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "タネばくだん",
              "type": "Grass",
              "category": "Physical",
              "power": 80,
              "description": "硬い殻をもつ大きなタネを上からたたきつけて相手を攻撃する。"
            },
            {
              "name": "ハイパーボイス",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "うるさく響く大きな振動を相手に与えて攻撃する。"
            },
            {
              "name": "ふるいたてる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "自分を奮いたてて攻撃と特攻をあげる。"
            }
          ],
          "actions": [
            "時間50% 強化解除"
          ],
          "hp": 976,
          "atk": 75,
          "def": 57,
          "sp_atk": 46,
          "sp_def": 61,
          "speed": 50
        },
        {
          "name": "パフュートン",
          "star": 3,
          "types": [],
          "abilities": [
            {
              "name": "とれないにおい",
              "description": "相手に触られるととれないにおいが相手にうつってしまう。"
            },
            {
              "name": "くいしんぼう",
              "description": "ＨＰが少なくなったら食べるきのみをＨＰ半分の時に食べてしまう。"
            },
            {
              "name": "あついしぼう",
              "description": "厚い脂肪で守られているのでほのおタイプとこおりタイプの技のダメージを半減させる。"
            }
          ],
          "moves": [
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "タネばくだん",
              "type": "Grass",
              "category": "Physical",
              "power": 80,
              "description": "硬い殻をもつ大きなタネを上からたたきつけて相手を攻撃する。"
            },
            {
              "name": "ハイパーボイス",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "うるさく響く大きな振動を相手に与えて攻撃する。"
            },
            {
              "name": "ふるいたてる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "自分を奮いたてて攻撃と特攻をあげる。"
            }
          ],
          "actions": [
            "時間50% 弱体解除"
          ],
          "hp": 0,
          "atk": 0,
          "def": 0,
          "sp_atk": 0,
          "sp_def": 0,
          "speed": 0
        },
        {
          "name": "パモット",
          "star": 3,
          "types": [
            "Electric",
            "Fighting"
          ],
          "abilities": [
            {
              "name": "ちくでん",
              "description": "でんきタイプの技を受けるとダメージを受けずに回復する。"
            },
            {
              "name": "しぜんかいふく",
              "description": "手持ちにひっこむと状態異常が治る。"
            },
            {
              "name": "てつのこぶし",
              "description": "パンチを使う技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "つっぱり",
              "type": "Fighting",
              "category": "Physical",
              "power": 15,
              "description": "ひらいた両手で相手をつっぱって攻撃する。２ー５回の間連続でだす。"
            },
            {
              "name": "スパーク",
              "type": "Electric",
              "category": "Physical",
              "power": 65,
              "description": "電気をまとい相手に突進して攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "ほっぺすりすり",
              "type": "Electric",
              "category": "Physical",
              "power": 20,
              "description": "電気を帯びたほっぺをすりつけて攻撃。相手をまひ状態にする。"
            },
            {
              "name": "ローキック",
              "type": "Fighting",
              "category": "Physical",
              "power": 65,
              "description": "素早い動きで相手の足をねらって攻撃する。相手の素早さをさげる。"
            }
          ],
          "actions": [
            "体力75% エレキフィールド"
          ],
          "hp": 696,
          "atk": 57,
          "def": 33,
          "sp_atk": 40,
          "sp_def": 33,
          "speed": 64
        },
        {
          "name": "ヒドイデ",
          "star": 3,
          "types": [
            "Poison",
            "Water"
          ],
          "abilities": [
            {
              "name": "ひとでなし",
              "description": "どく状態の相手を攻撃するとかならず急所に当たる。"
            },
            {
              "name": "じゅうなん",
              "description": "柔軟な体によってまひ状態にならない。"
            },
            {
              "name": "さいせいりょく",
              "description": "手持ちに引っ込むとＨＰが少し回復する。"
            }
          ],
          "moves": [
            {
              "name": "どくづき",
              "type": "Poison",
              "category": "Physical",
              "power": 80,
              "description": "毒にそまった触手や腕で相手を突き刺す。毒状態にすることがある。"
            },
            {
              "name": "アクアブレイク",
              "type": "Water",
              "category": "Physical",
              "power": 85,
              "description": "水の力で相手にぶつかって攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "どくどく",
              "type": "Poison",
              "category": "Status",
              "power": 0,
              "description": "相手を猛毒の状態にする。ターンがすすむほど毒のダメージが増えていく。"
            },
            {
              "name": "ミサイルばり",
              "type": "Bug",
              "category": "Physical",
              "power": 25,
              "description": "鋭いハリを相手に発射して攻撃する。２ー５回の間連続でだす。"
            }
          ],
          "actions": [
            "体力50% 強化解除"
          ],
          "hp": 640,
          "atk": 42,
          "def": 48,
          "sp_atk": 35,
          "sp_def": 41,
          "speed": 36
        },
        {
          "name": "ヒノヤコマ",
          "star": 3,
          "types": [
            "Fire",
            "Flying"
          ],
          "abilities": [
            {
              "name": "ほのおのからだ",
              "description": "自分に触った相手をやけど状態にすることがある。"
            },
            {
              "name": "はやてのつばさ",
              "description": "ＨＰが満タンだとひこうタイプの技を先制で出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "ニトロチャージ",
              "type": "Fire",
              "category": "Physical",
              "power": 50,
              "description": "炎をまとい相手を攻撃する。力をためて自分の素早さをあげる。"
            },
            {
              "name": "つばめがえし",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "素早い動きで相手をほんろうして切りつける。攻撃は必ず命中する。"
            },
            {
              "name": "つるぎのまい",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "戦いの舞を激しくおどって気合を高める。自分の攻撃をぐーんとあげる。"
            },
            {
              "name": "どろぼう",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "攻撃と同時に道具を盗む。自分が道具を持っている場合は盗めない。"
            }
          ],
          "actions": [
            "体力75% にほんばれ"
          ],
          "hp": 704,
          "atk": 56,
          "def": 43,
          "sp_atk": 44,
          "sp_def": 41,
          "speed": 63
        },
        {
          "name": "ヒポポタス",
          "star": 3,
          "types": [
            "Ground"
          ],
          "abilities": [
            {
              "name": "すなおこし",
              "description": "登場したとき天気を砂あらしにする。"
            },
            {
              "name": "すなのちから",
              "description": "天気がすなあらしのときいわタイプとじめんタイプとはがねタイプの威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "じならし",
              "type": "Ground",
              "category": "Physical",
              "power": 60,
              "description": "地面を踏みならして自分の周りにいるものを攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            }
          ],
          "actions": [
            "時間50% 強化解除"
          ],
          "hp": 736,
          "atk": 55,
          "def": 59,
          "sp_atk": 31,
          "sp_def": 34,
          "speed": 27
        },
        {
          "name": "ピカチュウ",
          "star": 3,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "せいでんき",
              "description": "静電気を体にまとい触った相手をまひさせることがある。"
            },
            {
              "name": "ひらいしん",
              "description": "でんきタイプの技を自分に寄せつけダメージを受けずに特攻が上がる。"
            }
          ],
          "moves": [
            {
              "name": "１０まんボルト",
              "type": "Electric",
              "category": "Special",
              "power": 90,
              "description": "強い電撃を相手に浴びせて攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "ほっぺすりすり",
              "type": "Electric",
              "category": "Physical",
              "power": 20,
              "description": "電気を帯びたほっぺをすりつけて攻撃。相手をまひ状態にする。"
            },
            {
              "name": "アイアンテール",
              "type": "Steel",
              "category": "Physical",
              "power": 100,
              "description": "硬いしっぽで相手をたたきつけて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "スパーク",
              "type": "Electric",
              "category": "Physical",
              "power": 65,
              "description": "電気をまとい相手に突進して攻撃する。まひ状態にすることがある。"
            }
          ],
          "actions": [
            "体力75% エレキフィールド"
          ],
          "hp": 552,
          "atk": 43,
          "def": 33,
          "sp_atk": 40,
          "sp_def": 40,
          "speed": 68
        },
        {
          "name": "フカマル",
          "star": 3,
          "types": [
            "Dragon",
            "Ground"
          ],
          "abilities": [
            {
              "name": "すながくれ",
              "description": "砂あらしのとき回避率が上がる。"
            },
            {
              "name": "さめはだ",
              "description": "攻撃を受けたとき自分に触れた相手をざらざらの肌でキズつける。"
            }
          ],
          "moves": [
            {
              "name": "ドラゴンクロー",
              "type": "Dragon",
              "category": "Physical",
              "power": 80,
              "description": "鋭くとがった巨大なツメで相手を切り裂いて攻撃する。"
            },
            {
              "name": "じならし",
              "type": "Ground",
              "category": "Physical",
              "power": 60,
              "description": "地面を踏みならして自分の周りにいるものを攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "がんせきふうじ",
              "type": "Rock",
              "category": "Physical",
              "power": 60,
              "description": "岩石を投げつけて攻撃する。相手の動きを封じることで素早さをさげる。"
            }
          ],
          "actions": [
            "体力50% つるぎのまい"
          ],
          "hp": 680,
          "atk": 54,
          "def": 36,
          "sp_atk": 33,
          "sp_def": 36,
          "speed": 34
        },
        {
          "name": "ペリッパー",
          "star": 3,
          "types": [
            "Water",
            "Flying"
          ],
          "abilities": [
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            },
            {
              "name": "あめふらし",
              "description": "登場したときに天気を雨にする。"
            },
            {
              "name": "あめうけざら",
              "description": "天気が雨のとき少しずつＨＰを回復する。"
            }
          ],
          "moves": [
            {
              "name": "エアスラッシュ",
              "type": "Flying",
              "category": "Special",
              "power": 75,
              "description": "空をも切り裂く空気の刃で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "なみのり",
              "type": "Water",
              "category": "Special",
              "power": 90,
              "description": "大きな波で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "しろいきり",
              "type": "Ice",
              "category": "Status",
              "power": 0,
              "description": "白い霧で体をおおう。５ターンの間相手に能力をさげられなくなる。"
            },
            {
              "name": "ちょうおんぱ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "特殊な音波を体から発して相手を混乱させる。"
            }
          ],
          "actions": [
            "体力50% おいかぜ"
          ],
          "hp": 696,
          "atk": 40,
          "def": 75,
          "sp_atk": 71,
          "sp_def": 54,
          "speed": 50
        },
        {
          "name": "ポポッコ",
          "star": 3,
          "types": [
            "Grass",
            "Flying"
          ],
          "abilities": [
            {
              "name": "ようりょくそ",
              "description": "天気が晴れのとき素早さが上がる。"
            },
            {
              "name": "リーフガード",
              "description": "天気が晴れのときは状態異常にならない。"
            },
            {
              "name": "すりぬけ",
              "description": "相手の壁や身代わりをすりぬけて攻撃できる。"
            }
          ],
          "moves": [
            {
              "name": "アクロバット",
              "type": "Flying",
              "category": "Physical",
              "power": 55,
              "description": "軽やかに相手を攻撃する。自分が道具を持っていないとき大きなダメージを与える。"
            },
            {
              "name": "タネばくだん",
              "type": "Grass",
              "category": "Physical",
              "power": 80,
              "description": "硬い殻をもつ大きなタネを上からたたきつけて相手を攻撃する。"
            },
            {
              "name": "わたほうし",
              "type": "Grass",
              "category": "Status",
              "power": 0,
              "description": "綿のようなフワフワの胞子をまとわりつかせて相手の素早さをがくっとさげる。"
            }
          ],
          "actions": [
            "体力75% にほんばれ"
          ],
          "hp": 664,
          "atk": 36,
          "def": 40,
          "sp_atk": 36,
          "sp_def": 50,
          "speed": 61
        },
        {
          "name": "マクノシタ",
          "star": 3,
          "types": [
            "Fighting"
          ],
          "abilities": [
            {
              "name": "あついしぼう",
              "description": "厚い脂肪で守られているのでほのおタイプとこおりタイプの技のダメージを半減させる。"
            },
            {
              "name": "こんじょう",
              "description": "状態異常になると根性をだして攻撃が上がる。"
            },
            {
              "name": "ちからずく",
              "description": "技の追加効果はなくなるがそのぶん高い威力で技を出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "かわらわり",
              "type": "Fighting",
              "category": "Physical",
              "power": 75,
              "description": "手刀を勢いよく振りおろして相手を攻撃する。ひかりのかべやリフレクターなども破壊できる。"
            },
            {
              "name": "すなかけ",
              "type": "Ground",
              "category": "Status",
              "power": 0,
              "description": "相手の顔に砂をかけて命中率をさげる。"
            },
            {
              "name": "のしかかり",
              "type": "Normal",
              "category": "Physical",
              "power": 85,
              "description": "全身で相手にのしかかり攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "きあいだめ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "深く息を吸い気合をこめる。自分の攻撃が急所に当たりやすくなる。"
            }
          ],
          "actions": [
            "体力50% 弱体解除"
          ],
          "hp": 760,
          "atk": 47,
          "def": 26,
          "sp_atk": 19,
          "sp_def": 26,
          "speed": 22
        },
        {
          "name": "マリル",
          "star": 3,
          "types": [
            "Water",
            "Fairy"
          ],
          "abilities": [
            {
              "name": "あついしぼう",
              "description": "厚い脂肪で守られているのでほのおタイプとこおりタイプの技のダメージを半減させる。"
            },
            {
              "name": "ちからもち",
              "description": "物理攻撃の威力が２倍になる。"
            },
            {
              "name": "そうしょく",
              "description": "くさタイプの技を受けるとダメージを受けずに攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "アクアジェット",
              "type": "Water",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "じゃれつく",
              "type": "Fairy",
              "category": "Physical",
              "power": 90,
              "description": "相手にじゃれついて攻撃する。相手の攻撃をさげることがある。"
            },
            {
              "name": "しっぽをふる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "しっぽを左右にかわいくふって油断を誘う。相手の防御をさげる。"
            }
          ],
          "actions": [
            "体力75% あまごい"
          ],
          "hp": 752,
          "atk": 19,
          "def": 40,
          "sp_atk": 19,
          "sp_def": 40,
          "speed": 33
        },
        {
          "name": "ミニリュウ",
          "star": 3,
          "types": [
            "Dragon"
          ],
          "abilities": [
            {
              "name": "だっぴ",
              "description": "体の皮を脱ぎ捨てることで状態異常を治すことがある。"
            },
            {
              "name": "ふしぎなうろこ",
              "description": "状態異常になると不思議なウロコが反応して防御が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ドラゴンダイブ",
              "type": "Dragon",
              "category": "Physical",
              "power": 100,
              "description": "すさまじい殺気で威圧しながら体当たりする。相手をひるませることがある。"
            },
            {
              "name": "しんぴのまもり",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "５ターンの間不思議な力に守られて状態異常にならなくなる。"
            },
            {
              "name": "アクアテール",
              "type": "Water",
              "category": "Physical",
              "power": 90,
              "description": "激しくあれくるう荒波のように大きなしっぽをふって相手を攻撃する。"
            },
            {
              "name": "たたきつける",
              "type": "Normal",
              "category": "Physical",
              "power": 80,
              "description": "長いしっぽやつるなどを使い相手をたたきつけて攻撃する。"
            }
          ],
          "actions": [
            "体力75% でんじは"
          ],
          "hp": 584,
          "atk": 49,
          "def": 36,
          "sp_atk": 40,
          "sp_def": 40,
          "speed": 40
        },
        {
          "name": "ムクバード",
          "star": 3,
          "types": [
            "Normal",
            "Flying"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "すてみ",
              "description": "反動でダメージを受ける技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "つばめがえし",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "素早い動きで相手をほんろうして切りつける。攻撃は必ず命中する。"
            },
            {
              "name": "こうそくいどう",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "力をぬいて体を軽くして高速で動く。自分の素早さをぐーんとあげる。"
            },
            {
              "name": "さわぐ",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "３ターンの間騒いで相手を攻撃する。そのあいだはだれも眠れなくなる。"
            }
          ],
          "actions": [
            "体力50% なきごえ"
          ],
          "hp": 664,
          "atk": 57,
          "def": 40,
          "sp_atk": 33,
          "sp_def": 33,
          "speed": 61
        },
        {
          "name": "メタモン",
          "star": 3,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "じゅうなん",
              "description": "柔軟な体によってまひ状態にならない。"
            },
            {
              "name": "かわりもの",
              "description": "目の前のポケモンに変身してしまう。"
            }
          ],
          "moves": [
            {
              "name": "へんしん",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手のポケモンに変身することで相手とまったく同じ技が使える。"
            }
          ],
          "actions": [
            "時間50% 強化解除"
          ],
          "hp": 624,
          "atk": 38,
          "def": 38,
          "sp_atk": 38,
          "sp_def": 38,
          "speed": 38
        },
        {
          "name": "メラルバ",
          "star": 3,
          "types": [
            "Bug",
            "Fire"
          ],
          "abilities": [
            {
              "name": "ほのおのからだ",
              "description": "自分に触った相手をやけど状態にすることがある。"
            },
            {
              "name": "むしのしらせ",
              "description": "ＨＰが減ったときむしタイプの技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ニトロチャージ",
              "type": "Fire",
              "category": "Physical",
              "power": 50,
              "description": "炎をまとい相手を攻撃する。力をためて自分の素早さをあげる。"
            },
            {
              "name": "かえんぐるま",
              "type": "Fire",
              "category": "Physical",
              "power": 60,
              "description": "炎をまとい相手に突進して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "きゅうけつ",
              "type": "Bug",
              "category": "Physical",
              "power": 80,
              "description": "血を吸い取って相手を攻撃する。与えたダメージの半分のＨＰを回復できる。"
            },
            {
              "name": "いやなおと",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "おもわず耳をふさぎたくなるいやなおとをだして相手の防御をがくっとさげる。"
            }
          ],
          "actions": [
            "体力50% いやなおと"
          ],
          "hp": 664,
          "atk": 64,
          "def": 43,
          "sp_atk": 40,
          "sp_def": 43,
          "speed": 47
        },
        {
          "name": "モココ",
          "star": 3,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "せいでんき",
              "description": "静電気を体にまとい触った相手をまひさせることがある。"
            },
            {
              "name": "プラス",
              "description": "プラスかマイナスの特性を持つポケモンが仲間にいると自分の特攻が上がる。"
            }
          ],
          "moves": [
            {
              "name": "あやしいひかり",
              "type": "Ghost",
              "category": "Status",
              "power": 0,
              "description": "怪しい光を相手にみせてまどわせる。相手を混乱させる。"
            },
            {
              "name": "パワージェム",
              "type": "Rock",
              "category": "Special",
              "power": 80,
              "description": "宝石のようにきらめく光を発射して相手を攻撃する。"
            },
            {
              "name": "ほうでん",
              "type": "Electric",
              "category": "Special",
              "power": 80,
              "description": "まばゆい電撃で自分の周りにいるものを攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "わたほうし",
              "type": "Grass",
              "category": "Status",
              "power": 0,
              "description": "綿のようなフワフワの胞子をまとわりつかせて相手の素早さをがくっとさげる。"
            }
          ],
          "actions": [
            "時間90% リフレクター"
          ],
          "hp": 752,
          "atk": 43,
          "def": 43,
          "sp_atk": 61,
          "sp_def": 47,
          "speed": 36
        },
        {
          "name": "モノズ",
          "star": 3,
          "types": [
            "Dark",
            "Dragon"
          ],
          "abilities": [
            {
              "name": "はりきり",
              "description": "自分の攻撃が高くなるが命中率が下がる。"
            }
          ],
          "moves": [
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "たたきつける",
              "type": "Normal",
              "category": "Physical",
              "power": 80,
              "description": "長いしっぽやつるなどを使い相手をたたきつけて攻撃する。"
            },
            {
              "name": "りゅうのいぶき",
              "type": "Dragon",
              "category": "Special",
              "power": 60,
              "description": "ものすごい息を相手に吹きつけて攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "バークアウト",
              "type": "Dark",
              "category": "Special",
              "power": 55,
              "description": "まくしたてるように怒鳴りつけて相手の特攻をさげる。"
            }
          ],
          "actions": [
            "時間90% きあいだめ"
          ],
          "hp": 648,
          "atk": 50,
          "def": 40,
          "sp_atk": 36,
          "sp_def": 40,
          "speed": 31
        },
        {
          "name": "モルフォン",
          "star": 3,
          "types": [
            "Bug",
            "Poison"
          ],
          "abilities": [
            {
              "name": "りんぷん",
              "description": "りんぷんに守られて技の追加効果を受けなくなる。"
            },
            {
              "name": "いろめがね",
              "description": "効果がいまひとつの技を通常の威力で出すことができる。"
            },
            {
              "name": "ミラクルスキン",
              "description": "変化技を受けにくい体になっている。"
            }
          ],
          "moves": [
            {
              "name": "エアスラッシュ",
              "type": "Flying",
              "category": "Special",
              "power": 75,
              "description": "空をも切り裂く空気の刃で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "むしのさざめき",
              "type": "Bug",
              "category": "Special",
              "power": 90,
              "description": "振動で音波をおこして攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "サイケこうせん",
              "type": "Psychic",
              "category": "Special",
              "power": 65,
              "description": "不思議な光線を相手に発射して攻撃する。混乱させることがある。"
            },
            {
              "name": "ねむりごな",
              "type": "Grass",
              "category": "Status",
              "power": 0,
              "description": "眠くなる粉をたくさんふりまいて相手を眠り状態にする。"
            }
          ],
          "actions": [
            "時間50% 強化解除"
          ],
          "hp": 752,
          "atk": 50,
          "def": 47,
          "sp_atk": 68,
          "sp_def": 57,
          "speed": 68
        },
        {
          "name": "ヤミラミ",
          "star": 3,
          "types": [
            "Dark",
            "Ghost"
          ],
          "abilities": [
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            },
            {
              "name": "あとだし",
              "description": "技を出す順番がかならず最後になる。"
            },
            {
              "name": "いたずらごころ",
              "description": "変化技を先制で出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "たたりめ",
              "type": "Ghost",
              "category": "Special",
              "power": 65,
              "description": "たたみかけるように攻撃する。状態異常の相手に大きなダメージを与える。"
            },
            {
              "name": "イカサマ",
              "type": "Dark",
              "category": "Physical",
              "power": 95,
              "description": "相手の力を利用する。戦っている相手の攻撃が高いほどダメージがあがる。"
            },
            {
              "name": "バークアウト",
              "type": "Dark",
              "category": "Special",
              "power": 55,
              "description": "まくしたてるように怒鳴りつけて相手の特攻をさげる。"
            },
            {
              "name": "つばめがえし",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "素早い動きで相手をほんろうして切りつける。攻撃は必ず命中する。"
            }
          ],
          "actions": [
            "体力50% おにび"
          ],
          "hp": 640,
          "atk": 57,
          "def": 57,
          "sp_atk": 50,
          "sp_def": 50,
          "speed": 40
        },
        {
          "name": "ヤレユータン",
          "star": 3,
          "types": [
            "Normal",
            "Psychic"
          ],
          "abilities": [
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            },
            {
              "name": "テレパシー",
              "description": "味方の攻撃を読み取って技を回避する。"
            },
            {
              "name": "きょうせい",
              "description": "味方が道具を使うと自分の持っている道具を味方に渡す。"
            }
          ],
          "moves": [
            {
              "name": "じんつうりき",
              "type": "Psychic",
              "category": "Special",
              "power": 80,
              "description": "みえない不思議な力を送って攻撃する。相手をひるませることがある。"
            },
            {
              "name": "からげんき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "自分が毒まひやけど状態のとき相手にくりだすと技の威力が２倍になる。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            },
            {
              "name": "イカサマ",
              "type": "Dark",
              "category": "Physical",
              "power": 95,
              "description": "相手の力を利用する。戦っている相手の攻撃が高いほどダメージがあがる。"
            }
          ],
          "actions": [
            "体力50% サイコフィールド"
          ],
          "hp": 864,
          "atk": 47,
          "def": 61,
          "sp_atk": 68,
          "sp_def": 82,
          "speed": 47
        },
        {
          "name": "ユキカブリ",
          "star": 3,
          "types": [
            "Grass",
            "Ice"
          ],
          "abilities": [
            {
              "name": "ゆきふらし",
              "description": "登場したときに天気をゆきにする。"
            },
            {
              "name": "ぼうおん",
              "description": "音を遮断することによって音の技を受けない。"
            }
          ],
          "moves": [
            {
              "name": "こごえるかぜ",
              "type": "Ice",
              "category": "Special",
              "power": 55,
              "description": "凍てつく冷気を相手に吹きつけて攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "はっぱカッター",
              "type": "Grass",
              "category": "Physical",
              "power": 55,
              "description": "はっぱをとばして相手を切りつけて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "ウェザーボール",
              "type": "Normal",
              "category": "Special",
              "power": 50,
              "description": "使ったときの天気によって技のタイプと威力が変わる。"
            },
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            }
          ],
          "actions": [
            "時間50% 強化解除"
          ],
          "hp": 696,
          "atk": 48,
          "def": 40,
          "sp_atk": 48,
          "sp_def": 47,
          "speed": 33
        },
        {
          "name": "ヨクバリス",
          "star": 3,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "ほおぶくろ",
              "description": "どんなきのみでも食べるとＨＰも回復する。"
            },
            {
              "name": "くいしんぼう",
              "description": "ＨＰが少なくなったら食べるきのみをＨＰ半分の時に食べてしまう。"
            }
          ],
          "moves": [
            {
              "name": "のしかかり",
              "type": "Normal",
              "category": "Physical",
              "power": 85,
              "description": "全身で相手にのしかかり攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "たくわえる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "力を蓄えて自分の防御と特防をあげる。最大３回まで蓄えられる。"
            },
            {
              "name": "ボディプレス",
              "type": "Fighting",
              "category": "Physical",
              "power": 80,
              "description": "体をぶつけて攻撃。防御が高いほど与えるダメージが増える。"
            }
          ],
          "actions": [
            "体力75% たくわえる"
          ],
          "hp": 1032,
          "atk": 71,
          "def": 71,
          "sp_atk": 43,
          "sp_def": 57,
          "speed": 19
        },
        {
          "name": "ヨーギラス",
          "star": 3,
          "types": [
            "Rock",
            "Ground"
          ],
          "abilities": [
            {
              "name": "こんじょう",
              "description": "状態異常になると根性をだして攻撃が上がる。"
            },
            {
              "name": "すながくれ",
              "description": "砂あらしのとき回避率が上がる。"
            }
          ],
          "moves": [
            {
              "name": "いわなだれ",
              "type": "Rock",
              "category": "Physical",
              "power": 75,
              "description": "大きな岩を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "じならし",
              "type": "Ground",
              "category": "Physical",
              "power": 60,
              "description": "地面を踏みならして自分の周りにいるものを攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "すなあらし",
              "type": "Rock",
              "category": "Status",
              "power": 0,
              "description": "５ターンの間砂あらしでいわじめんはがねタイプ以外にダメージ。いわタイプの特防があがる。"
            }
          ],
          "actions": [
            "体力75% すなあらし"
          ],
          "hp": 640,
          "atk": 49,
          "def": 40,
          "sp_atk": 36,
          "sp_def": 40,
          "speed": 33
        },
        {
          "name": "ラッキー",
          "star": 3,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "しぜんかいふく",
              "description": "手持ちにひっこむと状態異常が治る。"
            },
            {
              "name": "てんのめぐみ",
              "description": "天の恵みのおかげで技の追加効果がでやすい。"
            },
            {
              "name": "いやしのこころ",
              "description": "状態異常の味方をたまに治してあげる。"
            }
          ],
          "moves": [
            {
              "name": "エコーボイス",
              "type": "Normal",
              "category": "Special",
              "power": 40,
              "description": "響く声で相手を攻撃する。毎ターンだれかが技を使い続けると威力があがる。"
            },
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "まるくなる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体をまるめてちぢこまり自分の防御をあげる。"
            },
            {
              "name": "うたう",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "心地好いきれいな歌声を聞かせて相手を眠り状態にする。"
            }
          ],
          "actions": [
            "時間90% ひかりのかべ"
          ],
          "hp": 1760,
          "atk": 8,
          "def": 8,
          "sp_atk": 29,
          "sp_def": 78,
          "speed": 40
        },
        {
          "name": "リオル",
          "star": 3,
          "types": [
            "Fighting"
          ],
          "abilities": [
            {
              "name": "ふくつのこころ",
              "description": "ひるむたびに不屈の心を燃やして素早さが上がる。"
            },
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            },
            {
              "name": "いたずらごころ",
              "description": "変化技を先制で出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "はっけい",
              "type": "Fighting",
              "category": "Physical",
              "power": 60,
              "description": "相手の体に衝撃波を当てて攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "バレットパンチ",
              "type": "Steel",
              "category": "Physical",
              "power": 40,
              "description": "弾丸のような速くて硬いパンチを相手にくりだす。必ず先制攻撃できる。"
            },
            {
              "name": "しんくうは",
              "type": "Fighting",
              "category": "Special",
              "power": 40,
              "description": "こぶしをふって真空の波をまきおこす。必ず先制攻撃できる。"
            },
            {
              "name": "いやなおと",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "おもわず耳をふさぎたくなるいやなおとをだして相手の防御をがくっとさげる。"
            }
          ],
          "actions": [
            "時間50% 強化解除"
          ],
          "hp": 584,
          "atk": 54,
          "def": 33,
          "sp_atk": 29,
          "sp_def": 33,
          "speed": 47
        },
        {
          "name": "ルクシオ",
          "star": 3,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "とうそうしん",
              "description": "性別が同じだと闘争心を燃やして強くなる。性別が違うと弱くなる。"
            },
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "こんじょう",
              "description": "状態異常になると根性をだして攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "かみなりのキバ",
              "type": "Electric",
              "category": "Physical",
              "power": 65,
              "description": "電気をためたキバでかみつく。相手をひるませたりまひ状態にすることがある。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ほのおのキバ",
              "type": "Fire",
              "category": "Physical",
              "power": 65,
              "description": "炎をまとったキバでかみつく。相手をひるませたりやけど状態にすることがある。"
            },
            {
              "name": "じゅうでん",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "じゅうでん状態になり次にだすでんきタイプの技の威力をあげる。自分の特防もあがる。"
            }
          ],
          "actions": [
            "時間90% にらみつける"
          ],
          "hp": 696,
          "atk": 64,
          "def": 39,
          "sp_atk": 47,
          "sp_def": 39,
          "speed": 47
        },
        {
          "name": "ワシボン",
          "star": 3,
          "types": [
            "Normal",
            "Flying"
          ],
          "abilities": [
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            },
            {
              "name": "ちからずく",
              "description": "技の追加効果はなくなるがそのぶん高い威力で技を出すことができる。"
            },
            {
              "name": "はりきり",
              "description": "自分の攻撃が高くなるが命中率が下がる。"
            }
          ],
          "moves": [
            {
              "name": "きりさく",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "つばめがえし",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "素早い動きで相手をほんろうして切りつける。攻撃は必ず命中する。"
            },
            {
              "name": "いわくだき",
              "type": "Fighting",
              "category": "Physical",
              "power": 40,
              "description": "パンチで攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "こわいかお",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "恐ろしい顔でにらみおびえさせて相手の素早さをがくっとさげる。"
            }
          ],
          "actions": [
            "体力50% おいかぜ"
          ],
          "hp": 752,
          "atk": 63,
          "def": 40,
          "sp_atk": 30,
          "sp_def": 40,
          "speed": 47
        },
        {
          "name": "ワナイダー",
          "star": 3,
          "types": [
            "Bug"
          ],
          "abilities": [
            {
              "name": "ふみん",
              "description": "眠れない体質なのでねむり状態にならない。"
            },
            {
              "name": "はりこみ",
              "description": "交代で出てきた相手に２倍のダメージで攻撃できる。"
            }
          ],
          "moves": [
            {
              "name": "シザークロス",
              "type": "Bug",
              "category": "Physical",
              "power": 80,
              "description": "カマやツメをハサミのように交差させながら相手を切り裂く。"
            },
            {
              "name": "シャドークロー",
              "type": "Ghost",
              "category": "Physical",
              "power": 70,
              "description": "影からつくった鋭いツメで相手を切り裂く。急所に当たりやすい。"
            },
            {
              "name": "ダメおし",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "そのターンに相手がすでにダメージを受けていたら技の威力は２倍になる。"
            },
            {
              "name": "カウンター",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "相手から受けた物理攻撃のダメージを２倍にして同じ相手に返す。"
            }
          ],
          "actions": [
            "体力50% 強化解除"
          ],
          "hp": 696,
          "atk": 60,
          "def": 69,
          "sp_atk": 41,
          "sp_def": 65,
          "speed": 29
        },
        {
          "name": "アメモース",
          "star": 4,
          "types": [
            "Bug",
            "Flying"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            }
          ],
          "moves": [
            {
              "name": "むしのさざめき",
              "type": "Bug",
              "category": "Special",
              "power": 90,
              "description": "振動で音波をおこして攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "エアスラッシュ",
              "type": "Flying",
              "category": "Special",
              "power": 75,
              "description": "空をも切り裂く空気の刃で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "こうそくいどう",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "力をぬいて体を軽くして高速で動く。自分の素早さをぐーんとあげる。"
            },
            {
              "name": "しびれごな",
              "type": "Grass",
              "category": "Status",
              "power": 0,
              "description": "しびれる粉をたくさんふりまいて相手をまひ状態にする。"
            }
          ],
          "actions": [
            "時間40% 強化解除",
            "体力75% 弱体解除",
            "体力40% ちょうのまい"
          ],
          "hp": 1416,
          "atk": 59,
          "def": 60,
          "sp_atk": 95,
          "sp_def": 78,
          "speed": 77
        },
        {
          "name": "イキリンコ",
          "star": 4,
          "types": [
            "Normal",
            "Flying"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "はりきり",
              "description": "自分の攻撃が高くなるが命中率が下がる。"
            },
            {
              "name": "こんじょう",
              "description": "状態異常になると根性をだして攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "からげんき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "自分が毒まひやけど状態のとき相手にくりだすと技の威力が２倍になる。"
            },
            {
              "name": "ブレイブバード",
              "type": "Flying",
              "category": "Physical",
              "power": 120,
              "description": "はねをおりたたみ低空飛行で突撃する。自分もかなりダメージを受ける。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            },
            {
              "name": "さわぐ",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "３ターンの間騒いで相手を攻撃する。そのあいだはだれも眠れなくなる。"
            }
          ],
          "actions": [
            "時間60% 強化解除",
            "体力75% おだてる",
            "体力40% 強化解除"
          ],
          "hp": 1536,
          "atk": 91,
          "def": 50,
          "sp_atk": 45,
          "sp_def": 50,
          "speed": 87
        },
        {
          "name": "イシヘンジン",
          "star": 4,
          "types": [
            "Rock"
          ],
          "abilities": [
            {
              "name": "パワースポット",
              "description": "隣にいるだけで技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "いわなだれ",
              "type": "Rock",
              "category": "Physical",
              "power": 75,
              "description": "大きな岩を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "のしかかり",
              "type": "Normal",
              "category": "Physical",
              "power": 85,
              "description": "全身で相手にのしかかり攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "じゅうりょく",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "５ターンの間ふゆうやひこうタイプにじめんタイプの技が当たるようになる。空中に飛ぶ技も使えない。"
            },
            {
              "name": "じだんだ",
              "type": "Ground",
              "category": "Physical",
              "power": 75,
              "description": "悔しさをバネにして攻撃する。前のターンに技を外していると威力が倍になる。"
            }
          ],
          "actions": [
            "時間75% 弱体解除",
            "時間40% 強化解除",
            "体力75% のろい"
          ],
          "hp": 1740,
          "atk": 117,
          "def": 126,
          "sp_atk": 23,
          "sp_def": 23,
          "speed": 68
        },
        {
          "name": "イッカネズミ",
          "star": 4,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "フレンドガード",
              "description": "味方のダメージを減らすことができる。"
            },
            {
              "name": "ほおぶくろ",
              "description": "どんなきのみでも食べるとＨＰも回復する。"
            },
            {
              "name": "テクニシャン",
              "description": "威力が低い技の威力を高くして攻撃できる。"
            }
          ],
          "moves": [
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "じゃれつく",
              "type": "Fairy",
              "category": "Physical",
              "power": 90,
              "description": "相手にじゃれついて攻撃する。相手の攻撃をさげることがある。"
            },
            {
              "name": "アンコール",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手にアンコールした技を３回続けて出させる。"
            },
            {
              "name": "タネマシンガン",
              "type": "Grass",
              "category": "Physical",
              "power": 25,
              "description": "タネを勢いよく相手に発射して攻撃する。２ー５回の間連続でだす。"
            }
          ],
          "actions": [
            "時間75% けたぐり",
            "体力75% 弱体解除",
            "体力40% 強化解除"
          ],
          "hp": 1452,
          "atk": 72,
          "def": 68,
          "sp_atk": 63,
          "sp_def": 72,
          "speed": 104
        },
        {
          "name": "ウミトリオ",
          "star": 4,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "ぬめぬめ",
              "description": "攻撃で自分に触れた相手の素早さを下げる。"
            },
            {
              "name": "びびり",
              "description": "あくゴーストむしタイプの攻撃を受けたりいかくをされるとびびって素早さが上がる。"
            },
            {
              "name": "すながくれ",
              "description": "砂あらしのとき回避率が上がる。"
            }
          ],
          "moves": [
            {
              "name": "トリプルダイブ",
              "type": "Water",
              "category": "Physical",
              "power": 30,
              "description": "息のあった飛びこみをすることで相手に水しぶきをあてる。３回連続でダメージを与える。"
            },
            {
              "name": "ふいうち",
              "type": "Dark",
              "category": "Physical",
              "power": 70,
              "description": "相手より先に攻撃できる。相手がだす技が攻撃技でないと失敗する。"
            },
            {
              "name": "アクアジェット",
              "type": "Water",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "からげんき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "自分が毒まひやけど状態のとき相手にくりだすと技の威力が２倍になる。"
            }
          ],
          "actions": [
            "時間75% あまごい",
            "体力75% 弱体解除",
            "体力40% 強化解除"
          ],
          "hp": 1032,
          "atk": 95,
          "def": 50,
          "sp_atk": 50,
          "sp_def": 68,
          "speed": 113
        },
        {
          "name": "オコリザル",
          "star": 4,
          "types": [
            "Fighting"
          ],
          "abilities": [
            {
              "name": "やるき",
              "description": "やる気をだすことによってねむり状態にならない。"
            },
            {
              "name": "いかりのつぼ",
              "description": "急所に攻撃が当たると怒りくるって攻撃力が最大になる。"
            },
            {
              "name": "まけんき",
              "description": "相手に能力を下げられると攻撃がぐーんと上がる。"
            }
          ],
          "moves": [
            {
              "name": "かわらわり",
              "type": "Fighting",
              "category": "Physical",
              "power": 75,
              "description": "手刀を勢いよく振りおろして相手を攻撃する。ひかりのかべやリフレクターなども破壊できる。"
            },
            {
              "name": "ふんどのこぶし",
              "type": "Ghost",
              "category": "Physical",
              "power": 50,
              "description": "怒りをエネルギーに変えて攻撃。受けた攻撃の回数が多いほど技の威力があがる。"
            },
            {
              "name": "きしかいせい",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "力をふりしぼり攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            },
            {
              "name": "アンコール",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手にアンコールした技を３回続けて出させる。"
            }
          ],
          "actions": [
            "時間75% ビルドアップ",
            "体力75% 弱体解除",
            "体力40% 強化解除"
          ],
          "hp": 1356,
          "atk": 99,
          "def": 59,
          "sp_atk": 59,
          "sp_def": 68,
          "speed": 90
        },
        {
          "name": "オドリドリ",
          "star": 4,
          "types": [
            "Fire",
            "Flying"
          ],
          "abilities": [
            {
              "name": "おどりこ",
              "description": "だれかが踊り技を使うと自分もそれに続いて踊り技を出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "めざめるダンス",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "全力で踊って攻撃する。この技のタイプは自分のタイプと同じになる。"
            },
            {
              "name": "エアスラッシュ",
              "type": "Flying",
              "category": "Special",
              "power": 75,
              "description": "空をも切り裂く空気の刃で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "メロメロ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "♂なら♀を♀なら♂を誘惑してメロメロにする。相手は技がだしにくくなる。"
            },
            {
              "name": "フラフラダンス",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "フラフラとダンスをおどって自分の周りにいるものを混乱状態にさせる。"
            }
          ],
          "actions": [
            "時間90% にほんばれ",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1464,
          "atk": 68,
          "def": 68,
          "sp_atk": 93,
          "sp_def": 68,
          "speed": 88
        },
        {
          "name": "オノンド",
          "star": 4,
          "types": [
            "Dragon"
          ],
          "abilities": [
            {
              "name": "とうそうしん",
              "description": "性別が同じだと闘争心を燃やして強くなる。性別が違うと弱くなる。"
            },
            {
              "name": "かたやぶり",
              "description": "相手の特性にジャマされることなく相手に技を出すことができる。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            }
          ],
          "moves": [
            {
              "name": "ドラゴンクロー",
              "type": "Dragon",
              "category": "Physical",
              "power": 80,
              "description": "鋭くとがった巨大なツメで相手を切り裂いて攻撃する。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "こわいかお",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "恐ろしい顔でにらみおびえさせて相手の素早さをがくっとさげる。"
            },
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            }
          ],
          "actions": [
            "時間75% りゅうのまい",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1368,
          "atk": 110,
          "def": 68,
          "sp_atk": 41,
          "sp_def": 50,
          "speed": 65
        },
        {
          "name": "カエンジシ",
          "star": 4,
          "types": [
            "Fire",
            "Normal"
          ],
          "abilities": [
            {
              "name": "とうそうしん",
              "description": "性別が同じだと闘争心を燃やして強くなる。性別が違うと弱くなる。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            },
            {
              "name": "じしんかじょう",
              "description": "相手を倒すと自信がついて攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ハイパーボイス",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "うるさく響く大きな振動を相手に与えて攻撃する。"
            },
            {
              "name": "かえんほうしゃ",
              "type": "Fire",
              "category": "Special",
              "power": 90,
              "description": "激しい炎を相手に発射して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "おたけび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "おたけびをあげて相手を威嚇し相手の攻撃と特攻をさげる。"
            },
            {
              "name": "ほのおのキバ",
              "type": "Fire",
              "category": "Physical",
              "power": 65,
              "description": "炎をまとったキバでかみつく。相手をひるませたりやけど状態にすることがある。"
            }
          ],
          "actions": [
            "時間60% バークアウト",
            "時間50% 強化解除",
            "体力30% 弱体解除"
          ],
          "hp": 1584,
          "atk": 66,
          "def": 69,
          "sp_atk": 103,
          "sp_def": 64,
          "speed": 100
        },
        {
          "name": "カジリガメ",
          "star": 4,
          "types": [
            "Water",
            "Rock"
          ],
          "abilities": [
            {
              "name": "がんじょうあご",
              "description": "あごが頑丈で噛む技の威力が高くなる。"
            },
            {
              "name": "シェルアーマー",
              "description": "硬い殻に守られ相手の攻撃が急所に当たらない。"
            },
            {
              "name": "すいすい",
              "description": "天気が雨のとき素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "シェルブレード",
              "type": "Water",
              "category": "Physical",
              "power": 75,
              "description": "鋭い貝殻で切りつけて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "いわなだれ",
              "type": "Rock",
              "category": "Physical",
              "power": 75,
              "description": "大きな岩を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            },
            {
              "name": "こわいかお",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "恐ろしい顔でにらみおびえさせて相手の素早さをがくっとさげる。"
            }
          ],
          "actions": [
            "時間80% あまごい",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1632,
          "atk": 108,
          "def": 86,
          "sp_atk": 48,
          "sp_def": 66,
          "speed": 71
        },
        {
          "name": "カマスジョー",
          "star": 4,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "すいすい",
              "description": "天気が雨のとき素早さが上がる。"
            },
            {
              "name": "スクリューおびれ",
              "description": "相手の技を引き受ける特性や技の影響を無視できる。"
            }
          ],
          "moves": [
            {
              "name": "アクアブレイク",
              "type": "Water",
              "category": "Physical",
              "power": 85,
              "description": "水の力で相手にぶつかって攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "アクアジェット",
              "type": "Water",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "きあいだめ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "深く息を吸い気合をこめる。自分の攻撃が急所に当たりやすくなる。"
            }
          ],
          "actions": [
            "時間75% あまごい",
            "体力75% 弱体解除",
            "体力40% 強化解除"
          ],
          "hp": 1308,
          "atk": 115,
          "def": 59,
          "sp_atk": 59,
          "sp_def": 50,
          "speed": 127
        },
        {
          "name": "カラミンゴ",
          "star": 4,
          "types": [
            "Flying",
            "Fighting"
          ],
          "abilities": [
            {
              "name": "きもったま",
              "description": "ノーマルタイプとかくとうタイプの技をゴーストタイプに当てることができる。いかくにも動じない。"
            },
            {
              "name": "ちどりあし",
              "description": "こんらん状態のときは回避率がアップする。"
            },
            {
              "name": "きょうえん",
              "description": "登場したときに味方の能力変化をコピーする。"
            }
          ],
          "moves": [
            {
              "name": "メガトンキック",
              "type": "Normal",
              "category": "Physical",
              "power": 120,
              "description": "ものすごい力をこめたキックで相手をけとばして攻撃する。"
            },
            {
              "name": "つばめがえし",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "素早い動きで相手をほんろうして切りつける。攻撃は必ず命中する。"
            },
            {
              "name": "エアスラッシュ",
              "type": "Flying",
              "category": "Special",
              "power": 75,
              "description": "空をも切り裂く空気の刃で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "しっぺがえし",
              "type": "Dark",
              "category": "Physical",
              "power": 50,
              "description": "ためこんで攻撃する。相手よりあとに攻撃できると技の威力は２倍になる。"
            }
          ],
          "actions": [
            "時間60% 強化解除",
            "体力75% ちょうはつ",
            "体力40% 強化解除"
          ],
          "hp": 1536,
          "atk": 108,
          "def": 71,
          "sp_atk": 72,
          "sp_def": 62,
          "speed": 86
        },
        {
          "name": "ガバイト",
          "star": 4,
          "types": [
            "Dragon",
            "Ground"
          ],
          "abilities": [
            {
              "name": "すながくれ",
              "description": "砂あらしのとき回避率が上がる。"
            },
            {
              "name": "さめはだ",
              "description": "攻撃を受けたとき自分に触れた相手をざらざらの肌でキズつける。"
            }
          ],
          "moves": [
            {
              "name": "ドラゴンクロー",
              "type": "Dragon",
              "category": "Physical",
              "power": 80,
              "description": "鋭くとがった巨大なツメで相手を切り裂いて攻撃する。"
            },
            {
              "name": "すなあらし",
              "type": "Rock",
              "category": "Status",
              "power": 0,
              "description": "５ターンの間砂あらしでいわじめんはがねタイプ以外にダメージ。いわタイプの特防があがる。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "すなじごく",
              "type": "Ground",
              "category": "Physical",
              "power": 35,
              "description": "激しく吹きあれる砂あらしの中に４ー５ターンの間相手を閉じこめて攻撃する。"
            }
          ],
          "actions": [
            "時間80% 弱体解除",
            "体力50% すなあらし",
            "体力40% 弱体解除"
          ],
          "hp": 1392,
          "atk": 86,
          "def": 63,
          "sp_atk": 50,
          "sp_def": 54,
          "speed": 78
        },
        {
          "name": "キリキザン",
          "star": 4,
          "types": [
            "Dark",
            "Steel"
          ],
          "abilities": [
            {
              "name": "まけんき",
              "description": "相手に能力を下げられると攻撃がぐーんと上がる。"
            },
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            },
            {
              "name": "プレッシャー",
              "description": "プレッシャーをあたえて相手の使う技のＰＰを多く減らす。"
            }
          ],
          "moves": [
            {
              "name": "アイアンヘッド",
              "type": "Steel",
              "category": "Physical",
              "power": 80,
              "description": "鋼のような硬い頭で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "つじぎり",
              "type": "Dark",
              "category": "Physical",
              "power": 70,
              "description": "一瞬のすきをついて相手を切りはらう。急所に当たりやすい。"
            },
            {
              "name": "きりさく",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "こわいかお",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "恐ろしい顔でにらみおびえさせて相手の素早さをがくっとさげる。"
            }
          ],
          "actions": [
            "時間40% 強化解除",
            "時間40% てっぺき",
            "体力75% 弱体解除"
          ],
          "hp": 1356,
          "atk": 117,
          "def": 95,
          "sp_atk": 59,
          "sp_def": 68,
          "speed": 68
        },
        {
          "name": "クエスパトラ",
          "star": 4,
          "types": [
            "Psychic"
          ],
          "abilities": [
            {
              "name": "びんじょう",
              "description": "相手の能力が上がったとき自分も便乗して同じように能力を上げる。"
            },
            {
              "name": "おみとおし",
              "description": "登場したとき相手の持ち物を見通すことができる。"
            },
            {
              "name": "かそく",
              "description": "毎ターン素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "ルミナコリジョン",
              "type": "Psychic",
              "category": "Special",
              "power": 80,
              "description": "精神にも作用する奇妙な光を放って攻撃する。相手の特防をがくっとさげる。"
            },
            {
              "name": "マジカルシャイン",
              "type": "Fairy",
              "category": "Special",
              "power": 80,
              "description": "強力な光を放ち相手にダメージを与える。"
            },
            {
              "name": "つぶらなひとみ",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "つぶらなひとみで相手をみつめて攻撃をさげる。必ず先制攻撃できる。"
            },
            {
              "name": "フェザーダンス",
              "type": "Flying",
              "category": "Status",
              "power": 0,
              "description": "羽毛をふりまいて相手の体にからませる。相手の攻撃をがくっとさげる。"
            }
          ],
          "actions": [
            "時間75% サイコフィールド",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1680,
          "atk": 59,
          "def": 59,
          "sp_atk": 95,
          "sp_def": 59,
          "speed": 99
        },
        {
          "name": "ケケンカニ",
          "star": 4,
          "types": [
            "Fighting",
            "Ice"
          ],
          "abilities": [
            {
              "name": "かいりきバサミ",
              "description": "力自慢のハサミを持っているので相手に攻撃を下げられない。"
            },
            {
              "name": "てつのこぶし",
              "description": "パンチを使う技の威力が上がる。"
            },
            {
              "name": "いかりのつぼ",
              "description": "急所に攻撃が当たると怒りくるって攻撃力が最大になる。"
            }
          ],
          "moves": [
            {
              "name": "アイスハンマー",
              "type": "Ice",
              "category": "Physical",
              "power": 100,
              "description": "強くて重いこぶしをふるってダメージを与える。自分の素早さがさがる。"
            },
            {
              "name": "きしかいせい",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "力をふりしぼり攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            },
            {
              "name": "たたきつける",
              "type": "Normal",
              "category": "Physical",
              "power": 80,
              "description": "長いしっぽやつるなどを使い相手をたたきつけて攻撃する。"
            },
            {
              "name": "いわくだき",
              "type": "Fighting",
              "category": "Physical",
              "power": 40,
              "description": "パンチで攻撃する。相手の防御をさげることがある。"
            }
          ],
          "actions": [
            "時間90% ゆきげしき",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1704,
          "atk": 123,
          "def": 74,
          "sp_atk": 60,
          "sp_def": 65,
          "speed": 43
        },
        {
          "name": "ケンタロス",
          "star": 4,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "いかりのつぼ",
              "description": "急所に攻撃が当たると怒りくるって攻撃力が最大になる。"
            },
            {
              "name": "はんすう",
              "description": "きのみを食べると次のターンの終わりに胃から出してもう１回だけ食べる。"
            }
          ],
          "moves": [
            {
              "name": "レイジングブル",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "怒り狂うあばれうしの猛烈なタックル。フォルムで技のタイプが変わりひかりのかべやリフレクターなども破壊できる。"
            },
            {
              "name": "しねんのずつき",
              "type": "Psychic",
              "category": "Physical",
              "power": 80,
              "description": "思念の力を額に集めて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "こわいかお",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "恐ろしい顔でにらみおびえさせて相手の素早さをがくっとさげる。"
            },
            {
              "name": "いばる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせて混乱させる。怒りで相手の攻撃はぐーんとあがってしまう。"
            }
          ],
          "actions": [
            "時間60% 強化解除",
            "体力75% ふるいたてる",
            "体力40% 強化解除"
          ],
          "hp": 1464,
          "atk": 95,
          "def": 90,
          "sp_atk": 41,
          "sp_def": 68,
          "speed": 104
        },
        {
          "name": "コオリッポ",
          "star": 4,
          "types": [
            "Ice"
          ],
          "abilities": [
            {
              "name": "アイスフェイス",
              "description": "物理攻撃は頭の氷がみがわりになるが姿も変わる。氷はゆきが降ると元に戻る。"
            }
          ],
          "moves": [
            {
              "name": "なみのり",
              "type": "Water",
              "category": "Special",
              "power": 90,
              "description": "大きな波で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "つららおとし",
              "type": "Ice",
              "category": "Physical",
              "power": 85,
              "description": "大きな氷柱を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            },
            {
              "name": "しろいきり",
              "type": "Ice",
              "category": "Status",
              "power": 0,
              "description": "白い霧で体をおおう。５ターンの間相手に能力をさげられなくなる。"
            }
          ],
          "actions": [
            "時間90% ゆきげしき",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1464,
          "atk": 77,
          "def": 104,
          "sp_atk": 63,
          "sp_def": 86,
          "speed": 50
        },
        {
          "name": "コモルー",
          "star": 4,
          "types": [
            "Dragon"
          ],
          "abilities": [
            {
              "name": "いしあたま",
              "description": "反動を受ける技を出してもＨＰが減らない。"
            },
            {
              "name": "ぼうじん",
              "description": "すなあらしのダメージを受けない。粉や胞子の影響も受けない。"
            }
          ],
          "moves": [
            {
              "name": "ドラゴンクロー",
              "type": "Dragon",
              "category": "Physical",
              "power": 80,
              "description": "鋭くとがった巨大なツメで相手を切り裂いて攻撃する。"
            },
            {
              "name": "かえんほうしゃ",
              "type": "Fire",
              "category": "Special",
              "power": 90,
              "description": "激しい炎を相手に発射して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "きあいだめ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "深く息を吸い気合をこめる。自分の攻撃が急所に当たりやすくなる。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [
            "時間75% りゅうのまい",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1356,
          "atk": 90,
          "def": 95,
          "sp_atk": 59,
          "sp_def": 50,
          "speed": 50
        },
        {
          "name": "コータス",
          "star": 4,
          "types": [
            "Fire"
          ],
          "abilities": [
            {
              "name": "しろいけむり",
              "description": "白い煙に守られて相手に能力を下げられない。"
            },
            {
              "name": "ひでり",
              "description": "登場したときに天気を晴れにする。"
            },
            {
              "name": "シェルアーマー",
              "description": "硬い殻に守られ相手の攻撃が急所に当たらない。"
            }
          ],
          "moves": [
            {
              "name": "ふんえん",
              "type": "Fire",
              "category": "Special",
              "power": 80,
              "description": "真っ赤な炎で自分の周りにいるものを攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "のしかかり",
              "type": "Normal",
              "category": "Physical",
              "power": 85,
              "description": "全身で相手にのしかかり攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "クリアスモッグ",
              "type": "Poison",
              "category": "Special",
              "power": 50,
              "description": "特殊な泥の塊を相手に投げつけて攻撃する。能力変化をもとにもどす。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            }
          ],
          "actions": [
            "時間40% 弱体解除",
            "体力75% からにこもる",
            "体力35% からをやぶる"
          ],
          "hp": 1416,
          "atk": 81,
          "def": 131,
          "sp_atk": 81,
          "sp_def": 68,
          "speed": 23
        },
        {
          "name": "ゴルダック",
          "star": 4,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "しめりけ",
              "description": "あたりを湿らせることによってじばくなどの爆発する技をだれも使えなくなる。"
            },
            {
              "name": "ノーてんき",
              "description": "あらゆる天気の影響がなくなってしまう。"
            },
            {
              "name": "すいすい",
              "description": "天気が雨のとき素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "アクアテール",
              "type": "Water",
              "category": "Physical",
              "power": 90,
              "description": "激しくあれくるう荒波のように大きなしっぽをふって相手を攻撃する。"
            },
            {
              "name": "しねんのずつき",
              "type": "Psychic",
              "category": "Physical",
              "power": 80,
              "description": "思念の力を額に集めて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "さいみんじゅつ",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "眠気を誘う暗示をかけて相手を眠り状態にする。"
            }
          ],
          "actions": [
            "時間75% あまごい",
            "体力75% 弱体解除",
            "体力40% 強化解除"
          ],
          "hp": 1524,
          "atk": 78,
          "def": 75,
          "sp_atk": 90,
          "sp_def": 77,
          "speed": 81
        },
        {
          "name": "ゴーゴート",
          "star": 4,
          "types": [
            "Grass"
          ],
          "abilities": [
            {
              "name": "そうしょく",
              "description": "くさタイプの技を受けるとダメージを受けずに攻撃が上がる。"
            },
            {
              "name": "くさのけがわ",
              "description": "グラスフィールドのとき防御が上がる。"
            }
          ],
          "moves": [
            {
              "name": "タネばくだん",
              "type": "Grass",
              "category": "Physical",
              "power": 80,
              "description": "硬い殻をもつ大きなタネを上からたたきつけて相手を攻撃する。"
            },
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "せいちょう",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体を一気に大きく生長させて攻撃と特攻をあげる。"
            },
            {
              "name": "しっぽをふる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "しっぽを左右にかわいくふって油断を誘う。相手の防御をさげる。"
            }
          ],
          "actions": [
            "時間90% グラスフィールド",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1980,
          "atk": 95,
          "def": 60,
          "sp_atk": 92,
          "sp_def": 77,
          "speed": 66
        },
        {
          "name": "サダイジャ",
          "star": 4,
          "types": [
            "Ground"
          ],
          "abilities": [
            {
              "name": "すなはき",
              "description": "攻撃を受けると砂あらしを起こす。"
            },
            {
              "name": "だっぴ",
              "description": "体の皮を脱ぎ捨てることで状態異常を治すことがある。"
            },
            {
              "name": "すながくれ",
              "description": "砂あらしのとき回避率が上がる。"
            }
          ],
          "moves": [
            {
              "name": "じならし",
              "type": "Ground",
              "category": "Physical",
              "power": 60,
              "description": "地面を踏みならして自分の周りにいるものを攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "へびにらみ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "おなかの模様でおびえさせて相手をまひの状態にする。"
            },
            {
              "name": "ぶんまわす",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "自分の体をぶんまわして相手にダメージを与える。"
            },
            {
              "name": "たたきつける",
              "type": "Normal",
              "category": "Physical",
              "power": 80,
              "description": "長いしっぽやつるなどを使い相手をたたきつけて攻撃する。"
            }
          ],
          "actions": [
            "時間75% とぐろをまく",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1428,
          "atk": 101,
          "def": 117,
          "sp_atk": 63,
          "sp_def": 68,
          "speed": 68
        },
        {
          "name": "サナギラス",
          "star": 4,
          "types": [
            "Rock",
            "Ground"
          ],
          "abilities": [
            {
              "name": "だっぴ",
              "description": "体の皮を脱ぎ捨てることで状態異常を治すことがある。"
            }
          ],
          "moves": [
            {
              "name": "いわなだれ",
              "type": "Rock",
              "category": "Physical",
              "power": 75,
              "description": "大きな岩を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "じだんだ",
              "type": "Ground",
              "category": "Physical",
              "power": 75,
              "description": "悔しさをバネにして攻撃する。前のターンに技を外していると威力が倍になる。"
            },
            {
              "name": "うちおとす",
              "type": "Rock",
              "category": "Physical",
              "power": 50,
              "description": "石や弾を投げて飛んでいる相手を攻撃する。相手はうち落とされて地面に落ちる。"
            },
            {
              "name": "いやなおと",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "おもわず耳をふさぎたくなるいやなおとをだして相手の防御をがくっとさげる。"
            }
          ],
          "actions": [
            "時間40% 強化解除",
            "体力90% すなあらし",
            "体力75% 弱体解除"
          ],
          "hp": 1416,
          "atk": 80,
          "def": 68,
          "sp_atk": 63,
          "sp_def": 68,
          "speed": 50
        },
        {
          "name": "ザングース",
          "star": 4,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "めんえき",
              "description": "体内に免疫を持っているためどく状態にならない。"
            },
            {
              "name": "どくぼうそう",
              "description": "どく状態になったとき物理技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ブレイククロー",
              "type": "Normal",
              "category": "Physical",
              "power": 75,
              "description": "硬く鋭いツメで切り裂いて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "つじぎり",
              "type": "Dark",
              "category": "Physical",
              "power": 70,
              "description": "一瞬のすきをついて相手を切りはらう。急所に当たりやすい。"
            },
            {
              "name": "でんこうせっか",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "つけあがる",
              "type": "Dark",
              "category": "Physical",
              "power": 20,
              "description": "自分の強さを鼻高々に攻撃する。自分の能力があがっているほど威力があがる。"
            }
          ],
          "actions": [
            "時間40% 強化解除",
            "体力75% 弱体解除",
            "体力40% つるぎのまい"
          ],
          "hp": 1440,
          "atk": 108,
          "def": 59,
          "sp_atk": 59,
          "sp_def": 59,
          "speed": 86
        },
        {
          "name": "シビビール",
          "star": 4,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "ふゆう",
              "description": "地面から浮くことによってじめんタイプの技を受けない。"
            }
          ],
          "moves": [
            {
              "name": "ほうでん",
              "type": "Electric",
              "category": "Special",
              "power": 80,
              "description": "まばゆい電撃で自分の周りにいるものを攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "あばれる",
              "type": "Normal",
              "category": "Physical",
              "power": 120,
              "description": "２ー３ターンの間暴れまくって相手を攻撃する。暴れたあとは混乱する。"
            },
            {
              "name": "いえき",
              "type": "Poison",
              "category": "Status",
              "power": 0,
              "description": "胃液を相手の体に吐きつける。ついた胃液は相手の特性の効果を消す。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            }
          ],
          "actions": [
            "時間75% とぐろをまく",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1356,
          "atk": 81,
          "def": 68,
          "sp_atk": 72,
          "sp_def": 68,
          "speed": 41
        },
        {
          "name": "ジヘッド",
          "star": 4,
          "types": [
            "Dark",
            "Dragon"
          ],
          "abilities": [
            {
              "name": "はりきり",
              "description": "自分の攻撃が高くなるが命中率が下がる。"
            }
          ],
          "moves": [
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "あくのはどう",
              "type": "Dark",
              "category": "Special",
              "power": 80,
              "description": "体から悪意にみちた恐ろしいオーラを発する。相手をひるませることがある。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "こわいかお",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "恐ろしい顔でにらみおびえさせて相手の素早さをがくっとさげる。"
            }
          ],
          "actions": [
            "時間75% ふるいたてる",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1428,
          "atk": 81,
          "def": 68,
          "sp_atk": 63,
          "sp_def": 68,
          "speed": 57
        },
        {
          "name": "ジュペッタ",
          "star": 4,
          "types": [
            "Ghost"
          ],
          "abilities": [
            {
              "name": "ふみん",
              "description": "眠れない体質なのでねむり状態にならない。"
            },
            {
              "name": "おみとおし",
              "description": "登場したとき相手の持ち物を見通すことができる。"
            },
            {
              "name": "のろわれボディ",
              "description": "攻撃を受けると相手の技をかなしばり状態にすることがある。"
            }
          ],
          "moves": [
            {
              "name": "シャドークロー",
              "type": "Ghost",
              "category": "Physical",
              "power": 70,
              "description": "影からつくった鋭いツメで相手を切り裂く。急所に当たりやすい。"
            },
            {
              "name": "ふいうち",
              "type": "Dark",
              "category": "Physical",
              "power": 70,
              "description": "相手より先に攻撃できる。相手がだす技が攻撃技でないと失敗する。"
            },
            {
              "name": "たたりめ",
              "type": "Ghost",
              "category": "Special",
              "power": 65,
              "description": "たたみかけるように攻撃する。状態異常の相手に大きなダメージを与える。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            }
          ],
          "actions": [
            "時間50% わるだくみ",
            "時間40% 弱体解除",
            "体力90% おにび"
          ],
          "hp": 1344,
          "atk": 108,
          "def": 63,
          "sp_atk": 79,
          "sp_def": 61,
          "speed": 63
        },
        {
          "name": "スカタンク",
          "star": 4,
          "types": [
            "Poison",
            "Dark"
          ],
          "abilities": [
            {
              "name": "あくしゅう",
              "description": "臭いにおいを放つことによって攻撃したときに相手をひるませることがある。"
            },
            {
              "name": "ゆうばく",
              "description": "ひんしになったとき触った相手にダメージをあたえる。"
            },
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            }
          ],
          "moves": [
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "どくづき",
              "type": "Poison",
              "category": "Physical",
              "power": 80,
              "description": "毒にそまった触手や腕で相手を突き刺す。毒状態にすることがある。"
            },
            {
              "name": "のしかかり",
              "type": "Normal",
              "category": "Physical",
              "power": 85,
              "description": "全身で相手にのしかかり攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "きあいだめ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "深く息を吸い気合をこめる。自分の攻撃が急所に当たりやすくなる。"
            }
          ],
          "actions": [
            "時間40% 強化解除",
            "体力75% 弱体解除",
            "体力40% いやなおと"
          ],
          "hp": 1764,
          "atk": 88,
          "def": 65,
          "sp_atk": 68,
          "sp_def": 59,
          "speed": 80
        },
        {
          "name": "スコヴィラン",
          "star": 4,
          "types": [
            "Grass",
            "Fire"
          ],
          "abilities": [
            {
              "name": "ようりょくそ",
              "description": "天気が晴れのとき素早さが上がる。"
            },
            {
              "name": "ふみん",
              "description": "眠れない体質なのでねむり状態にならない。"
            },
            {
              "name": "ムラっけ",
              "description": "毎ターン能力のどれかがぐーんと上がってどれかが下がる。"
            }
          ],
          "moves": [
            {
              "name": "タネばくだん",
              "type": "Grass",
              "category": "Physical",
              "power": 80,
              "description": "硬い殻をもつ大きなタネを上からたたきつけて相手を攻撃する。"
            },
            {
              "name": "かえんほうしゃ",
              "type": "Fire",
              "category": "Special",
              "power": 90,
              "description": "激しい炎を相手に発射して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "なやみのタネ",
              "type": "Grass",
              "category": "Status",
              "power": 0,
              "description": "心をなやませるタネを植えつける。相手を眠れなくして特性をふみんにする。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [
            "時間95% にほんばれ",
            "時間65% せいちょう",
            "時間40% 弱体解除"
          ],
          "hp": 1356,
          "atk": 102,
          "def": 63,
          "sp_atk": 102,
          "sp_def": 63,
          "speed": 72
        },
        {
          "name": "ストライク",
          "star": 4,
          "types": [
            "Bug",
            "Flying"
          ],
          "abilities": [
            {
              "name": "むしのしらせ",
              "description": "ＨＰが減ったときむしタイプの技の威力が上がる。"
            },
            {
              "name": "テクニシャン",
              "description": "威力が低い技の威力を高くして攻撃できる。"
            },
            {
              "name": "ふくつのこころ",
              "description": "ひるむたびに不屈の心を燃やして素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "シザークロス",
              "type": "Bug",
              "category": "Physical",
              "power": 80,
              "description": "カマやツメをハサミのように交差させながら相手を切り裂く。"
            },
            {
              "name": "つばめがえし",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "素早い動きで相手をほんろうして切りつける。攻撃は必ず命中する。"
            },
            {
              "name": "きあいだめ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "深く息を吸い気合をこめる。自分の攻撃が急所に当たりやすくなる。"
            },
            {
              "name": "きりさく",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。"
            }
          ],
          "actions": [
            "時間40% 強化解除",
            "体力75% 弱体解除",
            "体力40% つるぎのまい"
          ],
          "hp": 1416,
          "atk": 104,
          "def": 77,
          "sp_atk": 54,
          "sp_def": 77,
          "speed": 99
        },
        {
          "name": "スリーパー",
          "star": 4,
          "types": [
            "Psychic"
          ],
          "abilities": [
            {
              "name": "ふみん",
              "description": "眠れない体質なのでねむり状態にならない。"
            },
            {
              "name": "よちむ",
              "description": "登場したとき相手の持つ技をひとつだけ読み取る。"
            },
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            }
          ],
          "moves": [
            {
              "name": "サイコショック",
              "type": "Psychic",
              "category": "Special",
              "power": 80,
              "description": "不思議な念波を実体化して相手を攻撃する。物理的なダメージを与える。"
            },
            {
              "name": "サイコキネシス",
              "type": "Psychic",
              "category": "Special",
              "power": 90,
              "description": "強い念力を相手に送って攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "さいみんじゅつ",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "眠気を誘う暗示をかけて相手を眠り状態にする。"
            },
            {
              "name": "れいとうパンチ",
              "type": "Ice",
              "category": "Physical",
              "power": 75,
              "description": "冷気をこめたパンチで相手を攻撃する。こおり状態にすることがある。"
            }
          ],
          "actions": [
            "時間60% 強化解除",
            "体力75% さいみんじゅつ",
            "体力45% めいそう"
          ],
          "hp": 1572,
          "atk": 70,
          "def": 68,
          "sp_atk": 70,
          "sp_def": 108,
          "speed": 65
        },
        {
          "name": "タイカイデン",
          "star": 4,
          "types": [
            "Electric",
            "Flying"
          ],
          "abilities": [
            {
              "name": "ふうりょくでんき",
              "description": "風技を受けるとじゅうでん状態になる。"
            },
            {
              "name": "ちくでん",
              "description": "でんきタイプの技を受けるとダメージを受けずに回復する。"
            },
            {
              "name": "かちき",
              "description": "相手に能力を下げられると特攻がぐーんと上がる。"
            }
          ],
          "moves": [
            {
              "name": "ほうでん",
              "type": "Electric",
              "category": "Special",
              "power": 80,
              "description": "まばゆい電撃で自分の周りにいるものを攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "エアスラッシュ",
              "type": "Flying",
              "category": "Special",
              "power": 75,
              "description": "空をも切り裂く空気の刃で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "エレキボール",
              "type": "Electric",
              "category": "Special",
              "power": 1,
              "description": "電気の塊を相手にぶつける。相手より素早さが速いほど威力があがる。"
            },
            {
              "name": "おいかぜ",
              "type": "Flying",
              "category": "Status",
              "power": 0,
              "description": "激しく吹きあれる風の渦をつくり４ターンの間味方全員の素早さをあげる。"
            }
          ],
          "actions": [
            "時間80% おいかぜ",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1416,
          "atk": 68,
          "def": 59,
          "sp_atk": 99,
          "sp_def": 59,
          "speed": 117
        },
        {
          "name": "タイレーツ",
          "star": 4,
          "types": [
            "Fighting"
          ],
          "abilities": [
            {
              "name": "カブトアーマー",
              "description": "硬い甲羅に守られて相手の攻撃が急所に当たらない。"
            },
            {
              "name": "まけんき",
              "description": "相手に能力を下げられると攻撃がぐーんと上がる。"
            }
          ],
          "moves": [
            {
              "name": "きしかいせい",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "力をふりしぼり攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            },
            {
              "name": "であいがしら",
              "type": "Bug",
              "category": "Physical",
              "power": 90,
              "description": "威力が高い技だが戦闘に出たらすぐに出さないと成功しない。"
            },
            {
              "name": "かわらわり",
              "type": "Fighting",
              "category": "Physical",
              "power": 75,
              "description": "手刀を勢いよく振りおろして相手を攻撃する。ひかりのかべやリフレクターなども破壊できる。"
            },
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            }
          ],
          "actions": [
            "時間40% 強化解除",
            "体力75% 弱体解除",
            "体力50% はいすいのじん"
          ],
          "hp": 1356,
          "atk": 95,
          "def": 95,
          "sp_atk": 68,
          "sp_def": 59,
          "speed": 72
        },
        {
          "name": "タギングル",
          "star": 4,
          "types": [
            "Poison",
            "Normal"
          ],
          "abilities": [
            {
              "name": "かるわざ",
              "description": "持っていた道具がなくなると素早さが上がる。"
            },
            {
              "name": "どくしゅ",
              "description": "触るだけで相手をどく状態にすることがある。"
            },
            {
              "name": "いたずらごころ",
              "description": "変化技を先制で出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "どくづき",
              "type": "Poison",
              "category": "Physical",
              "power": 80,
              "description": "毒にそまった触手や腕で相手を突き刺す。毒状態にすることがある。"
            },
            {
              "name": "きりさく",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            },
            {
              "name": "おだてる",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手をおだてて混乱させる。同時に相手の特攻もあげてしまう。"
            }
          ],
          "actions": [
            "時間75% 弱体解除",
            "時間75% どくどく",
            "時間40% 強化解除"
          ],
          "hp": 1332,
          "atk": 90,
          "def": 63,
          "sp_atk": 77,
          "sp_def": 69,
          "speed": 104
        },
        {
          "name": "ダグトリオ",
          "star": 4,
          "types": [
            "Ground"
          ],
          "abilities": [
            {
              "name": "すながくれ",
              "description": "砂あらしのとき回避率が上がる。"
            },
            {
              "name": "ありじごく",
              "description": "戦闘で相手を逃げられなくする。"
            },
            {
              "name": "すなのちから",
              "description": "天気がすなあらしのときいわタイプとじめんタイプとはがねタイプの威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "じしん",
              "type": "Ground",
              "category": "Physical",
              "power": 100,
              "description": "地震の衝撃で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "つじぎり",
              "type": "Dark",
              "category": "Physical",
              "power": 70,
              "description": "一瞬のすきをついて相手を切りはらう。急所に当たりやすい。"
            },
            {
              "name": "きりさく",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "おどろかす",
              "type": "Ghost",
              "category": "Physical",
              "power": 30,
              "description": "大きな声などで不意に驚かして攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [
            "時間80% 弱体解除",
            "体力50% すなあらし",
            "体力40% 弱体解除"
          ],
          "hp": 1032,
          "atk": 95,
          "def": 50,
          "sp_atk": 50,
          "sp_def": 68,
          "speed": 113
        },
        {
          "name": "チャーレム",
          "star": 4,
          "types": [
            "Fighting",
            "Psychic"
          ],
          "abilities": [
            {
              "name": "ヨガパワー",
              "description": "ヨガの力で物理攻撃の威力が２倍になる。"
            },
            {
              "name": "テレパシー",
              "description": "味方の攻撃を読み取って技を回避する。"
            }
          ],
          "moves": [
            {
              "name": "かわらわり",
              "type": "Fighting",
              "category": "Physical",
              "power": 75,
              "description": "手刀を勢いよく振りおろして相手を攻撃する。ひかりのかべやリフレクターなども破壊できる。"
            },
            {
              "name": "しねんのずつき",
              "type": "Psychic",
              "category": "Physical",
              "power": 80,
              "description": "思念の力を額に集めて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "サイケこうせん",
              "type": "Psychic",
              "category": "Special",
              "power": 65,
              "description": "不思議な光線を相手に発射して攻撃する。混乱させることがある。"
            },
            {
              "name": "ねこだまし",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "先制攻撃で相手をひるませる。戦闘にでたらすぐにださないと成功しない。"
            }
          ],
          "actions": [
            "時間75% ふるいたてる",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1308,
          "atk": 59,
          "def": 72,
          "sp_atk": 59,
          "sp_def": 72,
          "speed": 77
        },
        {
          "name": "ツンベアー",
          "star": 4,
          "types": [
            "Ice"
          ],
          "abilities": [
            {
              "name": "ゆきがくれ",
              "description": "天気がゆきのとき回避率が上がる。"
            },
            {
              "name": "ゆきかき",
              "description": "天気がゆきのとき素早さが上がる。"
            },
            {
              "name": "すいすい",
              "description": "天気が雨のとき素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "つららおとし",
              "type": "Ice",
              "category": "Physical",
              "power": 85,
              "description": "大きな氷柱を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "きりさく",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            },
            {
              "name": "れいとうビーム",
              "type": "Ice",
              "category": "Special",
              "power": 90,
              "description": "凍えるビームを相手に発射して攻撃する。こおり状態にすることがある。"
            }
          ],
          "actions": [
            "時間90% ゆきげしき",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1680,
          "atk": 122,
          "def": 77,
          "sp_atk": 68,
          "sp_def": 77,
          "speed": 50
        },
        {
          "name": "デデンネ",
          "star": 4,
          "types": [
            "Electric",
            "Fairy"
          ],
          "abilities": [
            {
              "name": "ほおぶくろ",
              "description": "どんなきのみでも食べるとＨＰも回復する。"
            },
            {
              "name": "ものひろい",
              "description": "相手の使った道具を拾ってくることがある。冒険中も拾ってくる。"
            },
            {
              "name": "プラス",
              "description": "プラスかマイナスの特性を持つポケモンが仲間にいると自分の特攻が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ほうでん",
              "type": "Electric",
              "category": "Special",
              "power": 80,
              "description": "まばゆい電撃で自分の周りにいるものを攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "マジカルシャイン",
              "type": "Fairy",
              "category": "Special",
              "power": 80,
              "description": "強力な光を放ち相手にダメージを与える。"
            },
            {
              "name": "ほっぺすりすり",
              "type": "Electric",
              "category": "Physical",
              "power": 20,
              "description": "電気を帯びたほっぺをすりつけて攻撃。相手をまひ状態にする。"
            },
            {
              "name": "あまえる",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。"
            }
          ],
          "actions": [
            "時間85% エレキフィールド",
            "体力75% 弱体解除",
            "体力25% あまえる"
          ],
          "hp": 1380,
          "atk": 57,
          "def": 56,
          "sp_atk": 77,
          "sp_def": 65,
          "speed": 95
        },
        {
          "name": "デンリュウ",
          "star": 4,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "せいでんき",
              "description": "静電気を体にまとい触った相手をまひさせることがある。"
            },
            {
              "name": "プラス",
              "description": "プラスかマイナスの特性を持つポケモンが仲間にいると自分の特攻が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ほうでん",
              "type": "Electric",
              "category": "Special",
              "power": 80,
              "description": "まばゆい電撃で自分の周りにいるものを攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "パワージェム",
              "type": "Rock",
              "category": "Special",
              "power": 80,
              "description": "宝石のようにきらめく光を発射して相手を攻撃する。"
            },
            {
              "name": "あやしいひかり",
              "type": "Ghost",
              "category": "Status",
              "power": 0,
              "description": "怪しい光を相手にみせてまどわせる。相手を混乱させる。"
            },
            {
              "name": "エレキボール",
              "type": "Electric",
              "category": "Special",
              "power": 1,
              "description": "電気の塊を相手にぶつける。相手より素早さが速いほど威力があがる。"
            }
          ],
          "actions": [
            "時間85% エレキフィールド",
            "体力75% 弱体解除",
            "体力25% コットンガード"
          ],
          "hp": 1632,
          "atk": 72,
          "def": 81,
          "sp_atk": 108,
          "sp_def": 86,
          "speed": 54
        },
        {
          "name": "トリトドン",
          "star": 4,
          "types": [
            "Water",
            "Ground"
          ],
          "abilities": [
            {
              "name": "ねんちゃく",
              "description": "粘着質の体に道具がくっついているため相手に道具を奪われない。"
            },
            {
              "name": "よびみず",
              "description": "みずタイプの技を自分によせつけダメージは受けずに特攻が上がる。"
            },
            {
              "name": "すなのちから",
              "description": "天気がすなあらしのときいわタイプとじめんタイプとはがねタイプの威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "だいちのちから",
              "type": "Ground",
              "category": "Special",
              "power": 90,
              "description": "相手の足下へ大地の力を放出する。相手の特防をさげることがある。"
            },
            {
              "name": "だくりゅう",
              "type": "Water",
              "category": "Special",
              "power": 90,
              "description": "濁った水を相手に発射して攻撃する。命中率をさげることがある。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            }
          ],
          "actions": [
            "時間75% 弱体解除",
            "時間40% 強化解除",
            "体力75% たくわえる"
          ],
          "hp": 1848,
          "atk": 79,
          "def": 66,
          "sp_atk": 87,
          "sp_def": 78,
          "speed": 40
        },
        {
          "name": "トリトドン",
          "star": 4,
          "types": [],
          "abilities": [
            {
              "name": "ねんちゃく",
              "description": "粘着質の体に道具がくっついているため相手に道具を奪われない。"
            },
            {
              "name": "よびみず",
              "description": "みずタイプの技を自分によせつけダメージは受けずに特攻が上がる。"
            },
            {
              "name": "すなのちから",
              "description": "天気がすなあらしのときいわタイプとじめんタイプとはがねタイプの威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "だいちのちから",
              "type": "Ground",
              "category": "Special",
              "power": 90,
              "description": "相手の足下へ大地の力を放出する。相手の特防をさげることがある。"
            },
            {
              "name": "だくりゅう",
              "type": "Water",
              "category": "Special",
              "power": 90,
              "description": "濁った水を相手に発射して攻撃する。命中率をさげることがある。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            }
          ],
          "actions": [
            "時間75% 弱体解除",
            "時間40% 強化解除",
            "体力75% たくわえる"
          ],
          "hp": 0,
          "atk": 0,
          "def": 0,
          "sp_atk": 0,
          "sp_def": 0,
          "speed": 0
        },
        {
          "name": "トロピウス",
          "star": 4,
          "types": [
            "Grass",
            "Flying"
          ],
          "abilities": [
            {
              "name": "ようりょくそ",
              "description": "天気が晴れのとき素早さが上がる。"
            },
            {
              "name": "サンパワー",
              "description": "天気が晴れると特攻が上がるが毎ターンＨＰが減る。"
            },
            {
              "name": "しゅうかく",
              "description": "使ったきのみを何回も作りだす。"
            }
          ],
          "moves": [
            {
              "name": "リーフブレード",
              "type": "Grass",
              "category": "Physical",
              "power": 90,
              "description": "はっぱを剣のようにあやつり相手を切りつけて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "つばめがえし",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "素早い動きで相手をほんろうして切りつける。攻撃は必ず命中する。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            },
            {
              "name": "おいかぜ",
              "type": "Flying",
              "category": "Status",
              "power": 0,
              "description": "激しく吹きあれる風の渦をつくり４ターンの間味方全員の素早さをあげる。"
            }
          ],
          "actions": [
            "時間75% りゅうのまい",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1728,
          "atk": 66,
          "def": 79,
          "sp_atk": 69,
          "sp_def": 83,
          "speed": 50
        },
        {
          "name": "ドオー",
          "star": 4,
          "types": [
            "Poison",
            "Ground"
          ],
          "abilities": [
            {
              "name": "どくのトゲ",
              "description": "自分に触った相手をどく状態にすることがある。"
            },
            {
              "name": "ちょすい",
              "description": "みずタイプの技を受けるとダメージを受けずに回復する。"
            },
            {
              "name": "てんねん",
              "description": "相手の能力の変化を無視して攻撃ができる。"
            }
          ],
          "moves": [
            {
              "name": "どくづき",
              "type": "Poison",
              "category": "Physical",
              "power": 80,
              "description": "毒にそまった触手や腕で相手を突き刺す。毒状態にすることがある。"
            },
            {
              "name": "じだんだ",
              "type": "Ground",
              "category": "Physical",
              "power": 75,
              "description": "悔しさをバネにして攻撃する。前のターンに技を外していると威力が倍になる。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            },
            {
              "name": "どくどく",
              "type": "Poison",
              "category": "Status",
              "power": 0,
              "description": "相手を猛毒の状態にする。ターンがすすむほど毒のダメージが増えていく。"
            }
          ],
          "actions": [
            "時間40% 強化解除",
            "体力75% たくわえる",
            "体力60% 弱体解除"
          ],
          "hp": 2064,
          "atk": 72,
          "def": 59,
          "sp_atk": 45,
          "sp_def": 95,
          "speed": 23
        },
        {
          "name": "ドクロッグ",
          "star": 4,
          "types": [
            "Poison",
            "Fighting"
          ],
          "abilities": [
            {
              "name": "きけんよち",
              "description": "相手の持つ危険な技を察知することができる。"
            },
            {
              "name": "かんそうはだ",
              "description": "天気が雨の時やみずタイプの技でＨＰが回復しはれの時やほのおタイプの技で減ってしまう。"
            },
            {
              "name": "どくしゅ",
              "description": "触るだけで相手をどく状態にすることがある。"
            }
          ],
          "moves": [
            {
              "name": "どくづき",
              "type": "Poison",
              "category": "Physical",
              "power": 80,
              "description": "毒にそまった触手や腕で相手を突き刺す。毒状態にすることがある。"
            },
            {
              "name": "かわらわり",
              "type": "Fighting",
              "category": "Physical",
              "power": 75,
              "description": "手刀を勢いよく振りおろして相手を攻撃する。ひかりのかべやリフレクターなども破壊できる。"
            },
            {
              "name": "ベノムショック",
              "type": "Poison",
              "category": "Special",
              "power": 65,
              "description": "特殊な毒液を浴びせかける。毒状態の相手には威力が２倍になる。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            }
          ],
          "actions": [
            "時間75% 弱体解除",
            "時間75% どくどく",
            "時間40% 強化解除"
          ],
          "hp": 1548,
          "atk": 100,
          "def": 63,
          "sp_atk": 82,
          "sp_def": 63,
          "speed": 81
        },
        {
          "name": "ドレディア",
          "star": 4,
          "types": [
            "Grass"
          ],
          "abilities": [
            {
              "name": "ようりょくそ",
              "description": "天気が晴れのとき素早さが上がる。"
            },
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "リーフガード",
              "description": "天気が晴れのときは状態異常にならない。"
            }
          ],
          "moves": [
            {
              "name": "エナジーボール",
              "type": "Grass",
              "category": "Special",
              "power": 90,
              "description": "自然から集めた命の力を発射する。相手の特防をさげることがある。"
            },
            {
              "name": "かふんだんご",
              "type": "Bug",
              "category": "Special",
              "power": 90,
              "description": "敵には爆発するだんごを使って攻撃。味方には回復するだんごを与える。"
            },
            {
              "name": "はなふぶき",
              "type": "Grass",
              "category": "Physical",
              "power": 90,
              "description": "激しい花吹雪を起こし周りにいるものに攻撃してダメージを与える。"
            },
            {
              "name": "あまえる",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。"
            }
          ],
          "actions": [
            "時間80% にほんばれ",
            "体力75% 弱体解除",
            "体力40% ちょうのまい"
          ],
          "hp": 1416,
          "atk": 59,
          "def": 72,
          "sp_atk": 104,
          "sp_def": 72,
          "speed": 86
        },
        {
          "name": "ドロンチ",
          "star": 4,
          "types": [
            "Dragon",
            "Ghost"
          ],
          "abilities": [
            {
              "name": "クリアボディ",
              "description": "相手の技や特性で能力を下げられない。"
            },
            {
              "name": "すりぬけ",
              "description": "相手の壁や身代わりをすりぬけて攻撃できる。"
            },
            {
              "name": "のろわれボディ",
              "description": "攻撃を受けると相手の技をかなしばり状態にすることがある。"
            }
          ],
          "moves": [
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "ドラゴンダイブ",
              "type": "Dragon",
              "category": "Physical",
              "power": 100,
              "description": "すさまじい殺気で威圧しながら体当たりする。相手をひるませることがある。"
            },
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "たたりめ",
              "type": "Ghost",
              "category": "Special",
              "power": 65,
              "description": "たたみかけるように攻撃する。状態異常の相手に大きなダメージを与える。"
            }
          ],
          "actions": [
            "体力80% おにび",
            "体力75% 弱体解除",
            "体力40% りゅうのまい"
          ],
          "hp": 1392,
          "atk": 77,
          "def": 50,
          "sp_atk": 59,
          "sp_def": 50,
          "speed": 96
        },
        {
          "name": "ドンファン",
          "star": 4,
          "types": [
            "Ground"
          ],
          "abilities": [
            {
              "name": "がんじょう",
              "description": "ＨＰが満タンのとき技を受けても一撃で倒されることがない。一撃必殺技も効かない。"
            },
            {
              "name": "すながくれ",
              "description": "砂あらしのとき回避率が上がる。"
            }
          ],
          "moves": [
            {
              "name": "じだんだ",
              "type": "Ground",
              "category": "Physical",
              "power": 75,
              "description": "悔しさをバネにして攻撃する。前のターンに技を外していると威力が倍になる。"
            },
            {
              "name": "たたきつける",
              "type": "Normal",
              "category": "Physical",
              "power": 80,
              "description": "長いしっぽやつるなどを使い相手をたたきつけて攻撃する。"
            },
            {
              "name": "ころがる",
              "type": "Rock",
              "category": "Physical",
              "power": 30,
              "description": "５ターンの間転がり続けて攻撃する。技が当たるたびに威力があがる。"
            },
            {
              "name": "ダメおし",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "そのターンに相手がすでにダメージを受けていたら技の威力は２倍になる。"
            }
          ],
          "actions": [
            "時間90% ころがる",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1632,
          "atk": 113,
          "def": 113,
          "sp_atk": 59,
          "sp_def": 59,
          "speed": 50
        },
        {
          "name": "ナマズン",
          "star": 4,
          "types": [
            "Water",
            "Ground"
          ],
          "abilities": [
            {
              "name": "どんかん",
              "description": "鈍感なのでメロメロやちょうはつ状態にならない。いかくにも動じない。"
            },
            {
              "name": "きけんよち",
              "description": "相手の持つ危険な技を察知することができる。"
            },
            {
              "name": "うるおいボディ",
              "description": "天気が雨のとき状態異常が治る。"
            }
          ],
          "moves": [
            {
              "name": "だくりゅう",
              "type": "Water",
              "category": "Special",
              "power": 90,
              "description": "濁った水を相手に発射して攻撃する。命中率をさげることがある。"
            },
            {
              "name": "だいちのちから",
              "type": "Ground",
              "category": "Special",
              "power": 90,
              "description": "相手の足下へ大地の力を放出する。相手の特防をさげることがある。"
            },
            {
              "name": "くすぐる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体をくすぐり笑わせることで相手の攻撃と防御をさげる。"
            },
            {
              "name": "アクアテール",
              "type": "Water",
              "category": "Physical",
              "power": 90,
              "description": "激しくあれくるう荒波のように大きなしっぽをふって相手を攻撃する。"
            }
          ],
          "actions": [
            "時間75% あまごい",
            "体力75% 弱体解除",
            "体力40% 強化解除"
          ],
          "hp": 1848,
          "atk": 75,
          "def": 70,
          "sp_atk": 73,
          "sp_def": 68,
          "speed": 59
        },
        {
          "name": "ヌメイル",
          "star": 4,
          "types": [
            "Dragon"
          ],
          "abilities": [
            {
              "name": "そうしょく",
              "description": "くさタイプの技を受けるとダメージを受けずに攻撃が上がる。"
            },
            {
              "name": "うるおいボディ",
              "description": "天気が雨のとき状態異常が治る。"
            },
            {
              "name": "ぬめぬめ",
              "description": "攻撃で自分に触れた相手の素早さを下げる。"
            }
          ],
          "moves": [
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "のしかかり",
              "type": "Normal",
              "category": "Physical",
              "power": 85,
              "description": "全身で相手にのしかかり攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "じたばた",
              "type": "Normal",
              "category": "Physical",
              "power": 1,
              "description": "じたばた暴れて攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            }
          ],
          "actions": [
            "時間80% あまごい",
            "体力75% 弱体解除",
            "体力40% 強化解除"
          ],
          "hp": 1392,
          "atk": 72,
          "def": 52,
          "sp_atk": 79,
          "sp_def": 106,
          "speed": 59
        },
        {
          "name": "ネオラント",
          "star": 4,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "すいすい",
              "description": "天気が雨のとき素早さが上がる。"
            },
            {
              "name": "よびみず",
              "description": "みずタイプの技を自分によせつけダメージは受けずに特攻が上がる。"
            },
            {
              "name": "みずのベール",
              "description": "水のベールを身にまといやけど状態にならない。"
            }
          ],
          "moves": [
            {
              "name": "なみのり",
              "type": "Water",
              "category": "Special",
              "power": 90,
              "description": "大きな波で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "オーロラビーム",
              "type": "Ice",
              "category": "Special",
              "power": 65,
              "description": "にじいろのビームを相手に発射して攻撃する。攻撃をさげることがある。"
            },
            {
              "name": "サイケこうせん",
              "type": "Psychic",
              "category": "Special",
              "power": 65,
              "description": "不思議な光線を相手に発射して攻撃する。混乱させることがある。"
            },
            {
              "name": "しんぴのまもり",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "５ターンの間不思議な力に守られて状態異常にならなくなる。"
            }
          ],
          "actions": [
            "時間90% おいかぜ",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1404,
          "atk": 67,
          "def": 73,
          "sp_atk": 67,
          "sp_def": 82,
          "speed": 86
        },
        {
          "name": "ネッコアラ",
          "star": 4,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "ぜったいねむり",
              "description": "つねに夢うつつの状態で絶対に目覚めない。眠ったまま攻撃ができる。"
            }
          ],
          "moves": [
            {
              "name": "たたきつける",
              "type": "Normal",
              "category": "Physical",
              "power": 80,
              "description": "長いしっぽやつるなどを使い相手をたたきつけて攻撃する。"
            },
            {
              "name": "ふいうち",
              "type": "Dark",
              "category": "Physical",
              "power": 70,
              "description": "相手より先に攻撃できる。相手がだす技が攻撃技でないと失敗する。"
            },
            {
              "name": "ウッドハンマー",
              "type": "Grass",
              "category": "Physical",
              "power": 120,
              "description": "硬い胴体を相手にたたきつけて攻撃する。自分もかなりダメージを受ける。"
            },
            {
              "name": "じたばた",
              "type": "Normal",
              "category": "Physical",
              "power": 1,
              "description": "じたばた暴れて攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            }
          ],
          "actions": [
            "時間75% 弱体解除",
            "時間40% 強化解除",
            "体力75% たくわえる"
          ],
          "hp": 1356,
          "atk": 108,
          "def": 63,
          "sp_atk": 72,
          "sp_def": 90,
          "speed": 63
        },
        {
          "name": "ノクタス",
          "star": 4,
          "types": [
            "Grass",
            "Dark"
          ],
          "abilities": [
            {
              "name": "すながくれ",
              "description": "砂あらしのとき回避率が上がる。"
            },
            {
              "name": "ちょすい",
              "description": "みずタイプの技を受けるとダメージを受けずに回復する。"
            }
          ],
          "moves": [
            {
              "name": "エナジーボール",
              "type": "Grass",
              "category": "Special",
              "power": 90,
              "description": "自然から集めた命の力を発射する。相手の特防をさげることがある。"
            },
            {
              "name": "しっぺがえし",
              "type": "Dark",
              "category": "Physical",
              "power": 50,
              "description": "ためこんで攻撃する。相手よりあとに攻撃できると技の威力は２倍になる。"
            },
            {
              "name": "ミサイルばり",
              "type": "Bug",
              "category": "Physical",
              "power": 25,
              "description": "鋭いハリを相手に発射して攻撃する。２ー５回の間連続でだす。"
            },
            {
              "name": "すなかけ",
              "type": "Ground",
              "category": "Status",
              "power": 0,
              "description": "相手の顔に砂をかけて命中率をさげる。"
            }
          ],
          "actions": [
            "時間90% すなあらし",
            "時間50% 弱体解除",
            "体力75% 弱体解除"
          ],
          "hp": 1416,
          "atk": 108,
          "def": 59,
          "sp_atk": 108,
          "sp_def": 59,
          "speed": 54
        },
        {
          "name": "ノココッチ",
          "star": 4,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "てんのめぐみ",
              "description": "天の恵みのおかげで技の追加効果がでやすい。"
            },
            {
              "name": "にげあし",
              "description": "野生のポケモンから必ず逃げられる。"
            },
            {
              "name": "びびり",
              "description": "あくゴーストむしタイプの攻撃を受けたりいかくをされるとびびって素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "ハイパードリル",
              "type": "Normal",
              "category": "Physical",
              "power": 100,
              "description": "とがった体の部位を急速に回転させつらぬく。まもるやみきりなども無視できる。"
            },
            {
              "name": "ドリルライナー",
              "type": "Ground",
              "category": "Physical",
              "power": 80,
              "description": "ドリルのように体を回転しながら相手に体当たりする。急所に当たりやすい。"
            },
            {
              "name": "へびにらみ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "おなかの模様でおびえさせて相手をまひの状態にする。"
            },
            {
              "name": "からげんき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "自分が毒まひやけど状態のとき相手にくりだすと技の威力が２倍になる。"
            }
          ],
          "actions": [
            "時間75% とぐろをまく",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 2004,
          "atk": 95,
          "def": 77,
          "sp_atk": 81,
          "sp_def": 72,
          "speed": 54
        },
        {
          "name": "ハカドッグ",
          "star": 4,
          "types": [
            "Ghost"
          ],
          "abilities": [
            {
              "name": "すなかき",
              "description": "天気がすなあらしのとき素早さが上がる。"
            },
            {
              "name": "もふもふ",
              "description": "相手から受けた接触する技のダメージを半減するがほのおタイプの技のダメージは２倍になる。"
            }
          ],
          "moves": [
            {
              "name": "おはかまいり",
              "type": "Ghost",
              "category": "Physical",
              "power": 50,
              "description": "仲間の無念を晴らすため攻撃する。倒された味方のポケモンが多いほど技の威力が増える。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "あまえる",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。"
            },
            {
              "name": "したでなめる",
              "type": "Ghost",
              "category": "Physical",
              "power": 30,
              "description": "長い舌で相手をなめまわして攻撃する。まひ状態にすることがある。"
            }
          ],
          "actions": [
            "時間65% あまえる",
            "時間40% 弱体解除",
            "体力75% 強化解除"
          ],
          "hp": 1428,
          "atk": 95,
          "def": 95,
          "sp_atk": 50,
          "sp_def": 92,
          "speed": 66
        },
        {
          "name": "ハクリュー",
          "star": 4,
          "types": [
            "Dragon"
          ],
          "abilities": [
            {
              "name": "だっぴ",
              "description": "体の皮を脱ぎ捨てることで状態異常を治すことがある。"
            },
            {
              "name": "ふしぎなうろこ",
              "description": "状態異常になると不思議なウロコが反応して防御が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ドラゴンダイブ",
              "type": "Dragon",
              "category": "Physical",
              "power": 100,
              "description": "すさまじい殺気で威圧しながら体当たりする。相手をひるませることがある。"
            },
            {
              "name": "アクアテール",
              "type": "Water",
              "category": "Physical",
              "power": 90,
              "description": "激しくあれくるう荒波のように大きなしっぽをふって相手を攻撃する。"
            },
            {
              "name": "しんぴのまもり",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "５ターンの間不思議な力に守られて状態異常にならなくなる。"
            },
            {
              "name": "のしかかり",
              "type": "Normal",
              "category": "Physical",
              "power": 85,
              "description": "全身で相手にのしかかり攻撃する。まひ状態にすることがある。"
            }
          ],
          "actions": [
            "時間40% 強化解除",
            "時間40% りゅうのまい",
            "体力75% 弱体解除"
          ],
          "hp": 1308,
          "atk": 80,
          "def": 63,
          "sp_atk": 68,
          "sp_def": 68,
          "speed": 68
        },
        {
          "name": "ハブネーク",
          "star": 4,
          "types": [
            "Poison"
          ],
          "abilities": [
            {
              "name": "だっぴ",
              "description": "体の皮を脱ぎ捨てることで状態異常を治すことがある。"
            },
            {
              "name": "すりぬけ",
              "description": "相手の壁や身代わりをすりぬけて攻撃できる。"
            }
          ],
          "moves": [
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "ヘドロばくだん",
              "type": "Poison",
              "category": "Special",
              "power": 90,
              "description": "汚いヘドロを相手に投げつけて攻撃する。毒状態にすることがある。"
            },
            {
              "name": "いばる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせて混乱させる。怒りで相手の攻撃はぐーんとあがってしまう。"
            },
            {
              "name": "へびにらみ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "おなかの模様でおびえさせて相手をまひの状態にする。"
            }
          ],
          "actions": [
            "時間75% とぐろをまく",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1440,
          "atk": 95,
          "def": 59,
          "sp_atk": 95,
          "sp_def": 59,
          "speed": 63
        },
        {
          "name": "ハラバリー",
          "star": 4,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "でんきにかえる",
              "description": "ダメージを受けるとじゅうでん状態になる。"
            },
            {
              "name": "せいでんき",
              "description": "静電気を体にまとい触った相手をまひさせることがある。"
            },
            {
              "name": "しめりけ",
              "description": "あたりを湿らせることによってじばくなどの爆発する技をだれも使えなくなる。"
            }
          ],
          "moves": [
            {
              "name": "ほうでん",
              "type": "Electric",
              "category": "Special",
              "power": 80,
              "description": "まばゆい電撃で自分の周りにいるものを攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "マッドショット",
              "type": "Ground",
              "category": "Special",
              "power": 55,
              "description": "泥の塊を相手に投げつけて攻撃する。同時に相手の素早さをさげる。"
            },
            {
              "name": "だくりゅう",
              "type": "Water",
              "category": "Special",
              "power": 90,
              "description": "濁った水を相手に発射して攻撃する。命中率をさげることがある。"
            },
            {
              "name": "あやしいひかり",
              "type": "Ghost",
              "category": "Status",
              "power": 0,
              "description": "怪しい光を相手にみせてまどわせる。相手を混乱させる。"
            }
          ],
          "actions": [
            "時間80% かいでんぱ",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1836,
          "atk": 62,
          "def": 86,
          "sp_atk": 97,
          "sp_def": 79,
          "speed": 45
        },
        {
          "name": "バウッツェル",
          "star": 4,
          "types": [
            "Fairy"
          ],
          "abilities": [
            {
              "name": "こんがりボディ",
              "description": "ほのおタイプの技を受けるとダメージを受けずに防御がぐーんと上がる。"
            },
            {
              "name": "アロマベール",
              "description": "自分と味方へのメンタル攻撃を防ぐことができる。"
            }
          ],
          "moves": [
            {
              "name": "じゃれつく",
              "type": "Fairy",
              "category": "Physical",
              "power": 90,
              "description": "相手にじゃれついて攻撃する。相手の攻撃をさげることがある。"
            },
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "あまえる",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [
            "時間60% 強化解除",
            "体力75% 弱体解除",
            "体力40% あまえる"
          ],
          "hp": 1272,
          "atk": 77,
          "def": 108,
          "sp_atk": 50,
          "sp_def": 77,
          "speed": 90
        },
        {
          "name": "パルシェン",
          "star": 4,
          "types": [
            "Water",
            "Ice"
          ],
          "abilities": [
            {
              "name": "シェルアーマー",
              "description": "硬い殻に守られ相手の攻撃が急所に当たらない。"
            },
            {
              "name": "スキルリンク",
              "description": "連続技を使うといつも最高回数出すことができる。"
            },
            {
              "name": "ぼうじん",
              "description": "すなあらしのダメージを受けない。粉や胞子の影響も受けない。"
            }
          ],
          "moves": [
            {
              "name": "シェルブレード",
              "type": "Water",
              "category": "Physical",
              "power": 75,
              "description": "鋭い貝殻で切りつけて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "つららおとし",
              "type": "Ice",
              "category": "Physical",
              "power": 85,
              "description": "大きな氷柱を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "こわいかお",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "恐ろしい顔でにらみおびえさせて相手の素早さをがくっとさげる。"
            },
            {
              "name": "こおりのつぶて",
              "type": "Ice",
              "category": "Physical",
              "power": 40,
              "description": "氷の塊を一瞬でつくり相手に素早く放つ。必ず先制攻撃できる。"
            }
          ],
          "actions": [
            "時間40% 弱体解除",
            "体力75% からにこもる",
            "体力35% からをやぶる"
          ],
          "hp": 1200,
          "atk": 90,
          "def": 167,
          "sp_atk": 81,
          "sp_def": 45,
          "speed": 68
        },
        {
          "name": "ビークイン",
          "star": 4,
          "types": [
            "Bug",
            "Flying"
          ],
          "abilities": [
            {
              "name": "プレッシャー",
              "description": "プレッシャーをあたえて相手の使う技のＰＰを多く減らす。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            }
          ],
          "moves": [
            {
              "name": "こうげきしれい",
              "type": "Bug",
              "category": "Physical",
              "power": 90,
              "description": "しもべを呼びだして相手にむかって攻撃させる。急所に当たりやすい。"
            },
            {
              "name": "むしのさざめき",
              "type": "Bug",
              "category": "Special",
              "power": 90,
              "description": "振動で音波をおこして攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "つばめがえし",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "素早い動きで相手をほんろうして切りつける。攻撃は必ず命中する。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            }
          ],
          "actions": [
            "時間65% ぼうぎょしれい",
            "時間30% ぼうぎょしれい",
            "体力60% 弱体解除"
          ],
          "hp": 1416,
          "atk": 77,
          "def": 96,
          "sp_atk": 77,
          "sp_def": 96,
          "speed": 41
        },
        {
          "name": "フォレトス",
          "star": 4,
          "types": [
            "Bug",
            "Steel"
          ],
          "abilities": [
            {
              "name": "がんじょう",
              "description": "ＨＰが満タンのとき技を受けても一撃で倒されることがない。一撃必殺技も効かない。"
            },
            {
              "name": "ぼうじん",
              "description": "すなあらしのダメージを受けない。粉や胞子の影響も受けない。"
            }
          ],
          "moves": [
            {
              "name": "ヘビーボンバー",
              "type": "Steel",
              "category": "Physical",
              "power": 1,
              "description": "重たい体で相手にぶつかって攻撃する。自分が相手より重いほど威力があがる。"
            },
            {
              "name": "むしくい",
              "type": "Bug",
              "category": "Physical",
              "power": 60,
              "description": "かみついて攻撃する。相手がきのみを持っているとき食べてきのみの効果を受けられる。"
            },
            {
              "name": "しっぺがえし",
              "type": "Dark",
              "category": "Physical",
              "power": 50,
              "description": "ためこんで攻撃する。相手よりあとに攻撃できると技の威力は２倍になる。"
            },
            {
              "name": "でんじほう",
              "type": "Electric",
              "category": "Special",
              "power": 120,
              "description": "大砲のような電気を発射して攻撃する。相手をまひの状態にする。"
            }
          ],
          "actions": [
            "時間90% かたくなる",
            "時間40% かたくなる",
            "体力60% 強化解除"
          ],
          "hp": 1464,
          "atk": 86,
          "def": 131,
          "sp_atk": 59,
          "sp_def": 59,
          "speed": 41
        },
        {
          "name": "フリージオ",
          "star": 4,
          "types": [
            "Ice"
          ],
          "abilities": [
            {
              "name": "ふゆう",
              "description": "地面から浮くことによってじめんタイプの技を受けない。"
            }
          ],
          "moves": [
            {
              "name": "フリーズドライ",
              "type": "Ice",
              "category": "Special",
              "power": 70,
              "description": "相手を急激に冷やしてこおり状態にすることがある。みずタイプにも効果バツグンになる。"
            },
            {
              "name": "げんしのちから",
              "type": "Rock",
              "category": "Special",
              "power": 60,
              "description": "原始の力で攻撃する。自分のすべての能力があがることがある。"
            },
            {
              "name": "くろいきり",
              "type": "Ice",
              "category": "Status",
              "power": 0,
              "description": "黒い霧をだして戦闘にでているポケモン全員の能力変化をもとにもどす。"
            },
            {
              "name": "あやしいひかり",
              "type": "Ghost",
              "category": "Status",
              "power": 0,
              "description": "怪しい光を相手にみせてまどわせる。相手を混乱させる。"
            }
          ],
          "actions": [
            "時間90% ゆきげしき",
            "体力50% 弱体解除",
            "体力30% オーロラベール"
          ],
          "hp": 1524,
          "atk": 50,
          "def": 50,
          "sp_atk": 90,
          "sp_def": 126,
          "speed": 99
        },
        {
          "name": "フローゼル",
          "star": 4,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "すいすい",
              "description": "天気が雨のとき素早さが上がる。"
            },
            {
              "name": "みずのベール",
              "description": "水のベールを身にまといやけど状態にならない。"
            }
          ],
          "moves": [
            {
              "name": "アクアブレイク",
              "type": "Water",
              "category": "Physical",
              "power": 85,
              "description": "水の力で相手にぶつかって攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "ダブルアタック",
              "type": "Normal",
              "category": "Physical",
              "power": 35,
              "description": "しっぽなどを使い相手をたたいて攻撃する。２回連続でダメージを与える。"
            },
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [
            "時間80% あまごい",
            "体力75% 弱体解除",
            "体力40% 強化解除"
          ],
          "hp": 1572,
          "atk": 99,
          "def": 54,
          "sp_atk": 81,
          "sp_def": 50,
          "speed": 108
        },
        {
          "name": "フワライド",
          "star": 4,
          "types": [
            "Ghost",
            "Flying"
          ],
          "abilities": [
            {
              "name": "ゆうばく",
              "description": "ひんしになったとき触った相手にダメージをあたえる。"
            },
            {
              "name": "かるわざ",
              "description": "持っていた道具がなくなると素早さが上がる。"
            },
            {
              "name": "ねつぼうそう",
              "description": "やけど状態になったとき特殊技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "たたりめ",
              "type": "Ghost",
              "category": "Special",
              "power": 65,
              "description": "たたみかけるように攻撃する。状態異常の相手に大きなダメージを与える。"
            },
            {
              "name": "エアスラッシュ",
              "type": "Flying",
              "category": "Special",
              "power": 75,
              "description": "空をも切り裂く空気の刃で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            },
            {
              "name": "エアカッター",
              "type": "Flying",
              "category": "Special",
              "power": 60,
              "description": "鋭い風で相手を切りつけて攻撃する。急所に当たりやすい。"
            }
          ],
          "actions": [
            "時間75% 弱体解除",
            "時間40% 強化解除",
            "体力75% おにび"
          ],
          "hp": 2280,
          "atk": 77,
          "def": 44,
          "sp_atk": 86,
          "sp_def": 53,
          "speed": 77
        },
        {
          "name": "ブーピッグ",
          "star": 4,
          "types": [
            "Psychic"
          ],
          "abilities": [
            {
              "name": "あついしぼう",
              "description": "厚い脂肪で守られているのでほのおタイプとこおりタイプの技のダメージを半減させる。"
            },
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "くいしんぼう",
              "description": "ＨＰが少なくなったら食べるきのみをＨＰ半分の時に食べてしまう。"
            }
          ],
          "moves": [
            {
              "name": "サイコショック",
              "type": "Psychic",
              "category": "Special",
              "power": 80,
              "description": "不思議な念波を実体化して相手を攻撃する。物理的なダメージを与える。"
            },
            {
              "name": "パワージェム",
              "type": "Rock",
              "category": "Special",
              "power": 80,
              "description": "宝石のようにきらめく光を発射して相手を攻撃する。"
            },
            {
              "name": "あやしいひかり",
              "type": "Ghost",
              "category": "Status",
              "power": 0,
              "description": "怪しい光を相手にみせてまどわせる。相手を混乱させる。"
            },
            {
              "name": "けたぐり",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "足を強くけり相手を転ばせて攻撃する。相手が重いほど威力があがる。"
            }
          ],
          "actions": [
            "時間40% 強化解除",
            "体力75% ひかりのかべ",
            "体力60% 弱体解除"
          ],
          "hp": 1524,
          "atk": 45,
          "def": 63,
          "sp_atk": 86,
          "sp_def": 104,
          "speed": 77
        },
        {
          "name": "プクリン",
          "star": 4,
          "types": [
            "Normal",
            "Fairy"
          ],
          "abilities": [
            {
              "name": "メロメロボディ",
              "description": "自分に触った相手をメロメロにすることがある。"
            },
            {
              "name": "かちき",
              "description": "相手に能力を下げられると特攻がぐーんと上がる。"
            },
            {
              "name": "おみとおし",
              "description": "登場したとき相手の持ち物を見通すことができる。"
            }
          ],
          "moves": [
            {
              "name": "マジカルシャイン",
              "type": "Fairy",
              "category": "Special",
              "power": 80,
              "description": "強力な光を放ち相手にダメージを与える。"
            },
            {
              "name": "ハイパーボイス",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "うるさく響く大きな振動を相手に与えて攻撃する。"
            },
            {
              "name": "あまえる",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。"
            },
            {
              "name": "アンコール",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手にアンコールした技を３回続けて出させる。"
            }
          ],
          "actions": [
            "時間70% うたう",
            "時間60% 強化解除",
            "体力30% うたう"
          ],
          "hp": 2172,
          "atk": 68,
          "def": 45,
          "sp_atk": 81,
          "sp_def": 50,
          "speed": 45
        },
        {
          "name": "ヘラクロス",
          "star": 4,
          "types": [
            "Bug",
            "Fighting"
          ],
          "abilities": [
            {
              "name": "むしのしらせ",
              "description": "ＨＰが減ったときむしタイプの技の威力が上がる。"
            },
            {
              "name": "こんじょう",
              "description": "状態異常になると根性をだして攻撃が上がる。"
            },
            {
              "name": "じしんかじょう",
              "description": "相手を倒すと自信がついて攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "かわらわり",
              "type": "Fighting",
              "category": "Physical",
              "power": 75,
              "description": "手刀を勢いよく振りおろして相手を攻撃する。ひかりのかべやリフレクターなども破壊できる。"
            },
            {
              "name": "じごくづき",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "この技を受けた相手は地獄の苦しみから２ターンの間音の技を出すことができなくなる。"
            },
            {
              "name": "からげんき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "自分が毒まひやけど状態のとき相手にくりだすと技の威力が２倍になる。"
            },
            {
              "name": "とびつく",
              "type": "Bug",
              "category": "Physical",
              "power": 50,
              "description": "相手に飛びついて攻撃する。相手の素早さをさげる。"
            }
          ],
          "actions": [
            "時間40% 強化解除",
            "体力75% 弱体解除",
            "体力40% つるぎのまい"
          ],
          "hp": 1524,
          "atk": 117,
          "def": 72,
          "sp_atk": 41,
          "sp_def": 90,
          "speed": 81
        },
        {
          "name": "ベトベトン",
          "star": 4,
          "types": [
            "Poison"
          ],
          "abilities": [
            {
              "name": "あくしゅう",
              "description": "臭いにおいを放つことによって攻撃したときに相手をひるませることがある。"
            },
            {
              "name": "ねんちゃく",
              "description": "粘着質の体に道具がくっついているため相手に道具を奪われない。"
            },
            {
              "name": "どくしゅ",
              "description": "触るだけで相手をどく状態にすることがある。"
            }
          ],
          "moves": [
            {
              "name": "ヘドロばくだん",
              "type": "Poison",
              "category": "Special",
              "power": 90,
              "description": "汚いヘドロを相手に投げつけて攻撃する。毒状態にすることがある。"
            },
            {
              "name": "ほのおのパンチ",
              "type": "Fire",
              "category": "Physical",
              "power": 75,
              "description": "炎をこめたパンチで相手を攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "どくどく",
              "type": "Poison",
              "category": "Status",
              "power": 0,
              "description": "相手を猛毒の状態にする。ターンがすすむほど毒のダメージが増えていく。"
            },
            {
              "name": "かなしばり",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手の動きをとめて直前にだしていた技を４ターンの間使えなくする。"
            }
          ],
          "actions": [
            "時間40% 強化解除",
            "体力75% かたくなる",
            "体力60% 弱体解除"
          ],
          "hp": 1788,
          "atk": 99,
          "def": 72,
          "sp_atk": 63,
          "sp_def": 95,
          "speed": 50
        },
        {
          "name": "ベラカス",
          "star": 4,
          "types": [
            "Bug",
            "Psychic"
          ],
          "abilities": [
            {
              "name": "シンクロ",
              "description": "自分がなってしまったどくやまひややけどを相手にうつす。"
            },
            {
              "name": "テレパシー",
              "description": "味方の攻撃を読み取って技を回避する。"
            }
          ],
          "moves": [
            {
              "name": "むしのさざめき",
              "type": "Bug",
              "category": "Special",
              "power": 90,
              "description": "振動で音波をおこして攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "じんつうりき",
              "type": "Psychic",
              "category": "Special",
              "power": 80,
              "description": "みえない不思議な力を送って攻撃する。相手をひるませることがある。"
            },
            {
              "name": "すなかけ",
              "type": "Ground",
              "category": "Status",
              "power": 0,
              "description": "相手の顔に砂をかけて命中率をさげる。"
            },
            {
              "name": "むしのていこう",
              "type": "Bug",
              "category": "Special",
              "power": 50,
              "description": "抵抗して相手を攻撃する。相手の特攻をさげる。"
            }
          ],
          "actions": [
            "時間75% サイコフィールド",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1464,
          "atk": 50,
          "def": 81,
          "sp_atk": 108,
          "sp_def": 95,
          "speed": 45
        },
        {
          "name": "ペルシアン",
          "star": 4,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "じゅうなん",
              "description": "柔軟な体によってまひ状態にならない。"
            },
            {
              "name": "テクニシャン",
              "description": "威力が低い技の威力を高くして攻撃できる。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            }
          ],
          "moves": [
            {
              "name": "きりさく",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "パワージェム",
              "type": "Rock",
              "category": "Special",
              "power": 80,
              "description": "宝石のようにきらめく光を発射して相手を攻撃する。"
            },
            {
              "name": "いやなおと",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "おもわず耳をふさぎたくなるいやなおとをだして相手の防御をがくっとさげる。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            }
          ],
          "actions": [
            "時間50% 強化解除",
            "体力90% バークアウト",
            "体力40% 弱体解除"
          ],
          "hp": 1356,
          "atk": 68,
          "def": 59,
          "sp_atk": 63,
          "sp_def": 63,
          "speed": 108
        },
        {
          "name": "ポットデス",
          "star": 4,
          "types": [
            "Ghost"
          ],
          "abilities": [
            {
              "name": "くだけるよろい",
              "description": "物理技でダメージを受けると防御が下がり素早さがぐーんと上がる。"
            },
            {
              "name": "のろわれボディ",
              "description": "攻撃を受けると相手の技をかなしばり状態にすることがある。"
            }
          ],
          "moves": [
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "メガドレイン",
              "type": "Grass",
              "category": "Special",
              "power": 40,
              "description": "養分を吸い取り攻撃する。相手に与えたダメージの半分のＨＰを回復できる。"
            },
            {
              "name": "おどろかす",
              "type": "Ghost",
              "category": "Physical",
              "power": 30,
              "description": "大きな声などで不意に驚かして攻撃する。相手をひるませることがある。"
            },
            {
              "name": "おにび",
              "type": "Fire",
              "category": "Status",
              "power": 0,
              "description": "不気味で怪しい炎を放って相手をやけどの状態にする。"
            }
          ],
          "actions": [
            "時間40% 弱体解除",
            "体力75% からにこもる",
            "体力35% からをやぶる"
          ],
          "hp": 1308,
          "atk": 63,
          "def": 63,
          "sp_atk": 125,
          "sp_def": 107,
          "speed": 68
        },
        {
          "name": "ママンボウ",
          "star": 4,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "いやしのこころ",
              "description": "状態異常の味方をたまに治してあげる。"
            },
            {
              "name": "うるおいボディ",
              "description": "天気が雨のとき状態異常が治る。"
            },
            {
              "name": "さいせいりょく",
              "description": "手持ちに引っ込むとＨＰが少し回復する。"
            }
          ],
          "moves": [
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "アクアジェット",
              "type": "Water",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "しおみず",
              "type": "Water",
              "category": "Special",
              "power": 65,
              "description": "相手がＨＰの半分くらいきずをおっていると技の威力が２倍になる。"
            },
            {
              "name": "しろいきり",
              "type": "Ice",
              "category": "Status",
              "power": 0,
              "description": "白い霧で体をおおう。５ターンの間相手に能力をさげられなくなる。"
            }
          ],
          "actions": [
            "時間75% 弱体解除",
            "時間40% 強化解除",
            "体力75% あまごい"
          ],
          "hp": 2436,
          "atk": 72,
          "def": 77,
          "sp_atk": 41,
          "sp_def": 45,
          "speed": 63
        },
        {
          "name": "マリルリ",
          "star": 4,
          "types": [
            "Water",
            "Fairy"
          ],
          "abilities": [
            {
              "name": "あついしぼう",
              "description": "厚い脂肪で守られているのでほのおタイプとこおりタイプの技のダメージを半減させる。"
            },
            {
              "name": "ちからもち",
              "description": "物理攻撃の威力が２倍になる。"
            },
            {
              "name": "そうしょく",
              "description": "くさタイプの技を受けるとダメージを受けずに攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "アクアテール",
              "type": "Water",
              "category": "Physical",
              "power": 90,
              "description": "激しくあれくるう荒波のように大きなしっぽをふって相手を攻撃する。"
            },
            {
              "name": "れいとうパンチ",
              "type": "Ice",
              "category": "Physical",
              "power": 75,
              "description": "冷気をこめたパンチで相手を攻撃する。こおり状態にすることがある。"
            },
            {
              "name": "すてみタックル",
              "type": "Normal",
              "category": "Physical",
              "power": 120,
              "description": "命を懸けて相手に突進して攻撃する。自分もかなりダメージを受ける。"
            },
            {
              "name": "アンコール",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手にアンコールした技を３回続けて出させる。"
            }
          ],
          "actions": [
            "時間75% あまごい",
            "体力75% 弱体解除",
            "体力40% 強化解除"
          ],
          "hp": 1740,
          "atk": 50,
          "def": 77,
          "sp_atk": 59,
          "sp_def": 77,
          "speed": 50
        },
        {
          "name": "マルノーム",
          "star": 4,
          "types": [
            "Poison"
          ],
          "abilities": [
            {
              "name": "ヘドロえき",
              "description": "ヘドロ液を吸い取った相手は強烈な悪臭でダメージを受けてＨＰを減らす。"
            },
            {
              "name": "ねんちゃく",
              "description": "粘着質の体に道具がくっついているため相手に道具を奪われない。"
            },
            {
              "name": "くいしんぼう",
              "description": "ＨＰが少なくなったら食べるきのみをＨＰ半分の時に食べてしまう。"
            }
          ],
          "moves": [
            {
              "name": "ヘドロばくだん",
              "type": "Poison",
              "category": "Special",
              "power": 90,
              "description": "汚いヘドロを相手に投げつけて攻撃する。毒状態にすることがある。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            },
            {
              "name": "アンコール",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手にアンコールした技を３回続けて出させる。"
            },
            {
              "name": "のしかかり",
              "type": "Normal",
              "category": "Physical",
              "power": 85,
              "description": "全身で相手にのしかかり攻撃する。まひ状態にすることがある。"
            }
          ],
          "actions": [
            "時間90% たくわえる",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1740,
          "atk": 70,
          "def": 79,
          "sp_atk": 70,
          "sp_def": 79,
          "speed": 54
        },
        {
          "name": "マルマイン",
          "star": 4,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "ぼうおん",
              "description": "音を遮断することによって音の技を受けない。"
            },
            {
              "name": "せいでんき",
              "description": "静電気を体にまとい触った相手をまひさせることがある。"
            },
            {
              "name": "ゆうばく",
              "description": "ひんしになったとき触った相手にダメージをあたえる。"
            }
          ],
          "moves": [
            {
              "name": "ほうでん",
              "type": "Electric",
              "category": "Special",
              "power": 80,
              "description": "まばゆい電撃で自分の周りにいるものを攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "スピードスター",
              "type": "Normal",
              "category": "Special",
              "power": 60,
              "description": "星型の光を発射して相手を攻撃する。攻撃は必ず命中する。"
            },
            {
              "name": "イカサマ",
              "type": "Dark",
              "category": "Physical",
              "power": 95,
              "description": "相手の力を利用する。戦っている相手の攻撃が高いほどダメージがあがる。"
            },
            {
              "name": "でんじふゆう",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "電気でつくった磁力の力で宙に浮かぶ。５ターンの間浮遊できる。"
            }
          ],
          "actions": [
            "時間80% ひかりのかべ",
            "時間40% 強化解除",
            "体力60% 弱体解除"
          ],
          "hp": 1308,
          "atk": 50,
          "def": 68,
          "sp_atk": 77,
          "sp_def": 77,
          "speed": 140
        },
        {
          "name": "ミガルーサ",
          "star": 4,
          "types": [
            "Water",
            "Psychic"
          ],
          "abilities": [
            {
              "name": "かたやぶり",
              "description": "相手の特性にジャマされることなく相手に技を出すことができる。"
            },
            {
              "name": "きれあじ",
              "description": "相手を切る技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "アクアカッター",
              "type": "Water",
              "category": "Physical",
              "power": 70,
              "description": "加圧された水を刃のように噴射して相手を切り裂く。急所に当たりやすい。"
            },
            {
              "name": "サイコカッター",
              "type": "Psychic",
              "category": "Physical",
              "power": 70,
              "description": "実体化させた心の刃で相手を切り裂く。急所に当たりやすい。"
            },
            {
              "name": "きりさく",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "アクアジェット",
              "type": "Water",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            }
          ],
          "actions": [
            "時間75% 弱体解除",
            "体力35% みをけずる",
            "体力30% 弱体解除"
          ],
          "hp": 1632,
          "atk": 96,
          "def": 70,
          "sp_atk": 75,
          "sp_def": 63,
          "speed": 68
        },
        {
          "name": "メタモン",
          "star": 4,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "じゅうなん",
              "description": "柔軟な体によってまひ状態にならない。"
            },
            {
              "name": "かわりもの",
              "description": "目の前のポケモンに変身してしまう。"
            }
          ],
          "moves": [
            {
              "name": "へんしん",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手のポケモンに変身することで相手とまったく同じ技が使える。"
            }
          ],
          "actions": [
            "時間40% 強化解除",
            "体力90% 弱体解除",
            "体力75% 強化解除"
          ],
          "hp": 1176,
          "atk": 48,
          "def": 48,
          "sp_atk": 48,
          "sp_def": 48,
          "speed": 48
        },
        {
          "name": "メブキジカ",
          "star": 4,
          "types": [
            "Normal",
            "Grass"
          ],
          "abilities": [
            {
              "name": "ようりょくそ",
              "description": "天気が晴れのとき素早さが上がる。"
            },
            {
              "name": "そうしょく",
              "description": "くさタイプの技を受けるとダメージを受けずに攻撃が上がる。"
            },
            {
              "name": "てんのめぐみ",
              "description": "天の恵みのおかげで技の追加効果がでやすい。"
            }
          ],
          "moves": [
            {
              "name": "エナジーボール",
              "type": "Grass",
              "category": "Special",
              "power": 90,
              "description": "自然から集めた命の力を発射する。相手の特防をさげることがある。"
            },
            {
              "name": "すてみタックル",
              "type": "Normal",
              "category": "Physical",
              "power": 120,
              "description": "命を懸けて相手に突進して攻撃する。自分もかなりダメージを受ける。"
            },
            {
              "name": "にどげり",
              "type": "Fighting",
              "category": "Physical",
              "power": 30,
              "description": "２本の足で相手をけとばして攻撃する。２回連続でダメージを与える。"
            },
            {
              "name": "あまえる",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。"
            }
          ],
          "actions": [
            "時間80% にほんばれ",
            "体力75% 弱体解除",
            "体力40% 強化解除"
          ],
          "hp": 1524,
          "atk": 95,
          "def": 68,
          "sp_atk": 59,
          "sp_def": 68,
          "speed": 90
        },
        {
          "name": "モスノウ",
          "star": 4,
          "types": [
            "Ice",
            "Bug"
          ],
          "abilities": [
            {
              "name": "りんぷん",
              "description": "りんぷんに守られて技の追加効果を受けなくなる。"
            },
            {
              "name": "こおりのりんぷん",
              "description": "こおりのりんぷんに守られて特殊攻撃で受けるダメージが半減する。"
            }
          ],
          "moves": [
            {
              "name": "むしのさざめき",
              "type": "Bug",
              "category": "Special",
              "power": 90,
              "description": "振動で音波をおこして攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "しびれごな",
              "type": "Grass",
              "category": "Status",
              "power": 0,
              "description": "しびれる粉をたくさんふりまいて相手をまひ状態にする。"
            },
            {
              "name": "しろいきり",
              "type": "Ice",
              "category": "Status",
              "power": 0,
              "description": "白い霧で体をおおう。５ターンの間相手に能力をさげられなくなる。"
            },
            {
              "name": "オーロラビーム",
              "type": "Ice",
              "category": "Special",
              "power": 65,
              "description": "にじいろのビームを相手に発射して攻撃する。攻撃をさげることがある。"
            }
          ],
          "actions": [
            "時間90% ゆきげしき",
            "体力75% 弱体解除",
            "体力50% ちょうのまい"
          ],
          "hp": 1416,
          "atk": 63,
          "def": 59,
          "sp_atk": 117,
          "sp_def": 86,
          "speed": 63
        },
        {
          "name": "モトトカゲ",
          "star": 4,
          "types": [
            "Dragon",
            "Normal"
          ],
          "abilities": [
            {
              "name": "だっぴ",
              "description": "体の皮を脱ぎ捨てることで状態異常を治すことがある。"
            },
            {
              "name": "さいせいりょく",
              "description": "手持ちに引っ込むとＨＰが少し回復する。"
            }
          ],
          "moves": [
            {
              "name": "ドラゴンクロー",
              "type": "Dragon",
              "category": "Physical",
              "power": 80,
              "description": "鋭くとがった巨大なツメで相手を切り裂いて攻撃する。"
            },
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "すてみタックル",
              "type": "Normal",
              "category": "Physical",
              "power": 120,
              "description": "命を懸けて相手に突進して攻撃する。自分もかなりダメージを受ける。"
            },
            {
              "name": "でんこうせっか",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            }
          ],
          "actions": [
            "時間40% 強化解除",
            "時間40% ギアチェンジ",
            "体力75% 弱体解除"
          ],
          "hp": 1416,
          "atk": 90,
          "def": 63,
          "sp_atk": 81,
          "sp_def": 63,
          "speed": 113
        },
        {
          "name": "ヤルキモノ",
          "star": 4,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "やるき",
              "description": "やる気をだすことによってねむり状態にならない。"
            }
          ],
          "moves": [
            {
              "name": "からげんき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "自分が毒まひやけど状態のとき相手にくりだすと技の威力が２倍になる。"
            },
            {
              "name": "きりさく",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "シャドークロー",
              "type": "Ghost",
              "category": "Physical",
              "power": 70,
              "description": "影からつくった鋭いツメで相手を切り裂く。急所に当たりやすい。"
            },
            {
              "name": "きあいだめ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "深く息を吸い気合をこめる。自分の攻撃が急所に当たりやすくなる。"
            }
          ],
          "actions": [
            "時間40% 強化解除",
            "体力75% 弱体解除",
            "体力75% あくび"
          ],
          "hp": 1524,
          "atk": 77,
          "def": 77,
          "sp_atk": 54,
          "sp_def": 54,
          "speed": 86
        },
        {
          "name": "ライチュウ",
          "star": 4,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "せいでんき",
              "description": "静電気を体にまとい触った相手をまひさせることがある。"
            },
            {
              "name": "ひらいしん",
              "description": "でんきタイプの技を自分に寄せつけダメージを受けずに特攻が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ほうでん",
              "type": "Electric",
              "category": "Special",
              "power": 80,
              "description": "まばゆい電撃で自分の周りにいるものを攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "アイアンテール",
              "type": "Steel",
              "category": "Physical",
              "power": 100,
              "description": "硬いしっぽで相手をたたきつけて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "あまえる",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。"
            },
            {
              "name": "ほっぺすりすり",
              "type": "Electric",
              "category": "Physical",
              "power": 20,
              "description": "電気を帯びたほっぺをすりつけて攻撃。相手をまひ状態にする。"
            }
          ],
          "actions": [
            "時間85% エレキフィールド",
            "体力75% 弱体解除",
            "体力25% わるだくみ"
          ],
          "hp": 1308,
          "atk": 86,
          "def": 54,
          "sp_atk": 86,
          "sp_def": 77,
          "speed": 104
        },
        {
          "name": "ラランテス",
          "star": 4,
          "types": [
            "Grass"
          ],
          "abilities": [
            {
              "name": "リーフガード",
              "description": "天気が晴れのときは状態異常にならない。"
            },
            {
              "name": "あまのじゃく",
              "description": "能力の変化が逆転して上がるときに下がり下がるときに上がる。"
            }
          ],
          "moves": [
            {
              "name": "リーフブレード",
              "type": "Grass",
              "category": "Physical",
              "power": 90,
              "description": "はっぱを剣のようにあやつり相手を切りつけて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "シザークロス",
              "type": "Bug",
              "category": "Physical",
              "power": 80,
              "description": "カマやツメをハサミのように交差させながら相手を切り裂く。"
            },
            {
              "name": "こわいかお",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "恐ろしい顔でにらみおびえさせて相手の素早さをがくっとさげる。"
            },
            {
              "name": "きりさく",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。"
            }
          ],
          "actions": [
            "時間75% 弱体解除",
            "時間40% 強化解除",
            "体力75% せいちょう"
          ],
          "hp": 1416,
          "atk": 99,
          "def": 86,
          "sp_atk": 77,
          "sp_def": 86,
          "speed": 45
        },
        {
          "name": "リキキリン",
          "star": 4,
          "types": [
            "Normal",
            "Psychic"
          ],
          "abilities": [
            {
              "name": "はんすう",
              "description": "きのみを食べると次のターンの終わりに胃から出してもう１回だけ食べる。"
            },
            {
              "name": "テイルアーマー",
              "description": "頭を包む謎のしっぽがこちらにむかって先制技を出せないようにする。"
            },
            {
              "name": "そうしょく",
              "description": "くさタイプの技を受けるとダメージを受けずに攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ツインビーム",
              "type": "Psychic",
              "category": "Special",
              "power": 40,
              "description": "両目から不可思議な光線を発射して攻撃する。２回連続でダメージを与える。"
            },
            {
              "name": "ふみつけ",
              "type": "Normal",
              "category": "Physical",
              "power": 65,
              "description": "大きな足で相手を踏みつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            },
            {
              "name": "サイコキネシス",
              "type": "Psychic",
              "category": "Special",
              "power": 90,
              "description": "強い念力を相手に送って攻撃する。相手の特防をさげることがある。"
            }
          ],
          "actions": [
            "時間75% めいそう",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1956,
          "atk": 86,
          "def": 68,
          "sp_atk": 104,
          "sp_def": 68,
          "speed": 59
        },
        {
          "name": "リククラゲ",
          "star": 4,
          "types": [
            "Ground",
            "Grass"
          ],
          "abilities": [
            {
              "name": "きんしのちから",
              "description": "変化技を出すとき必ず行動が遅くなるが相手の特性にジャマされない。"
            }
          ],
          "moves": [
            {
              "name": "だいちのちから",
              "type": "Ground",
              "category": "Special",
              "power": 90,
              "description": "相手の足下へ大地の力を放出する。相手の特防をさげることがある。"
            },
            {
              "name": "タネばくだん",
              "type": "Grass",
              "category": "Physical",
              "power": 80,
              "description": "硬い殻をもつ大きなタネを上からたたきつけて相手を攻撃する。"
            },
            {
              "name": "しびれごな",
              "type": "Grass",
              "category": "Status",
              "power": 0,
              "description": "しびれる粉をたくさんふりまいて相手をまひ状態にする。"
            },
            {
              "name": "イカサマ",
              "type": "Dark",
              "category": "Physical",
              "power": 95,
              "description": "相手の力を利用する。戦っている相手の攻撃が高いほどダメージがあがる。"
            }
          ],
          "actions": [
            "時間75% 弱体解除",
            "時間40% 強化解除",
            "体力75% せいちょう"
          ],
          "hp": 1524,
          "atk": 68,
          "def": 63,
          "sp_atk": 77,
          "sp_def": 113,
          "speed": 95
        },
        {
          "name": "リングマ",
          "star": 4,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "こんじょう",
              "description": "状態異常になると根性をだして攻撃が上がる。"
            },
            {
              "name": "はやあし",
              "description": "状態異常になると素早さが上がる。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            }
          ],
          "moves": [
            {
              "name": "からげんき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "自分が毒まひやけど状態のとき相手にくりだすと技の威力が２倍になる。"
            },
            {
              "name": "きりさく",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "じゃれつく",
              "type": "Fairy",
              "category": "Physical",
              "power": 90,
              "description": "相手にじゃれついて攻撃する。相手の攻撃をさげることがある。"
            },
            {
              "name": "にらみつける",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "鋭い目つきでおびえさせて相手の防御をさげる。"
            }
          ],
          "actions": [
            "時間90% くさわけ",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1632,
          "atk": 122,
          "def": 72,
          "sp_atk": 72,
          "sp_def": 72,
          "speed": 54
        },
        {
          "name": "ルガルガン",
          "star": 4,
          "types": [
            "Rock"
          ],
          "abilities": [
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            },
            {
              "name": "すなかき",
              "description": "天気がすなあらしのとき素早さが上がる。"
            },
            {
              "name": "ふくつのこころ",
              "description": "ひるむたびに不屈の心を燃やして素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "いわなだれ",
              "type": "Rock",
              "category": "Physical",
              "power": 75,
              "description": "大きな岩を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "カウンター",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "相手から受けた物理攻撃のダメージを２倍にして同じ相手に返す。"
            },
            {
              "name": "がむしゃら",
              "type": "Normal",
              "category": "Physical",
              "power": 1,
              "description": "相手のＨＰが自分のＨＰと同じくらいになるようにダメージを与える。"
            }
          ],
          "actions": [
            "時間90% すなあらし",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1464,
          "atk": 108,
          "def": 63,
          "sp_atk": 54,
          "sp_def": 63,
          "speed": 105
        },
        {
          "name": "ルガルガン",
          "star": 4,
          "types": [],
          "abilities": [
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            },
            {
              "name": "すなかき",
              "description": "天気がすなあらしのとき素早さが上がる。"
            },
            {
              "name": "ふくつのこころ",
              "description": "ひるむたびに不屈の心を燃やして素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "いわなだれ",
              "type": "Rock",
              "category": "Physical",
              "power": 75,
              "description": "大きな岩を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "アクセルロック",
              "type": "Rock",
              "category": "Physical",
              "power": 40,
              "description": "素早いスピードで相手にぶつかって攻撃する。必ず先制攻撃できる。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            }
          ],
          "actions": [
            "時間90% すなあらし",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 0,
          "atk": 0,
          "def": 0,
          "sp_atk": 0,
          "sp_def": 0,
          "speed": 0
        },
        {
          "name": "ルチャブル",
          "star": 4,
          "types": [
            "Fighting",
            "Flying"
          ],
          "abilities": [
            {
              "name": "じゅうなん",
              "description": "柔軟な体によってまひ状態にならない。"
            },
            {
              "name": "かるわざ",
              "description": "持っていた道具がなくなると素早さが上がる。"
            },
            {
              "name": "かたやぶり",
              "description": "相手の特性にジャマされることなく相手に技を出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "かわらわり",
              "type": "Fighting",
              "category": "Physical",
              "power": 75,
              "description": "手刀を勢いよく振りおろして相手を攻撃する。ひかりのかべやリフレクターなども破壊できる。"
            },
            {
              "name": "フライングプレス",
              "type": "Fighting",
              "category": "Physical",
              "power": 100,
              "description": "空中から相手にダイブする。この技はかくとうタイプと同時にひこうタイプでもある。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            },
            {
              "name": "フェザーダンス",
              "type": "Flying",
              "category": "Status",
              "power": 0,
              "description": "羽毛をふりまいて相手の体にからませる。相手の攻撃をがくっとさげる。"
            }
          ],
          "actions": [
            "時間40% 強化解除",
            "体力75% 弱体解除",
            "体力40% つるぎのまい"
          ],
          "hp": 1500,
          "atk": 87,
          "def": 72,
          "sp_atk": 71,
          "sp_def": 61,
          "speed": 111
        },
        {
          "name": "レアコイル",
          "star": 4,
          "types": [
            "Electric",
            "Steel"
          ],
          "abilities": [
            {
              "name": "じりょく",
              "description": "はがねタイプのポケモンを磁力で引きつけて逃げられなくする。"
            },
            {
              "name": "がんじょう",
              "description": "ＨＰが満タンのとき技を受けても一撃で倒されることがない。一撃必殺技も効かない。"
            },
            {
              "name": "アナライズ",
              "description": "いちばん最後に技を出すと技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ほうでん",
              "type": "Electric",
              "category": "Special",
              "power": 80,
              "description": "まばゆい電撃で自分の周りにいるものを攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "ラスターカノン",
              "type": "Steel",
              "category": "Special",
              "power": 80,
              "description": "体の光を一点に集めて力を放つ。相手の特防をさげることがある。"
            },
            {
              "name": "きんぞくおん",
              "type": "Steel",
              "category": "Status",
              "power": 0,
              "description": "金属をこすってでるようないやな音を聞かせる。相手の特防をがくっとさげる。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            }
          ],
          "actions": [
            "時間75% エレキフィールド",
            "時間40% 強化解除",
            "体力75% 弱体解除"
          ],
          "hp": 1200,
          "atk": 59,
          "def": 90,
          "sp_atk": 113,
          "sp_def": 68,
          "speed": 68
        },
        {
          "name": "ロトム",
          "star": 4,
          "types": [
            "Electric",
            "Ghost"
          ],
          "abilities": [
            {
              "name": "ふゆう",
              "description": "地面から浮くことによってじめんタイプの技を受けない。"
            }
          ],
          "moves": [
            {
              "name": "ほうでん",
              "type": "Electric",
              "category": "Special",
              "power": 80,
              "description": "まばゆい電撃で自分の周りにいるものを攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "さわぐ",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "３ターンの間騒いで相手を攻撃する。そのあいだはだれも眠れなくなる。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            }
          ],
          "actions": [
            "時間40% 強化解除",
            "体力75% 弱体解除",
            "体力45% わるだくみ"
          ],
          "hp": 1200,
          "atk": 50,
          "def": 74,
          "sp_atk": 90,
          "sp_def": 74,
          "speed": 86
        },
        {
          "name": "ワルビル",
          "star": 4,
          "types": [
            "Ground",
            "Dark"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "じしんかじょう",
              "description": "相手を倒すと自信がついて攻撃が上がる。"
            },
            {
              "name": "いかりのつぼ",
              "description": "急所に攻撃が当たると怒りくるって攻撃力が最大になる。"
            }
          ],
          "moves": [
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "じしん",
              "type": "Ground",
              "category": "Physical",
              "power": 100,
              "description": "地震の衝撃で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "いちゃもん",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手にいちゃもんをつけて同じ技を２回連続でだせなくする。"
            },
            {
              "name": "いばる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせて混乱させる。怒りで相手の攻撃はぐーんとあがってしまう。"
            }
          ],
          "actions": [
            "時間40% 強化解除",
            "体力75% 弱体解除",
            "体力75% いやなおと"
          ],
          "hp": 1308,
          "atk": 78,
          "def": 45,
          "sp_atk": 45,
          "sp_def": 45,
          "speed": 71
        },
        {
          "name": "アップリュー",
          "star": 5,
          "types": [
            "Grass",
            "Dragon"
          ],
          "abilities": [
            {
              "name": "じゅくせい",
              "description": "熟成させることできのみの効果が倍になる。"
            },
            {
              "name": "くいしんぼう",
              "description": "ＨＰが少なくなったら食べるきのみをＨＰ半分の時に食べてしまう。"
            },
            {
              "name": "はりきり",
              "description": "自分の攻撃が高くなるが命中率が下がる。"
            }
          ],
          "moves": [
            {
              "name": "Ｇのちから",
              "type": "Grass",
              "category": "Physical",
              "power": 80,
              "description": "高いところからりんごを落としてダメージを与える。相手の防御を下げる。"
            },
            {
              "name": "りゅうのいぶき",
              "type": "Dragon",
              "category": "Special",
              "power": 60,
              "description": "ものすごい息を相手に吹きつけて攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "ドラゴンダイブ",
              "type": "Dragon",
              "category": "Physical",
              "power": 100,
              "description": "すさまじい殺気で威圧しながら体当たりする。相手をひるませることがある。"
            },
            {
              "name": "くさわけ",
              "type": "Grass",
              "category": "Physical",
              "power": 50,
              "description": "草むらから飛びだすように攻撃する。軽快な足どりによって自分の素早さをあげる。"
            }
          ],
          "actions": [
            "時間85% グラスフィールド",
            "体力75% てっぺき",
            "体力50% 弱体解除",
            "体力40% りゅうのまい",
            "体力20% 強化解除"
          ],
          "hp": 3800,
          "atk": 170,
          "def": 125,
          "sp_atk": 147,
          "sp_def": 95,
          "speed": 110
        },
        {
          "name": "アノホラグサ",
          "star": 5,
          "types": [
            "Grass",
            "Ghost"
          ],
          "abilities": [
            {
              "name": "かぜのり",
              "description": "おいかぜが吹いたり風技を受けるとダメージを受けずに攻撃が上がる。"
            },
            {
              "name": "すりぬけ",
              "description": "相手の壁や身代わりをすりぬけて攻撃できる。"
            }
          ],
          "moves": [
            {
              "name": "ギガドレイン",
              "type": "Grass",
              "category": "Special",
              "power": 75,
              "description": "養分を吸い取り攻撃する。与えたダメージの半分のＨＰを回復できる。"
            },
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "パワーウィップ",
              "type": "Grass",
              "category": "Physical",
              "power": 120,
              "description": "ツタや触手を激しくふるって相手をたたきつけ攻撃する。"
            },
            {
              "name": "まとわりつく",
              "type": "Bug",
              "category": "Special",
              "power": 20,
              "description": "４ー５ターンの間相手にまとわりついて攻撃する。そのあいだ相手は逃げられない。"
            }
          ],
          "actions": [
            "時間80% グラスフィールド",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力50% テラ回収",
            "体力20% グラスフィールド"
          ],
          "hp": 3340,
          "atk": 177,
          "def": 110,
          "sp_atk": 125,
          "sp_def": 110,
          "speed": 140
        },
        {
          "name": "アマージョ",
          "star": 5,
          "types": [
            "Grass"
          ],
          "abilities": [
            {
              "name": "リーフガード",
              "description": "天気が晴れのときは状態異常にならない。"
            },
            {
              "name": "じょおうのいげん",
              "description": "相手に威圧感をあたえこちらにむかって先制技を出せないようにする。"
            },
            {
              "name": "スイートベール",
              "description": "自分と味方のポケモンは眠らなくなる。"
            }
          ],
          "moves": [
            {
              "name": "とびひざげり",
              "type": "Fighting",
              "category": "Physical",
              "power": 130,
              "description": "ジャンプからのひざげりで相手を攻撃する。はずすと自分がダメージを受ける。"
            },
            {
              "name": "パワーウィップ",
              "type": "Grass",
              "category": "Physical",
              "power": 120,
              "description": "ツタや触手を激しくふるって相手をたたきつけ攻撃する。"
            },
            {
              "name": "ふみつけ",
              "type": "Normal",
              "category": "Physical",
              "power": 65,
              "description": "大きな足で相手を踏みつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "トロピカルキック",
              "type": "Grass",
              "category": "Physical",
              "power": 70,
              "description": "南国由来の熱いキックを相手に浴びせる。相手の攻撃をさげる。"
            }
          ],
          "actions": [
            "時間85% リフレクター",
            "体力75% グラスフィールド",
            "体力50% 弱体解除",
            "体力40% 強化解除",
            "体力25% グラスフィールド"
          ],
          "hp": 3860,
          "atk": 185,
          "def": 152,
          "sp_atk": 80,
          "sp_def": 152,
          "speed": 113
        },
        {
          "name": "アーマーガア",
          "star": 5,
          "types": [
            "Flying",
            "Steel"
          ],
          "abilities": [
            {
              "name": "プレッシャー",
              "description": "プレッシャーをあたえて相手の使う技のＰＰを多く減らす。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            },
            {
              "name": "ミラーアーマー",
              "description": "自分が受けた能力ダウンの効果だけを跳ね返す。"
            }
          ],
          "moves": [
            {
              "name": "はがねのつばさ",
              "type": "Steel",
              "category": "Physical",
              "power": 70,
              "description": "硬い翼を相手にたたきつけて攻撃する。自分の防御があがることがある。"
            },
            {
              "name": "ドリルくちばし",
              "type": "Flying",
              "category": "Physical",
              "power": 80,
              "description": "回転しながらとがったくちばしを相手に突き刺して攻撃する。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            },
            {
              "name": "ボディプレス",
              "type": "Fighting",
              "category": "Physical",
              "power": 80,
              "description": "体をぶつけて攻撃。防御が高いほど与えるダメージが増える。"
            }
          ],
          "actions": [
            "時間85% 弱体解除",
            "体力75% てっぺき",
            "体力50% 弱体解除",
            "体力40% 強化解除",
            "体力25% つめとぎ"
          ],
          "hp": 4640,
          "atk": 135,
          "def": 162,
          "sp_atk": 84,
          "sp_def": 132,
          "speed": 105
        },
        {
          "name": "イエッサン",
          "star": 5,
          "types": [
            "Psychic",
            "Normal"
          ],
          "abilities": [
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            },
            {
              "name": "シンクロ",
              "description": "自分がなってしまったどくやまひややけどを相手にうつす。"
            },
            {
              "name": "サイコメイカー",
              "description": "登場したときにサイコフィールドをはりめぐらせる。"
            }
          ],
          "moves": [
            {
              "name": "サイコキネシス",
              "type": "Psychic",
              "category": "Special",
              "power": 90,
              "description": "強い念力を相手に送って攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "ハイパーボイス",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "うるさく響く大きな振動を相手に与えて攻撃する。"
            },
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "トリックルーム",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "まか不思議な空間をつくる。５ターンの間遅いポケモンから行動できる。"
            }
          ],
          "actions": [
            "時間80% なかよくする",
            "体力75% めいそう",
            "体力50% 弱体解除",
            "体力40% 強化解除",
            "体力25% めいそう"
          ],
          "hp": 3500,
          "atk": 102,
          "def": 87,
          "sp_atk": 162,
          "sp_def": 147,
          "speed": 147
        },
        {
          "name": "イエッサン",
          "star": 5,
          "types": [],
          "abilities": [
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            },
            {
              "name": "シンクロ",
              "description": "自分がなってしまったどくやまひややけどを相手にうつす。"
            },
            {
              "name": "サイコメイカー",
              "description": "登場したときにサイコフィールドをはりめぐらせる。"
            }
          ],
          "moves": [
            {
              "name": "サイコキネシス",
              "type": "Psychic",
              "category": "Special",
              "power": 90,
              "description": "強い念力を相手に送って攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "ハイパーボイス",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "うるさく響く大きな振動を相手に与えて攻撃する。"
            },
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "トリックルーム",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "まか不思議な空間をつくる。５ターンの間遅いポケモンから行動できる。"
            }
          ],
          "actions": [
            "時間80% なかよくする",
            "体力75% めいそう",
            "体力50% 弱体解除",
            "体力40% 強化解除",
            "体力25% めいそう"
          ],
          "hp": 0,
          "atk": 0,
          "def": 0,
          "sp_atk": 0,
          "sp_def": 0,
          "speed": 0
        },
        {
          "name": "イルカマン",
          "star": 5,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "マイティチェンジ",
              "description": "手持ちにひっこむとマイティフォルムに変化する。"
            }
          ],
          "moves": [
            {
              "name": "アクアブレイク",
              "type": "Water",
              "category": "Physical",
              "power": 85,
              "description": "水の力で相手にぶつかって攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "アクロバット",
              "type": "Flying",
              "category": "Physical",
              "power": 55,
              "description": "軽やかに相手を攻撃する。自分が道具を持っていないとき大きなダメージを与える。"
            },
            {
              "name": "あまえる",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。"
            },
            {
              "name": "ばくおんぱ",
              "type": "Normal",
              "category": "Special",
              "power": 140,
              "description": "すさまじい爆音の破壊力によって周りにいるものを攻撃する。"
            }
          ],
          "actions": [
            "時間85% テラ回収",
            "体力85% あまごい",
            "体力50% 弱体解除",
            "体力40% ビルドアップ",
            "体力20% 強化解除"
          ],
          "hp": 4700,
          "atk": 110,
          "def": 113,
          "sp_atk": 84,
          "sp_def": 98,
          "speed": 155
        },
        {
          "name": "イーブイ",
          "star": 5,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "にげあし",
              "description": "野生のポケモンから必ず逃げられる。"
            },
            {
              "name": "てきおうりょく",
              "description": "自分とおなじタイプの技の威力が上がる。"
            },
            {
              "name": "きけんよち",
              "description": "相手の持つ危険な技を察知することができる。"
            }
          ],
          "moves": [
            {
              "name": "テラバースト",
              "type": "Normal",
              "category": "Special",
              "power": 80,
              "description": "テラスタルだとテラスタイプのエネルギーを放出して攻撃する。攻撃と特攻を比べて高いほうでダメージを与える。"
            },
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "くすぐる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体をくすぐり笑わせることで相手の攻撃と防御をさげる。"
            }
          ],
          "actions": [
            "時間85% あくび",
            "体力75% 強化解除",
            "体力50% 弱体解除",
            "体力25% めいそう"
          ],
          "hp": 3340,
          "atk": 87,
          "def": 80,
          "sp_atk": 72,
          "sp_def": 102,
          "speed": 87
        },
        {
          "name": "ウインディ",
          "star": 5,
          "types": [
            "Fire"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "もらいび",
              "description": "ほのおタイプの技を受けると炎をもらい自分が出すほのおタイプの技が強くなる。"
            },
            {
              "name": "せいぎのこころ",
              "description": "あくタイプの攻撃を受けると正義感で攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "かえんほうしゃ",
              "type": "Fire",
              "category": "Special",
              "power": 90,
              "description": "激しい炎を相手に発射して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "しんそく",
              "type": "Normal",
              "category": "Physical",
              "power": 80,
              "description": "目にも留まらぬものすごい速さで相手に突進して攻撃する。必ず先制攻撃できる。"
            },
            {
              "name": "ほのおのキバ",
              "type": "Fire",
              "category": "Physical",
              "power": 65,
              "description": "炎をまとったキバでかみつく。相手をひるませたりやけど状態にすることがある。"
            }
          ],
          "actions": [
            "時間85% にほんばれ",
            "体力75% にらみつける",
            "体力50% 弱体解除",
            "体力50% テラ回収",
            "体力35% にほんばれ"
          ],
          "hp": 4400,
          "atk": 170,
          "def": 125,
          "sp_atk": 155,
          "sp_def": 125,
          "speed": 147
        },
        {
          "name": "ウォーグル",
          "star": 5,
          "types": [
            "Normal",
            "Flying"
          ],
          "abilities": [
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            },
            {
              "name": "ちからずく",
              "description": "技の追加効果はなくなるがそのぶん高い威力で技を出すことができる。"
            },
            {
              "name": "まけんき",
              "description": "相手に能力を下げられると攻撃がぐーんと上がる。"
            }
          ],
          "moves": [
            {
              "name": "アクロバット",
              "type": "Flying",
              "category": "Physical",
              "power": 55,
              "description": "軽やかに相手を攻撃する。自分が道具を持っていないとき大きなダメージを与える。"
            },
            {
              "name": "ブレイククロー",
              "type": "Normal",
              "category": "Physical",
              "power": 75,
              "description": "硬く鋭いツメで切り裂いて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "ばかぢから",
              "type": "Fighting",
              "category": "Physical",
              "power": 120,
              "description": "すごい力を発揮して相手を攻撃する。自分の攻撃と防御がさがる。"
            },
            {
              "name": "エアスラッシュ",
              "type": "Flying",
              "category": "Special",
              "power": 75,
              "description": "空をも切り裂く空気の刃で攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [
            "時間85% おいかぜ",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力45% つめとぎ",
            "体力25% 弱体解除"
          ],
          "hp": 4700,
          "atk": 189,
          "def": 117,
          "sp_atk": 90,
          "sp_def": 117,
          "speed": 125
        },
        {
          "name": "ウルガモス",
          "star": 5,
          "types": [
            "Bug",
            "Fire"
          ],
          "abilities": [
            {
              "name": "ほのおのからだ",
              "description": "自分に触った相手をやけど状態にすることがある。"
            },
            {
              "name": "むしのしらせ",
              "description": "ＨＰが減ったときむしタイプの技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "だいもんじ",
              "type": "Fire",
              "category": "Special",
              "power": 110,
              "description": "大の字の炎で相手を焼きつくす。やけど状態にすることがある。"
            },
            {
              "name": "むしのさざめき",
              "type": "Bug",
              "category": "Special",
              "power": 90,
              "description": "振動で音波をおこして攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "ぼうふう",
              "type": "Flying",
              "category": "Special",
              "power": 110,
              "description": "強烈な風で相手を包みこんで攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "おにび",
              "type": "Fire",
              "category": "Status",
              "power": 0,
              "description": "不気味で怪しい炎を放って相手をやけどの状態にする。"
            }
          ],
          "actions": [
            "時間85% にほんばれ",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力40% ちょうのまい",
            "体力20% ちょうのまい"
          ],
          "hp": 4240,
          "atk": 95,
          "def": 102,
          "sp_atk": 207,
          "sp_def": 162,
          "speed": 155
        },
        {
          "name": "エルレイド",
          "star": 5,
          "types": [
            "Psychic",
            "Fighting"
          ],
          "abilities": [
            {
              "name": "ふくつのこころ",
              "description": "ひるむたびに不屈の心を燃やして素早さが上がる。"
            },
            {
              "name": "きれあじ",
              "description": "相手を切る技の威力が上がる。"
            },
            {
              "name": "せいぎのこころ",
              "description": "あくタイプの攻撃を受けると正義感で攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "サイコカッター",
              "type": "Psychic",
              "category": "Physical",
              "power": 70,
              "description": "実体化させた心の刃で相手を切り裂く。急所に当たりやすい。"
            },
            {
              "name": "かわらわり",
              "type": "Fighting",
              "category": "Physical",
              "power": 75,
              "description": "手刀を勢いよく振りおろして相手を攻撃する。ひかりのかべやリフレクターなども破壊できる。"
            },
            {
              "name": "かげうち",
              "type": "Ghost",
              "category": "Physical",
              "power": 40,
              "description": "影をのばして相手の背後から攻撃する。必ず先制攻撃できる。"
            },
            {
              "name": "れんぞくぎり",
              "type": "Bug",
              "category": "Physical",
              "power": 40,
              "description": "カマやツメなどで相手を切りつけて攻撃する。連続で当てると威力があがる。"
            }
          ],
          "actions": [
            "時間85% さいみんじゅつ",
            "体力75% かなしばり",
            "体力50% 弱体解除",
            "体力50% テラ回収",
            "体力25% サイコフィールド"
          ],
          "hp": 3740,
          "atk": 192,
          "def": 102,
          "sp_atk": 102,
          "sp_def": 177,
          "speed": 125
        },
        {
          "name": "オトシドリ",
          "star": 5,
          "types": [
            "Flying",
            "Dark"
          ],
          "abilities": [
            {
              "name": "はとむね",
              "description": "防御を下げる効果を受けない。"
            },
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            },
            {
              "name": "いわはこび",
              "description": "いわタイプの技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "いわなだれ",
              "type": "Rock",
              "category": "Physical",
              "power": 75,
              "description": "大きな岩を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ふいうち",
              "type": "Dark",
              "category": "Physical",
              "power": 70,
              "description": "相手より先に攻撃できる。相手がだす技が攻撃技でないと失敗する。"
            },
            {
              "name": "ブレイブバード",
              "type": "Flying",
              "category": "Physical",
              "power": 120,
              "description": "はねをおりたたみ低空飛行で突撃する。自分もかなりダメージを受ける。"
            },
            {
              "name": "いちゃもん",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手にいちゃもんをつけて同じ技を２回連続でだせなくする。"
            }
          ],
          "actions": [
            "時間80% はたきおとす",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力45% 強化解除",
            "体力30% フェザーダンス"
          ],
          "hp": 3800,
          "atk": 159,
          "def": 132,
          "sp_atk": 95,
          "sp_def": 132,
          "speed": 128
        },
        {
          "name": "オニゴーリ",
          "star": 5,
          "types": [
            "Ice"
          ],
          "abilities": [
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            },
            {
              "name": "アイスボディ",
              "description": "天気がゆきのときＨＰを少しずつ回復する。"
            },
            {
              "name": "ムラっけ",
              "description": "毎ターン能力のどれかがぐーんと上がってどれかが下がる。"
            }
          ],
          "moves": [
            {
              "name": "フリーズドライ",
              "type": "Ice",
              "category": "Special",
              "power": 70,
              "description": "相手を急激に冷やしてこおり状態にすることがある。みずタイプにも効果バツグンになる。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            },
            {
              "name": "こおりのいぶき",
              "type": "Ice",
              "category": "Special",
              "power": 60,
              "description": "冷たい息を相手に吹きつけて攻撃する。必ず急所に当たる。"
            }
          ],
          "actions": [
            "時間85% ゆきげしき",
            "体力85% ゆきげしき",
            "体力75% かなしばり",
            "体力50% 弱体解除",
            "体力50% テラ回収"
          ],
          "hp": 4100,
          "atk": 125,
          "def": 125,
          "sp_atk": 125,
          "sp_def": 125,
          "speed": 125
        },
        {
          "name": "オノノクス",
          "star": 5,
          "types": [
            "Dragon"
          ],
          "abilities": [
            {
              "name": "とうそうしん",
              "description": "性別が同じだと闘争心を燃やして強くなる。性別が違うと弱くなる。"
            },
            {
              "name": "かたやぶり",
              "description": "相手の特性にジャマされることなく相手に技を出すことができる。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            }
          ],
          "moves": [
            {
              "name": "ドラゴンクロー",
              "type": "Dragon",
              "category": "Physical",
              "power": 80,
              "description": "鋭くとがった巨大なツメで相手を切り裂いて攻撃する。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "ギガインパクト",
              "type": "Normal",
              "category": "Physical",
              "power": 150,
              "description": "持てる力をすべて使って相手に突撃する。次のターンは動けなくなる。"
            },
            {
              "name": "であいがしら",
              "type": "Bug",
              "category": "Physical",
              "power": 90,
              "description": "威力が高い技だが戦闘に出たらすぐに出さないと成功しない。"
            }
          ],
          "actions": [
            "時間85% かたくなる",
            "体力75% 強化解除",
            "体力50% 弱体解除",
            "体力40% りゅうのまい",
            "体力25% 強化解除"
          ],
          "hp": 3980,
          "atk": 225,
          "def": 140,
          "sp_atk": 95,
          "sp_def": 110,
          "speed": 150
        },
        {
          "name": "オリーヴァ",
          "star": 5,
          "types": [
            "Grass",
            "Normal"
          ],
          "abilities": [
            {
              "name": "こぼれダネ",
              "description": "攻撃を受けるとグラスフィールドにする。"
            },
            {
              "name": "しゅうかく",
              "description": "使ったきのみを何回も作りだす。"
            }
          ],
          "moves": [
            {
              "name": "エナジーボール",
              "type": "Grass",
              "category": "Special",
              "power": 90,
              "description": "自然から集めた命の力を発射する。相手の特防をさげることがある。"
            },
            {
              "name": "ハイパーボイス",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "うるさく響く大きな振動を相手に与えて攻撃する。"
            },
            {
              "name": "だいちのちから",
              "type": "Ground",
              "category": "Special",
              "power": 90,
              "description": "相手の足下へ大地の力を放出する。相手の特防をさげることがある。"
            },
            {
              "name": "あまえる",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。"
            }
          ],
          "actions": [
            "時間85% にほんばれ",
            "体力75% せいちょう",
            "体力50% 弱体解除",
            "体力40% 強化解除",
            "体力20% リーフストーム"
          ],
          "hp": 4040,
          "atk": 108,
          "def": 140,
          "sp_atk": 192,
          "sp_def": 168,
          "speed": 63
        },
        {
          "name": "オンバーン",
          "star": 5,
          "types": [
            "Flying",
            "Dragon"
          ],
          "abilities": [
            {
              "name": "おみとおし",
              "description": "登場したとき相手の持ち物を見通すことができる。"
            },
            {
              "name": "すりぬけ",
              "description": "相手の壁や身代わりをすりぬけて攻撃できる。"
            },
            {
              "name": "テレパシー",
              "description": "味方の攻撃を読み取って技を回避する。"
            }
          ],
          "moves": [
            {
              "name": "エアスラッシュ",
              "type": "Flying",
              "category": "Special",
              "power": 75,
              "description": "空をも切り裂く空気の刃で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "アクロバット",
              "type": "Flying",
              "category": "Physical",
              "power": 55,
              "description": "軽やかに相手を攻撃する。自分が道具を持っていないとき大きなダメージを与える。"
            },
            {
              "name": "ばくおんぱ",
              "type": "Normal",
              "category": "Special",
              "power": 140,
              "description": "すさまじい爆音の破壊力によって周りにいるものを攻撃する。"
            }
          ],
          "actions": [
            "時間85% おいかぜ",
            "体力75% 強化解除",
            "体力50% 弱体解除",
            "体力45% おいかぜ",
            "体力25% 弱体解除"
          ],
          "hp": 4240,
          "atk": 110,
          "def": 125,
          "sp_atk": 150,
          "sp_def": 125,
          "speed": 189
        },
        {
          "name": "オーロンゲ",
          "star": 5,
          "types": [
            "Dark",
            "Fairy"
          ],
          "abilities": [
            {
              "name": "いたずらごころ",
              "description": "変化技を先制で出すことができる。"
            },
            {
              "name": "おみとおし",
              "description": "登場したとき相手の持ち物を見通すことができる。"
            },
            {
              "name": "わるいてぐせ",
              "description": "触られた相手の道具を盗んでしまう。"
            }
          ],
          "moves": [
            {
              "name": "ソウルクラッシュ",
              "type": "Fairy",
              "category": "Physical",
              "power": 75,
              "description": "食らうとくじけるほどの勢いで攻撃。相手の特攻を下げる。"
            },
            {
              "name": "どげざつき",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "頭を下げるふりをしながら振りみだした髪の毛を突き刺す。攻撃は必ず命中する。"
            },
            {
              "name": "こわいかお",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "恐ろしい顔でにらみおびえさせて相手の素早さをがくっとさげる。"
            },
            {
              "name": "イカサマ",
              "type": "Dark",
              "category": "Physical",
              "power": 95,
              "description": "相手の力を利用する。戦っている相手の攻撃が高いほどダメージがあがる。"
            }
          ],
          "actions": [
            "時間80% ひかりのかべ",
            "体力50% 弱体解除",
            "体力40% ビルドアップ",
            "体力20% 強化解除"
          ],
          "hp": 4540,
          "atk": 185,
          "def": 102,
          "sp_atk": 147,
          "sp_def": 117,
          "speed": 95
        },
        {
          "name": "カイリュー",
          "star": 5,
          "types": [
            "Dragon",
            "Flying"
          ],
          "abilities": [
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            },
            {
              "name": "マルチスケイル",
              "description": "ＨＰが満タンのときに受けるダメージが少なくなる。"
            }
          ],
          "moves": [
            {
              "name": "ドラゴンダイブ",
              "type": "Dragon",
              "category": "Physical",
              "power": 100,
              "description": "すさまじい殺気で威圧しながら体当たりする。相手をひるませることがある。"
            },
            {
              "name": "つばめがえし",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "素早い動きで相手をほんろうして切りつける。攻撃は必ず命中する。"
            },
            {
              "name": "しんそく",
              "type": "Normal",
              "category": "Physical",
              "power": 80,
              "description": "目にも留まらぬものすごい速さで相手に突進して攻撃する。必ず先制攻撃できる。"
            },
            {
              "name": "ぼうふう",
              "type": "Flying",
              "category": "Special",
              "power": 110,
              "description": "強烈な風で相手を包みこんで攻撃する。相手を混乱させることがある。"
            }
          ],
          "actions": [
            "時間85% しんぴのまもり",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力45% りゅうのまい",
            "体力35% あまごい"
          ],
          "hp": 4420,
          "atk": 206,
          "def": 147,
          "sp_atk": 155,
          "sp_def": 155,
          "speed": 125
        },
        {
          "name": "カバルドン",
          "star": 5,
          "types": [
            "Ground"
          ],
          "abilities": [
            {
              "name": "すなおこし",
              "description": "登場したとき天気を砂あらしにする。"
            },
            {
              "name": "すなのちから",
              "description": "天気がすなあらしのときいわタイプとじめんタイプとはがねタイプの威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "じしん",
              "type": "Ground",
              "category": "Physical",
              "power": 100,
              "description": "地震の衝撃で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            },
            {
              "name": "いわなだれ",
              "type": "Rock",
              "category": "Physical",
              "power": 75,
              "description": "大きな岩を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "のしかかり",
              "type": "Normal",
              "category": "Physical",
              "power": 85,
              "description": "全身で相手にのしかかり攻撃する。まひ状態にすることがある。"
            }
          ],
          "actions": [
            "時間85% あくび",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力40% 強化解除",
            "体力25% たくわえる"
          ],
          "hp": 4940,
          "atk": 173,
          "def": 182,
          "sp_atk": 107,
          "sp_def": 113,
          "speed": 75
        },
        {
          "name": "ガブリアス",
          "star": 5,
          "types": [
            "Dragon",
            "Ground"
          ],
          "abilities": [
            {
              "name": "すながくれ",
              "description": "砂あらしのとき回避率が上がる。"
            },
            {
              "name": "さめはだ",
              "description": "攻撃を受けたとき自分に触れた相手をざらざらの肌でキズつける。"
            }
          ],
          "moves": [
            {
              "name": "じしん",
              "type": "Ground",
              "category": "Physical",
              "power": 100,
              "description": "地震の衝撃で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "ドラゴンクロー",
              "type": "Dragon",
              "category": "Physical",
              "power": 80,
              "description": "鋭くとがった巨大なツメで相手を切り裂いて攻撃する。"
            },
            {
              "name": "アイアンヘッド",
              "type": "Steel",
              "category": "Physical",
              "power": 80,
              "description": "鋼のような硬い頭で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "きりさく",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。"
            }
          ],
          "actions": [
            "時間85% すなあらし",
            "体力75% じならし",
            "体力50% 弱体解除",
            "体力40% 強化解除",
            "体力35% すなあらし"
          ],
          "hp": 4940,
          "atk": 200,
          "def": 147,
          "sp_atk": 125,
          "sp_def": 132,
          "speed": 158
        },
        {
          "name": "キノガッサ",
          "star": 5,
          "types": [
            "Grass",
            "Fighting"
          ],
          "abilities": [
            {
              "name": "ほうし",
              "description": "攻撃で自分に触れた相手をどくやまひやねむり状態にすることがある。"
            },
            {
              "name": "ポイズンヒール",
              "description": "どく状態になるとＨＰが減らずに増えていく。"
            },
            {
              "name": "テクニシャン",
              "description": "威力が低い技の威力を高くして攻撃できる。"
            }
          ],
          "moves": [
            {
              "name": "タネばくだん",
              "type": "Grass",
              "category": "Physical",
              "power": 80,
              "description": "硬い殻をもつ大きなタネを上からたたきつけて相手を攻撃する。"
            },
            {
              "name": "マッハパンチ",
              "type": "Fighting",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さでパンチをくりだす。必ず先制攻撃できる。"
            },
            {
              "name": "なやみのタネ",
              "type": "Grass",
              "category": "Status",
              "power": 0,
              "description": "心をなやませるタネを植えつける。相手を眠れなくして特性をふみんにする。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [
            "時間80% グラスフィールド",
            "体力75% キノコのほうし",
            "体力50% 弱体解除",
            "体力45% テラ回収",
            "体力20% 強化解除"
          ],
          "hp": 3500,
          "atk": 200,
          "def": 125,
          "sp_atk": 95,
          "sp_def": 95,
          "speed": 110
        },
        {
          "name": "キョジオーン",
          "star": 5,
          "types": [
            "Rock"
          ],
          "abilities": [
            {
              "name": "きよめのしお",
              "description": "清らかな塩で状態異常にならない。ゴーストタイプの技のダメージを半減させる。"
            },
            {
              "name": "がんじょう",
              "description": "ＨＰが満タンのとき技を受けても一撃で倒されることがない。一撃必殺技も効かない。"
            },
            {
              "name": "クリアボディ",
              "description": "相手の技や特性で能力を下げられない。"
            }
          ],
          "moves": [
            {
              "name": "しおづけ",
              "type": "Rock",
              "category": "Physical",
              "power": 40,
              "description": "相手をしおづけ状態にして毎ターンダメージを与える。はがねみずタイプはより苦しむ。"
            },
            {
              "name": "いわなだれ",
              "type": "Rock",
              "category": "Physical",
              "power": 75,
              "description": "大きな岩を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "アームハンマー",
              "type": "Fighting",
              "category": "Physical",
              "power": 100,
              "description": "強くて重いこぶしをふるってダメージを与える。自分の素早さがさがる。"
            },
            {
              "name": "すなあらし",
              "type": "Rock",
              "category": "Status",
              "power": 0,
              "description": "５ターンの間砂あらしでいわじめんはがねタイプ以外にダメージ。いわタイプの特防があがる。"
            }
          ],
          "actions": [
            "時間80% すなあらし",
            "体力75% 強化解除",
            "体力50% 弱体解除",
            "体力50% テラ回収",
            "体力20% いわなだれ"
          ],
          "hp": 4700,
          "atk": 155,
          "def": 200,
          "sp_atk": 72,
          "sp_def": 140,
          "speed": 57
        },
        {
          "name": "キラフロル",
          "star": 5,
          "types": [
            "Rock",
            "Poison"
          ],
          "abilities": [
            {
              "name": "どくげしょう",
              "description": "物理技でダメージを受けると相手の足下にどくびしがちらばる。"
            },
            {
              "name": "ふしょく",
              "description": "はがねタイプやどくタイプもどく状態にすることができる。"
            }
          ],
          "moves": [
            {
              "name": "パワージェム",
              "type": "Rock",
              "category": "Special",
              "power": 80,
              "description": "宝石のようにきらめく光を発射して相手を攻撃する。"
            },
            {
              "name": "ヘドロばくだん",
              "type": "Poison",
              "category": "Special",
              "power": 90,
              "description": "汚いヘドロを相手に投げつけて攻撃する。毒状態にすることがある。"
            },
            {
              "name": "キラースピン",
              "type": "Poison",
              "category": "Physical",
              "power": 30,
              "description": "回転して相手を攻撃する。しめつけるまきつくやどりぎのタネなど吹きとばす。相手を毒状態にする。"
            },
            {
              "name": "げんしのちから",
              "type": "Rock",
              "category": "Special",
              "power": 60,
              "description": "原始の力で攻撃する。自分のすべての能力があがることがある。"
            }
          ],
          "actions": [
            "時間80% すなあらし",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力45% テラ回収",
            "体力20% テラバースト"
          ],
          "hp": 4180,
          "atk": 87,
          "def": 140,
          "sp_atk": 200,
          "sp_def": 126,
          "speed": 134
        },
        {
          "name": "ギャラドス",
          "star": 5,
          "types": [
            "Water",
            "Flying"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "じしんかじょう",
              "description": "相手を倒すと自信がついて攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "アクアテール",
              "type": "Water",
              "category": "Physical",
              "power": 90,
              "description": "激しくあれくるう荒波のように大きなしっぽをふって相手を攻撃する。"
            },
            {
              "name": "たつまき",
              "type": "Dragon",
              "category": "Special",
              "power": 40,
              "description": "竜巻をおこして相手をまきこみ攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ぼうふう",
              "type": "Flying",
              "category": "Special",
              "power": 110,
              "description": "強烈な風で相手を包みこんで攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            }
          ],
          "actions": [
            "時間85% こわいかお",
            "体力75% ちょうはつ",
            "体力50% 弱体解除",
            "体力45% りゅうのまい",
            "体力35% あまごい"
          ],
          "hp": 4540,
          "atk": 192,
          "def": 123,
          "sp_atk": 95,
          "sp_def": 155,
          "speed": 126
        },
        {
          "name": "クレベース",
          "star": 5,
          "types": [
            "Ice"
          ],
          "abilities": [
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "アイスボディ",
              "description": "天気がゆきのときＨＰを少しずつ回復する。"
            },
            {
              "name": "がんじょう",
              "description": "ＨＰが満タンのとき技を受けても一撃で倒されることがない。一撃必殺技も効かない。"
            }
          ],
          "moves": [
            {
              "name": "つららおとし",
              "type": "Ice",
              "category": "Physical",
              "power": 85,
              "description": "大きな氷柱を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "すてみタックル",
              "type": "Normal",
              "category": "Physical",
              "power": 120,
              "description": "命を懸けて相手に突進して攻撃する。自分もかなりダメージを受ける。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "こおりのキバ",
              "type": "Ice",
              "category": "Physical",
              "power": 65,
              "description": "冷気をひめたキバでかみつく。相手をひるませたりこおり状態にすることがある。"
            }
          ],
          "actions": [
            "時間85% ゆきげしき",
            "体力75% 強化解除",
            "体力50% 弱体解除",
            "体力45% ゆきげしき",
            "体力30% てっぺき"
          ],
          "hp": 4540,
          "atk": 180,
          "def": 281,
          "sp_atk": 71,
          "sp_def": 74,
          "speed": 47
        },
        {
          "name": "グレンアルマ",
          "star": 5,
          "types": [
            "Fire",
            "Psychic"
          ],
          "abilities": [
            {
              "name": "もらいび",
              "description": "ほのおタイプの技を受けると炎をもらい自分が出すほのおタイプの技が強くなる。"
            },
            {
              "name": "くだけるよろい",
              "description": "物理技でダメージを受けると防御が下がり素早さがぐーんと上がる。"
            }
          ],
          "moves": [
            {
              "name": "アーマーキャノン",
              "type": "Fire",
              "category": "Special",
              "power": 120,
              "description": "みずからのヨロイを燃えたぎる弾として撃ち出して攻撃する。自分の防御と特防がさがる。"
            },
            {
              "name": "サイコキネシス",
              "type": "Psychic",
              "category": "Special",
              "power": 90,
              "description": "強い念力を相手に送って攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "ナイトヘッド",
              "type": "Ghost",
              "category": "Special",
              "power": 1,
              "description": "恐ろしい幻をみせて自分のレベルと同じだけのダメージを相手に与える。"
            },
            {
              "name": "おにび",
              "type": "Fire",
              "category": "Status",
              "power": 0,
              "description": "不気味で怪しい炎を放って相手をやけどの状態にする。"
            }
          ],
          "actions": [
            "時間80% 弱体解除",
            "体力75% にほんばれ",
            "体力50% 弱体解除",
            "体力40% めいそう",
            "体力25% 強化解除"
          ],
          "hp": 4240,
          "atk": 95,
          "def": 155,
          "sp_atk": 192,
          "sp_def": 125,
          "speed": 117
        },
        {
          "name": "ケッキング",
          "star": 5,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "なまけ",
              "description": "技を出すと次のターンは休んでしまう。"
            }
          ],
          "moves": [
            {
              "name": "からげんき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "自分が毒まひやけど状態のとき相手にくりだすと技の威力が２倍になる。"
            },
            {
              "name": "シャドークロー",
              "type": "Ghost",
              "category": "Physical",
              "power": 70,
              "description": "影からつくった鋭いツメで相手を切り裂く。急所に当たりやすい。"
            },
            {
              "name": "じゃれつく",
              "type": "Fairy",
              "category": "Physical",
              "power": 90,
              "description": "相手にじゃれついて攻撃する。相手の攻撃をさげることがある。"
            },
            {
              "name": "いばる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせて混乱させる。怒りで相手の攻撃はぐーんとあがってしまう。"
            }
          ],
          "actions": [
            "時間85% アンコール",
            "体力75% 弱体解除",
            "体力50% 弱体解除",
            "体力50% テラ回収"
          ],
          "hp": 6200,
          "atk": 245,
          "def": 155,
          "sp_atk": 147,
          "sp_def": 102,
          "speed": 155
        },
        {
          "name": "ケンタロス",
          "star": 5,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "いかりのつぼ",
              "description": "急所に攻撃が当たると怒りくるって攻撃力が最大になる。"
            },
            {
              "name": "はんすう",
              "description": "きのみを食べると次のターンの終わりに胃から出してもう１回だけ食べる。"
            }
          ],
          "moves": [
            {
              "name": "フレアドライブ",
              "type": "Fire",
              "category": "Physical",
              "power": 120,
              "description": "炎をまとって突進する。自分もかなりダメージを受ける。やけど状態にすることがある。"
            },
            {
              "name": "インファイト",
              "type": "Fighting",
              "category": "Physical",
              "power": 120,
              "description": "守りを捨てて相手のふところに突撃する。自分の防御と特防がさがる。"
            },
            {
              "name": "かえんほうしゃ",
              "type": "Fire",
              "category": "Special",
              "power": 90,
              "description": "激しい炎を相手に発射して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [
            "時間80% ふるいたてる",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力40% にほんばれ",
            "体力25% 強化解除"
          ],
          "hp": 3940,
          "atk": 155,
          "def": 147,
          "sp_atk": 65,
          "sp_def": 110,
          "speed": 170
        },
        {
          "name": "ケンタロス",
          "star": 5,
          "types": [],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "いかりのつぼ",
              "description": "急所に攻撃が当たると怒りくるって攻撃力が最大になる。"
            },
            {
              "name": "はんすう",
              "description": "きのみを食べると次のターンの終わりに胃から出してもう１回だけ食べる。"
            }
          ],
          "moves": [
            {
              "name": "ウェーブタックル",
              "type": "Water",
              "category": "Physical",
              "power": 120,
              "description": "水をまといつつ全身で相手にぶつかるが自分もかなりのダメージを受ける。"
            },
            {
              "name": "インファイト",
              "type": "Fighting",
              "category": "Physical",
              "power": 120,
              "description": "守りを捨てて相手のふところに突撃する。自分の防御と特防がさがる。"
            },
            {
              "name": "なみのり",
              "type": "Water",
              "category": "Special",
              "power": 90,
              "description": "大きな波で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [
            "時間80% ふるいたてる",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力40% あまごい",
            "体力25% 強化解除"
          ],
          "hp": 0,
          "atk": 0,
          "def": 0,
          "sp_atk": 0,
          "sp_def": 0,
          "speed": 0
        },
        {
          "name": "ゲンガー",
          "star": 5,
          "types": [
            "Ghost",
            "Poison"
          ],
          "abilities": [
            {
              "name": "のろわれボディ",
              "description": "攻撃を受けると相手の技をかなしばり状態にすることがある。"
            }
          ],
          "moves": [
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "ヘドロばくだん",
              "type": "Poison",
              "category": "Special",
              "power": 90,
              "description": "汚いヘドロを相手に投げつけて攻撃する。毒状態にすることがある。"
            },
            {
              "name": "あやしいひかり",
              "type": "Ghost",
              "category": "Status",
              "power": 0,
              "description": "怪しい光を相手にみせてまどわせる。相手を混乱させる。"
            },
            {
              "name": "うらみ",
              "type": "Ghost",
              "category": "Status",
              "power": 0,
              "description": "相手が最後に使った技に恨みを抱いてその技のＰＰを４だけ減らす。"
            }
          ],
          "actions": [
            "時間85% さいみんじゅつ",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力45% 強化解除",
            "体力25% さいみんじゅつ"
          ],
          "hp": 3500,
          "atk": 102,
          "def": 95,
          "sp_atk": 200,
          "sp_def": 117,
          "speed": 170
        },
        {
          "name": "コノヨザル",
          "star": 5,
          "types": [
            "Fighting",
            "Ghost"
          ],
          "abilities": [
            {
              "name": "やるき",
              "description": "やる気をだすことによってねむり状態にならない。"
            },
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            },
            {
              "name": "まけんき",
              "description": "相手に能力を下げられると攻撃がぐーんと上がる。"
            }
          ],
          "moves": [
            {
              "name": "シャドークロー",
              "type": "Ghost",
              "category": "Physical",
              "power": 70,
              "description": "影からつくった鋭いツメで相手を切り裂く。急所に当たりやすい。"
            },
            {
              "name": "インファイト",
              "type": "Fighting",
              "category": "Physical",
              "power": 120,
              "description": "守りを捨てて相手のふところに突撃する。自分の防御と特防がさがる。"
            },
            {
              "name": "げきりん",
              "type": "Dragon",
              "category": "Physical",
              "power": 120,
              "description": "２ー３ターンの間暴れまくって攻撃する。暴れたあとは混乱する。"
            },
            {
              "name": "にらみつける",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "鋭い目つきでおびえさせて相手の防御をさげる。"
            }
          ],
          "actions": [
            "時間80% ちょうはつ",
            "体力75% 弱体解除",
            "体力50% 弱体解除",
            "体力45% テラ回収",
            "体力35% ビルドアップ"
          ],
          "hp": 5000,
          "atk": 177,
          "def": 125,
          "sp_atk": 80,
          "sp_def": 140,
          "speed": 140
        },
        {
          "name": "ゴチルゼル",
          "star": 5,
          "types": [
            "Psychic"
          ],
          "abilities": [
            {
              "name": "おみとおし",
              "description": "登場したとき相手の持ち物を見通すことができる。"
            },
            {
              "name": "かちき",
              "description": "相手に能力を下げられると特攻がぐーんと上がる。"
            },
            {
              "name": "かげふみ",
              "description": "相手の影を踏み逃げたり交代できなくする。"
            }
          ],
          "moves": [
            {
              "name": "サイコキネシス",
              "type": "Psychic",
              "category": "Special",
              "power": 90,
              "description": "強い念力を相手に送って攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            },
            {
              "name": "１０まんボルト",
              "type": "Electric",
              "category": "Special",
              "power": 90,
              "description": "強い電撃を相手に浴びせて攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "アシストパワー",
              "type": "Psychic",
              "category": "Special",
              "power": 20,
              "description": "蓄積されたパワーで相手を攻撃する。自分の能力があがっているほど威力があがる。"
            }
          ],
          "actions": [
            "時間85% めいそう",
            "体力75% 強化解除",
            "体力50% 弱体解除",
            "体力40% めいそう",
            "体力25% ひかりのかべ"
          ],
          "hp": 3800,
          "atk": 87,
          "def": 147,
          "sp_atk": 147,
          "sp_def": 170,
          "speed": 102
        },
        {
          "name": "サザンドラ",
          "star": 5,
          "types": [
            "Dark",
            "Dragon"
          ],
          "abilities": [
            {
              "name": "ふゆう",
              "description": "地面から浮くことによってじめんタイプの技を受けない。"
            }
          ],
          "moves": [
            {
              "name": "あくのはどう",
              "type": "Dark",
              "category": "Special",
              "power": 80,
              "description": "体から悪意にみちた恐ろしいオーラを発する。相手をひるませることがある。"
            },
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "こわいかお",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "恐ろしい顔でにらみおびえさせて相手の素早さをがくっとさげる。"
            },
            {
              "name": "ドラゴンダイブ",
              "type": "Dragon",
              "category": "Physical",
              "power": 100,
              "description": "すさまじい殺気で威圧しながら体当たりする。相手をひるませることがある。"
            }
          ],
          "actions": [
            "時間85% ちょうはつ",
            "体力75% リフレクター",
            "体力50% 弱体解除",
            "体力50% テラ回収",
            "体力30% わるだくみ"
          ],
          "hp": 4460,
          "atk": 162,
          "def": 140,
          "sp_atk": 192,
          "sp_def": 140,
          "speed": 152
        },
        {
          "name": "サーナイト",
          "star": 5,
          "types": [
            "Psychic",
            "Fairy"
          ],
          "abilities": [
            {
              "name": "シンクロ",
              "description": "自分がなってしまったどくやまひややけどを相手にうつす。"
            },
            {
              "name": "トレース",
              "description": "登場したとき相手の特性をトレースして同じ特性になる。"
            },
            {
              "name": "テレパシー",
              "description": "味方の攻撃を読み取って技を回避する。"
            }
          ],
          "moves": [
            {
              "name": "サイコキネシス",
              "type": "Psychic",
              "category": "Special",
              "power": 90,
              "description": "強い念力を相手に送って攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "ムーンフォース",
              "type": "Fairy",
              "category": "Special",
              "power": 95,
              "description": "月のパワーをかりて相手を攻撃する。相手の特攻をさげることがある。"
            },
            {
              "name": "かなしばり",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手の動きをとめて直前にだしていた技を４ターンの間使えなくする。"
            },
            {
              "name": "ドレインキッス",
              "type": "Fairy",
              "category": "Special",
              "power": 50,
              "description": "キッスによって相手からＨＰを吸い取る。与えたダメージの半分以上ＨＰを回復する。"
            }
          ],
          "actions": [
            "時間85% ミストフィールド",
            "体力75% めいそう",
            "体力50% 弱体解除",
            "体力45% 強化解除",
            "体力35% サイコフィールド"
          ],
          "hp": 3740,
          "atk": 102,
          "def": 102,
          "sp_atk": 192,
          "sp_def": 177,
          "speed": 125
        },
        {
          "name": "シビルドン",
          "star": 5,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "ふゆう",
              "description": "地面から浮くことによってじめんタイプの技を受けない。"
            }
          ],
          "moves": [
            {
              "name": "ワイルドボルト",
              "type": "Electric",
              "category": "Physical",
              "power": 90,
              "description": "電気をまとって相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "かえんほうしゃ",
              "type": "Fire",
              "category": "Special",
              "power": 90,
              "description": "激しい炎を相手に発射して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "ほうでん",
              "type": "Electric",
              "category": "Special",
              "power": 80,
              "description": "まばゆい電撃で自分の周りにいるものを攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "ブレイククロー",
              "type": "Normal",
              "category": "Physical",
              "power": 75,
              "description": "硬く鋭いツメで切り裂いて攻撃する。相手の防御をさげることがある。"
            }
          ],
          "actions": [
            "時間85% プラズマシャワー",
            "体力75% でんじは",
            "体力75% でんじは",
            "体力50% 弱体解除",
            "体力25% とぐろをまく"
          ],
          "hp": 4240,
          "atk": 177,
          "def": 125,
          "sp_atk": 162,
          "sp_def": 125,
          "speed": 80
        },
        {
          "name": "シャリタツ",
          "star": 5,
          "types": [
            "Dragon",
            "Water"
          ],
          "abilities": [
            {
              "name": "しれいとう",
              "description": "登場したとき味方にヘイラッシャがいると口の中に入ってそこから指令をだす。"
            },
            {
              "name": "よびみず",
              "description": "みずタイプの技を自分によせつけダメージは受けずに特攻が上がる。"
            }
          ],
          "moves": [
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "こうそくスピン",
              "type": "Normal",
              "category": "Physical",
              "power": 50,
              "description": "回転して相手を攻撃する。しめつけるまきつくやどりぎのタネなど吹きとばす。自分の素早さもあがる。"
            },
            {
              "name": "カウンター",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "相手から受けた物理攻撃のダメージを２倍にして同じ相手に返す。"
            }
          ],
          "actions": [
            "時間80% テラ回収",
            "体力75% ひやみず",
            "体力50% 弱体解除",
            "体力45% テラ回収",
            "体力20% 強化解除"
          ],
          "hp": 3740,
          "atk": 80,
          "def": 95,
          "sp_atk": 185,
          "sp_def": 147,
          "speed": 128
        },
        {
          "name": "シャリタツ",
          "star": 5,
          "types": [],
          "abilities": [
            {
              "name": "しれいとう",
              "description": "登場したとき味方にヘイラッシャがいると口の中に入ってそこから指令をだす。"
            },
            {
              "name": "よびみず",
              "description": "みずタイプの技を自分によせつけダメージは受けずに特攻が上がる。"
            }
          ],
          "moves": [
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "こうそくスピン",
              "type": "Normal",
              "category": "Physical",
              "power": 50,
              "description": "回転して相手を攻撃する。しめつけるまきつくやどりぎのタネなど吹きとばす。自分の素早さもあがる。"
            },
            {
              "name": "カウンター",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "相手から受けた物理攻撃のダメージを２倍にして同じ相手に返す。"
            }
          ],
          "actions": [
            "時間80% テラ回収",
            "体力75% ひやみず",
            "体力50% 弱体解除",
            "体力45% テラ回収",
            "体力20% 強化解除"
          ],
          "hp": 0,
          "atk": 0,
          "def": 0,
          "sp_atk": 0,
          "sp_def": 0,
          "speed": 0
        },
        {
          "name": "シャリタツ",
          "star": 5,
          "types": [],
          "abilities": [
            {
              "name": "しれいとう",
              "description": "登場したとき味方にヘイラッシャがいると口の中に入ってそこから指令をだす。"
            },
            {
              "name": "よびみず",
              "description": "みずタイプの技を自分によせつけダメージは受けずに特攻が上がる。"
            }
          ],
          "moves": [
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "こうそくスピン",
              "type": "Normal",
              "category": "Physical",
              "power": 50,
              "description": "回転して相手を攻撃する。しめつけるまきつくやどりぎのタネなど吹きとばす。自分の素早さもあがる。"
            },
            {
              "name": "カウンター",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "相手から受けた物理攻撃のダメージを２倍にして同じ相手に返す。"
            }
          ],
          "actions": [
            "時間80% テラ回収",
            "体力75% ひやみず",
            "体力50% 弱体解除",
            "体力45% テラ回収",
            "体力20% 強化解除"
          ],
          "hp": 0,
          "atk": 0,
          "def": 0,
          "sp_atk": 0,
          "sp_def": 0,
          "speed": 0
        },
        {
          "name": "ジバコイル",
          "star": 5,
          "types": [
            "Electric",
            "Steel"
          ],
          "abilities": [
            {
              "name": "じりょく",
              "description": "はがねタイプのポケモンを磁力で引きつけて逃げられなくする。"
            },
            {
              "name": "がんじょう",
              "description": "ＨＰが満タンのとき技を受けても一撃で倒されることがない。一撃必殺技も効かない。"
            },
            {
              "name": "アナライズ",
              "description": "いちばん最後に技を出すと技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "１０まんボルト",
              "type": "Electric",
              "category": "Special",
              "power": 90,
              "description": "強い電撃を相手に浴びせて攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "ラスターカノン",
              "type": "Steel",
              "category": "Special",
              "power": 80,
              "description": "体の光を一点に集めて力を放つ。相手の特防をさげることがある。"
            },
            {
              "name": "トライアタック",
              "type": "Normal",
              "category": "Special",
              "power": 80,
              "description": "３つの光線で攻撃する。まひかやけどかこおり状態のどれかにすることがある。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            }
          ],
          "actions": [
            "時間85% でんじふゆう",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力40% エレキフィールド",
            "体力25% 強化解除"
          ],
          "hp": 3800,
          "atk": 110,
          "def": 177,
          "sp_atk": 200,
          "sp_def": 140,
          "speed": 95
        },
        {
          "name": "ストライク",
          "star": 5,
          "types": [
            "Bug",
            "Flying"
          ],
          "abilities": [
            {
              "name": "むしのしらせ",
              "description": "ＨＰが減ったときむしタイプの技の威力が上がる。"
            },
            {
              "name": "テクニシャン",
              "description": "威力が低い技の威力を高くして攻撃できる。"
            },
            {
              "name": "ふくつのこころ",
              "description": "ひるむたびに不屈の心を燃やして素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "つばめがえし",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "素早い動きで相手をほんろうして切りつける。攻撃は必ず命中する。"
            },
            {
              "name": "シザークロス",
              "type": "Bug",
              "category": "Physical",
              "power": 80,
              "description": "カマやツメをハサミのように交差させながら相手を切り裂く。"
            },
            {
              "name": "きりさく",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "こうそくいどう",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "力をぬいて体を軽くして高速で動く。自分の素早さをぐーんとあげる。"
            }
          ],
          "actions": [
            "時間85% きあいだめ",
            "体力75% 強化解除",
            "体力50% 弱体解除",
            "体力45% つるぎのまい"
          ],
          "hp": 3800,
          "atk": 170,
          "def": 125,
          "sp_atk": 87,
          "sp_def": 125,
          "speed": 162
        },
        {
          "name": "ストリンダー",
          "star": 5,
          "types": [
            "Electric",
            "Poison"
          ],
          "abilities": [
            {
              "name": "パンクロック",
              "description": "音技の威力が上がる。受けた音技のダメージは半分になる。"
            },
            {
              "name": "プラス",
              "description": "プラスかマイナスの特性を持つポケモンが仲間にいると自分の特攻が上がる。"
            },
            {
              "name": "テクニシャン",
              "description": "威力が低い技の威力を高くして攻撃できる。"
            }
          ],
          "moves": [
            {
              "name": "オーバードライブ",
              "type": "Electric",
              "category": "Special",
              "power": 80,
              "description": "ギターやベースをかきならして激しく響く大きな振動を相手に与えて攻撃する。"
            },
            {
              "name": "どくづき",
              "type": "Poison",
              "category": "Physical",
              "power": 80,
              "description": "毒にそまった触手や腕で相手を突き刺す。毒状態にすることがある。"
            },
            {
              "name": "ほっぺすりすり",
              "type": "Electric",
              "category": "Physical",
              "power": 20,
              "description": "電気を帯びたほっぺをすりつけて攻撃。相手をまひ状態にする。"
            },
            {
              "name": "ばくおんぱ",
              "type": "Normal",
              "category": "Special",
              "power": 140,
              "description": "すさまじい爆音の破壊力によって周りにいるものを攻撃する。"
            }
          ],
          "actions": [
            "時間85% エレキフィールド",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力40% 強化解除",
            "体力25% エレキフィールド"
          ],
          "hp": 3940,
          "atk": 152,
          "def": 110,
          "sp_atk": 176,
          "sp_def": 110,
          "speed": 117
        },
        {
          "name": "ストリンダー",
          "star": 5,
          "types": [],
          "abilities": [
            {
              "name": "パンクロック",
              "description": "音技の威力が上がる。受けた音技のダメージは半分になる。"
            },
            {
              "name": "プラス",
              "description": "プラスかマイナスの特性を持つポケモンが仲間にいると自分の特攻が上がる。"
            },
            {
              "name": "テクニシャン",
              "description": "威力が低い技の威力を高くして攻撃できる。"
            }
          ],
          "moves": [
            {
              "name": "オーバードライブ",
              "type": "Electric",
              "category": "Special",
              "power": 80,
              "description": "ギターやベースをかきならして激しく響く大きな振動を相手に与えて攻撃する。"
            },
            {
              "name": "どくづき",
              "type": "Poison",
              "category": "Physical",
              "power": 80,
              "description": "毒にそまった触手や腕で相手を突き刺す。毒状態にすることがある。"
            },
            {
              "name": "ほっぺすりすり",
              "type": "Electric",
              "category": "Physical",
              "power": 20,
              "description": "電気を帯びたほっぺをすりつけて攻撃。相手をまひ状態にする。"
            },
            {
              "name": "ばくおんぱ",
              "type": "Normal",
              "category": "Special",
              "power": 140,
              "description": "すさまじい爆音の破壊力によって周りにいるものを攻撃する。"
            }
          ],
          "actions": [
            "時間85% エレキフィールド",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力40% 強化解除",
            "体力25% エレキフィールド"
          ],
          "hp": 0,
          "atk": 0,
          "def": 0,
          "sp_atk": 0,
          "sp_def": 0,
          "speed": 0
        },
        {
          "name": "セキタンザン",
          "star": 5,
          "types": [
            "Rock",
            "Fire"
          ],
          "abilities": [
            {
              "name": "じょうききかん",
              "description": "みずタイプほのおタイプの技を受けると素早さがぐぐーんと上がる。"
            },
            {
              "name": "ほのおのからだ",
              "description": "自分に触った相手をやけど状態にすることがある。"
            },
            {
              "name": "もらいび",
              "description": "ほのおタイプの技を受けると炎をもらい自分が出すほのおタイプの技が強くなる。"
            }
          ],
          "moves": [
            {
              "name": "ヒートスタンプ",
              "type": "Fire",
              "category": "Physical",
              "power": 1,
              "description": "燃える体で相手にぶつかって攻撃する。自分が相手より重いほど威力があがる。"
            },
            {
              "name": "ストーンエッジ",
              "type": "Rock",
              "category": "Physical",
              "power": 100,
              "description": "とがった岩を相手に突き刺して攻撃する。急所に当たりやすい。"
            },
            {
              "name": "やきつくす",
              "type": "Fire",
              "category": "Special",
              "power": 60,
              "description": "炎で相手を攻撃する。相手がきのみなどを持っているとき燃やして使えなくする。"
            },
            {
              "name": "げんしのちから",
              "type": "Rock",
              "category": "Special",
              "power": 60,
              "description": "原始の力で攻撃する。自分のすべての能力があがることがある。"
            }
          ],
          "actions": [
            "時間80% すなあらし",
            "体力75% タールショット",
            "体力50% 弱体解除",
            "体力40% 強化解除",
            "体力20% だいもんじ"
          ],
          "hp": 5000,
          "atk": 125,
          "def": 185,
          "sp_atk": 125,
          "sp_def": 140,
          "speed": 50
        },
        {
          "name": "セグレイブ",
          "star": 5,
          "types": [
            "Dragon",
            "Ice"
          ],
          "abilities": [
            {
              "name": "ねつこうかん",
              "description": "ほのおタイプの技を受けると攻撃が上がる。やけど状態にならない。"
            },
            {
              "name": "アイスボディ",
              "description": "天気がゆきのときＨＰを少しずつ回復する。"
            }
          ],
          "moves": [
            {
              "name": "ドラゴンクロー",
              "type": "Dragon",
              "category": "Physical",
              "power": 80,
              "description": "鋭くとがった巨大なツメで相手を切り裂いて攻撃する。"
            },
            {
              "name": "つららおとし",
              "type": "Ice",
              "category": "Physical",
              "power": 85,
              "description": "大きな氷柱を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "こおりのつぶて",
              "type": "Ice",
              "category": "Physical",
              "power": 40,
              "description": "氷の塊を一瞬でつくり相手に素早く放つ。必ず先制攻撃できる。"
            },
            {
              "name": "ボディプレス",
              "type": "Fighting",
              "category": "Physical",
              "power": 80,
              "description": "体をぶつけて攻撃。防御が高いほど与えるダメージが増える。"
            }
          ],
          "actions": [
            "時間80% ゆきげしき",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力40% ゆきげしき",
            "体力20% 強化解除"
          ],
          "hp": 5140,
          "atk": 222,
          "def": 143,
          "sp_atk": 117,
          "sp_def": 134,
          "speed": 135
        },
        {
          "name": "ソウブレイズ",
          "star": 5,
          "types": [
            "Fire",
            "Ghost"
          ],
          "abilities": [
            {
              "name": "もらいび",
              "description": "ほのおタイプの技を受けると炎をもらい自分が出すほのおタイプの技が強くなる。"
            },
            {
              "name": "くだけるよろい",
              "description": "物理技でダメージを受けると防御が下がり素早さがぐーんと上がる。"
            }
          ],
          "moves": [
            {
              "name": "むねんのつるぎ",
              "type": "Fire",
              "category": "Physical",
              "power": 90,
              "description": "この世への未練を剣先にこめて切りつける。与えたダメージの半分のＨＰを回復できる。"
            },
            {
              "name": "シャドークロー",
              "type": "Ghost",
              "category": "Physical",
              "power": 70,
              "description": "影からつくった鋭いツメで相手を切り裂く。急所に当たりやすい。"
            },
            {
              "name": "サイコカッター",
              "type": "Psychic",
              "category": "Physical",
              "power": 70,
              "description": "実体化させた心の刃で相手を切り裂く。急所に当たりやすい。"
            },
            {
              "name": "おにび",
              "type": "Fire",
              "category": "Status",
              "power": 0,
              "description": "不気味で怪しい炎を放って相手をやけどの状態にする。"
            }
          ],
          "actions": [
            "時間80% 弱体解除",
            "体力75% にほんばれ",
            "体力50% 弱体解除",
            "体力40% つるぎのまい",
            "体力25% 強化解除"
          ],
          "hp": 3940,
          "atk": 192,
          "def": 125,
          "sp_atk": 95,
          "sp_def": 155,
          "speed": 132
        },
        {
          "name": "ゾロアーク",
          "star": 5,
          "types": [
            "Dark"
          ],
          "abilities": [
            {
              "name": "イリュージョン",
              "description": "手持ちのいちばんうしろにいるポケモンになりきって登場して相手を化かす。"
            }
          ],
          "moves": [
            {
              "name": "ナイトバースト",
              "type": "Dark",
              "category": "Special",
              "power": 85,
              "description": "暗黒の衝撃波をとばして相手を攻撃する。命中率をさげることがある。"
            },
            {
              "name": "シャドークロー",
              "type": "Ghost",
              "category": "Physical",
              "power": 70,
              "description": "影からつくった鋭いツメで相手を切り裂く。急所に当たりやすい。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            },
            {
              "name": "ハイパーボイス",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "うるさく響く大きな振動を相手に与えて攻撃する。"
            }
          ],
          "actions": [
            "時間85% いちゃもん",
            "体力75% 強化解除",
            "体力50% 弱体解除",
            "体力50% テラ回収",
            "体力30% わるだくみ"
          ],
          "hp": 3500,
          "atk": 162,
          "def": 95,
          "sp_atk": 185,
          "sp_def": 95,
          "speed": 162
        },
        {
          "name": "タイレーツ",
          "star": 5,
          "types": [
            "Fighting"
          ],
          "abilities": [
            {
              "name": "カブトアーマー",
              "description": "硬い甲羅に守られて相手の攻撃が急所に当たらない。"
            },
            {
              "name": "まけんき",
              "description": "相手に能力を下げられると攻撃がぐーんと上がる。"
            }
          ],
          "moves": [
            {
              "name": "メガホーン",
              "type": "Bug",
              "category": "Physical",
              "power": 120,
              "description": "硬くてりっぱなつのでおもいっきり相手を突き刺して攻撃する。"
            },
            {
              "name": "きしかいせい",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "力をふりしぼり攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            },
            {
              "name": "かわらわり",
              "type": "Fighting",
              "category": "Physical",
              "power": 75,
              "description": "手刀を勢いよく振りおろして相手を攻撃する。ひかりのかべやリフレクターなども破壊できる。"
            }
          ],
          "actions": [
            "体力50% 弱体解除",
            "体力40% はいすいのじん",
            "体力20% 強化解除"
          ],
          "hp": 3640,
          "atk": 155,
          "def": 155,
          "sp_atk": 110,
          "sp_def": 95,
          "speed": 117
        },
        {
          "name": "タルップル",
          "star": 5,
          "types": [
            "Grass",
            "Dragon"
          ],
          "abilities": [
            {
              "name": "じゅくせい",
              "description": "熟成させることできのみの効果が倍になる。"
            },
            {
              "name": "くいしんぼう",
              "description": "ＨＰが少なくなったら食べるきのみをＨＰ半分の時に食べてしまう。"
            },
            {
              "name": "あついしぼう",
              "description": "厚い脂肪で守られているのでほのおタイプとこおりタイプの技のダメージを半減させる。"
            }
          ],
          "moves": [
            {
              "name": "りんごさん",
              "type": "Grass",
              "category": "Special",
              "power": 80,
              "description": "すっぱいりんごからつくりだした酸性の液体で攻撃。相手の特防を下げる。"
            },
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "ギガドレイン",
              "type": "Grass",
              "category": "Special",
              "power": 75,
              "description": "養分を吸い取り攻撃する。与えたダメージの半分のＨＰを回復できる。"
            },
            {
              "name": "ボディプレス",
              "type": "Fighting",
              "category": "Physical",
              "power": 80,
              "description": "体をぶつけて攻撃。防御が高いほど与えるダメージが増える。"
            }
          ],
          "actions": [
            "時間85 行動3",
            "体力75% せいちょう",
            "体力50% 弱体解除",
            "体力45% テラ回収",
            "体力20% 強化解除"
          ],
          "hp": 5000,
          "atk": 132,
          "def": 125,
          "sp_atk": 155,
          "sp_def": 125,
          "speed": 50
        },
        {
          "name": "ダイオウドウ",
          "star": 5,
          "types": [
            "Steel"
          ],
          "abilities": [
            {
              "name": "ちからずく",
              "description": "技の追加効果はなくなるがそのぶん高い威力で技を出すことができる。"
            },
            {
              "name": "ヘヴィメタル",
              "description": "自分の重さが２倍になる。"
            }
          ],
          "moves": [
            {
              "name": "ヘビーボンバー",
              "type": "Steel",
              "category": "Physical",
              "power": 1,
              "description": "重たい体で相手にぶつかって攻撃する。自分が相手より重いほど威力があがる。"
            },
            {
              "name": "かいりき",
              "type": "Normal",
              "category": "Physical",
              "power": 80,
              "description": "こん身の力で相手をなぐりつけて攻撃する。"
            },
            {
              "name": "のろい",
              "type": "Ghost",
              "category": "Status",
              "power": 0,
              "description": "使うポケモンがゴーストタイプとそれ以外とでは効果が変わる。"
            },
            {
              "name": "１０まんばりき",
              "type": "Ground",
              "category": "Physical",
              "power": 95,
              "description": "全身を使って相手に猛アタックする。"
            }
          ],
          "actions": [
            "時間80% すなあらし",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力40% 強化解除",
            "体力30% てっぺき"
          ],
          "hp": 5360,
          "atk": 200,
          "def": 108,
          "sp_atk": 125,
          "sp_def": 108,
          "speed": 50
        },
        {
          "name": "チルタリス",
          "star": 5,
          "types": [
            "Dragon",
            "Flying"
          ],
          "abilities": [
            {
              "name": "しぜんかいふく",
              "description": "手持ちにひっこむと状態異常が治る。"
            },
            {
              "name": "ノーてんき",
              "description": "あらゆる天気の影響がなくなってしまう。"
            }
          ],
          "moves": [
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "ぼうふう",
              "type": "Flying",
              "category": "Special",
              "power": 110,
              "description": "強烈な風で相手を包みこんで攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "うたう",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "心地好いきれいな歌声を聞かせて相手を眠り状態にする。"
            },
            {
              "name": "しろいきり",
              "type": "Ice",
              "category": "Status",
              "power": 0,
              "description": "白い霧で体をおおう。５ターンの間相手に能力をさげられなくなる。"
            }
          ],
          "actions": [
            "時間85% しんぴのまもり",
            "体力75 行動3",
            "体力50% 弱体解除",
            "体力45% コットンガード",
            "体力25% うたう"
          ],
          "hp": 3940,
          "atk": 110,
          "def": 140,
          "sp_atk": 110,
          "sp_def": 162,
          "speed": 125
        },
        {
          "name": "デカヌチャン",
          "star": 5,
          "types": [
            "Fairy",
            "Steel"
          ],
          "abilities": [
            {
              "name": "かたやぶり",
              "description": "相手の特性にジャマされることなく相手に技を出すことができる。"
            },
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "わるいてぐせ",
              "description": "触られた相手の道具を盗んでしまう。"
            }
          ],
          "moves": [
            {
              "name": "デカハンマー",
              "type": "Steel",
              "category": "Physical",
              "power": 160,
              "description": "大きなハンマーを体ごとぶんまわして攻撃する。この技は２回連続でだせない。"
            },
            {
              "name": "じゃれつく",
              "type": "Fairy",
              "category": "Physical",
              "power": 90,
              "description": "相手にじゃれついて攻撃する。相手の攻撃をさげることがある。"
            },
            {
              "name": "ぶんまわす",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "自分の体をぶんまわして相手にダメージを与える。"
            },
            {
              "name": "いわくだき",
              "type": "Fighting",
              "category": "Physical",
              "power": 40,
              "description": "パンチで攻撃する。相手の防御をさげることがある。"
            }
          ],
          "actions": [
            "時間80% ミストフィールド",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力40% でんじは",
            "体力25% あまえる"
          ],
          "hp": 4240,
          "atk": 117,
          "def": 120,
          "sp_atk": 110,
          "sp_def": 162,
          "speed": 146
        },
        {
          "name": "デリバード",
          "star": 5,
          "types": [
            "Ice",
            "Flying"
          ],
          "abilities": [
            {
              "name": "やるき",
              "description": "やる気をだすことによってねむり状態にならない。"
            },
            {
              "name": "はりきり",
              "description": "自分の攻撃が高くなるが命中率が下がる。"
            },
            {
              "name": "ふみん",
              "description": "眠れない体質なのでねむり状態にならない。"
            }
          ],
          "moves": [
            {
              "name": "プレゼント",
              "type": "Normal",
              "category": "Physical",
              "power": 1,
              "description": "わなをしかけた箱を相手にわたして攻撃する。ＨＰが回復してしまうこともある。"
            },
            {
              "name": "ドリルくちばし",
              "type": "Flying",
              "category": "Physical",
              "power": 80,
              "description": "回転しながらとがったくちばしを相手に突き刺して攻撃する。"
            },
            {
              "name": "れいとうパンチ",
              "type": "Ice",
              "category": "Physical",
              "power": 75,
              "description": "冷気をこめたパンチで相手を攻撃する。こおり状態にすることがある。"
            },
            {
              "name": "ふぶき",
              "type": "Ice",
              "category": "Special",
              "power": 110,
              "description": "激しい吹雪を相手に吹きつけて攻撃する。こおり状態にすることがある。"
            }
          ],
          "actions": [
            "時間85% ゆきげしき",
            "体力75% プレゼント",
            "体力50% 弱体解除",
            "体力45% 強化解除",
            "体力35% プレゼント"
          ],
          "hp": 3040,
          "atk": 87,
          "def": 72,
          "sp_atk": 102,
          "sp_def": 72,
          "speed": 117
        },
        {
          "name": "ドドゲザン",
          "star": 5,
          "types": [
            "Dark",
            "Steel"
          ],
          "abilities": [
            {
              "name": "まけんき",
              "description": "相手に能力を下げられると攻撃がぐーんと上がる。"
            },
            {
              "name": "そうだいしょう",
              "description": "登場したとき今まで倒された味方の数が多いほど少しずつ攻撃と特攻が上がる。"
            },
            {
              "name": "プレッシャー",
              "description": "プレッシャーをあたえて相手の使う技のＰＰを多く減らす。"
            }
          ],
          "moves": [
            {
              "name": "アイアンヘッド",
              "type": "Steel",
              "category": "Physical",
              "power": 80,
              "description": "鋼のような硬い頭で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "つじぎり",
              "type": "Dark",
              "category": "Physical",
              "power": 70,
              "description": "一瞬のすきをついて相手を切りはらう。急所に当たりやすい。"
            },
            {
              "name": "いちゃもん",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手にいちゃもんをつけて同じ技を２回連続でだせなくする。"
            },
            {
              "name": "きりさく",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。"
            }
          ],
          "actions": [
            "時間80% ちょうはつ",
            "時間15% メタルバースト",
            "体力75% 強化解除",
            "体力50% 弱体解除",
            "体力40% いちゃもん"
          ],
          "hp": 4700,
          "atk": 207,
          "def": 185,
          "sp_atk": 95,
          "sp_def": 132,
          "speed": 80
        },
        {
          "name": "ドラパルト",
          "star": 5,
          "types": [
            "Dragon",
            "Ghost"
          ],
          "abilities": [
            {
              "name": "クリアボディ",
              "description": "相手の技や特性で能力を下げられない。"
            },
            {
              "name": "すりぬけ",
              "description": "相手の壁や身代わりをすりぬけて攻撃できる。"
            },
            {
              "name": "のろわれボディ",
              "description": "攻撃を受けると相手の技をかなしばり状態にすることがある。"
            }
          ],
          "moves": [
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "ドラゴンアロー",
              "type": "Dragon",
              "category": "Physical",
              "power": 50,
              "description": "ドラメシヤで２回攻撃。相手が２匹いるときはそれぞれに１回ずつ攻撃する。"
            },
            {
              "name": "１０まんボルト",
              "type": "Electric",
              "category": "Special",
              "power": 90,
              "description": "強い電撃を相手に浴びせて攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "たたりめ",
              "type": "Ghost",
              "category": "Special",
              "power": 65,
              "description": "たたみかけるように攻撃する。状態異常の相手に大きなダメージを与える。"
            }
          ],
          "actions": [
            "時間85% リフレクター",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力40% 強化解除",
            "体力20% ひかりのかべ"
          ],
          "hp": 4340,
          "atk": 185,
          "def": 117,
          "sp_atk": 155,
          "sp_def": 117,
          "speed": 218
        },
        {
          "name": "ドラミドロ",
          "star": 5,
          "types": [
            "Poison",
            "Dragon"
          ],
          "abilities": [
            {
              "name": "どくのトゲ",
              "description": "自分に触った相手をどく状態にすることがある。"
            },
            {
              "name": "どくしゅ",
              "description": "触るだけで相手をどく状態にすることがある。"
            },
            {
              "name": "てきおうりょく",
              "description": "自分とおなじタイプの技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "ヘドロばくだん",
              "type": "Poison",
              "category": "Special",
              "power": 90,
              "description": "汚いヘドロを相手に投げつけて攻撃する。毒状態にすることがある。"
            },
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "どくどく",
              "type": "Poison",
              "category": "Status",
              "power": 0,
              "description": "相手を猛毒の状態にする。ターンがすすむほど毒のダメージが増えていく。"
            }
          ],
          "actions": [
            "時間85% アシッドボム",
            "体力75% りゅうせいぐん",
            "体力50% 弱体解除",
            "体力45% テラ回収",
            "体力30% りゅうせいぐん"
          ],
          "hp": 3640,
          "atk": 117,
          "def": 140,
          "sp_atk": 150,
          "sp_def": 189,
          "speed": 71
        },
        {
          "name": "ドンカラス",
          "star": 5,
          "types": [
            "Dark",
            "Flying"
          ],
          "abilities": [
            {
              "name": "ふみん",
              "description": "眠れない体質なのでねむり状態にならない。"
            },
            {
              "name": "きょううん",
              "description": "強運を持っているため相手の急所に攻撃が当たりやすい。"
            },
            {
              "name": "じしんかじょう",
              "description": "相手を倒すと自信がついて攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "つじぎり",
              "type": "Dark",
              "category": "Physical",
              "power": 70,
              "description": "一瞬のすきをついて相手を切りはらう。急所に当たりやすい。"
            },
            {
              "name": "ぼうふう",
              "type": "Flying",
              "category": "Special",
              "power": 110,
              "description": "強烈な風で相手を包みこんで攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "くろいきり",
              "type": "Ice",
              "category": "Status",
              "power": 0,
              "description": "黒い霧をだして戦闘にでているポケモン全員の能力変化をもとにもどす。"
            },
            {
              "name": "つばさでうつ",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "大きくひろげたりっぱな翼を相手にぶつけて攻撃する。"
            }
          ],
          "actions": [
            "時間85% テラ回収",
            "体力75% 強化解除",
            "体力50% 弱体解除",
            "体力50% テラ回収",
            "体力25% わるだくみ"
          ],
          "hp": 4700,
          "atk": 192,
          "def": 83,
          "sp_atk": 162,
          "sp_def": 83,
          "speed": 111
        },
        {
          "name": "ドータクン",
          "star": 5,
          "types": [
            "Steel",
            "Psychic"
          ],
          "abilities": [
            {
              "name": "ふゆう",
              "description": "地面から浮くことによってじめんタイプの技を受けない。"
            },
            {
              "name": "たいねつ",
              "description": "耐熱の体によってほのおタイプの技のダメージを半減させる。"
            },
            {
              "name": "ヘヴィメタル",
              "description": "自分の重さが２倍になる。"
            }
          ],
          "moves": [
            {
              "name": "ラスターカノン",
              "type": "Steel",
              "category": "Special",
              "power": 80,
              "description": "体の光を一点に集めて力を放つ。相手の特防をさげることがある。"
            },
            {
              "name": "じんつうりき",
              "type": "Psychic",
              "category": "Special",
              "power": 80,
              "description": "みえない不思議な力を送って攻撃する。相手をひるませることがある。"
            },
            {
              "name": "きんぞくおん",
              "type": "Steel",
              "category": "Status",
              "power": 0,
              "description": "金属をこすってでるようないやな音を聞かせる。相手の特防をがくっとさげる。"
            },
            {
              "name": "しっぺがえし",
              "type": "Dark",
              "category": "Physical",
              "power": 50,
              "description": "ためこんで攻撃する。相手よりあとに攻撃できると技の威力は２倍になる。"
            }
          ],
          "actions": [
            "時間85% あまごい",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力45% めいそう",
            "体力25% リフレクター"
          ],
          "hp": 3700,
          "atk": 138,
          "def": 179,
          "sp_atk": 123,
          "sp_def": 179,
          "speed": 54
        },
        {
          "name": "ナゲツケサル",
          "star": 5,
          "types": [
            "Fighting"
          ],
          "abilities": [
            {
              "name": "レシーバー",
              "description": "倒された味方の特性を受け継いで同じ特性になる。"
            },
            {
              "name": "まけんき",
              "description": "相手に能力を下げられると攻撃がぐーんと上がる。"
            }
          ],
          "moves": [
            {
              "name": "きしかいせい",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "力をふりしぼり攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            },
            {
              "name": "いわくだき",
              "type": "Fighting",
              "category": "Physical",
              "power": 40,
              "description": "パンチで攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "からげんき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "自分が毒まひやけど状態のとき相手にくりだすと技の威力が２倍になる。"
            },
            {
              "name": "ダストシュート",
              "type": "Poison",
              "category": "Physical",
              "power": 120,
              "description": "汚いゴミを相手にぶつけて攻撃する。毒状態にすることがある。"
            }
          ],
          "actions": [
            "時間80% ちょうはつ",
            "体力75% くさわけ",
            "体力50% 弱体解除",
            "体力40% 強化解除",
            "体力35% ビルドアップ"
          ],
          "hp": 4700,
          "atk": 185,
          "def": 140,
          "sp_atk": 65,
          "sp_def": 95,
          "speed": 125
        },
        {
          "name": "ヌメルゴン",
          "star": 5,
          "types": [
            "Dragon"
          ],
          "abilities": [
            {
              "name": "そうしょく",
              "description": "くさタイプの技を受けるとダメージを受けずに攻撃が上がる。"
            },
            {
              "name": "うるおいボディ",
              "description": "天気が雨のとき状態異常が治る。"
            },
            {
              "name": "ぬめぬめ",
              "description": "攻撃で自分に触れた相手の素早さを下げる。"
            }
          ],
          "moves": [
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "ヘドロばくだん",
              "type": "Poison",
              "category": "Special",
              "power": 90,
              "description": "汚いヘドロを相手に投げつけて攻撃する。毒状態にすることがある。"
            },
            {
              "name": "パワーウィップ",
              "type": "Grass",
              "category": "Physical",
              "power": 120,
              "description": "ツタや触手を激しくふるって相手をたたきつけ攻撃する。"
            }
          ],
          "actions": [
            "時間85% あまごい",
            "体力75% りゅうせいぐん",
            "体力50% 弱体解除",
            "体力45% 強化解除",
            "体力30% とける"
          ],
          "hp": 4400,
          "atk": 155,
          "def": 110,
          "sp_atk": 170,
          "sp_def": 230,
          "speed": 125
        },
        {
          "name": "ハッサム",
          "star": 5,
          "types": [
            "Bug",
            "Steel"
          ],
          "abilities": [
            {
              "name": "むしのしらせ",
              "description": "ＨＰが減ったときむしタイプの技の威力が上がる。"
            },
            {
              "name": "テクニシャン",
              "description": "威力が低い技の威力を高くして攻撃できる。"
            },
            {
              "name": "ライトメタル",
              "description": "自分の重さが半分になる。"
            }
          ],
          "moves": [
            {
              "name": "アイアンヘッド",
              "type": "Steel",
              "category": "Physical",
              "power": 80,
              "description": "鋼のような硬い頭で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "シザークロス",
              "type": "Bug",
              "category": "Physical",
              "power": 80,
              "description": "カマやツメをハサミのように交差させながら相手を切り裂く。"
            },
            {
              "name": "バレットパンチ",
              "type": "Steel",
              "category": "Physical",
              "power": 40,
              "description": "弾丸のような速くて硬いパンチを相手にくりだす。必ず先制攻撃できる。"
            },
            {
              "name": "きりさく",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。"
            }
          ],
          "actions": [
            "時間85% てっぺき",
            "体力75% きあいだめ",
            "体力50% 弱体解除",
            "体力45% 強化解除"
          ],
          "hp": 3800,
          "atk": 200,
          "def": 155,
          "sp_atk": 87,
          "sp_def": 125,
          "speed": 102
        },
        {
          "name": "ハピナス",
          "star": 5,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "しぜんかいふく",
              "description": "手持ちにひっこむと状態異常が治る。"
            },
            {
              "name": "てんのめぐみ",
              "description": "天の恵みのおかげで技の追加効果がでやすい。"
            },
            {
              "name": "いやしのこころ",
              "description": "状態異常の味方をたまに治してあげる。"
            }
          ],
          "moves": [
            {
              "name": "マジカルシャイン",
              "type": "Fairy",
              "category": "Special",
              "power": 80,
              "description": "強力な光を放ち相手にダメージを与える。"
            },
            {
              "name": "ハイパーボイス",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "うるさく響く大きな振動を相手に与えて攻撃する。"
            },
            {
              "name": "うたう",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "心地好いきれいな歌声を聞かせて相手を眠り状態にする。"
            },
            {
              "name": "ちきゅうなげ",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "引力を使い投げとばす。自分のレベルと同じダメージを相手に与える。"
            }
          ],
          "actions": [
            "時間85% じゅうりょく",
            "体力75% 弱体解除",
            "体力50% 弱体解除",
            "体力25% 強化解除"
          ],
          "hp": 9340,
          "atk": 20,
          "def": 20,
          "sp_atk": 117,
          "sp_def": 207,
          "speed": 87
        },
        {
          "name": "ハリテヤマ",
          "star": 5,
          "types": [
            "Fighting"
          ],
          "abilities": [
            {
              "name": "あついしぼう",
              "description": "厚い脂肪で守られているのでほのおタイプとこおりタイプの技のダメージを半減させる。"
            },
            {
              "name": "こんじょう",
              "description": "状態異常になると根性をだして攻撃が上がる。"
            },
            {
              "name": "ちからずく",
              "description": "技の追加効果はなくなるがそのぶん高い威力で技を出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "きしかいせい",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "力をふりしぼり攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            },
            {
              "name": "かわらわり",
              "type": "Fighting",
              "category": "Physical",
              "power": 75,
              "description": "手刀を勢いよく振りおろして相手を攻撃する。ひかりのかべやリフレクターなども破壊できる。"
            },
            {
              "name": "しおみず",
              "type": "Water",
              "category": "Special",
              "power": 65,
              "description": "相手がＨＰの半分くらいきずをおっていると技の威力が２倍になる。"
            },
            {
              "name": "ヘビーボンバー",
              "type": "Steel",
              "category": "Physical",
              "power": 1,
              "description": "重たい体で相手にぶつかって攻撃する。自分が相手より重いほど威力があがる。"
            }
          ],
          "actions": [
            "時間85% こわいかお",
            "体力75% ちょうはつ",
            "体力50% 弱体解除",
            "体力45% ビルドアップ",
            "体力30% 強化解除"
          ],
          "hp": 6020,
          "atk": 185,
          "def": 95,
          "sp_atk": 65,
          "sp_def": 95,
          "speed": 80
        },
        {
          "name": "ハルクジラ",
          "star": 5,
          "types": [
            "Ice"
          ],
          "abilities": [
            {
              "name": "あついしぼう",
              "description": "厚い脂肪で守られているのでほのおタイプとこおりタイプの技のダメージを半減させる。"
            },
            {
              "name": "ゆきかき",
              "description": "天気がゆきのとき素早さが上がる。"
            },
            {
              "name": "ちからずく",
              "description": "技の追加効果はなくなるがそのぶん高い威力で技を出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "アイススピナー",
              "type": "Ice",
              "category": "Physical",
              "power": 80,
              "description": "足に薄い氷をまといクルクルと回りながらぶつかる。回転の動きによってフィールドを壊す。"
            },
            {
              "name": "アクアブレイク",
              "type": "Water",
              "category": "Physical",
              "power": 85,
              "description": "水の力で相手にぶつかって攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            },
            {
              "name": "なかまづくり",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "不思議なリズムでおどる。動きをまねさせて自分と相手の特性を同じにする。"
            }
          ],
          "actions": [
            "時間80% ゆきげしき",
            "体力75% あくび",
            "体力50% 弱体解除",
            "体力40% ゆきげしき",
            "体力20% 強化解除"
          ],
          "hp": 6800,
          "atk": 174,
          "def": 102,
          "sp_atk": 72,
          "sp_def": 87,
          "speed": 114
        },
        {
          "name": "バクーダ",
          "star": 5,
          "types": [
            "Fire",
            "Ground"
          ],
          "abilities": [
            {
              "name": "マグマのよろい",
              "description": "熱いマグマを身にまといこおり状態にならない。"
            },
            {
              "name": "ハードロック",
              "description": "効果バツグンになってしまう攻撃の威力を弱めることができる。"
            },
            {
              "name": "いかりのつぼ",
              "description": "急所に攻撃が当たると怒りくるって攻撃力が最大になる。"
            }
          ],
          "moves": [
            {
              "name": "かえんほうしゃ",
              "type": "Fire",
              "category": "Special",
              "power": 90,
              "description": "激しい炎を相手に発射して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "だいちのちから",
              "type": "Ground",
              "category": "Special",
              "power": 90,
              "description": "相手の足下へ大地の力を放出する。相手の特防をさげることがある。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            },
            {
              "name": "ふんか",
              "type": "Fire",
              "category": "Special",
              "power": 150,
              "description": "怒りを爆発させて相手を攻撃する。自分のＨＰが少ないほど技の威力はさがる。"
            }
          ],
          "actions": [
            "時間85% にほんばれ",
            "体力75% 強化解除",
            "体力50% 弱体解除",
            "体力50% テラ回収",
            "体力35% にほんばれ"
          ],
          "hp": 3800,
          "atk": 155,
          "def": 110,
          "sp_atk": 162,
          "sp_def": 117,
          "speed": 65
        },
        {
          "name": "バチンウニ",
          "star": 5,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "ひらいしん",
              "description": "でんきタイプの技を自分に寄せつけダメージを受けずに特攻が上がる。"
            },
            {
              "name": "エレキメイカー",
              "description": "登場したときにエレキフィールドをはりめぐらせる。"
            }
          ],
          "moves": [
            {
              "name": "びりびりちくちく",
              "type": "Electric",
              "category": "Physical",
              "power": 80,
              "description": "相手にぶつかって強力な電気を浴びせびりびりちくちくさせる。相手をひるませることがある。"
            },
            {
              "name": "かみなり",
              "type": "Electric",
              "category": "Special",
              "power": 110,
              "description": "激しい雷を相手に落として攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "なみのり",
              "type": "Water",
              "category": "Special",
              "power": 90,
              "description": "大きな波で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "どくづき",
              "type": "Poison",
              "category": "Physical",
              "power": 80,
              "description": "毒にそまった触手や腕で相手を突き刺す。毒状態にすることがある。"
            }
          ],
          "actions": [
            "時間80% あまごい",
            "体力50% 弱体解除",
            "体力40% 強化解除",
            "体力25% エレキフィールド"
          ],
          "hp": 3140,
          "atk": 156,
          "def": 147,
          "sp_atk": 141,
          "sp_def": 132,
          "speed": 27
        },
        {
          "name": "バンギラス",
          "star": 5,
          "types": [
            "Rock",
            "Dark"
          ],
          "abilities": [
            {
              "name": "すなおこし",
              "description": "登場したとき天気を砂あらしにする。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            }
          ],
          "moves": [
            {
              "name": "いわなだれ",
              "type": "Rock",
              "category": "Physical",
              "power": 75,
              "description": "大きな岩を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "いやなおと",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "おもわず耳をふさぎたくなるいやなおとをだして相手の防御をがくっとさげる。"
            },
            {
              "name": "あくのはどう",
              "type": "Dark",
              "category": "Special",
              "power": 80,
              "description": "体から悪意にみちた恐ろしいオーラを発する。相手をひるませることがある。"
            }
          ],
          "actions": [
            "時間85% テラ回収",
            "体力75% 弱体解除",
            "体力50% 弱体解除",
            "体力45% りゅうのまい",
            "体力35% すなあらし"
          ],
          "hp": 4700,
          "atk": 206,
          "def": 170,
          "sp_atk": 147,
          "sp_def": 155,
          "speed": 96
        },
        {
          "name": "バンバドロ",
          "star": 5,
          "types": [
            "Ground"
          ],
          "abilities": [
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "じきゅうりょく",
              "description": "攻撃を受けると防御が上がる。"
            },
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            }
          ],
          "moves": [
            {
              "name": "１０まんばりき",
              "type": "Ground",
              "category": "Physical",
              "power": 95,
              "description": "全身を使って相手に猛アタックする。"
            },
            {
              "name": "ボディプレス",
              "type": "Fighting",
              "category": "Physical",
              "power": 80,
              "description": "体をぶつけて攻撃。防御が高いほど与えるダメージが増える。"
            },
            {
              "name": "いわくだき",
              "type": "Fighting",
              "category": "Physical",
              "power": 40,
              "description": "パンチで攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "ヘビーボンバー",
              "type": "Steel",
              "category": "Physical",
              "power": 1,
              "description": "重たい体で相手にぶつかって攻撃する。自分が相手より重いほど威力があがる。"
            }
          ],
          "actions": [
            "時間80% こわいかお",
            "時間75% てっぺき",
            "体力50% 弱体解除",
            "体力40% 強化解除"
          ],
          "hp": 4700,
          "atk": 192,
          "def": 155,
          "sp_atk": 87,
          "sp_def": 132,
          "speed": 57
        },
        {
          "name": "パルシェン",
          "star": 5,
          "types": [
            "Water",
            "Ice"
          ],
          "abilities": [
            {
              "name": "シェルアーマー",
              "description": "硬い殻に守られ相手の攻撃が急所に当たらない。"
            },
            {
              "name": "スキルリンク",
              "description": "連続技を使うといつも最高回数出すことができる。"
            },
            {
              "name": "ぼうじん",
              "description": "すなあらしのダメージを受けない。粉や胞子の影響も受けない。"
            }
          ],
          "moves": [
            {
              "name": "つららばり",
              "type": "Ice",
              "category": "Physical",
              "power": 25,
              "description": "鋭い氷柱を相手に発射して攻撃する。２ー５回の間連続でだす。"
            },
            {
              "name": "ハイドロポンプ",
              "type": "Water",
              "category": "Special",
              "power": 110,
              "description": "大量の水を激しい勢いで相手に発射して攻撃する。"
            },
            {
              "name": "こおりのつぶて",
              "type": "Ice",
              "category": "Physical",
              "power": 40,
              "description": "氷の塊を一瞬でつくり相手に素早く放つ。必ず先制攻撃できる。"
            },
            {
              "name": "ちょうおんぱ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "特殊な音波を体から発して相手を混乱させる。"
            }
          ],
          "actions": [
            "時間85% テラ回収",
            "体力50% 弱体解除",
            "体力35% からをやぶる",
            "体力30% 弱体解除"
          ],
          "hp": 3200,
          "atk": 147,
          "def": 275,
          "sp_atk": 132,
          "sp_def": 72,
          "speed": 110
        },
        {
          "name": "パーモット",
          "star": 5,
          "types": [
            "Electric",
            "Fighting"
          ],
          "abilities": [
            {
              "name": "ちくでん",
              "description": "でんきタイプの技を受けるとダメージを受けずに回復する。"
            },
            {
              "name": "しぜんかいふく",
              "description": "手持ちにひっこむと状態異常が治る。"
            },
            {
              "name": "てつのこぶし",
              "description": "パンチを使う技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ワイルドボルト",
              "type": "Electric",
              "category": "Physical",
              "power": 90,
              "description": "電気をまとって相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "インファイト",
              "type": "Fighting",
              "category": "Physical",
              "power": 120,
              "description": "守りを捨てて相手のふところに突撃する。自分の防御と特防がさがる。"
            },
            {
              "name": "ほっぺすりすり",
              "type": "Electric",
              "category": "Physical",
              "power": 20,
              "description": "電気を帯びたほっぺをすりつけて攻撃。相手をまひ状態にする。"
            },
            {
              "name": "てんしのキッス",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "天使のようにかわいくキスして相手を混乱させる。"
            }
          ],
          "actions": [
            "時間80% ほっぺすりすり",
            "体力75% 強化解除",
            "体力50% 弱体解除",
            "体力40% エレキフィールド",
            "体力20% でんこうそうげき"
          ],
          "hp": 3800,
          "atk": 177,
          "def": 110,
          "sp_atk": 110,
          "sp_def": 95,
          "speed": 162
        },
        {
          "name": "ファイアロー",
          "star": 5,
          "types": [
            "Fire",
            "Flying"
          ],
          "abilities": [
            {
              "name": "ほのおのからだ",
              "description": "自分に触った相手をやけど状態にすることがある。"
            },
            {
              "name": "はやてのつばさ",
              "description": "ＨＰが満タンだとひこうタイプの技を先制で出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "アクロバット",
              "type": "Flying",
              "category": "Physical",
              "power": 55,
              "description": "軽やかに相手を攻撃する。自分が道具を持っていないとき大きなダメージを与える。"
            },
            {
              "name": "フレアドライブ",
              "type": "Fire",
              "category": "Physical",
              "power": 120,
              "description": "炎をまとって突進する。自分もかなりダメージを受ける。やけど状態にすることがある。"
            },
            {
              "name": "はがねのつばさ",
              "type": "Steel",
              "category": "Physical",
              "power": 70,
              "description": "硬い翼を相手にたたきつけて攻撃する。自分の防御があがることがある。"
            },
            {
              "name": "ねっぷう",
              "type": "Fire",
              "category": "Special",
              "power": 95,
              "description": "熱い息を相手に吹きつけて攻撃する。やけど状態にすることがある。"
            }
          ],
          "actions": [
            "時間85% ビルドアップ",
            "体力75% 強化解除",
            "体力50% 弱体解除",
            "体力45% テラ回収",
            "体力30% ビルドアップ"
          ],
          "hp": 4040,
          "atk": 126,
          "def": 111,
          "sp_atk": 116,
          "sp_def": 108,
          "speed": 194
        },
        {
          "name": "フラージェス",
          "star": 5,
          "types": [
            "Fairy"
          ],
          "abilities": [
            {
              "name": "フラワーベール",
              "description": "味方の草ポケモンは能力が下がらず状態異常にもならない。"
            },
            {
              "name": "きょうせい",
              "description": "味方が道具を使うと自分の持っている道具を味方に渡す。"
            }
          ],
          "moves": [
            {
              "name": "はなびらのまい",
              "type": "Grass",
              "category": "Special",
              "power": 120,
              "description": "２ー３ターンの間花をまきちらして相手を攻撃する。まきちらしたあとは混乱する。"
            },
            {
              "name": "ムーンフォース",
              "type": "Fairy",
              "category": "Special",
              "power": 95,
              "description": "月のパワーをかりて相手を攻撃する。相手の特攻をさげることがある。"
            },
            {
              "name": "サイコキネシス",
              "type": "Psychic",
              "category": "Special",
              "power": 90,
              "description": "強い念力を相手に送って攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "しんぴのまもり",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "５ターンの間不思議な力に守られて状態異常にならなくなる。"
            }
          ],
          "actions": [
            "時間85% グラスフィールド",
            "体力75% めいそう",
            "体力50% 弱体解除",
            "体力45% テラ回収",
            "体力30% グラスフィールド"
          ],
          "hp": 4040,
          "atk": 102,
          "def": 107,
          "sp_atk": 173,
          "sp_def": 236,
          "speed": 117
        },
        {
          "name": "フワライド",
          "star": 5,
          "types": [
            "Ghost",
            "Flying"
          ],
          "abilities": [
            {
              "name": "ゆうばく",
              "description": "ひんしになったとき触った相手にダメージをあたえる。"
            },
            {
              "name": "かるわざ",
              "description": "持っていた道具がなくなると素早さが上がる。"
            },
            {
              "name": "ねつぼうそう",
              "description": "やけど状態になったとき特殊技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "たたりめ",
              "type": "Ghost",
              "category": "Special",
              "power": 65,
              "description": "たたみかけるように攻撃する。状態異常の相手に大きなダメージを与える。"
            },
            {
              "name": "エアスラッシュ",
              "type": "Flying",
              "category": "Special",
              "power": 75,
              "description": "空をも切り裂く空気の刃で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            },
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            }
          ],
          "actions": [
            "時間85% おにび",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力25% 強化解除"
          ],
          "hp": 6200,
          "atk": 125,
          "def": 71,
          "sp_atk": 140,
          "sp_def": 86,
          "speed": 125
        },
        {
          "name": "ブリムオン",
          "star": 5,
          "types": [
            "Psychic",
            "Fairy"
          ],
          "abilities": [
            {
              "name": "いやしのこころ",
              "description": "状態異常の味方をたまに治してあげる。"
            },
            {
              "name": "きけんよち",
              "description": "相手の持つ危険な技を察知することができる。"
            },
            {
              "name": "マジックミラー",
              "description": "相手にだされた変化技を受けずにそのまま返すことができる。"
            }
          ],
          "moves": [
            {
              "name": "マジカルシャイン",
              "type": "Fairy",
              "category": "Special",
              "power": 80,
              "description": "強力な光を放ち相手にダメージを与える。"
            },
            {
              "name": "サイコキネシス",
              "type": "Psychic",
              "category": "Special",
              "power": 90,
              "description": "強い念力を相手に送って攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "あくのはどう",
              "type": "Dark",
              "category": "Special",
              "power": 80,
              "description": "体から悪意にみちた恐ろしいオーラを発する。相手をひるませることがある。"
            },
            {
              "name": "あまえる",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。"
            }
          ],
          "actions": [
            "時間85% ミストフィールド",
            "体力75% めいそう",
            "体力50% 弱体解除",
            "体力40% めいそう",
            "体力20% サイコフィールド"
          ],
          "hp": 3400,
          "atk": 140,
          "def": 147,
          "sp_atk": 209,
          "sp_def": 159,
          "speed": 48
        },
        {
          "name": "ブロスター",
          "star": 5,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "メガランチャー",
              "description": "波動の技の威力が高くなる。"
            }
          ],
          "moves": [
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "はどうだん",
              "type": "Fighting",
              "category": "Special",
              "power": 80,
              "description": "体の奥から波導の力を相手にうち放つ。攻撃は必ず命中する。"
            },
            {
              "name": "クラブハンマー",
              "type": "Water",
              "category": "Physical",
              "power": 100,
              "description": "大きなハサミを相手にたたきつけて攻撃する。急所に当たりやすい。"
            }
          ],
          "actions": [
            "時間85% あまごい",
            "体力50% 弱体解除",
            "体力45% 強化解除",
            "体力30% みずのはどう"
          ],
          "hp": 3820,
          "atk": 114,
          "def": 137,
          "sp_atk": 185,
          "sp_def": 138,
          "speed": 93
        },
        {
          "name": "ブロロローム",
          "star": 5,
          "types": [
            "Steel",
            "Poison"
          ],
          "abilities": [
            {
              "name": "ぼうじん",
              "description": "すなあらしのダメージを受けない。粉や胞子の影響も受けない。"
            },
            {
              "name": "フィルター",
              "description": "効果バツグンになってしまう攻撃の威力を弱めることができる。"
            }
          ],
          "moves": [
            {
              "name": "ホイールスピン",
              "type": "Steel",
              "category": "Physical",
              "power": 100,
              "description": "足に負荷をかけることにより激しく回転してダメージを与える。自分の素早さががくっとさがる。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            },
            {
              "name": "ダストシュート",
              "type": "Poison",
              "category": "Physical",
              "power": 120,
              "description": "汚いゴミを相手にぶつけて攻撃する。毒状態にすることがある。"
            },
            {
              "name": "オーバーヒート",
              "type": "Fire",
              "category": "Special",
              "power": 130,
              "description": "フルパワーで相手を攻撃する。使うと反動で自分の特攻ががくっとさがる。"
            }
          ],
          "actions": [
            "時間85% こわいかお",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力40% ギアチェンジ",
            "体力20% ギアチェンジ"
          ],
          "hp": 4100,
          "atk": 183,
          "def": 140,
          "sp_atk": 86,
          "sp_def": 105,
          "speed": 140
        },
        {
          "name": "ヘイラッシャ",
          "star": 5,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "てんねん",
              "description": "相手の能力の変化を無視して攻撃ができる。"
            },
            {
              "name": "どんかん",
              "description": "鈍感なのでメロメロやちょうはつ状態にならない。いかくにも動じない。"
            },
            {
              "name": "みずのベール",
              "description": "水のベールを身にまといやけど状態にならない。"
            }
          ],
          "moves": [
            {
              "name": "いっちょうあがり",
              "type": "Dragon",
              "category": "Physical",
              "power": 80,
              "description": "いなせな身のこなしで攻撃。口の中にシャリタツがいるとそのすがたによって能力があがる。"
            },
            {
              "name": "たきのぼり",
              "type": "Water",
              "category": "Physical",
              "power": 80,
              "description": "すごい勢いで相手につっこむ。相手をひるませることがある。"
            },
            {
              "name": "ヘビーボンバー",
              "type": "Steel",
              "category": "Physical",
              "power": 1,
              "description": "重たい体で相手にぶつかって攻撃する。自分が相手より重いほど威力があがる。"
            },
            {
              "name": "くすぐる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体をくすぐり笑わせることで相手の攻撃と防御をさげる。"
            }
          ],
          "actions": [
            "時間85% あまごい",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力40% たくわえる",
            "体力20% 強化解除"
          ],
          "hp": 6200,
          "atk": 155,
          "def": 177,
          "sp_atk": 102,
          "sp_def": 102,
          "speed": 57
        },
        {
          "name": "ヘルガー",
          "star": 5,
          "types": [
            "Dark",
            "Fire"
          ],
          "abilities": [
            {
              "name": "はやおき",
              "description": "ねむり状態になっても２倍の早さで目覚めることができる。"
            },
            {
              "name": "もらいび",
              "description": "ほのおタイプの技を受けると炎をもらい自分が出すほのおタイプの技が強くなる。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            }
          ],
          "moves": [
            {
              "name": "かえんほうしゃ",
              "type": "Fire",
              "category": "Special",
              "power": 90,
              "description": "激しい炎を相手に発射して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            },
            {
              "name": "おにび",
              "type": "Fire",
              "category": "Status",
              "power": 0,
              "description": "不気味で怪しい炎を放って相手をやけどの状態にする。"
            }
          ],
          "actions": [
            "時間85% にほんばれ",
            "体力75% とおぼえ",
            "体力50% 弱体解除",
            "体力50% テラ回収",
            "体力35% にほんばれ"
          ],
          "hp": 3940,
          "atk": 140,
          "def": 80,
          "sp_atk": 170,
          "sp_def": 125,
          "speed": 147
        },
        {
          "name": "ボーマンダ",
          "star": 5,
          "types": [
            "Dragon",
            "Flying"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "じしんかじょう",
              "description": "相手を倒すと自信がついて攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ドラゴンダイブ",
              "type": "Dragon",
              "category": "Physical",
              "power": 100,
              "description": "すさまじい殺気で威圧しながら体当たりする。相手をひるませることがある。"
            },
            {
              "name": "つばめがえし",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "素早い動きで相手をほんろうして切りつける。攻撃は必ず命中する。"
            },
            {
              "name": "ハイパーボイス",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "うるさく響く大きな振動を相手に与えて攻撃する。"
            },
            {
              "name": "りゅうせいぐん",
              "type": "Dragon",
              "category": "Special",
              "power": 130,
              "description": "天空から隕石を相手に落とす。使うと反動で自分の特攻ががくっとさがる。"
            }
          ],
          "actions": [
            "時間85% テラ回収",
            "体力75% 強化解除",
            "体力50% 弱体解除",
            "体力45% りゅうのまい",
            "体力25% きあいだめ"
          ],
          "hp": 4540,
          "atk": 207,
          "def": 125,
          "sp_atk": 170,
          "sp_def": 125,
          "speed": 155
        },
        {
          "name": "ポットデス",
          "star": 5,
          "types": [
            "Ghost"
          ],
          "abilities": [
            {
              "name": "くだけるよろい",
              "description": "物理技でダメージを受けると防御が下がり素早さがぐーんと上がる。"
            },
            {
              "name": "のろわれボディ",
              "description": "攻撃を受けると相手の技をかなしばり状態にすることがある。"
            }
          ],
          "moves": [
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "メガドレイン",
              "type": "Grass",
              "category": "Special",
              "power": 40,
              "description": "養分を吸い取り攻撃する。相手に与えたダメージの半分のＨＰを回復できる。"
            },
            {
              "name": "おどろかす",
              "type": "Ghost",
              "category": "Physical",
              "power": 30,
              "description": "大きな声などで不意に驚かして攻撃する。相手をひるませることがある。"
            },
            {
              "name": "おにび",
              "type": "Fire",
              "category": "Status",
              "power": 0,
              "description": "不気味で怪しい炎を放って相手をやけどの状態にする。"
            }
          ],
          "actions": [
            "時間85% テラ回収",
            "体力50% 弱体解除",
            "体力35% からをやぶる",
            "体力30% 弱体解除"
          ],
          "hp": 3500,
          "atk": 102,
          "def": 102,
          "sp_atk": 206,
          "sp_def": 176,
          "speed": 110
        },
        {
          "name": "マニューラ",
          "star": 5,
          "types": [
            "Dark",
            "Ice"
          ],
          "abilities": [
            {
              "name": "プレッシャー",
              "description": "プレッシャーをあたえて相手の使う技のＰＰを多く減らす。"
            },
            {
              "name": "わるいてぐせ",
              "description": "触られた相手の道具を盗んでしまう。"
            }
          ],
          "moves": [
            {
              "name": "れいとうパンチ",
              "type": "Ice",
              "category": "Physical",
              "power": 75,
              "description": "冷気をこめたパンチで相手を攻撃する。こおり状態にすることがある。"
            },
            {
              "name": "つじぎり",
              "type": "Dark",
              "category": "Physical",
              "power": 70,
              "description": "一瞬のすきをついて相手を切りはらう。急所に当たりやすい。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            },
            {
              "name": "からげんき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "自分が毒まひやけど状態のとき相手にくりだすと技の威力が２倍になる。"
            }
          ],
          "actions": [
            "時間85% リフレクター",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力50% テラ回収",
            "体力25% つるぎのまい"
          ],
          "hp": 3800,
          "atk": 185,
          "def": 102,
          "sp_atk": 72,
          "sp_def": 132,
          "speed": 192
        },
        {
          "name": "マフィティフ",
          "star": 5,
          "types": [
            "Dark"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "ばんけん",
              "description": "いかくされると攻撃が上がる。ポケモンを入れ替えさせる技や道具が効かない。"
            },
            {
              "name": "はりこみ",
              "description": "交代で出てきた相手に２倍のダメージで攻撃できる。"
            }
          ],
          "moves": [
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "じゃれつく",
              "type": "Fairy",
              "category": "Physical",
              "power": 90,
              "description": "相手にじゃれついて攻撃する。相手の攻撃をさげることがある。"
            },
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "いばる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせて混乱させる。怒りで相手の攻撃はぐーんとあがってしまう。"
            }
          ],
          "actions": [
            "時間80% ちょうはつ",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力45% テラ回収",
            "体力20% 強化解除"
          ],
          "hp": 4100,
          "atk": 185,
          "def": 140,
          "sp_atk": 95,
          "sp_def": 110,
          "speed": 132
        },
        {
          "name": "ミミズズ",
          "star": 5,
          "types": [
            "Steel"
          ],
          "abilities": [
            {
              "name": "どしょく",
              "description": "じめんタイプの技を受けるとダメージを受けずに回復する。"
            },
            {
              "name": "すながくれ",
              "description": "砂あらしのとき回避率が上がる。"
            }
          ],
          "moves": [
            {
              "name": "アイアンヘッド",
              "type": "Steel",
              "category": "Physical",
              "power": 80,
              "description": "鋼のような硬い頭で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "じしん",
              "type": "Ground",
              "category": "Physical",
              "power": 100,
              "description": "地震の衝撃で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "じだんだ",
              "type": "Ground",
              "category": "Physical",
              "power": 75,
              "description": "悔しさをバネにして攻撃する。前のターンに技を外していると威力が倍になる。"
            },
            {
              "name": "まきつく",
              "type": "Normal",
              "category": "Physical",
              "power": 15,
              "description": "長い体やつるなどを使って４ー５ターンの間相手にまきついて攻撃する。"
            }
          ],
          "actions": [
            "時間85% すなあらし",
            "体力75% とぐろをまく",
            "体力50% 弱体解除",
            "体力45% 強化解除",
            "体力35% すなあらし"
          ],
          "hp": 3800,
          "atk": 132,
          "def": 222,
          "sp_atk": 95,
          "sp_def": 87,
          "speed": 102
        },
        {
          "name": "ミミッキュ",
          "star": 5,
          "types": [
            "Ghost",
            "Fairy"
          ],
          "abilities": [
            {
              "name": "ばけのかわ",
              "description": "体を被う化けの皮で１回攻撃を防ぐことができる。"
            }
          ],
          "moves": [
            {
              "name": "じゃれつく",
              "type": "Fairy",
              "category": "Physical",
              "power": 90,
              "description": "相手にじゃれついて攻撃する。相手の攻撃をさげることがある。"
            },
            {
              "name": "シャドークロー",
              "type": "Ghost",
              "category": "Physical",
              "power": 70,
              "description": "影からつくった鋭いツメで相手を切り裂く。急所に当たりやすい。"
            },
            {
              "name": "おにび",
              "type": "Fire",
              "category": "Status",
              "power": 0,
              "description": "不気味で怪しい炎を放って相手をやけどの状態にする。"
            },
            {
              "name": "かげうち",
              "type": "Ghost",
              "category": "Physical",
              "power": 40,
              "description": "影をのばして相手の背後から攻撃する。必ず先制攻撃できる。"
            }
          ],
          "actions": [
            "時間85% ひかりのかべ",
            "体力75% ちょうはつ",
            "体力50% 弱体解除",
            "体力40% おにび",
            "体力25% 強化解除"
          ],
          "hp": 3340,
          "atk": 140,
          "def": 125,
          "sp_atk": 80,
          "sp_def": 162,
          "speed": 149
        },
        {
          "name": "ムウマージ",
          "star": 5,
          "types": [
            "Ghost"
          ],
          "abilities": [
            {
              "name": "ふゆう",
              "description": "地面から浮くことによってじめんタイプの技を受けない。"
            }
          ],
          "moves": [
            {
              "name": "マジカルフレイム",
              "type": "Fire",
              "category": "Special",
              "power": 75,
              "description": "口から吐きだす特別熱い炎で攻撃する。相手の特攻をさげる。"
            },
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "あやしいひかり",
              "type": "Ghost",
              "category": "Status",
              "power": 0,
              "description": "怪しい光を相手にみせてまどわせる。相手を混乱させる。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            }
          ],
          "actions": [
            "時間85% ひかりのかべ",
            "体力75% 強化解除",
            "体力50% 弱体解除",
            "体力50% テラ回収",
            "体力25% わるだくみ"
          ],
          "hp": 3500,
          "atk": 95,
          "def": 95,
          "sp_atk": 162,
          "sp_def": 162,
          "speed": 162
        },
        {
          "name": "ムクホーク",
          "star": 5,
          "types": [
            "Normal",
            "Flying"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "すてみ",
              "description": "反動でダメージを受ける技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "インファイト",
              "type": "Fighting",
              "category": "Physical",
              "power": 120,
              "description": "守りを捨てて相手のふところに突撃する。自分の防御と特防がさがる。"
            },
            {
              "name": "ブレイブバード",
              "type": "Flying",
              "category": "Physical",
              "power": 120,
              "description": "はねをおりたたみ低空飛行で突撃する。自分もかなりダメージを受ける。"
            },
            {
              "name": "でんこうせっか",
              "type": "Normal",
              "category": "Physical",
              "power": 40,
              "description": "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。"
            },
            {
              "name": "すてみタックル",
              "type": "Normal",
              "category": "Physical",
              "power": 120,
              "description": "命を懸けて相手に突進して攻撃する。自分もかなりダメージを受ける。"
            }
          ],
          "actions": [
            "時間85% 弱体解除",
            "体力75% 強化解除",
            "体力50% 強化解除",
            "体力50% 弱体解除",
            "体力25% ブレイブバード"
          ],
          "hp": 4240,
          "atk": 185,
          "def": 110,
          "sp_atk": 80,
          "sp_def": 95,
          "speed": 155
        },
        {
          "name": "メタモン",
          "star": 5,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "じゅうなん",
              "description": "柔軟な体によってまひ状態にならない。"
            },
            {
              "name": "かわりもの",
              "description": "目の前のポケモンに変身してしまう。"
            }
          ],
          "moves": [
            {
              "name": "へんしん",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手のポケモンに変身することで相手とまったく同じ技が使える。"
            }
          ],
          "actions": [
            "体力50% 弱体解除"
          ],
          "hp": 3140,
          "atk": 77,
          "def": 77,
          "sp_atk": 77,
          "sp_def": 77,
          "speed": 77
        },
        {
          "name": "モロバレル",
          "star": 5,
          "types": [
            "Grass",
            "Poison"
          ],
          "abilities": [
            {
              "name": "ほうし",
              "description": "攻撃で自分に触れた相手をどくやまひやねむり状態にすることがある。"
            },
            {
              "name": "さいせいりょく",
              "description": "手持ちに引っ込むとＨＰが少し回復する。"
            }
          ],
          "moves": [
            {
              "name": "エナジーボール",
              "type": "Grass",
              "category": "Special",
              "power": 90,
              "description": "自然から集めた命の力を発射する。相手の特防をさげることがある。"
            },
            {
              "name": "ヘドロばくだん",
              "type": "Poison",
              "category": "Special",
              "power": 90,
              "description": "汚いヘドロを相手に投げつけて攻撃する。毒状態にすることがある。"
            },
            {
              "name": "キノコのほうし",
              "type": "Grass",
              "category": "Status",
              "power": 0,
              "description": "催眠効果のある胞子をパラパラとふりまき相手を眠り状態にする。"
            },
            {
              "name": "クリアスモッグ",
              "type": "Poison",
              "category": "Special",
              "power": 50,
              "description": "特殊な泥の塊を相手に投げつけて攻撃する。能力変化をもとにもどす。"
            }
          ],
          "actions": [
            "時間85% グラスフィールド",
            "体力75% キノコのほうし",
            "体力50% 弱体解除",
            "体力40% キノコのほうし",
            "体力25% 強化解除"
          ],
          "hp": 5120,
          "atk": 132,
          "def": 110,
          "sp_atk": 132,
          "sp_def": 125,
          "speed": 50
        },
        {
          "name": "ヤドキング",
          "star": 5,
          "types": [
            "Water",
            "Psychic"
          ],
          "abilities": [
            {
              "name": "どんかん",
              "description": "鈍感なのでメロメロやちょうはつ状態にならない。いかくにも動じない。"
            },
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "さいせいりょく",
              "description": "手持ちに引っ込むとＨＰが少し回復する。"
            }
          ],
          "moves": [
            {
              "name": "サイコキネシス",
              "type": "Psychic",
              "category": "Special",
              "power": 90,
              "description": "強い念力を相手に送って攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "なみのり",
              "type": "Water",
              "category": "Special",
              "power": 90,
              "description": "大きな波で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            },
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            }
          ],
          "actions": [
            "時間85% サイコフィールド",
            "体力70% あくび",
            "体力50% 弱体解除",
            "体力45% 強化解除",
            "体力35% めいそう"
          ],
          "hp": 4540,
          "atk": 117,
          "def": 125,
          "sp_atk": 155,
          "sp_def": 170,
          "speed": 50
        },
        {
          "name": "ヤドラン",
          "star": 5,
          "types": [
            "Water",
            "Psychic"
          ],
          "abilities": [
            {
              "name": "どんかん",
              "description": "鈍感なのでメロメロやちょうはつ状態にならない。いかくにも動じない。"
            },
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "さいせいりょく",
              "description": "手持ちに引っ込むとＨＰが少し回復する。"
            }
          ],
          "moves": [
            {
              "name": "しねんのずつき",
              "type": "Psychic",
              "category": "Physical",
              "power": 80,
              "description": "思念の力を額に集めて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "アクアブレイク",
              "type": "Water",
              "category": "Physical",
              "power": 85,
              "description": "水の力で相手にぶつかって攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            },
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            }
          ],
          "actions": [
            "時間85% のろい",
            "体力70% あくび",
            "体力50% 弱体解除",
            "体力45% 強化解除",
            "体力35% のろい"
          ],
          "hp": 4540,
          "atk": 117,
          "def": 170,
          "sp_atk": 155,
          "sp_def": 125,
          "speed": 50
        },
        {
          "name": "ヤミラミ",
          "star": 5,
          "types": [
            "Dark",
            "Ghost"
          ],
          "abilities": [
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            },
            {
              "name": "あとだし",
              "description": "技を出す順番がかならず最後になる。"
            },
            {
              "name": "いたずらごころ",
              "description": "変化技を先制で出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "シャドークロー",
              "type": "Ghost",
              "category": "Physical",
              "power": 70,
              "description": "影からつくった鋭いツメで相手を切り裂く。急所に当たりやすい。"
            },
            {
              "name": "イカサマ",
              "type": "Dark",
              "category": "Physical",
              "power": 95,
              "description": "相手の力を利用する。戦っている相手の攻撃が高いほどダメージがあがる。"
            },
            {
              "name": "おにび",
              "type": "Fire",
              "category": "Status",
              "power": 0,
              "description": "不気味で怪しい炎を放って相手をやけどの状態にする。"
            },
            {
              "name": "ナイトヘッド",
              "type": "Ghost",
              "category": "Special",
              "power": 1,
              "description": "恐ろしい幻をみせて自分のレベルと同じだけのダメージを相手に与える。"
            }
          ],
          "actions": [
            "時間85% 強化解除",
            "体力75% おだてる",
            "体力50% 弱体解除",
            "体力45% いちゃもん",
            "体力30% 強化解除"
          ],
          "hp": 3200,
          "atk": 117,
          "def": 117,
          "sp_atk": 102,
          "sp_def": 102,
          "speed": 80
        },
        {
          "name": "ヤレユータン",
          "star": 5,
          "types": [
            "Normal",
            "Psychic"
          ],
          "abilities": [
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            },
            {
              "name": "テレパシー",
              "description": "味方の攻撃を読み取って技を回避する。"
            },
            {
              "name": "きょうせい",
              "description": "味方が道具を使うと自分の持っている道具を味方に渡す。"
            }
          ],
          "moves": [
            {
              "name": "からげんき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "自分が毒まひやけど状態のとき相手にくりだすと技の威力が２倍になる。"
            },
            {
              "name": "サイコキネシス",
              "type": "Psychic",
              "category": "Special",
              "power": 90,
              "description": "強い念力を相手に送って攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "アシストパワー",
              "type": "Psychic",
              "category": "Special",
              "power": 20,
              "description": "蓄積されたパワーで相手を攻撃する。自分の能力があがっているほど威力があがる。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            }
          ],
          "actions": [
            "時間80% めいそう",
            "体力75% ひかりのかべ",
            "体力50% 弱体解除",
            "体力40% めいそう",
            "体力20% めいそう"
          ],
          "hp": 4400,
          "atk": 95,
          "def": 125,
          "sp_atk": 140,
          "sp_def": 170,
          "speed": 95
        },
        {
          "name": "ユキノオー",
          "star": 5,
          "types": [
            "Grass",
            "Ice"
          ],
          "abilities": [
            {
              "name": "ゆきふらし",
              "description": "登場したときに天気をゆきにする。"
            },
            {
              "name": "ぼうおん",
              "description": "音を遮断することによって音の技を受けない。"
            }
          ],
          "moves": [
            {
              "name": "エナジーボール",
              "type": "Grass",
              "category": "Special",
              "power": 90,
              "description": "自然から集めた命の力を発射する。相手の特防をさげることがある。"
            },
            {
              "name": "れいとうパンチ",
              "type": "Ice",
              "category": "Physical",
              "power": 75,
              "description": "冷気をこめたパンチで相手を攻撃する。こおり状態にすることがある。"
            },
            {
              "name": "こおりのつぶて",
              "type": "Ice",
              "category": "Physical",
              "power": 40,
              "description": "氷の塊を一瞬でつくり相手に素早く放つ。必ず先制攻撃できる。"
            },
            {
              "name": "にらみつける",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "鋭い目つきでおびえさせて相手の防御をさげる。"
            }
          ],
          "actions": [
            "時間85% ふぶき",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力45% ゆきげしき",
            "体力25% オーロラベール"
          ],
          "hp": 4400,
          "atk": 143,
          "def": 117,
          "sp_atk": 143,
          "sp_def": 132,
          "speed": 95
        },
        {
          "name": "ユキメノコ",
          "star": 5,
          "types": [
            "Ice",
            "Ghost"
          ],
          "abilities": [
            {
              "name": "ゆきがくれ",
              "description": "天気がゆきのとき回避率が上がる。"
            },
            {
              "name": "のろわれボディ",
              "description": "攻撃を受けると相手の技をかなしばり状態にすることがある。"
            }
          ],
          "moves": [
            {
              "name": "こおりのいぶき",
              "type": "Ice",
              "category": "Special",
              "power": 60,
              "description": "冷たい息を相手に吹きつけて攻撃する。必ず急所に当たる。"
            },
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "こわいかお",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "恐ろしい顔でにらみおびえさせて相手の素早さをがくっとさげる。"
            },
            {
              "name": "ドレインキッス",
              "type": "Fairy",
              "category": "Special",
              "power": 50,
              "description": "キッスによって相手からＨＰを吸い取る。与えたダメージの半分以上ＨＰを回復する。"
            }
          ],
          "actions": [
            "時間85% ゆきげしき",
            "体力75% かなしばり",
            "体力50% 弱体解除",
            "体力40% ゆきげしき",
            "体力25% オーロラベール"
          ],
          "hp": 3800,
          "atk": 125,
          "def": 110,
          "sp_atk": 125,
          "sp_def": 110,
          "speed": 170
        },
        {
          "name": "ヨクバリス",
          "star": 5,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "ほおぶくろ",
              "description": "どんなきのみでも食べるとＨＰも回復する。"
            },
            {
              "name": "くいしんぼう",
              "description": "ＨＰが少なくなったら食べるきのみをＨＰ半分の時に食べてしまう。"
            }
          ],
          "moves": [
            {
              "name": "のしかかり",
              "type": "Normal",
              "category": "Physical",
              "power": 85,
              "description": "全身で相手にのしかかり攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "ボディプレス",
              "type": "Fighting",
              "category": "Physical",
              "power": 80,
              "description": "体をぶつけて攻撃。防御が高いほど与えるダメージが増える。"
            },
            {
              "name": "タネマシンガン",
              "type": "Grass",
              "category": "Physical",
              "power": 25,
              "description": "タネを勢いよく相手に発射して攻撃する。２ー５回の間連続でだす。"
            },
            {
              "name": "しっぽをふる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "しっぽを左右にかわいくふって油断を誘う。相手の防御をさげる。"
            }
          ],
          "actions": [
            "時間85% たくわえる",
            "体力75% 強化解除",
            "体力50% 弱体解除",
            "体力50% テラ回収",
            "体力25% たくわえる"
          ],
          "hp": 5300,
          "atk": 147,
          "def": 147,
          "sp_atk": 87,
          "sp_def": 117,
          "speed": 35
        },
        {
          "name": "ライチュウ",
          "star": 5,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "せいでんき",
              "description": "静電気を体にまとい触った相手をまひさせることがある。"
            },
            {
              "name": "ひらいしん",
              "description": "でんきタイプの技を自分に寄せつけダメージを受けずに特攻が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ほうでん",
              "type": "Electric",
              "category": "Special",
              "power": 80,
              "description": "まばゆい電撃で自分の周りにいるものを攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "アイアンテール",
              "type": "Steel",
              "category": "Physical",
              "power": 100,
              "description": "硬いしっぽで相手をたたきつけて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "あまえる",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。"
            },
            {
              "name": "ほっぺすりすり",
              "type": "Electric",
              "category": "Physical",
              "power": 20,
              "description": "電気を帯びたほっぺをすりつけて攻撃。相手をまひ状態にする。"
            }
          ],
          "actions": [
            "時間85% エレキフィールド",
            "体力75% でんじは",
            "体力50% 強化解除",
            "体力50% 弱体解除",
            "体力35% エレキフィールド"
          ],
          "hp": 3500,
          "atk": 140,
          "def": 87,
          "sp_atk": 140,
          "sp_def": 125,
          "speed": 170
        },
        {
          "name": "レントラー",
          "star": 5,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "とうそうしん",
              "description": "性別が同じだと闘争心を燃やして強くなる。性別が違うと弱くなる。"
            },
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "こんじょう",
              "description": "状態異常になると根性をだして攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "ワイルドボルト",
              "type": "Electric",
              "category": "Physical",
              "power": 90,
              "description": "電気をまとって相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "ほうでん",
              "type": "Electric",
              "category": "Special",
              "power": 80,
              "description": "まばゆい電撃で自分の周りにいるものを攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            }
          ],
          "actions": [
            "時間85% エレキフィールド",
            "体力75% にらみつける",
            "体力50% 弱体解除",
            "体力50% テラ回収",
            "体力35% エレキフィールド"
          ],
          "hp": 4100,
          "atk": 185,
          "def": 123,
          "sp_atk": 147,
          "sp_def": 123,
          "speed": 110
        },
        {
          "name": "ロトム",
          "star": 5,
          "types": [
            "Electric",
            "Ghost"
          ],
          "abilities": [
            {
              "name": "ふゆう",
              "description": "地面から浮くことによってじめんタイプの技を受けない。"
            }
          ],
          "moves": [
            {
              "name": "ほうでん",
              "type": "Electric",
              "category": "Special",
              "power": 80,
              "description": "まばゆい電撃で自分の周りにいるものを攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "さわぐ",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "３ターンの間騒いで相手を攻撃する。そのあいだはだれも眠れなくなる。"
            },
            {
              "name": "たたりめ",
              "type": "Ghost",
              "category": "Special",
              "power": 65,
              "description": "たたみかけるように攻撃する。状態異常の相手に大きなダメージを与える。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            }
          ],
          "actions": [
            "時間85% じゅうでん",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力45% 強化解除",
            "体力25% かいでんぱ"
          ],
          "hp": 3200,
          "atk": 80,
          "def": 120,
          "sp_atk": 147,
          "sp_def": 120,
          "speed": 141
        },
        {
          "name": "ワルビアル",
          "star": 5,
          "types": [
            "Ground",
            "Dark"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "じしんかじょう",
              "description": "相手を倒すと自信がついて攻撃が上がる。"
            },
            {
              "name": "いかりのつぼ",
              "description": "急所に攻撃が当たると怒りくるって攻撃力が最大になる。"
            }
          ],
          "moves": [
            {
              "name": "じしん",
              "type": "Ground",
              "category": "Physical",
              "power": 100,
              "description": "地震の衝撃で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "すなじごく",
              "type": "Ground",
              "category": "Physical",
              "power": 35,
              "description": "激しく吹きあれる砂あらしの中に４ー５ターンの間相手を閉じこめて攻撃する。"
            },
            {
              "name": "カウンター",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "相手から受けた物理攻撃のダメージを２倍にして同じ相手に返す。"
            }
          ],
          "actions": [
            "時間85% いちゃもん",
            "体力75% 強化解除",
            "体力50% 弱体解除",
            "体力50% テラ回収",
            "体力35% つめとぎ"
          ],
          "hp": 4540,
          "atk": 180,
          "def": 125,
          "sp_atk": 102,
          "sp_def": 110,
          "speed": 143
        },
        {
          "name": "アーマーガア",
          "star": 6,
          "types": [
            "Flying",
            "Steel"
          ],
          "abilities": [
            {
              "name": "プレッシャー",
              "description": "プレッシャーをあたえて相手の使う技のＰＰを多く減らす。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            },
            {
              "name": "ミラーアーマー",
              "description": "自分が受けた能力ダウンの効果だけを跳ね返す。"
            }
          ],
          "moves": [
            {
              "name": "アイアンヘッド",
              "type": "Steel",
              "category": "Physical",
              "power": 80,
              "description": "鋼のような硬い頭で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ドリルくちばし",
              "type": "Flying",
              "category": "Physical",
              "power": 80,
              "description": "回転しながらとがったくちばしを相手に突き刺して攻撃する。"
            },
            {
              "name": "ボディプレス",
              "type": "Fighting",
              "category": "Physical",
              "power": 80,
              "description": "体をぶつけて攻撃。防御が高いほど与えるダメージが増える。"
            },
            {
              "name": "つめとぎ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "ツメを磨いて鋭くする。自分の攻撃と命中率をあげる。"
            }
          ],
          "actions": [
            "時間90% テラ回収",
            "時間60% 強化解除",
            "体力75% つめとぎ",
            "体力50% 弱体解除",
            "体力30% おいかぜ"
          ],
          "hp": 6900,
          "atk": 161,
          "def": 222,
          "sp_atk": 100,
          "sp_def": 186,
          "speed": 125
        },
        {
          "name": "イッカネズミ",
          "star": 6,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "フレンドガード",
              "description": "味方のダメージを減らすことができる。"
            },
            {
              "name": "ほおぶくろ",
              "description": "どんなきのみでも食べるとＨＰも回復する。"
            },
            {
              "name": "テクニシャン",
              "description": "威力が低い技の威力を高くして攻撃できる。"
            }
          ],
          "moves": [
            {
              "name": "じゃれつく",
              "type": "Fairy",
              "category": "Physical",
              "power": 90,
              "description": "相手にじゃれついて攻撃する。相手の攻撃をさげることがある。"
            },
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "けたぐり",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "足を強くけり相手を転ばせて攻撃する。相手が重いほど威力があがる。"
            },
            {
              "name": "あまえる",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。"
            }
          ],
          "actions": [
            "時間95% 弱体解除",
            "時間75% あまえる",
            "時間60% 強化解除",
            "体力50% 弱体解除",
            "体力30% おかたづけ"
          ],
          "hp": 5825,
          "atk": 140,
          "def": 159,
          "sp_atk": 122,
          "sp_def": 168,
          "speed": 204
        },
        {
          "name": "ウルガモス",
          "star": 6,
          "types": [
            "Bug",
            "Fire"
          ],
          "abilities": [
            {
              "name": "ほのおのからだ",
              "description": "自分に触った相手をやけど状態にすることがある。"
            },
            {
              "name": "むしのしらせ",
              "description": "ＨＰが減ったときむしタイプの技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "むしのさざめき",
              "type": "Bug",
              "category": "Special",
              "power": 90,
              "description": "振動で音波をおこして攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "かえんほうしゃ",
              "type": "Fire",
              "category": "Special",
              "power": 90,
              "description": "激しい炎を相手に発射して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "ぼうふう",
              "type": "Flying",
              "category": "Special",
              "power": 110,
              "description": "強烈な風で相手を包みこんで攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "おいかぜ",
              "type": "Flying",
              "category": "Status",
              "power": 0,
              "description": "激しく吹きあれる風の渦をつくり４ターンの間味方全員の素早さをあげる。"
            }
          ],
          "actions": [
            "時間60% 強化解除",
            "時間20% ちょうのまい",
            "体力85% ドわすれ",
            "体力75% にほんばれ",
            "体力50% 弱体解除",
            "体力30% ひかりのかべ"
          ],
          "hp": 6325,
          "atk": 113,
          "def": 150,
          "sp_atk": 248,
          "sp_def": 222,
          "speed": 185
        },
        {
          "name": "エルレイド",
          "star": 6,
          "types": [
            "Psychic",
            "Fighting"
          ],
          "abilities": [
            {
              "name": "ふくつのこころ",
              "description": "ひるむたびに不屈の心を燃やして素早さが上がる。"
            },
            {
              "name": "きれあじ",
              "description": "相手を切る技の威力が上がる。"
            },
            {
              "name": "せいぎのこころ",
              "description": "あくタイプの攻撃を受けると正義感で攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "サイコカッター",
              "type": "Psychic",
              "category": "Physical",
              "power": 70,
              "description": "実体化させた心の刃で相手を切り裂く。急所に当たりやすい。"
            },
            {
              "name": "インファイト",
              "type": "Fighting",
              "category": "Physical",
              "power": 120,
              "description": "守りを捨てて相手のふところに突撃する。自分の防御と特防がさがる。"
            },
            {
              "name": "おにび",
              "type": "Fire",
              "category": "Status",
              "power": 0,
              "description": "不気味で怪しい炎を放って相手をやけどの状態にする。"
            },
            {
              "name": "つばめがえし",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "素早い動きで相手をほんろうして切りつける。攻撃は必ず命中する。"
            }
          ],
          "actions": [
            "時間85% さいみんじゅつ",
            "体力75% かなしばり",
            "体力50% 弱体解除",
            "体力50% テラ回収",
            "体力25% サイコフィールド"
          ],
          "hp": 5550,
          "atk": 230,
          "def": 150,
          "sp_atk": 122,
          "sp_def": 240,
          "speed": 149
        },
        {
          "name": "エーフィ",
          "star": 6,
          "types": [
            "Psychic"
          ],
          "abilities": [
            {
              "name": "シンクロ",
              "description": "自分がなってしまったどくやまひややけどを相手にうつす。"
            },
            {
              "name": "マジックミラー",
              "description": "相手にだされた変化技を受けずにそのまま返すことができる。"
            }
          ],
          "moves": [
            {
              "name": "テラバースト",
              "type": "Normal",
              "category": "Special",
              "power": 80,
              "description": "テラスタルだとテラスタイプのエネルギーを放出して攻撃する。攻撃と特攻を比べて高いほうでダメージを与える。"
            },
            {
              "name": "サイコキネシス",
              "type": "Psychic",
              "category": "Special",
              "power": 90,
              "description": "強い念力を相手に送って攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "サイコショック",
              "type": "Psychic",
              "category": "Special",
              "power": 80,
              "description": "不思議な念波を実体化して相手を攻撃する。物理的なダメージを与える。"
            },
            {
              "name": "くすぐる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体をくすぐり笑わせることで相手の攻撃と防御をさげる。"
            }
          ],
          "actions": [
            "時間70% テラ回収",
            "時間60% 強化解除",
            "体力90% サイコフィールド",
            "体力50% 弱体解除",
            "体力35% めいそう",
            "体力20% サイコフィールド"
          ],
          "hp": 5425,
          "atk": 122,
          "def": 113,
          "sp_atk": 239,
          "sp_def": 204,
          "speed": 203
        },
        {
          "name": "オトシドリ",
          "star": 6,
          "types": [
            "Flying",
            "Dark"
          ],
          "abilities": [
            {
              "name": "はとむね",
              "description": "防御を下げる効果を受けない。"
            },
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            },
            {
              "name": "いわはこび",
              "description": "いわタイプの技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "いわなだれ",
              "type": "Rock",
              "category": "Physical",
              "power": 75,
              "description": "大きな岩を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "アクロバット",
              "type": "Flying",
              "category": "Physical",
              "power": 55,
              "description": "軽やかに相手を攻撃する。自分が道具を持っていないとき大きなダメージを与える。"
            },
            {
              "name": "はたきおとす",
              "type": "Dark",
              "category": "Physical",
              "power": 65,
              "description": "相手の持ち物をはたき落として戦闘が終わるまで使えなくする。物を持つ相手にはダメージが増す。"
            },
            {
              "name": "フェザーダンス",
              "type": "Flying",
              "category": "Status",
              "power": 0,
              "description": "羽毛をふりまいて相手の体にからませる。相手の攻撃をがくっとさげる。"
            }
          ],
          "actions": [
            "時間80% はたきおとす",
            "時間60% 強化解除",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力30% フェザーダンス"
          ],
          "hp": 5650,
          "atk": 190,
          "def": 186,
          "sp_atk": 113,
          "sp_def": 186,
          "speed": 152
        },
        {
          "name": "オノノクス",
          "star": 6,
          "types": [
            "Dragon"
          ],
          "abilities": [
            {
              "name": "とうそうしん",
              "description": "性別が同じだと闘争心を燃やして強くなる。性別が違うと弱くなる。"
            },
            {
              "name": "かたやぶり",
              "description": "相手の特性にジャマされることなく相手に技を出すことができる。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            }
          ],
          "moves": [
            {
              "name": "げきりん",
              "type": "Dragon",
              "category": "Physical",
              "power": 120,
              "description": "２ー３ターンの間暴れまくって攻撃する。暴れたあとは混乱する。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "ギガインパクト",
              "type": "Normal",
              "category": "Physical",
              "power": 150,
              "description": "持てる力をすべて使って相手に突撃する。次のターンは動けなくなる。"
            },
            {
              "name": "であいがしら",
              "type": "Bug",
              "category": "Physical",
              "power": 90,
              "description": "威力が高い技だが戦闘に出たらすぐに出さないと成功しない。"
            }
          ],
          "actions": [
            "時間90% 弱体解除",
            "時間70% テラ回収",
            "時間60% 強化解除",
            "体力50% 弱体解除",
            "体力30% りゅうのまい"
          ],
          "hp": 5900,
          "atk": 269,
          "def": 195,
          "sp_atk": 113,
          "sp_def": 159,
          "speed": 179
        },
        {
          "name": "カイリュー",
          "star": 6,
          "types": [
            "Dragon",
            "Flying"
          ],
          "abilities": [
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            },
            {
              "name": "マルチスケイル",
              "description": "ＨＰが満タンのときに受けるダメージが少なくなる。"
            }
          ],
          "moves": [
            {
              "name": "ドラゴンダイブ",
              "type": "Dragon",
              "category": "Physical",
              "power": 100,
              "description": "すさまじい殺気で威圧しながら体当たりする。相手をひるませることがある。"
            },
            {
              "name": "しんそく",
              "type": "Normal",
              "category": "Physical",
              "power": 80,
              "description": "目にも留まらぬものすごい速さで相手に突進して攻撃する。必ず先制攻撃できる。"
            },
            {
              "name": "りゅうのまい",
              "type": "Dragon",
              "category": "Status",
              "power": 0,
              "description": "神秘的で力強い舞を激しくおどる。自分の攻撃と素早さをあげる。"
            },
            {
              "name": "アクアテール",
              "type": "Water",
              "category": "Physical",
              "power": 90,
              "description": "激しくあれくるう荒波のように大きなしっぽをふって相手を攻撃する。"
            }
          ],
          "actions": [
            "時間70% テラ回収",
            "時間65% ひかりのかべ",
            "時間50% 強化解除",
            "体力95% 弱体解除",
            "体力50% 弱体解除",
            "体力30% りゅうのまい"
          ],
          "hp": 6575,
          "atk": 246,
          "def": 176,
          "sp_atk": 185,
          "sp_def": 213,
          "speed": 149
        },
        {
          "name": "カバルドン",
          "star": 6,
          "types": [
            "Ground"
          ],
          "abilities": [
            {
              "name": "すなおこし",
              "description": "登場したとき天気を砂あらしにする。"
            },
            {
              "name": "すなのちから",
              "description": "天気がすなあらしのときいわタイプとじめんタイプとはがねタイプの威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "じしん",
              "type": "Ground",
              "category": "Physical",
              "power": 100,
              "description": "地震の衝撃で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "こおりのキバ",
              "type": "Ice",
              "category": "Physical",
              "power": 65,
              "description": "冷気をひめたキバでかみつく。相手をひるませたりこおり状態にすることがある。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            },
            {
              "name": "いわなだれ",
              "type": "Rock",
              "category": "Physical",
              "power": 75,
              "description": "大きな岩を激しくぶつけて攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [
            "時間90% あくび",
            "時間60% 強化解除",
            "体力60% テラ回収",
            "体力50% 弱体解除",
            "体力20% あくび"
          ],
          "hp": 7350,
          "atk": 206,
          "def": 246,
          "sp_atk": 127,
          "sp_def": 163,
          "speed": 89
        },
        {
          "name": "ガケガニ",
          "star": 6,
          "types": [
            "Rock"
          ],
          "abilities": [
            {
              "name": "いかりのこうら",
              "description": "相手の攻撃でＨＰが半分になると怒りで防御と特防が下がるが攻撃特攻素早さが上がる。"
            },
            {
              "name": "シェルアーマー",
              "description": "硬い殻に守られ相手の攻撃が急所に当たらない。"
            },
            {
              "name": "さいせいりょく",
              "description": "手持ちに引っ込むとＨＰが少し回復する。"
            }
          ],
          "moves": [
            {
              "name": "ストーンエッジ",
              "type": "Rock",
              "category": "Physical",
              "power": 100,
              "description": "とがった岩を相手に突き刺して攻撃する。急所に当たりやすい。"
            },
            {
              "name": "いわくだき",
              "type": "Fighting",
              "category": "Physical",
              "power": 40,
              "description": "パンチで攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "シザークロス",
              "type": "Bug",
              "category": "Physical",
              "power": 80,
              "description": "カマやツメをハサミのように交差させながら相手を切り裂く。"
            },
            {
              "name": "すなあらし",
              "type": "Rock",
              "category": "Status",
              "power": 0,
              "description": "５ターンの間砂あらしでいわじめんはがねタイプ以外にダメージ。いわタイプの特防があがる。"
            }
          ],
          "actions": [
            "時間95% 弱体解除",
            "時間60% 強化解除",
            "体力80% はたきおとす",
            "体力49% 弱体解除",
            "体力30% てっぺき"
          ],
          "hp": 5650,
          "atk": 185,
          "def": 240,
          "sp_atk": 68,
          "sp_def": 132,
          "speed": 140
        },
        {
          "name": "ガブリアス",
          "star": 6,
          "types": [
            "Dragon",
            "Ground"
          ],
          "abilities": [
            {
              "name": "すながくれ",
              "description": "砂あらしのとき回避率が上がる。"
            },
            {
              "name": "さめはだ",
              "description": "攻撃を受けたとき自分に触れた相手をざらざらの肌でキズつける。"
            }
          ],
          "moves": [
            {
              "name": "げきりん",
              "type": "Dragon",
              "category": "Physical",
              "power": 120,
              "description": "２ー３ターンの間暴れまくって攻撃する。暴れたあとは混乱する。"
            },
            {
              "name": "じしん",
              "type": "Ground",
              "category": "Physical",
              "power": 100,
              "description": "地震の衝撃で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "かえんほうしゃ",
              "type": "Fire",
              "category": "Special",
              "power": 90,
              "description": "激しい炎を相手に発射して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "いわなだれ",
              "type": "Rock",
              "category": "Physical",
              "power": 75,
              "description": "大きな岩を激しくぶつけて攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [
            "時間75% テラ回収",
            "時間60% 強化解除",
            "体力90% 弱体解除",
            "体力50% 弱体解除",
            "体力30% 強化解除",
            "体力30% つるぎのまい"
          ],
          "hp": 7350,
          "atk": 239,
          "def": 176,
          "sp_atk": 149,
          "sp_def": 186,
          "speed": 188
        },
        {
          "name": "キノガッサ",
          "star": 6,
          "types": [
            "Grass",
            "Fighting"
          ],
          "abilities": [
            {
              "name": "ほうし",
              "description": "攻撃で自分に触れた相手をどくやまひやねむり状態にすることがある。"
            },
            {
              "name": "ポイズンヒール",
              "description": "どく状態になるとＨＰが減らずに増えていく。"
            },
            {
              "name": "テクニシャン",
              "description": "威力が低い技の威力を高くして攻撃できる。"
            }
          ],
          "moves": [
            {
              "name": "タネマシンガン",
              "type": "Grass",
              "category": "Physical",
              "power": 25,
              "description": "タネを勢いよく相手に発射して攻撃する。２ー５回の間連続でだす。"
            },
            {
              "name": "ローキック",
              "type": "Fighting",
              "category": "Physical",
              "power": 65,
              "description": "素早い動きで相手の足をねらって攻撃する。相手の素早さをさげる。"
            },
            {
              "name": "キノコのほうし",
              "type": "Grass",
              "category": "Status",
              "power": 0,
              "description": "催眠効果のある胞子をパラパラとふりまき相手を眠り状態にする。"
            },
            {
              "name": "つばめがえし",
              "type": "Flying",
              "category": "Physical",
              "power": 60,
              "description": "素早い動きで相手をほんろうして切りつける。攻撃は必ず命中する。"
            }
          ],
          "actions": [
            "時間80% グラスフィールド",
            "体力75% キノコのほうし",
            "体力50% 弱体解除",
            "体力45% テラ回収",
            "体力20% 強化解除"
          ],
          "hp": 5200,
          "atk": 239,
          "def": 177,
          "sp_atk": 113,
          "sp_def": 141,
          "speed": 131
        },
        {
          "name": "キョジオーン",
          "star": 6,
          "types": [
            "Rock"
          ],
          "abilities": [
            {
              "name": "きよめのしお",
              "description": "清らかな塩で状態異常にならない。ゴーストタイプの技のダメージを半減させる。"
            },
            {
              "name": "がんじょう",
              "description": "ＨＰが満タンのとき技を受けても一撃で倒されることがない。一撃必殺技も効かない。"
            },
            {
              "name": "クリアボディ",
              "description": "相手の技や特性で能力を下げられない。"
            }
          ],
          "moves": [
            {
              "name": "ストーンエッジ",
              "type": "Rock",
              "category": "Physical",
              "power": 100,
              "description": "とがった岩を相手に突き刺して攻撃する。急所に当たりやすい。"
            },
            {
              "name": "ヘビーボンバー",
              "type": "Steel",
              "category": "Physical",
              "power": 1,
              "description": "重たい体で相手にぶつかって攻撃する。自分が相手より重いほど威力があがる。"
            },
            {
              "name": "しおづけ",
              "type": "Rock",
              "category": "Physical",
              "power": 40,
              "description": "相手をしおづけ状態にして毎ターンダメージを与える。はがねみずタイプはより苦しむ。"
            },
            {
              "name": "アームハンマー",
              "type": "Fighting",
              "category": "Physical",
              "power": 100,
              "description": "強くて重いこぶしをふるってダメージを与える。自分の素早さがさがる。"
            }
          ],
          "actions": [
            "時間60% 強化解除",
            "時間60% テラ回収",
            "体力90% すなあらし",
            "体力50% 弱体解除",
            "体力20% いわなだれ"
          ],
          "hp": 7000,
          "atk": 185,
          "def": 267,
          "sp_atk": 86,
          "sp_def": 195,
          "speed": 68
        },
        {
          "name": "キラフロル",
          "star": 6,
          "types": [
            "Rock",
            "Poison"
          ],
          "abilities": [
            {
              "name": "どくげしょう",
              "description": "物理技でダメージを受けると相手の足下にどくびしがちらばる。"
            },
            {
              "name": "ふしょく",
              "description": "はがねタイプやどくタイプもどく状態にすることができる。"
            }
          ],
          "moves": [
            {
              "name": "パワージェム",
              "type": "Rock",
              "category": "Special",
              "power": 80,
              "description": "宝石のようにきらめく光を発射して相手を攻撃する。"
            },
            {
              "name": "ヘドロウェーブ",
              "type": "Poison",
              "category": "Special",
              "power": 95,
              "description": "ヘドロの波で自分の周りにいるものを攻撃する。毒状態にすることがある。"
            },
            {
              "name": "はかいこうせん",
              "type": "Normal",
              "category": "Special",
              "power": 150,
              "description": "強い光線を相手に発射して攻撃する。次のターンは動けなくなる。"
            },
            {
              "name": "ロックカット",
              "type": "Rock",
              "category": "Status",
              "power": 0,
              "description": "自分の体を磨いて空気の抵抗を少なくする。素早さをぐーんとあげることができる。"
            }
          ],
          "actions": [
            "時間85% テラ回収",
            "時間60% 強化解除",
            "時間50% テラ回収",
            "体力90% すなあらし",
            "体力55% テラ回収",
            "体力50% 弱体解除"
          ],
          "hp": 6225,
          "atk": 104,
          "def": 195,
          "sp_atk": 239,
          "sp_def": 179,
          "speed": 159
        },
        {
          "name": "ギャラドス",
          "star": 6,
          "types": [
            "Water",
            "Flying"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "じしんかじょう",
              "description": "相手を倒すと自信がついて攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "アクアテール",
              "type": "Water",
              "category": "Physical",
              "power": 90,
              "description": "激しくあれくるう荒波のように大きなしっぽをふって相手を攻撃する。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "ぼうふう",
              "type": "Flying",
              "category": "Special",
              "power": 110,
              "description": "強烈な風で相手を包みこんで攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "こおりのキバ",
              "type": "Ice",
              "category": "Physical",
              "power": 65,
              "description": "冷気をひめたキバでかみつく。相手をひるませたりこおり状態にすることがある。"
            }
          ],
          "actions": [
            "時間70% テラ回収",
            "時間60% 強化解除",
            "体力90% 弱体解除",
            "体力50% 弱体解除",
            "体力40% ちょうはつ",
            "体力20% りゅうのまい"
          ],
          "hp": 6775,
          "atk": 230,
          "def": 147,
          "sp_atk": 113,
          "sp_def": 213,
          "speed": 150
        },
        {
          "name": "クレベース",
          "star": 6,
          "types": [
            "Ice"
          ],
          "abilities": [
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "アイスボディ",
              "description": "天気がゆきのときＨＰを少しずつ回復する。"
            },
            {
              "name": "がんじょう",
              "description": "ＨＰが満タンのとき技を受けても一撃で倒されることがない。一撃必殺技も効かない。"
            }
          ],
          "moves": [
            {
              "name": "つららおとし",
              "type": "Ice",
              "category": "Physical",
              "power": 85,
              "description": "大きな氷柱を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ヘビーボンバー",
              "type": "Steel",
              "category": "Physical",
              "power": 1,
              "description": "重たい体で相手にぶつかって攻撃する。自分が相手より重いほど威力があがる。"
            },
            {
              "name": "ゆきげしき",
              "type": "Ice",
              "category": "Status",
              "power": 0,
              "description": "５ターンの間ゆきを降らせる。こおりタイプの防御があがる。"
            },
            {
              "name": "アイススピナー",
              "type": "Ice",
              "category": "Physical",
              "power": 80,
              "description": "足に薄い氷をまといクルクルと回りながらぶつかる。回転の動きによってフィールドを壊す。"
            }
          ],
          "actions": [
            "時間90% ゆきげしき",
            "時間75% てっぺき",
            "時間60% 強化解除",
            "体力50% 弱体解除",
            "体力30% 強化解除"
          ],
          "hp": 6775,
          "atk": 215,
          "def": 365,
          "sp_atk": 84,
          "sp_def": 116,
          "speed": 55
        },
        {
          "name": "グレイシア",
          "star": 6,
          "types": [
            "Ice"
          ],
          "abilities": [
            {
              "name": "ゆきがくれ",
              "description": "天気がゆきのとき回避率が上がる。"
            },
            {
              "name": "アイスボディ",
              "description": "天気がゆきのときＨＰを少しずつ回復する。"
            }
          ],
          "moves": [
            {
              "name": "テラバースト",
              "type": "Normal",
              "category": "Special",
              "power": 80,
              "description": "テラスタルだとテラスタイプのエネルギーを放出して攻撃する。攻撃と特攻を比べて高いほうでダメージを与える。"
            },
            {
              "name": "れいとうビーム",
              "type": "Ice",
              "category": "Special",
              "power": 90,
              "description": "凍えるビームを相手に発射して攻撃する。こおり状態にすることがある。"
            },
            {
              "name": "ふぶき",
              "type": "Ice",
              "category": "Special",
              "power": 110,
              "description": "激しい吹雪を相手に吹きつけて攻撃する。こおり状態にすることがある。"
            },
            {
              "name": "あまえる",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。"
            }
          ],
          "actions": [
            "時間70% テラ回収",
            "時間60% 強化解除",
            "体力90% ゆきげしき",
            "体力50% 弱体解除",
            "体力35% めいそう",
            "体力20% ゆきげしき"
          ],
          "hp": 5425,
          "atk": 113,
          "def": 203,
          "sp_atk": 239,
          "sp_def": 204,
          "speed": 122
        },
        {
          "name": "グレンアルマ",
          "star": 6,
          "types": [
            "Fire",
            "Psychic"
          ],
          "abilities": [
            {
              "name": "もらいび",
              "description": "ほのおタイプの技を受けると炎をもらい自分が出すほのおタイプの技が強くなる。"
            },
            {
              "name": "くだけるよろい",
              "description": "物理技でダメージを受けると防御が下がり素早さがぐーんと上がる。"
            }
          ],
          "moves": [
            {
              "name": "アーマーキャノン",
              "type": "Fire",
              "category": "Special",
              "power": 120,
              "description": "みずからのヨロイを燃えたぎる弾として撃ち出して攻撃する。自分の防御と特防がさがる。"
            },
            {
              "name": "サイコキネシス",
              "type": "Psychic",
              "category": "Special",
              "power": 90,
              "description": "強い念力を相手に送って攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "ナイトヘッド",
              "type": "Ghost",
              "category": "Special",
              "power": 1,
              "description": "恐ろしい幻をみせて自分のレベルと同じだけのダメージを相手に与える。"
            },
            {
              "name": "おにび",
              "type": "Fire",
              "category": "Status",
              "power": 0,
              "description": "不気味で怪しい炎を放って相手をやけどの状態にする。"
            }
          ],
          "actions": [
            "時間90% めいそう",
            "時間65% テラ回収",
            "時間60% 強化解除",
            "体力50% 弱体解除",
            "体力20% にほんばれ"
          ],
          "hp": 6325,
          "atk": 113,
          "def": 213,
          "sp_atk": 230,
          "sp_def": 177,
          "speed": 140
        },
        {
          "name": "ケンタロス",
          "star": 6,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "いかりのつぼ",
              "description": "急所に攻撃が当たると怒りくるって攻撃力が最大になる。"
            },
            {
              "name": "はんすう",
              "description": "きのみを食べると次のターンの終わりに胃から出してもう１回だけ食べる。"
            }
          ],
          "moves": [
            {
              "name": "フレアドライブ",
              "type": "Fire",
              "category": "Physical",
              "power": 120,
              "description": "炎をまとって突進する。自分もかなりダメージを受ける。やけど状態にすることがある。"
            },
            {
              "name": "インファイト",
              "type": "Fighting",
              "category": "Physical",
              "power": 120,
              "description": "守りを捨てて相手のふところに突撃する。自分の防御と特防がさがる。"
            },
            {
              "name": "かえんほうしゃ",
              "type": "Fire",
              "category": "Special",
              "power": 90,
              "description": "激しい炎を相手に発射して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [
            "時間70% テラ回収",
            "時間60% 強化解除",
            "体力90% にほんばれ",
            "体力50% 弱体解除",
            "体力35% ビルドアップ",
            "体力20% にほんばれ"
          ],
          "hp": 5875,
          "atk": 185,
          "def": 204,
          "sp_atk": 77,
          "sp_def": 159,
          "speed": 203
        },
        {
          "name": "ケンタロス",
          "star": 6,
          "types": [],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "いかりのつぼ",
              "description": "急所に攻撃が当たると怒りくるって攻撃力が最大になる。"
            },
            {
              "name": "はんすう",
              "description": "きのみを食べると次のターンの終わりに胃から出してもう１回だけ食べる。"
            }
          ],
          "moves": [
            {
              "name": "ウェーブタックル",
              "type": "Water",
              "category": "Physical",
              "power": 120,
              "description": "水をまといつつ全身で相手にぶつかるが自分もかなりのダメージを受ける。"
            },
            {
              "name": "インファイト",
              "type": "Fighting",
              "category": "Physical",
              "power": 120,
              "description": "守りを捨てて相手のふところに突撃する。自分の防御と特防がさがる。"
            },
            {
              "name": "なみのり",
              "type": "Water",
              "category": "Special",
              "power": 90,
              "description": "大きな波で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "ずつき",
              "type": "Normal",
              "category": "Physical",
              "power": 70,
              "description": "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [
            "時間70% テラ回収",
            "時間60% 強化解除",
            "体力90% あまごい",
            "体力50% 弱体解除",
            "体力35% ビルドアップ",
            "体力20% あまごい"
          ],
          "hp": 0,
          "atk": 0,
          "def": 0,
          "sp_atk": 0,
          "sp_def": 0,
          "speed": 0
        },
        {
          "name": "ケンタロス",
          "star": 6,
          "types": [],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "いかりのつぼ",
              "description": "急所に攻撃が当たると怒りくるって攻撃力が最大になる。"
            },
            {
              "name": "はんすう",
              "description": "きのみを食べると次のターンの終わりに胃から出してもう１回だけ食べる。"
            }
          ],
          "moves": [
            {
              "name": "インファイト",
              "type": "Fighting",
              "category": "Physical",
              "power": 120,
              "description": "守りを捨てて相手のふところに突撃する。自分の防御と特防がさがる。"
            },
            {
              "name": "あばれる",
              "type": "Normal",
              "category": "Physical",
              "power": 120,
              "description": "２ー３ターンの間暴れまくって相手を攻撃する。暴れたあとは混乱する。"
            },
            {
              "name": "しねんのずつき",
              "type": "Psychic",
              "category": "Physical",
              "power": 80,
              "description": "思念の力を額に集めて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "レイジングブル",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "怒り狂うあばれうしの猛烈なタックル。フォルムで技のタイプが変わりひかりのかべやリフレクターなども破壊できる。"
            }
          ],
          "actions": [
            "時間95% 弱体解除",
            "時間60% 強化解除",
            "体力90% ビルドアップ",
            "体力50% 弱体解除",
            "体力25% いやなおと"
          ],
          "hp": 0,
          "atk": 0,
          "def": 0,
          "sp_atk": 0,
          "sp_def": 0,
          "speed": 0
        },
        {
          "name": "ゲンガー",
          "star": 6,
          "types": [
            "Ghost",
            "Poison"
          ],
          "abilities": [
            {
              "name": "のろわれボディ",
              "description": "攻撃を受けると相手の技をかなしばり状態にすることがある。"
            }
          ],
          "moves": [
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "ヘドロばくだん",
              "type": "Poison",
              "category": "Special",
              "power": 90,
              "description": "汚いヘドロを相手に投げつけて攻撃する。毒状態にすることがある。"
            },
            {
              "name": "マジカルシャイン",
              "type": "Fairy",
              "category": "Special",
              "power": 80,
              "description": "強力な光を放ち相手にダメージを与える。"
            },
            {
              "name": "おにび",
              "type": "Fire",
              "category": "Status",
              "power": 0,
              "description": "不気味で怪しい炎を放って相手をやけどの状態にする。"
            }
          ],
          "actions": [
            "時間85% さいみんじゅつ",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力45% 強化解除",
            "体力25% さいみんじゅつ"
          ],
          "hp": 5200,
          "atk": 122,
          "def": 141,
          "sp_atk": 239,
          "sp_def": 168,
          "speed": 203
        },
        {
          "name": "コノヨザル",
          "star": 6,
          "types": [
            "Fighting",
            "Ghost"
          ],
          "abilities": [
            {
              "name": "やるき",
              "description": "やる気をだすことによってねむり状態にならない。"
            },
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            },
            {
              "name": "まけんき",
              "description": "相手に能力を下げられると攻撃がぐーんと上がる。"
            }
          ],
          "moves": [
            {
              "name": "インファイト",
              "type": "Fighting",
              "category": "Physical",
              "power": 120,
              "description": "守りを捨てて相手のふところに突撃する。自分の防御と特防がさがる。"
            },
            {
              "name": "シャドークロー",
              "type": "Ghost",
              "category": "Physical",
              "power": 70,
              "description": "影からつくった鋭いツメで相手を切り裂く。急所に当たりやすい。"
            },
            {
              "name": "ダメおし",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "そのターンに相手がすでにダメージを受けていたら技の威力は２倍になる。"
            },
            {
              "name": "きあいだめ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "深く息を吸い気合をこめる。自分の攻撃が急所に当たりやすくなる。"
            }
          ],
          "actions": [
            "時間95% 弱体解除",
            "時間60% 強化解除",
            "時間5% ふんどのこぶし",
            "体力90% ビルドアップ",
            "体力50% 弱体解除",
            "体力25% ビルドアップ"
          ],
          "hp": 7450,
          "atk": 212,
          "def": 177,
          "sp_atk": 95,
          "sp_def": 195,
          "speed": 167
        },
        {
          "name": "コータス",
          "star": 6,
          "types": [
            "Fire"
          ],
          "abilities": [
            {
              "name": "しろいけむり",
              "description": "白い煙に守られて相手に能力を下げられない。"
            },
            {
              "name": "ひでり",
              "description": "登場したときに天気を晴れにする。"
            },
            {
              "name": "シェルアーマー",
              "description": "硬い殻に守られ相手の攻撃が急所に当たらない。"
            }
          ],
          "moves": [
            {
              "name": "ふんえん",
              "type": "Fire",
              "category": "Special",
              "power": 80,
              "description": "真っ赤な炎で自分の周りにいるものを攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            },
            {
              "name": "クリアスモッグ",
              "type": "Poison",
              "category": "Special",
              "power": 50,
              "description": "特殊な泥の塊を相手に投げつけて攻撃する。能力変化をもとにもどす。"
            },
            {
              "name": "のしかかり",
              "type": "Normal",
              "category": "Physical",
              "power": 85,
              "description": "全身で相手にのしかかり攻撃する。まひ状態にすることがある。"
            }
          ],
          "actions": [
            "時間75% テラ回収",
            "時間60% 強化解除",
            "体力90% にほんばれ",
            "体力50% 弱体解除",
            "体力30% あくび",
            "体力20% てっぺき"
          ],
          "hp": 5650,
          "atk": 158,
          "def": 257,
          "sp_atk": 158,
          "sp_def": 159,
          "speed": 41
        },
        {
          "name": "サザンドラ",
          "star": 6,
          "types": [
            "Dark",
            "Dragon"
          ],
          "abilities": [
            {
              "name": "ふゆう",
              "description": "地面から浮くことによってじめんタイプの技を受けない。"
            }
          ],
          "moves": [
            {
              "name": "あくのはどう",
              "type": "Dark",
              "category": "Special",
              "power": 80,
              "description": "体から悪意にみちた恐ろしいオーラを発する。相手をひるませることがある。"
            },
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            }
          ],
          "actions": [
            "時間75% ふるいたてる",
            "時間60% 強化解除",
            "時間20% わるだくみ",
            "体力85% ちょうはつ",
            "体力50% 弱体解除",
            "体力25% 弱体解除"
          ],
          "hp": 6625,
          "atk": 194,
          "def": 195,
          "sp_atk": 230,
          "sp_def": 195,
          "speed": 181
        },
        {
          "name": "サンダース",
          "star": 6,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "ちくでん",
              "description": "でんきタイプの技を受けるとダメージを受けずに回復する。"
            },
            {
              "name": "はやあし",
              "description": "状態異常になると素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "テラバースト",
              "type": "Normal",
              "category": "Special",
              "power": 80,
              "description": "テラスタルだとテラスタイプのエネルギーを放出して攻撃する。攻撃と特攻を比べて高いほうでダメージを与える。"
            },
            {
              "name": "１０まんボルト",
              "type": "Electric",
              "category": "Special",
              "power": 90,
              "description": "強い電撃を相手に浴びせて攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            }
          ],
          "actions": [
            "時間70% テラ回収",
            "時間60% 強化解除",
            "体力90% エレキフィールド",
            "体力50% 弱体解除",
            "体力35% めいそう",
            "体力20% エレキフィールド"
          ],
          "hp": 5425,
          "atk": 122,
          "def": 113,
          "sp_atk": 203,
          "sp_def": 204,
          "speed": 239
        },
        {
          "name": "サーナイト",
          "star": 6,
          "types": [
            "Psychic",
            "Fairy"
          ],
          "abilities": [
            {
              "name": "シンクロ",
              "description": "自分がなってしまったどくやまひややけどを相手にうつす。"
            },
            {
              "name": "トレース",
              "description": "登場したとき相手の特性をトレースして同じ特性になる。"
            },
            {
              "name": "テレパシー",
              "description": "味方の攻撃を読み取って技を回避する。"
            }
          ],
          "moves": [
            {
              "name": "ムーンフォース",
              "type": "Fairy",
              "category": "Special",
              "power": 95,
              "description": "月のパワーをかりて相手を攻撃する。相手の特攻をさげることがある。"
            },
            {
              "name": "サイコキネシス",
              "type": "Psychic",
              "category": "Special",
              "power": 90,
              "description": "強い念力を相手に送って攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "めいそう",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "静かに精神を統一し心を鎮めることで自分の特攻と特防をあげる。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            }
          ],
          "actions": [
            "時間85% ミストフィールド",
            "体力75% めいそう",
            "体力50% 弱体解除",
            "体力45% 強化解除",
            "体力35% サイコフィールド"
          ],
          "hp": 5550,
          "atk": 122,
          "def": 150,
          "sp_atk": 230,
          "sp_def": 240,
          "speed": 149
        },
        {
          "name": "シャワーズ",
          "star": 6,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "ちょすい",
              "description": "みずタイプの技を受けるとダメージを受けずに回復する。"
            },
            {
              "name": "うるおいボディ",
              "description": "天気が雨のとき状態異常が治る。"
            }
          ],
          "moves": [
            {
              "name": "テラバースト",
              "type": "Normal",
              "category": "Special",
              "power": 80,
              "description": "テラスタルだとテラスタイプのエネルギーを放出して攻撃する。攻撃と特攻を比べて高いほうでダメージを与える。"
            },
            {
              "name": "なみのり",
              "type": "Water",
              "category": "Special",
              "power": 90,
              "description": "大きな波で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "ハイパーボイス",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "うるさく響く大きな振動を相手に与えて攻撃する。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            }
          ],
          "actions": [
            "時間70% テラ回収",
            "時間60% 強化解除",
            "体力90% あまごい",
            "体力50% 弱体解除",
            "体力35% めいそう",
            "体力20% あまごい"
          ],
          "hp": 8350,
          "atk": 122,
          "def": 113,
          "sp_atk": 203,
          "sp_def": 204,
          "speed": 122
        },
        {
          "name": "ジバコイル",
          "star": 6,
          "types": [
            "Electric",
            "Steel"
          ],
          "abilities": [
            {
              "name": "じりょく",
              "description": "はがねタイプのポケモンを磁力で引きつけて逃げられなくする。"
            },
            {
              "name": "がんじょう",
              "description": "ＨＰが満タンのとき技を受けても一撃で倒されることがない。一撃必殺技も効かない。"
            },
            {
              "name": "アナライズ",
              "description": "いちばん最後に技を出すと技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "かみなり",
              "type": "Electric",
              "category": "Special",
              "power": 110,
              "description": "激しい雷を相手に落として攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "ラスターカノン",
              "type": "Steel",
              "category": "Special",
              "power": 80,
              "description": "体の光を一点に集めて力を放つ。相手の特防をさげることがある。"
            },
            {
              "name": "トライアタック",
              "type": "Normal",
              "category": "Special",
              "power": 80,
              "description": "３つの光線で攻撃する。まひかやけどかこおり状態のどれかにすることがある。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            }
          ],
          "actions": [
            "時間75% てっぺき",
            "時間60% 強化解除",
            "時間20% エレキフィールド",
            "体力80% あまごい",
            "体力50% 弱体解除",
            "体力30% でんじは"
          ],
          "hp": 5650,
          "atk": 131,
          "def": 212,
          "sp_atk": 239,
          "sp_def": 195,
          "speed": 113
        },
        {
          "name": "セグレイブ",
          "star": 6,
          "types": [
            "Dragon",
            "Ice"
          ],
          "abilities": [
            {
              "name": "ねつこうかん",
              "description": "ほのおタイプの技を受けると攻撃が上がる。やけど状態にならない。"
            },
            {
              "name": "アイスボディ",
              "description": "天気がゆきのときＨＰを少しずつ回復する。"
            }
          ],
          "moves": [
            {
              "name": "つららばり",
              "type": "Ice",
              "category": "Physical",
              "power": 25,
              "description": "鋭い氷柱を相手に発射して攻撃する。２ー５回の間連続でだす。"
            },
            {
              "name": "ドラゴンダイブ",
              "type": "Dragon",
              "category": "Physical",
              "power": 100,
              "description": "すさまじい殺気で威圧しながら体当たりする。相手をひるませることがある。"
            },
            {
              "name": "ゆきげしき",
              "type": "Ice",
              "category": "Status",
              "power": 0,
              "description": "５ターンの間ゆきを降らせる。こおりタイプの防御があがる。"
            },
            {
              "name": "ボディプレス",
              "type": "Fighting",
              "category": "Physical",
              "power": 80,
              "description": "体をぶつけて攻撃。防御が高いほど与えるダメージが増える。"
            }
          ],
          "actions": [
            "時間80% ゆきげしき",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力40% ゆきげしき",
            "体力20% 強化解除"
          ],
          "hp": 7675,
          "atk": 266,
          "def": 199,
          "sp_atk": 140,
          "sp_def": 188,
          "speed": 161
        },
        {
          "name": "ソウブレイズ",
          "star": 6,
          "types": [
            "Fire",
            "Ghost"
          ],
          "abilities": [
            {
              "name": "もらいび",
              "description": "ほのおタイプの技を受けると炎をもらい自分が出すほのおタイプの技が強くなる。"
            },
            {
              "name": "くだけるよろい",
              "description": "物理技でダメージを受けると防御が下がり素早さがぐーんと上がる。"
            }
          ],
          "moves": [
            {
              "name": "むねんのつるぎ",
              "type": "Fire",
              "category": "Physical",
              "power": 90,
              "description": "この世への未練を剣先にこめて切りつける。与えたダメージの半分のＨＰを回復できる。"
            },
            {
              "name": "シャドークロー",
              "type": "Ghost",
              "category": "Physical",
              "power": 70,
              "description": "影からつくった鋭いツメで相手を切り裂く。急所に当たりやすい。"
            },
            {
              "name": "サイコカッター",
              "type": "Psychic",
              "category": "Physical",
              "power": 70,
              "description": "実体化させた心の刃で相手を切り裂く。急所に当たりやすい。"
            },
            {
              "name": "おにび",
              "type": "Fire",
              "category": "Status",
              "power": 0,
              "description": "不気味で怪しい炎を放って相手をやけどの状態にする。"
            }
          ],
          "actions": [
            "時間85% 弱体解除",
            "時間65% おにび",
            "時間60% 強化解除",
            "体力50% 弱体解除",
            "体力20% にほんばれ"
          ],
          "hp": 5875,
          "atk": 230,
          "def": 177,
          "sp_atk": 113,
          "sp_def": 213,
          "speed": 158
        },
        {
          "name": "タイカイデン",
          "star": 6,
          "types": [
            "Electric",
            "Flying"
          ],
          "abilities": [
            {
              "name": "ふうりょくでんき",
              "description": "風技を受けるとじゅうでん状態になる。"
            },
            {
              "name": "ちくでん",
              "description": "でんきタイプの技を受けるとダメージを受けずに回復する。"
            },
            {
              "name": "かちき",
              "description": "相手に能力を下げられると特攻がぐーんと上がる。"
            }
          ],
          "moves": [
            {
              "name": "ぼうふう",
              "type": "Flying",
              "category": "Special",
              "power": 110,
              "description": "強烈な風で相手を包みこんで攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "かみなり",
              "type": "Electric",
              "category": "Special",
              "power": 110,
              "description": "激しい雷を相手に落として攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "さわぐ",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "３ターンの間騒いで相手を攻撃する。そのあいだはだれも眠れなくなる。"
            },
            {
              "name": "こわいかお",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "恐ろしい顔でにらみおびえさせて相手の素早さをがくっとさげる。"
            }
          ],
          "actions": [
            "時間85% テラ回収",
            "時間60% 強化解除",
            "体力90% じゅうでん",
            "体力50% 弱体解除",
            "体力20% あまごい"
          ],
          "hp": 5650,
          "atk": 131,
          "def": 141,
          "sp_atk": 194,
          "sp_def": 141,
          "speed": 230
        },
        {
          "name": "タギングル",
          "star": 6,
          "types": [
            "Poison",
            "Normal"
          ],
          "abilities": [
            {
              "name": "かるわざ",
              "description": "持っていた道具がなくなると素早さが上がる。"
            },
            {
              "name": "どくしゅ",
              "description": "触るだけで相手をどく状態にすることがある。"
            },
            {
              "name": "いたずらごころ",
              "description": "変化技を先制で出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "はたきおとす",
              "type": "Dark",
              "category": "Physical",
              "power": 65,
              "description": "相手の持ち物をはたき落として戦闘が終わるまで使えなくする。物を持つ相手にはダメージが増す。"
            },
            {
              "name": "ダストシュート",
              "type": "Poison",
              "category": "Physical",
              "power": 120,
              "description": "汚いゴミを相手にぶつけて攻撃する。毒状態にすることがある。"
            },
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "おだてる",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手をおだてて混乱させる。同時に相手の特攻もあげてしまう。"
            }
          ],
          "actions": [
            "時間95% 弱体解除",
            "時間60% 強化解除",
            "体力75% どくどく",
            "体力50% 弱体解除",
            "体力20% ダストシュート"
          ],
          "hp": 5325,
          "atk": 176,
          "def": 150,
          "sp_atk": 149,
          "sp_def": 163,
          "speed": 203
        },
        {
          "name": "デカヌチャン",
          "star": 6,
          "types": [
            "Fairy",
            "Steel"
          ],
          "abilities": [
            {
              "name": "かたやぶり",
              "description": "相手の特性にジャマされることなく相手に技を出すことができる。"
            },
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "わるいてぐせ",
              "description": "触られた相手の道具を盗んでしまう。"
            }
          ],
          "moves": [
            {
              "name": "デカハンマー",
              "type": "Steel",
              "category": "Physical",
              "power": 160,
              "description": "大きなハンマーを体ごとぶんまわして攻撃する。この技は２回連続でだせない。"
            },
            {
              "name": "じゃれつく",
              "type": "Fairy",
              "category": "Physical",
              "power": 90,
              "description": "相手にじゃれついて攻撃する。相手の攻撃をさげることがある。"
            },
            {
              "name": "はたきおとす",
              "type": "Dark",
              "category": "Physical",
              "power": 65,
              "description": "相手の持ち物をはたき落として戦闘が終わるまで使えなくする。物を持つ相手にはダメージが増す。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            }
          ],
          "actions": [
            "時間90% ミストフィールド",
            "時間60% 強化解除",
            "体力75% 弱体解除",
            "体力50% 弱体解除",
            "体力30% てんしのキッス",
            "体力15% てんしのキッス"
          ],
          "hp": 6325,
          "atk": 140,
          "def": 172,
          "sp_atk": 131,
          "sp_def": 222,
          "speed": 174
        },
        {
          "name": "ドオー",
          "star": 6,
          "types": [
            "Poison",
            "Ground"
          ],
          "abilities": [
            {
              "name": "どくのトゲ",
              "description": "自分に触った相手をどく状態にすることがある。"
            },
            {
              "name": "ちょすい",
              "description": "みずタイプの技を受けるとダメージを受けずに回復する。"
            },
            {
              "name": "てんねん",
              "description": "相手の能力の変化を無視して攻撃ができる。"
            }
          ],
          "moves": [
            {
              "name": "じしん",
              "type": "Ground",
              "category": "Physical",
              "power": 100,
              "description": "地震の衝撃で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "どくづき",
              "type": "Poison",
              "category": "Physical",
              "power": 80,
              "description": "毒にそまった触手や腕で相手を突き刺す。毒状態にすることがある。"
            },
            {
              "name": "メガホーン",
              "type": "Bug",
              "category": "Physical",
              "power": 120,
              "description": "硬くてりっぱなつのでおもいっきり相手を突き刺して攻撃する。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            }
          ],
          "actions": [
            "時間95% 強化解除",
            "時間60% 強化解除",
            "体力75% あくび",
            "体力50% 弱体解除",
            "体力20% じしん"
          ],
          "hp": 8350,
          "atk": 140,
          "def": 141,
          "sp_atk": 86,
          "sp_def": 213,
          "speed": 41
        },
        {
          "name": "ドドゲザン",
          "star": 6,
          "types": [
            "Dark",
            "Steel"
          ],
          "abilities": [
            {
              "name": "まけんき",
              "description": "相手に能力を下げられると攻撃がぐーんと上がる。"
            },
            {
              "name": "そうだいしょう",
              "description": "登場したとき今まで倒された味方の数が多いほど少しずつ攻撃と特攻が上がる。"
            },
            {
              "name": "プレッシャー",
              "description": "プレッシャーをあたえて相手の使う技のＰＰを多く減らす。"
            }
          ],
          "moves": [
            {
              "name": "アイアンヘッド",
              "type": "Steel",
              "category": "Physical",
              "power": 80,
              "description": "鋼のような硬い頭で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "つじぎり",
              "type": "Dark",
              "category": "Physical",
              "power": 70,
              "description": "一瞬のすきをついて相手を切りはらう。急所に当たりやすい。"
            },
            {
              "name": "ドゲザン",
              "type": "Dark",
              "category": "Physical",
              "power": 85,
              "description": "土下座して相手を油断させておいて切りかかる。攻撃は必ず命中する。"
            },
            {
              "name": "でんじは",
              "type": "Electric",
              "category": "Status",
              "power": 0,
              "description": "弱い電撃を浴びせることで相手をまひ状態にする。"
            }
          ],
          "actions": [
            "時間85% 弱体解除",
            "時間65% テラ回収",
            "時間60% 強化解除",
            "体力50% 弱体解除",
            "体力20% つるぎのまい"
          ],
          "hp": 7000,
          "atk": 248,
          "def": 249,
          "sp_atk": 113,
          "sp_def": 186,
          "speed": 95
        },
        {
          "name": "ドヒドイデ",
          "star": 6,
          "types": [
            "Poison",
            "Water"
          ],
          "abilities": [
            {
              "name": "ひとでなし",
              "description": "どく状態の相手を攻撃するとかならず急所に当たる。"
            },
            {
              "name": "じゅうなん",
              "description": "柔軟な体によってまひ状態にならない。"
            },
            {
              "name": "さいせいりょく",
              "description": "手持ちに引っ込むとＨＰが少し回復する。"
            }
          ],
          "moves": [
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "アクアブレイク",
              "type": "Water",
              "category": "Physical",
              "power": 85,
              "description": "水の力で相手にぶつかって攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "どくづき",
              "type": "Poison",
              "category": "Physical",
              "power": 80,
              "description": "毒にそまった触手や腕で相手を突き刺す。毒状態にすることがある。"
            },
            {
              "name": "ミサイルばり",
              "type": "Bug",
              "category": "Physical",
              "power": 25,
              "description": "鋭いハリを相手に発射して攻撃する。２ー５回の間連続でだす。"
            }
          ],
          "actions": [
            "時間75% どくどく",
            "時間60% 強化解除",
            "体力95% ひやみず",
            "体力50% 弱体解除",
            "体力20% ひやみず"
          ],
          "hp": 4750,
          "atk": 118,
          "def": 307,
          "sp_atk": 100,
          "sp_def": 289,
          "speed": 68
        },
        {
          "name": "ドラパルト",
          "star": 6,
          "types": [
            "Dragon",
            "Ghost"
          ],
          "abilities": [
            {
              "name": "クリアボディ",
              "description": "相手の技や特性で能力を下げられない。"
            },
            {
              "name": "すりぬけ",
              "description": "相手の壁や身代わりをすりぬけて攻撃できる。"
            },
            {
              "name": "のろわれボディ",
              "description": "攻撃を受けると相手の技をかなしばり状態にすることがある。"
            }
          ],
          "moves": [
            {
              "name": "シャドーボール",
              "type": "Ghost",
              "category": "Special",
              "power": 80,
              "description": "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "１０まんボルト",
              "type": "Electric",
              "category": "Special",
              "power": 90,
              "description": "強い電撃を相手に浴びせて攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "かえんほうしゃ",
              "type": "Fire",
              "category": "Special",
              "power": 90,
              "description": "激しい炎を相手に発射して攻撃する。やけど状態にすることがある。"
            }
          ],
          "actions": [
            "時間85% リフレクター",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力40% 強化解除",
            "体力20% ひかりのかべ"
          ],
          "hp": 6450,
          "atk": 221,
          "def": 168,
          "sp_atk": 185,
          "sp_def": 168,
          "speed": 260
        },
        {
          "name": "ドラミドロ",
          "star": 6,
          "types": [
            "Poison",
            "Dragon"
          ],
          "abilities": [
            {
              "name": "どくのトゲ",
              "description": "自分に触った相手をどく状態にすることがある。"
            },
            {
              "name": "どくしゅ",
              "description": "触るだけで相手をどく状態にすることがある。"
            },
            {
              "name": "てきおうりょく",
              "description": "自分とおなじタイプの技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "ヘドロばくだん",
              "type": "Poison",
              "category": "Special",
              "power": 90,
              "description": "汚いヘドロを相手に投げつけて攻撃する。毒状態にすることがある。"
            },
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "どくどく",
              "type": "Poison",
              "category": "Status",
              "power": 0,
              "description": "相手を猛毒の状態にする。ターンがすすむほど毒のダメージが増えていく。"
            }
          ],
          "actions": [
            "時間85% アシッドボム",
            "体力75% りゅうせいぐん",
            "体力50% 弱体解除",
            "体力45% テラ回収",
            "体力30% りゅうせいぐん"
          ],
          "hp": 5425,
          "atk": 140,
          "def": 195,
          "sp_atk": 179,
          "sp_def": 255,
          "speed": 84
        },
        {
          "name": "ニンフィア",
          "star": 6,
          "types": [
            "Fairy"
          ],
          "abilities": [
            {
              "name": "メロメロボディ",
              "description": "自分に触った相手をメロメロにすることがある。"
            },
            {
              "name": "フェアリースキン",
              "description": "ノーマルタイプの技がフェアリータイプになる。威力が少し上がる。"
            }
          ],
          "moves": [
            {
              "name": "テラバースト",
              "type": "Normal",
              "category": "Special",
              "power": 80,
              "description": "テラスタルだとテラスタイプのエネルギーを放出して攻撃する。攻撃と特攻を比べて高いほうでダメージを与える。"
            },
            {
              "name": "ハイパーボイス",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "うるさく響く大きな振動を相手に与えて攻撃する。"
            },
            {
              "name": "ムーンフォース",
              "type": "Fairy",
              "category": "Special",
              "power": 95,
              "description": "月のパワーをかりて相手を攻撃する。相手の特攻をさげることがある。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            }
          ],
          "actions": [
            "時間70% テラ回収",
            "時間60% 強化解除",
            "体力90% ミストフィールド",
            "体力50% 弱体解除",
            "体力35% めいそう",
            "体力20% ミストフィールド"
          ],
          "hp": 6775,
          "atk": 122,
          "def": 122,
          "sp_atk": 203,
          "sp_def": 267,
          "speed": 113
        },
        {
          "name": "ヌメルゴン",
          "star": 6,
          "types": [
            "Dragon"
          ],
          "abilities": [
            {
              "name": "そうしょく",
              "description": "くさタイプの技を受けるとダメージを受けずに攻撃が上がる。"
            },
            {
              "name": "うるおいボディ",
              "description": "天気が雨のとき状態異常が治る。"
            },
            {
              "name": "ぬめぬめ",
              "description": "攻撃で自分に触れた相手の素早さを下げる。"
            }
          ],
          "moves": [
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "なみのり",
              "type": "Water",
              "category": "Special",
              "power": 90,
              "description": "大きな波で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "ヘドロばくだん",
              "type": "Poison",
              "category": "Special",
              "power": 90,
              "description": "汚いヘドロを相手に投げつけて攻撃する。毒状態にすることがある。"
            },
            {
              "name": "パワーウィップ",
              "type": "Grass",
              "category": "Physical",
              "power": 120,
              "description": "ツタや触手を激しくふるって相手をたたきつけ攻撃する。"
            }
          ],
          "actions": [
            "時間70% テラ回収",
            "時間60% 強化解除",
            "体力90% あまごい",
            "体力50% 弱体解除",
            "体力30% あまごい"
          ],
          "hp": 6550,
          "atk": 185,
          "def": 159,
          "sp_atk": 203,
          "sp_def": 303,
          "speed": 149
        },
        {
          "name": "ハッサム",
          "star": 6,
          "types": [
            "Bug",
            "Steel"
          ],
          "abilities": [
            {
              "name": "むしのしらせ",
              "description": "ＨＰが減ったときむしタイプの技の威力が上がる。"
            },
            {
              "name": "テクニシャン",
              "description": "威力が低い技の威力を高くして攻撃できる。"
            },
            {
              "name": "ライトメタル",
              "description": "自分の重さが半分になる。"
            }
          ],
          "moves": [
            {
              "name": "シザークロス",
              "type": "Bug",
              "category": "Physical",
              "power": 80,
              "description": "カマやツメをハサミのように交差させながら相手を切り裂く。"
            },
            {
              "name": "バレットパンチ",
              "type": "Steel",
              "category": "Physical",
              "power": 40,
              "description": "弾丸のような速くて硬いパンチを相手にくりだす。必ず先制攻撃できる。"
            },
            {
              "name": "インファイト",
              "type": "Fighting",
              "category": "Physical",
              "power": 120,
              "description": "守りを捨てて相手のふところに突撃する。自分の防御と特防がさがる。"
            },
            {
              "name": "アイアンヘッド",
              "type": "Steel",
              "category": "Physical",
              "power": 80,
              "description": "鋼のような硬い頭で攻撃する。相手をひるませることがある。"
            }
          ],
          "actions": [
            "時間85% てっぺき",
            "時間60% 強化解除",
            "体力75% きあいだめ",
            "体力50% 弱体解除"
          ],
          "hp": 5650,
          "atk": 239,
          "def": 213,
          "sp_atk": 104,
          "sp_def": 177,
          "speed": 122
        },
        {
          "name": "ハピナス",
          "star": 6,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "しぜんかいふく",
              "description": "手持ちにひっこむと状態異常が治る。"
            },
            {
              "name": "てんのめぐみ",
              "description": "天の恵みのおかげで技の追加効果がでやすい。"
            },
            {
              "name": "いやしのこころ",
              "description": "状態異常の味方をたまに治してあげる。"
            }
          ],
          "moves": [
            {
              "name": "マジカルシャイン",
              "type": "Fairy",
              "category": "Special",
              "power": 80,
              "description": "強力な光を放ち相手にダメージを与える。"
            },
            {
              "name": "ハイパーボイス",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "うるさく響く大きな振動を相手に与えて攻撃する。"
            },
            {
              "name": "うたう",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "心地好いきれいな歌声を聞かせて相手を眠り状態にする。"
            },
            {
              "name": "ひかりのかべ",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "５ターンの間不思議なかべで相手から受ける特殊攻撃のダメージを弱める。"
            }
          ],
          "actions": [
            "時間60% 強化解除",
            "体力95% まるくなる",
            "体力75% まるくなる",
            "体力50% 弱体解除",
            "体力30% 弱体解除",
            "体力30% うたう"
          ],
          "hp": 13975,
          "atk": 23,
          "def": 23,
          "sp_atk": 140,
          "sp_def": 276,
          "speed": 104
        },
        {
          "name": "ハルクジラ",
          "star": 6,
          "types": [
            "Ice"
          ],
          "abilities": [
            {
              "name": "あついしぼう",
              "description": "厚い脂肪で守られているのでほのおタイプとこおりタイプの技のダメージを半減させる。"
            },
            {
              "name": "ゆきかき",
              "description": "天気がゆきのとき素早さが上がる。"
            },
            {
              "name": "ちからずく",
              "description": "技の追加効果はなくなるがそのぶん高い威力で技を出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "アイススピナー",
              "type": "Ice",
              "category": "Physical",
              "power": 80,
              "description": "足に薄い氷をまといクルクルと回りながらぶつかる。回転の動きによってフィールドを壊す。"
            },
            {
              "name": "のしかかり",
              "type": "Normal",
              "category": "Physical",
              "power": 85,
              "description": "全身で相手にのしかかり攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "ゆきげしき",
              "type": "Ice",
              "category": "Status",
              "power": 0,
              "description": "５ターンの間ゆきを降らせる。こおりタイプの防御があがる。"
            },
            {
              "name": "じだんだ",
              "type": "Ground",
              "category": "Physical",
              "power": 75,
              "description": "悔しさをバネにして攻撃する。前のターンに技を外していると威力が倍になる。"
            }
          ],
          "actions": [
            "時間95% 弱体解除",
            "時間75% ゆきげしき",
            "時間60% 強化解除",
            "体力50% 弱体解除",
            "体力30% あくび"
          ],
          "hp": 10150,
          "atk": 208,
          "def": 150,
          "sp_atk": 86,
          "sp_def": 132,
          "speed": 136
        },
        {
          "name": "バウッツェル",
          "star": 6,
          "types": [
            "Fairy"
          ],
          "abilities": [
            {
              "name": "こんがりボディ",
              "description": "ほのおタイプの技を受けるとダメージを受けずに防御がぐーんと上がる。"
            },
            {
              "name": "アロマベール",
              "description": "自分と味方へのメンタル攻撃を防ぐことができる。"
            }
          ],
          "moves": [
            {
              "name": "じゃれつく",
              "type": "Fairy",
              "category": "Physical",
              "power": 90,
              "description": "相手にじゃれついて攻撃する。相手の攻撃をさげることがある。"
            },
            {
              "name": "すてみタックル",
              "type": "Normal",
              "category": "Physical",
              "power": 120,
              "description": "命を懸けて相手に突進して攻撃する。自分もかなりダメージを受ける。"
            },
            {
              "name": "かみつく",
              "type": "Dark",
              "category": "Physical",
              "power": 60,
              "description": "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "つぶらなひとみ",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "つぶらなひとみで相手をみつめて攻撃をさげる。必ず先制攻撃できる。"
            }
          ],
          "actions": [
            "時間95% 弱体解除",
            "時間60% 強化解除",
            "時間60% テラ回収",
            "体力50% 弱体解除",
            "体力20% じゃれつく"
          ],
          "hp": 5050,
          "atk": 149,
          "def": 240,
          "sp_atk": 95,
          "sp_def": 177,
          "speed": 176
        },
        {
          "name": "バチンウニ",
          "star": 6,
          "types": [
            "Electric"
          ],
          "abilities": [
            {
              "name": "ひらいしん",
              "description": "でんきタイプの技を自分に寄せつけダメージを受けずに特攻が上がる。"
            },
            {
              "name": "エレキメイカー",
              "description": "登場したときにエレキフィールドをはりめぐらせる。"
            }
          ],
          "moves": [
            {
              "name": "びりびりちくちく",
              "type": "Electric",
              "category": "Physical",
              "power": 80,
              "description": "相手にぶつかって強力な電気を浴びせびりびりちくちくさせる。相手をひるませることがある。"
            },
            {
              "name": "かみなり",
              "type": "Electric",
              "category": "Special",
              "power": 110,
              "description": "激しい雷を相手に落として攻撃する。まひ状態にすることがある。"
            },
            {
              "name": "なみのり",
              "type": "Water",
              "category": "Special",
              "power": 90,
              "description": "大きな波で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "どくづき",
              "type": "Poison",
              "category": "Physical",
              "power": 80,
              "description": "毒にそまった触手や腕で相手を突き刺す。毒状態にすることがある。"
            }
          ],
          "actions": [
            "時間90% でんじは",
            "時間65% エレキフィールド",
            "時間60% 強化解除",
            "体力50% 弱体解除",
            "体力20% エレキフィールド"
          ],
          "hp": 4650,
          "atk": 186,
          "def": 204,
          "sp_atk": 168,
          "sp_def": 186,
          "speed": 32
        },
        {
          "name": "バンギラス",
          "star": 6,
          "types": [
            "Rock",
            "Dark"
          ],
          "abilities": [
            {
              "name": "すなおこし",
              "description": "登場したとき天気を砂あらしにする。"
            },
            {
              "name": "きんちょうかん",
              "description": "相手を緊張させてきのみを食べられなくさせる。"
            }
          ],
          "moves": [
            {
              "name": "ストーンエッジ",
              "type": "Rock",
              "category": "Physical",
              "power": 100,
              "description": "とがった岩を相手に突き刺して攻撃する。急所に当たりやすい。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "いやなおと",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "おもわず耳をふさぎたくなるいやなおとをだして相手の防御をがくっとさげる。"
            },
            {
              "name": "ロックブラスト",
              "type": "Rock",
              "category": "Physical",
              "power": 25,
              "description": "硬い岩石を相手に発射して攻撃する。２ー５回の間連続でだす。"
            }
          ],
          "actions": [
            "時間75% テラ回収",
            "時間60% 強化解除",
            "体力90% 弱体解除",
            "体力50% 弱体解除",
            "体力30% かみくだく",
            "体力20% てっぺき"
          ],
          "hp": 7000,
          "atk": 246,
          "def": 203,
          "sp_atk": 176,
          "sp_def": 213,
          "speed": 114
        },
        {
          "name": "パーモット",
          "star": 6,
          "types": [
            "Electric",
            "Fighting"
          ],
          "abilities": [
            {
              "name": "ちくでん",
              "description": "でんきタイプの技を受けるとダメージを受けずに回復する。"
            },
            {
              "name": "しぜんかいふく",
              "description": "手持ちにひっこむと状態異常が治る。"
            },
            {
              "name": "てつのこぶし",
              "description": "パンチを使う技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ワイルドボルト",
              "type": "Electric",
              "category": "Physical",
              "power": 90,
              "description": "電気をまとって相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            },
            {
              "name": "インファイト",
              "type": "Fighting",
              "category": "Physical",
              "power": 120,
              "description": "守りを捨てて相手のふところに突撃する。自分の防御と特防がさがる。"
            },
            {
              "name": "でんこうそうげき",
              "type": "Electric",
              "category": "Physical",
              "power": 120,
              "description": "全身のでんきをすべて放って大ダメージを与える。自分のでんきタイプがなくなる。"
            },
            {
              "name": "ほっぺすりすり",
              "type": "Electric",
              "category": "Physical",
              "power": 20,
              "description": "電気を帯びたほっぺをすりつけて攻撃。相手をまひ状態にする。"
            }
          ],
          "actions": [
            "時間95% 弱体解除",
            "時間60% 強化解除",
            "体力80% テラ回収",
            "体力50% 弱体解除",
            "体力30% エレキフィールド"
          ],
          "hp": 5650,
          "atk": 212,
          "def": 159,
          "sp_atk": 131,
          "sp_def": 141,
          "speed": 194
        },
        {
          "name": "ファイアロー",
          "star": 6,
          "types": [
            "Fire",
            "Flying"
          ],
          "abilities": [
            {
              "name": "ほのおのからだ",
              "description": "自分に触った相手をやけど状態にすることがある。"
            },
            {
              "name": "はやてのつばさ",
              "description": "ＨＰが満タンだとひこうタイプの技を先制で出すことができる。"
            }
          ],
          "moves": [
            {
              "name": "ブレイブバード",
              "type": "Flying",
              "category": "Physical",
              "power": 120,
              "description": "はねをおりたたみ低空飛行で突撃する。自分もかなりダメージを受ける。"
            },
            {
              "name": "フレアドライブ",
              "type": "Fire",
              "category": "Physical",
              "power": 120,
              "description": "炎をまとって突進する。自分もかなりダメージを受ける。やけど状態にすることがある。"
            },
            {
              "name": "かえんほうしゃ",
              "type": "Fire",
              "category": "Special",
              "power": 90,
              "description": "激しい炎を相手に発射して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "テラバースト",
              "type": "Normal",
              "category": "Special",
              "power": 80,
              "description": "テラスタルだとテラスタイプのエネルギーを放出して攻撃する。攻撃と特攻を比べて高いほうでダメージを与える。"
            }
          ],
          "actions": [
            "時間70% テラ回収",
            "時間60% 強化解除",
            "体力90% にほんばれ",
            "体力50% 弱体解除",
            "体力20% つるぎのまい"
          ],
          "hp": 6000,
          "atk": 150,
          "def": 161,
          "sp_atk": 138,
          "sp_def": 158,
          "speed": 231
        },
        {
          "name": "ブラッキー",
          "star": 6,
          "types": [
            "Dark"
          ],
          "abilities": [
            {
              "name": "シンクロ",
              "description": "自分がなってしまったどくやまひややけどを相手にうつす。"
            },
            {
              "name": "せいしんりょく",
              "description": "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。"
            }
          ],
          "moves": [
            {
              "name": "テラバースト",
              "type": "Normal",
              "category": "Special",
              "power": 80,
              "description": "テラスタルだとテラスタイプのエネルギーを放出して攻撃する。攻撃と特攻を比べて高いほうでダメージを与える。"
            },
            {
              "name": "あくのはどう",
              "type": "Dark",
              "category": "Special",
              "power": 80,
              "description": "体から悪意にみちた恐ろしいオーラを発する。相手をひるませることがある。"
            },
            {
              "name": "イカサマ",
              "type": "Dark",
              "category": "Physical",
              "power": 95,
              "description": "相手の力を利用する。戦っている相手の攻撃が高いほどダメージがあがる。"
            },
            {
              "name": "くすぐる",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "体をくすぐり笑わせることで相手の攻撃と防御をさげる。"
            }
          ],
          "actions": [
            "時間70% テラ回収",
            "時間60% 強化解除",
            "体力90% めいそう",
            "体力50% 弱体解除",
            "体力35% のろい",
            "体力20% めいそう"
          ],
          "hp": 6775,
          "atk": 122,
          "def": 203,
          "sp_atk": 113,
          "sp_def": 267,
          "speed": 122
        },
        {
          "name": "ブロスター",
          "star": 6,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "メガランチャー",
              "description": "波動の技の威力が高くなる。"
            }
          ],
          "moves": [
            {
              "name": "みずのはどう",
              "type": "Water",
              "category": "Special",
              "power": 60,
              "description": "水の振動を相手に与えて攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "はどうだん",
              "type": "Fighting",
              "category": "Special",
              "power": 80,
              "description": "体の奥から波導の力を相手にうち放つ。攻撃は必ず命中する。"
            },
            {
              "name": "クラブハンマー",
              "type": "Water",
              "category": "Physical",
              "power": 100,
              "description": "大きなハサミを相手にたたきつけて攻撃する。急所に当たりやすい。"
            }
          ],
          "actions": [
            "時間85% あまごい",
            "時間75% テラ回収",
            "体力50% 弱体解除",
            "体力45% 強化解除",
            "体力30% みずのはどう"
          ],
          "hp": 5675,
          "atk": 136,
          "def": 192,
          "sp_atk": 221,
          "sp_def": 194,
          "speed": 111
        },
        {
          "name": "ブロロローム",
          "star": 6,
          "types": [
            "Steel",
            "Poison"
          ],
          "abilities": [
            {
              "name": "ぼうじん",
              "description": "すなあらしのダメージを受けない。粉や胞子の影響も受けない。"
            },
            {
              "name": "フィルター",
              "description": "効果バツグンになってしまう攻撃の威力を弱めることができる。"
            }
          ],
          "moves": [
            {
              "name": "ダストシュート",
              "type": "Poison",
              "category": "Physical",
              "power": 120,
              "description": "汚いゴミを相手にぶつけて攻撃する。毒状態にすることがある。"
            },
            {
              "name": "オーバーヒート",
              "type": "Fire",
              "category": "Special",
              "power": 130,
              "description": "フルパワーで相手を攻撃する。使うと反動で自分の特攻ががくっとさがる。"
            },
            {
              "name": "アイアンヘッド",
              "type": "Steel",
              "category": "Physical",
              "power": 80,
              "description": "鋼のような硬い頭で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            }
          ],
          "actions": [
            "時間85% こわいかお",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力40% ギアチェンジ",
            "体力20% ギアチェンジ"
          ],
          "hp": 6100,
          "atk": 219,
          "def": 195,
          "sp_atk": 102,
          "sp_def": 154,
          "speed": 167
        },
        {
          "name": "ブースター",
          "star": 6,
          "types": [
            "Fire"
          ],
          "abilities": [
            {
              "name": "もらいび",
              "description": "ほのおタイプの技を受けると炎をもらい自分が出すほのおタイプの技が強くなる。"
            },
            {
              "name": "こんじょう",
              "description": "状態異常になると根性をだして攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "テラバースト",
              "type": "Normal",
              "category": "Special",
              "power": 80,
              "description": "テラスタルだとテラスタイプのエネルギーを放出して攻撃する。攻撃と特攻を比べて高いほうでダメージを与える。"
            },
            {
              "name": "フレアドライブ",
              "type": "Fire",
              "category": "Physical",
              "power": 120,
              "description": "炎をまとって突進する。自分もかなりダメージを受ける。やけど状態にすることがある。"
            },
            {
              "name": "ふんえん",
              "type": "Fire",
              "category": "Special",
              "power": 80,
              "description": "真っ赤な炎で自分の周りにいるものを攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "おにび",
              "type": "Fire",
              "category": "Status",
              "power": 0,
              "description": "不気味で怪しい炎を放って相手をやけどの状態にする。"
            }
          ],
          "actions": [
            "時間70% テラ回収",
            "時間60% 強化解除",
            "体力90% にほんばれ",
            "体力50% 弱体解除",
            "体力35% のろい",
            "体力20% にほんばれ"
          ],
          "hp": 5425,
          "atk": 239,
          "def": 113,
          "sp_atk": 176,
          "sp_def": 231,
          "speed": 122
        },
        {
          "name": "ヘイラッシャ",
          "star": 6,
          "types": [
            "Water"
          ],
          "abilities": [
            {
              "name": "てんねん",
              "description": "相手の能力の変化を無視して攻撃ができる。"
            },
            {
              "name": "どんかん",
              "description": "鈍感なのでメロメロやちょうはつ状態にならない。いかくにも動じない。"
            },
            {
              "name": "みずのベール",
              "description": "水のベールを身にまといやけど状態にならない。"
            }
          ],
          "moves": [
            {
              "name": "ウェーブタックル",
              "type": "Water",
              "category": "Physical",
              "power": 120,
              "description": "水をまといつつ全身で相手にぶつかるが自分もかなりのダメージを受ける。"
            },
            {
              "name": "いっちょうあがり",
              "type": "Dragon",
              "category": "Physical",
              "power": 80,
              "description": "いなせな身のこなしで攻撃。口の中にシャリタツがいるとそのすがたによって能力があがる。"
            },
            {
              "name": "ヘビーボンバー",
              "type": "Steel",
              "category": "Physical",
              "power": 1,
              "description": "重たい体で相手にぶつかって攻撃する。自分が相手より重いほど威力があがる。"
            },
            {
              "name": "あくび",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。"
            }
          ],
          "actions": [
            "時間95% 弱体解除",
            "時間60% 強化解除",
            "体力75% あまごい",
            "体力50% 弱体解除",
            "体力20% のろい"
          ],
          "hp": 9250,
          "atk": 185,
          "def": 240,
          "sp_atk": 122,
          "sp_def": 150,
          "speed": 68
        },
        {
          "name": "ヘラクロス",
          "star": 6,
          "types": [
            "Bug",
            "Fighting"
          ],
          "abilities": [
            {
              "name": "むしのしらせ",
              "description": "ＨＰが減ったときむしタイプの技の威力が上がる。"
            },
            {
              "name": "こんじょう",
              "description": "状態異常になると根性をだして攻撃が上がる。"
            },
            {
              "name": "じしんかじょう",
              "description": "相手を倒すと自信がついて攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "メガホーン",
              "type": "Bug",
              "category": "Physical",
              "power": 120,
              "description": "硬くてりっぱなつのでおもいっきり相手を突き刺して攻撃する。"
            },
            {
              "name": "インファイト",
              "type": "Fighting",
              "category": "Physical",
              "power": 120,
              "description": "守りを捨てて相手のふところに突撃する。自分の防御と特防がさがる。"
            },
            {
              "name": "あばれる",
              "type": "Normal",
              "category": "Physical",
              "power": 120,
              "description": "２ー３ターンの間暴れまくって相手を攻撃する。暴れたあとは混乱する。"
            },
            {
              "name": "にらみつける",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "鋭い目つきでおびえさせて相手の防御をさげる。"
            }
          ],
          "actions": [
            "時間60% 強化解除",
            "体力75% ビルドアップ",
            "体力50% 弱体解除",
            "体力20% きあいだめ"
          ],
          "hp": 6100,
          "atk": 230,
          "def": 168,
          "sp_atk": 77,
          "sp_def": 204,
          "speed": 158
        },
        {
          "name": "ペリッパー",
          "star": 6,
          "types": [
            "Water",
            "Flying"
          ],
          "abilities": [
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            },
            {
              "name": "あめふらし",
              "description": "登場したときに天気を雨にする。"
            },
            {
              "name": "あめうけざら",
              "description": "天気が雨のとき少しずつＨＰを回復する。"
            }
          ],
          "moves": [
            {
              "name": "ぼうふう",
              "type": "Flying",
              "category": "Special",
              "power": 110,
              "description": "強烈な風で相手を包みこんで攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "ハイドロポンプ",
              "type": "Water",
              "category": "Special",
              "power": 110,
              "description": "大量の水を激しい勢いで相手に発射して攻撃する。"
            },
            {
              "name": "しろいきり",
              "type": "Ice",
              "category": "Status",
              "power": 0,
              "description": "白い霧で体をおおう。５ターンの間相手に能力をさげられなくなる。"
            },
            {
              "name": "ちょうおんぱ",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "特殊な音波を体から発して相手を混乱させる。"
            }
          ],
          "actions": [
            "時間75% テラ回収",
            "時間60% 強化解除",
            "体力90% あまごい",
            "体力50% 弱体解除",
            "体力30% こうそくいどう",
            "体力20% あまごい"
          ],
          "hp": 5200,
          "atk": 95,
          "def": 185,
          "sp_atk": 176,
          "sp_def": 159,
          "speed": 122
        },
        {
          "name": "ボーマンダ",
          "star": 6,
          "types": [
            "Dragon",
            "Flying"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "じしんかじょう",
              "description": "相手を倒すと自信がついて攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "げきりん",
              "type": "Dragon",
              "category": "Physical",
              "power": 120,
              "description": "２ー３ターンの間暴れまくって攻撃する。暴れたあとは混乱する。"
            },
            {
              "name": "ダブルウイング",
              "type": "Flying",
              "category": "Physical",
              "power": 40,
              "description": "翼を相手にぶつけて攻撃する。２回連続でダメージを与える。"
            },
            {
              "name": "かえんほうしゃ",
              "type": "Fire",
              "category": "Special",
              "power": 90,
              "description": "激しい炎を相手に発射して攻撃する。やけど状態にすることがある。"
            },
            {
              "name": "テラバースト",
              "type": "Normal",
              "category": "Special",
              "power": 80,
              "description": "テラスタルだとテラスタイプのエネルギーを放出して攻撃する。攻撃と特攻を比べて高いほうでダメージを与える。"
            }
          ],
          "actions": [
            "時間75% テラ回収",
            "時間60% 強化解除",
            "体力90% 弱体解除",
            "体力50% 弱体解除",
            "体力30% 弱体解除",
            "体力30% りゅうのまい"
          ],
          "hp": 6775,
          "atk": 248,
          "def": 149,
          "sp_atk": 203,
          "sp_def": 177,
          "speed": 185
        },
        {
          "name": "マフィティフ",
          "star": 6,
          "types": [
            "Dark"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "ばんけん",
              "description": "いかくされると攻撃が上がる。ポケモンを入れ替えさせる技や道具が効かない。"
            },
            {
              "name": "はりこみ",
              "description": "交代で出てきた相手に２倍のダメージで攻撃できる。"
            }
          ],
          "moves": [
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "きしかいせい",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "力をふりしぼり攻撃する。自分のＨＰが少ないほど技の威力はあがる。"
            },
            {
              "name": "げきりん",
              "type": "Dragon",
              "category": "Physical",
              "power": 120,
              "description": "２ー３ターンの間暴れまくって攻撃する。暴れたあとは混乱する。"
            },
            {
              "name": "とっしん",
              "type": "Normal",
              "category": "Physical",
              "power": 90,
              "description": "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。"
            }
          ],
          "actions": [
            "時間80% ちょうはつ",
            "時間60% 強化解除",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力20% 強化解除"
          ],
          "hp": 6100,
          "atk": 221,
          "def": 195,
          "sp_atk": 113,
          "sp_def": 159,
          "speed": 158
        },
        {
          "name": "ミミズズ",
          "star": 6,
          "types": [
            "Steel"
          ],
          "abilities": [
            {
              "name": "どしょく",
              "description": "じめんタイプの技を受けるとダメージを受けずに回復する。"
            },
            {
              "name": "すながくれ",
              "description": "砂あらしのとき回避率が上がる。"
            }
          ],
          "moves": [
            {
              "name": "アイアンヘッド",
              "type": "Steel",
              "category": "Physical",
              "power": 80,
              "description": "鋼のような硬い頭で攻撃する。相手をひるませることがある。"
            },
            {
              "name": "じしん",
              "type": "Ground",
              "category": "Physical",
              "power": 100,
              "description": "地震の衝撃で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "うちおとす",
              "type": "Rock",
              "category": "Physical",
              "power": 50,
              "description": "石や弾を投げて飛んでいる相手を攻撃する。相手はうち落とされて地面に落ちる。"
            },
            {
              "name": "すなあらし",
              "type": "Rock",
              "category": "Status",
              "power": 0,
              "description": "５ターンの間砂あらしでいわじめんはがねタイプ以外にダメージ。いわタイプの特防があがる。"
            }
          ],
          "actions": [
            "時間95% とぐろをまく",
            "時間60% 強化解除",
            "体力80% すなあらし",
            "体力50% 弱体解除",
            "体力20% すなあらし"
          ],
          "hp": 5650,
          "atk": 158,
          "def": 294,
          "sp_atk": 113,
          "sp_def": 132,
          "speed": 122
        },
        {
          "name": "ミミッキュ",
          "star": 6,
          "types": [
            "Ghost",
            "Fairy"
          ],
          "abilities": [
            {
              "name": "ばけのかわ",
              "description": "体を被う化けの皮で１回攻撃を防ぐことができる。"
            }
          ],
          "moves": [
            {
              "name": "じゃれつく",
              "type": "Fairy",
              "category": "Physical",
              "power": 90,
              "description": "相手にじゃれついて攻撃する。相手の攻撃をさげることがある。"
            },
            {
              "name": "シャドークロー",
              "type": "Ghost",
              "category": "Physical",
              "power": 70,
              "description": "影からつくった鋭いツメで相手を切り裂く。急所に当たりやすい。"
            },
            {
              "name": "かげうち",
              "type": "Ghost",
              "category": "Physical",
              "power": 40,
              "description": "影をのばして相手の背後から攻撃する。必ず先制攻撃できる。"
            },
            {
              "name": "ウッドハンマー",
              "type": "Grass",
              "category": "Physical",
              "power": 120,
              "description": "硬い胴体を相手にたたきつけて攻撃する。自分もかなりダメージを受ける。"
            }
          ],
          "actions": [
            "時間90% ミストフィールド",
            "時間75% 弱体解除",
            "時間60% 強化解除",
            "体力50% 弱体解除",
            "体力30% つるぎのまい"
          ],
          "hp": 4975,
          "atk": 167,
          "def": 177,
          "sp_atk": 95,
          "sp_def": 222,
          "speed": 177
        },
        {
          "name": "ムクホーク",
          "star": 6,
          "types": [
            "Normal",
            "Flying"
          ],
          "abilities": [
            {
              "name": "いかく",
              "description": "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。"
            },
            {
              "name": "すてみ",
              "description": "反動でダメージを受ける技の威力が上がる。"
            }
          ],
          "moves": [
            {
              "name": "インファイト",
              "type": "Fighting",
              "category": "Physical",
              "power": 120,
              "description": "守りを捨てて相手のふところに突撃する。自分の防御と特防がさがる。"
            },
            {
              "name": "ブレイブバード",
              "type": "Flying",
              "category": "Physical",
              "power": 120,
              "description": "はねをおりたたみ低空飛行で突撃する。自分もかなりダメージを受ける。"
            },
            {
              "name": "すてみタックル",
              "type": "Normal",
              "category": "Physical",
              "power": 120,
              "description": "命を懸けて相手に突進して攻撃する。自分もかなりダメージを受ける。"
            },
            {
              "name": "フェザーダンス",
              "type": "Flying",
              "category": "Status",
              "power": 0,
              "description": "羽毛をふりまいて相手の体にからませる。相手の攻撃をがくっとさげる。"
            }
          ],
          "actions": [
            "時間85% 弱体解除",
            "体力75% 強化解除",
            "体力50% 強化解除",
            "体力50% 弱体解除",
            "体力25% ブレイブバード"
          ],
          "hp": 6325,
          "atk": 221,
          "def": 159,
          "sp_atk": 95,
          "sp_def": 141,
          "speed": 185
        },
        {
          "name": "メタモン",
          "star": 6,
          "types": [
            "Normal"
          ],
          "abilities": [
            {
              "name": "じゅうなん",
              "description": "柔軟な体によってまひ状態にならない。"
            },
            {
              "name": "かわりもの",
              "description": "目の前のポケモンに変身してしまう。"
            }
          ],
          "moves": [
            {
              "name": "へんしん",
              "type": "Normal",
              "category": "Status",
              "power": 0,
              "description": "相手のポケモンに変身することで相手とまったく同じ技が使える。"
            }
          ],
          "actions": [
            "時間60% 強化解除",
            "体力50% 弱体解除"
          ],
          "hp": 4650,
          "atk": 91,
          "def": 120,
          "sp_atk": 91,
          "sp_def": 120,
          "speed": 91
        },
        {
          "name": "モスノウ",
          "star": 6,
          "types": [
            "Ice",
            "Bug"
          ],
          "abilities": [
            {
              "name": "りんぷん",
              "description": "りんぷんに守られて技の追加効果を受けなくなる。"
            },
            {
              "name": "こおりのりんぷん",
              "description": "こおりのりんぷんに守られて特殊攻撃で受けるダメージが半減する。"
            }
          ],
          "moves": [
            {
              "name": "ふぶき",
              "type": "Ice",
              "category": "Special",
              "power": 110,
              "description": "激しい吹雪を相手に吹きつけて攻撃する。こおり状態にすることがある。"
            },
            {
              "name": "むしのさざめき",
              "type": "Bug",
              "category": "Special",
              "power": 90,
              "description": "振動で音波をおこして攻撃する。相手の特防をさげることがある。"
            },
            {
              "name": "ぼうふう",
              "type": "Flying",
              "category": "Special",
              "power": 110,
              "description": "強烈な風で相手を包みこんで攻撃する。相手を混乱させることがある。"
            },
            {
              "name": "ゆきげしき",
              "type": "Ice",
              "category": "Status",
              "power": 0,
              "description": "５ターンの間ゆきを降らせる。こおりタイプの防御があがる。"
            }
          ],
          "actions": [
            "時間80% ゆきげしき",
            "時間60% 強化解除",
            "体力75% テラ回収",
            "体力50% 弱体解除",
            "体力20% ちょうのまい"
          ],
          "hp": 5650,
          "atk": 122,
          "def": 141,
          "sp_atk": 230,
          "sp_def": 195,
          "speed": 122
        },
        {
          "name": "モトトカゲ",
          "star": 6,
          "types": [
            "Dragon",
            "Normal"
          ],
          "abilities": [
            {
              "name": "だっぴ",
              "description": "体の皮を脱ぎ捨てることで状態異常を治すことがある。"
            },
            {
              "name": "さいせいりょく",
              "description": "手持ちに引っ込むとＨＰが少し回復する。"
            }
          ],
          "moves": [
            {
              "name": "すてみタックル",
              "type": "Normal",
              "category": "Physical",
              "power": 120,
              "description": "命を懸けて相手に突進して攻撃する。自分もかなりダメージを受ける。"
            },
            {
              "name": "ドラゴンクロー",
              "type": "Dragon",
              "category": "Physical",
              "power": 80,
              "description": "鋭くとがった巨大なツメで相手を切り裂いて攻撃する。"
            },
            {
              "name": "りゅうのはどう",
              "type": "Dragon",
              "category": "Special",
              "power": 85,
              "description": "大きな口から衝撃波をまきおこして相手を攻撃する。"
            },
            {
              "name": "はたきおとす",
              "type": "Dark",
              "category": "Physical",
              "power": 65,
              "description": "相手の持ち物をはたき落として戦闘が終わるまで使えなくする。物を持つ相手にはダメージが増す。"
            }
          ],
          "actions": [
            "時間95% 弱体解除",
            "時間80% はたきおとす",
            "時間60% 強化解除",
            "体力50% 弱体解除",
            "体力20% ギアチェンジ"
          ],
          "hp": 5650,
          "atk": 176,
          "def": 150,
          "sp_atk": 158,
          "sp_def": 150,
          "speed": 222
        },
        {
          "name": "モロバレル",
          "star": 6,
          "types": [
            "Grass",
            "Poison"
          ],
          "abilities": [
            {
              "name": "ほうし",
              "description": "攻撃で自分に触れた相手をどくやまひやねむり状態にすることがある。"
            },
            {
              "name": "さいせいりょく",
              "description": "手持ちに引っ込むとＨＰが少し回復する。"
            }
          ],
          "moves": [
            {
              "name": "エナジーボール",
              "type": "Grass",
              "category": "Special",
              "power": 90,
              "description": "自然から集めた命の力を発射する。相手の特防をさげることがある。"
            },
            {
              "name": "イカサマ",
              "type": "Dark",
              "category": "Physical",
              "power": 95,
              "description": "相手の力を利用する。戦っている相手の攻撃が高いほどダメージがあがる。"
            },
            {
              "name": "キノコのほうし",
              "type": "Grass",
              "category": "Status",
              "power": 0,
              "description": "催眠効果のある胞子をパラパラとふりまき相手を眠り状態にする。"
            },
            {
              "name": "ヘドロばくだん",
              "type": "Poison",
              "category": "Special",
              "power": 90,
              "description": "汚いヘドロを相手に投げつけて攻撃する。毒状態にすることがある。"
            }
          ],
          "actions": [
            "時間90% グラスフィールド",
            "時間70% テラ回収",
            "時間60% 強化解除",
            "体力50% 弱体解除",
            "体力20% グラスフィールド"
          ],
          "hp": 7625,
          "atk": 158,
          "def": 159,
          "sp_atk": 158,
          "sp_def": 177,
          "speed": 59
        },
        {
          "name": "ヤドキング",
          "star": 6,
          "types": [
            "Water",
            "Psychic"
          ],
          "abilities": [
            {
              "name": "どんかん",
              "description": "鈍感なのでメロメロやちょうはつ状態にならない。いかくにも動じない。"
            },
            {
              "name": "マイペース",
              "description": "マイペースなのでこんらん状態にならない。いかくにも動じない。"
            },
            {
              "name": "さいせいりょく",
              "description": "手持ちに引っ込むとＨＰが少し回復する。"
            }
          ],
          "moves": [
            {
              "name": "なみのり",
              "type": "Water",
              "category": "Special",
              "power": 90,
              "description": "大きな波で自分の周りにいるものを攻撃する。"
            },
            {
              "name": "サイコショック",
              "type": "Psychic",
              "category": "Special",
              "power": 80,
              "description": "不思議な念波を実体化して相手を攻撃する。物理的なダメージを与える。"
            },
            {
              "name": "トリックルーム",
              "type": "Psychic",
              "category": "Status",
              "power": 0,
              "description": "まか不思議な空間をつくる。５ターンの間遅いポケモンから行動できる。"
            },
            {
              "name": "かえんほうしゃ",
              "type": "Fire",
              "category": "Special",
              "power": 90,
              "description": "激しい炎を相手に発射して攻撃する。やけど状態にすることがある。"
            }
          ],
          "actions": [
            "時間70% ひかりのかべ",
            "時間60% 強化解除",
            "体力90% あまごい",
            "体力50% 弱体解除",
            "体力35% めいそう",
            "体力20% トリックルーム"
          ],
          "hp": 6775,
          "atk": 140,
          "def": 149,
          "sp_atk": 185,
          "sp_def": 231,
          "speed": 59
        },
        {
          "name": "リキキリン",
          "star": 6,
          "types": [
            "Normal",
            "Psychic"
          ],
          "abilities": [
            {
              "name": "はんすう",
              "description": "きのみを食べると次のターンの終わりに胃から出してもう１回だけ食べる。"
            },
            {
              "name": "テイルアーマー",
              "description": "頭を包む謎のしっぽがこちらにむかって先制技を出せないようにする。"
            },
            {
              "name": "そうしょく",
              "description": "くさタイプの技を受けるとダメージを受けずに攻撃が上がる。"
            }
          ],
          "moves": [
            {
              "name": "ツインビーム",
              "type": "Psychic",
              "category": "Special",
              "power": 40,
              "description": "両目から不可思議な光線を発射して攻撃する。２回連続でダメージを与える。"
            },
            {
              "name": "ハイパーボイス",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "うるさく響く大きな振動を相手に与えて攻撃する。"
            },
            {
              "name": "けたぐり",
              "type": "Fighting",
              "category": "Physical",
              "power": 1,
              "description": "足を強くけり相手を転ばせて攻撃する。相手が重いほど威力があがる。"
            },
            {
              "name": "さわぐ",
              "type": "Normal",
              "category": "Special",
              "power": 90,
              "description": "３ターンの間騒いで相手を攻撃する。そのあいだはだれも眠れなくなる。"
            }
          ],
          "actions": [
            "時間90% こうそくいどう",
            "時間75% サイコフィールド",
            "時間60% 強化解除",
            "体力50% 弱体解除",
            "体力20% さわぐ"
          ],
          "hp": 7900,
          "atk": 167,
          "def": 159,
          "sp_atk": 203,
          "sp_def": 159,
          "speed": 113
        },
        {
          "name": "リククラゲ",
          "star": 6,
          "types": [
            "Ground",
            "Grass"
          ],
          "abilities": [
            {
              "name": "きんしのちから",
              "description": "変化技を出すとき必ず行動が遅くなるが相手の特性にジャマされない。"
            }
          ],
          "moves": [
            {
              "name": "エナジーボール",
              "type": "Grass",
              "category": "Special",
              "power": 90,
              "description": "自然から集めた命の力を発射する。相手の特防をさげることがある。"
            },
            {
              "name": "だいちのちから",
              "type": "Ground",
              "category": "Special",
              "power": 90,
              "description": "相手の足下へ大地の力を放出する。相手の特防をさげることがある。"
            },
            {
              "name": "キノコのほうし",
              "type": "Grass",
              "category": "Status",
              "power": 0,
              "description": "催眠効果のある胞子をパラパラとふりまき相手を眠り状態にする。"
            },
            {
              "name": "たたりめ",
              "type": "Ghost",
              "category": "Special",
              "power": 65,
              "description": "たたみかけるように攻撃する。状態異常の相手に大きなダメージを与える。"
            }
          ],
          "actions": [
            "時間75% キノコのほうし",
            "時間60% 強化解除",
            "体力90% グラスフィールド",
            "体力50% 弱体解除",
            "体力25% キノコのほうし"
          ],
          "hp": 6100,
          "atk": 131,
          "def": 150,
          "sp_atk": 149,
          "sp_def": 249,
          "speed": 185
        },
        {
          "name": "リーフィア",
          "star": 6,
          "types": [
            "Grass"
          ],
          "abilities": [
            {
              "name": "リーフガード",
              "description": "天気が晴れのときは状態異常にならない。"
            },
            {
              "name": "ようりょくそ",
              "description": "天気が晴れのとき素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "テラバースト",
              "type": "Normal",
              "category": "Special",
              "power": 80,
              "description": "テラスタルだとテラスタイプのエネルギーを放出して攻撃する。攻撃と特攻を比べて高いほうでダメージを与える。"
            },
            {
              "name": "リーフブレード",
              "type": "Grass",
              "category": "Physical",
              "power": 90,
              "description": "はっぱを剣のようにあやつり相手を切りつけて攻撃する。急所に当たりやすい。"
            },
            {
              "name": "にどげり",
              "type": "Fighting",
              "category": "Physical",
              "power": 30,
              "description": "２本の足で相手をけとばして攻撃する。２回連続でダメージを与える。"
            },
            {
              "name": "あまえる",
              "type": "Fairy",
              "category": "Status",
              "power": 0,
              "description": "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。"
            }
          ],
          "actions": [
            "時間70% テラ回収",
            "時間60% 強化解除",
            "体力90% にほんばれ",
            "体力50% 弱体解除",
            "体力35% つるぎのまい",
            "体力20% つるぎのまい"
          ],
          "hp": 5425,
          "atk": 203,
          "def": 239,
          "sp_atk": 113,
          "sp_def": 150,
          "speed": 176
        },
        {
          "name": "ルガルガン",
          "star": 6,
          "types": [
            "Rock"
          ],
          "abilities": [
            {
              "name": "するどいめ",
              "description": "鋭い目のおかげで命中率を下げられない。"
            },
            {
              "name": "すなかき",
              "description": "天気がすなあらしのとき素早さが上がる。"
            },
            {
              "name": "ふくつのこころ",
              "description": "ひるむたびに不屈の心を燃やして素早さが上がる。"
            }
          ],
          "moves": [
            {
              "name": "アクセルロック",
              "type": "Rock",
              "category": "Physical",
              "power": 40,
              "description": "素早いスピードで相手にぶつかって攻撃する。必ず先制攻撃できる。"
            },
            {
              "name": "いわなだれ",
              "type": "Rock",
              "category": "Physical",
              "power": 75,
              "description": "大きな岩を激しくぶつけて攻撃する。相手をひるませることがある。"
            },
            {
              "name": "かみくだく",
              "type": "Dark",
              "category": "Physical",
              "power": 80,
              "description": "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。"
            },
            {
              "name": "ちょうはつ",
              "type": "Dark",
              "category": "Status",
              "power": 0,
              "description": "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。"
            }
          ],
          "actions": [
            "時間80% すなあらし",
            "体力75% 強化解除",
            "体力50% 弱体解除",
            "体力50% テラ回収",
            "体力20% いわなだれ"
          ],
          "hp": 5875,
          "atk": 212,
          "def": 150,
          "sp_atk": 104,
          "sp_def": 150,
          "speed": 206
        }
      ]"#).unwrap()
}