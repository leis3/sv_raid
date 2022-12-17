use crate::{
    Move,
    Type::*,
    Category::*
};

/// わざの一覧を返す
pub(crate) fn mv() -> Vec<Move> {
    vec![
        Move {
            name: "じばく".to_string(),
            r#type: Normal,
            category: Physical,
            power: 200,
            description: "爆発をおこして自分の周りにいるものを攻撃する。使ったあとにひんしになる。".to_string()
        },
        Move {
            name: "はかいこうせん".to_string(),
            r#type: Normal,
            category: Special,
            power: 150,
            description: "強い光線を相手に発射して攻撃する。次のターンは動けなくなる。".to_string()
        },
        Move {
            name: "ギガインパクト".to_string(),
            r#type: Normal,
            category: Physical,
            power: 150,
            description: "持てる力をすべて使って相手に突撃する。次のターンは動けなくなる。".to_string()
        },
        Move {
            name: "ばくおんぱ".to_string(),
            r#type: Normal,
            category: Special,
            power: 140,
            description: "すさまじい爆音の破壊力によって周りにいるものを攻撃する。".to_string()
        },
        Move {
            name: "とっておき".to_string(),
            r#type: Normal,
            category: Physical,
            power: 140,
            description: "戦闘中におぼえている技をすべて使うとはじめてだせるとっておきの技。".to_string()
        },
        Move {
            name: "すてみタックル".to_string(),
            r#type: Normal,
            category: Physical,
            power: 120,
            description: "命を懸けて相手に突進して攻撃する。自分もかなりダメージを受ける。".to_string()
        },
        Move {
            name: "あばれる".to_string(),
            r#type: Normal,
            category: Physical,
            power: 120,
            description: "２ー３ターンの間暴れまくって相手を攻撃する。暴れたあとは混乱する。".to_string()
        },
        Move {
            name: "メガトンキック".to_string(),
            r#type: Normal,
            category: Physical,
            power: 120,
            description: "ものすごい力をこめたキックで相手をけとばして攻撃する。".to_string()
        },
        Move {
            name: "さばきのつぶて".to_string(),
            r#type: Normal,
            category: Special,
            power: 100,
            description: "無数の光弾を相手に放出する。自分の持つプレートによりタイプが変わる。".to_string()
        },
        Move {
            name: "ハイパードリル".to_string(),
            r#type: Normal,
            category: Physical,
            power: 100,
            description: "とがった体の部位を急速に回転させつらぬく。まもるやみきりなども無視できる。".to_string()
        },
        Move {
            name: "とっしん".to_string(),
            r#type: Normal,
            category: Physical,
            power: 90,
            description: "すごい勢いで相手にぶつかって攻撃する。自分も少しダメージを受ける。".to_string()
        },
        Move {
            name: "めざめるダンス".to_string(),
            r#type: Normal,
            category: Special,
            power: 90,
            description: "全力で踊って攻撃する。この技のタイプは自分のタイプと同じになる。".to_string()
        },
        Move {
            name: "さわぐ".to_string(),
            r#type: Normal,
            category: Special,
            power: 90,
            description: "３ターンの間騒いで相手を攻撃する。そのあいだはだれも眠れなくなる。".to_string()
        },
        Move {
            name: "ハイパーボイス".to_string(),
            r#type: Normal,
            category: Special,
            power: 90,
            description: "うるさく響く大きな振動を相手に与えて攻撃する。".to_string()
        },
        Move {
            name: "レイジングブル".to_string(),
            r#type: Normal,
            category: Physical,
            power: 90,
            description: "怒り狂うあばれうしの猛烈なタックル。フォルムで技のタイプが変わりひかりのかべやリフレクターなども破壊できる。".to_string()
        },
        Move {
            name: "のしかかり".to_string(),
            r#type: Normal,
            category: Physical,
            power: 85,
            description: "全身で相手にのしかかり攻撃する。まひ状態にすることがある。".to_string()
        },
        Move {
            name: "メガトンパンチ".to_string(),
            r#type: Normal,
            category: Physical,
            power: 80,
            description: "力をこめたパンチで相手を攻撃する。".to_string()
        },
        Move {
            name: "たたきつける".to_string(),
            r#type: Normal,
            category: Physical,
            power: 80,
            description: "長いしっぽやつるなどを使い相手をたたきつけて攻撃する。".to_string()
        },
        Move {
            name: "かいりき".to_string(),
            r#type: Normal,
            category: Physical,
            power: 80,
            description: "こん身の力で相手をなぐりつけて攻撃する。".to_string()
        },
        Move {
            name: "トライアタック".to_string(),
            r#type: Normal,
            category: Special,
            power: 80,
            description: "３つの光線で攻撃する。まひかやけどかこおり状態のどれかにすることがある。".to_string()
        },
        Move {
            name: "テラバースト".to_string(),
            r#type: Normal,
            category: Special,
            power: 80,
            description: "テラスタルだとテラスタイプのエネルギーを放出して攻撃する。攻撃と特攻を比べて高いほうでダメージを与える。".to_string()
        },
        Move {
            name: "しんそく".to_string(),
            r#type: Normal,
            category: Physical,
            power: 80,
            description: "目にも留まらぬものすごい速さで相手に突進して攻撃する。必ず先制攻撃できる。".to_string()
        },
        Move {
            name: "ブレイククロー".to_string(),
            r#type: Normal,
            category: Physical,
            power: 75,
            description: "硬く鋭いツメで切り裂いて攻撃する。相手の防御をさげることがある。".to_string()
        },
        Move {
            name: "いにしえのうた".to_string(),
            r#type: Normal,
            category: Special,
            power: 75,
            description: "いにしえのうたを相手に聞かせて心にうったえて攻撃する。眠り状態にすることがある。".to_string()
        },
        Move {
            name: "きりさく".to_string(),
            r#type: Normal,
            category: Physical,
            power: 70,
            description: "ツメやカマなどで相手を切り裂いて攻撃する。急所に当たりやすい。".to_string()
        },
        Move {
            name: "からげんき".to_string(),
            r#type: Normal,
            category: Physical,
            power: 70,
            description: "自分が毒まひやけど状態のとき相手にくりだすと技の威力が２倍になる。".to_string()
        },
        Move {
            name: "ずつき".to_string(),
            r#type: Normal,
            category: Physical,
            power: 70,
            description: "頭を突きだしてまっすぐつっこんで攻撃する。相手をひるませることがある。".to_string()
        },
        Move {
            name: "かたきうち".to_string(),
            r#type: Normal,
            category: Physical,
            power: 70,
            description: "倒れた味方のかたきを討つ。前のターンに味方が倒されていると威力があがる。".to_string()
        },
        Move {
            name: "つのでつく".to_string(),
            r#type: Normal,
            category: Physical,
            power: 65,
            description: "鋭くとがったつので相手を攻撃する。".to_string()
        },
        Move {
            name: "ふみつけ".to_string(),
            r#type: Normal,
            category: Physical,
            power: 65,
            description: "大きな足で相手を踏みつけて攻撃する。相手をひるませることがある。".to_string()
        },
        Move {
            name: "ほしがる".to_string(),
            r#type: Normal,
            category: Physical,
            power: 60,
            description: "かわいくあまえながら相手にちかづき持っている道具をうばう。".to_string()
        },
        Move {
            name: "スピードスター".to_string(),
            r#type: Normal,
            category: Special,
            power: 60,
            description: "星型の光を発射して相手を攻撃する。攻撃は必ず命中する。".to_string()
        },
        Move {
            name: "りんしょう".to_string(),
            r#type: Normal,
            category: Special,
            power: 60,
            description: "歌で相手を攻撃する。みんなで輪唱すると続けてだすことができ威力もあがる。".to_string()
        },
        Move {
            name: "はさむ".to_string(),
            r#type: Normal,
            category: Physical,
            power: 55,
            description: "相手を両側からはさんでダメージをあたえる。".to_string()
        },
        Move {
            name: "こうそくスピン".to_string(),
            r#type: Normal,
            category: Physical,
            power: 50,
            description: "回転して相手を攻撃する。しめつけるまきつくやどりぎのタネなど吹きとばす。自分の素早さもあがる。".to_string()
        },
        Move {
            name: "いあいぎり".to_string(),
            r#type: Normal,
            category: Physical,
            power: 50,
            description: "カマやツメなどで相手を切りつけて攻撃する。".to_string()
        },
        Move {
            name: "いびき".to_string(),
            r#type: Normal,
            category: Special,
            power: 50,
            description: "自分が寝ているときに雑音をだして攻撃する。相手をひるませることがある。".to_string()
        },
        Move {
            name: "ウェザーボール".to_string(),
            r#type: Normal,
            category: Special,
            power: 50,
            description: "使ったときの天気によって技のタイプと威力が変わる。".to_string()
        },
        Move {
            name: "だいちのはどう".to_string(),
            r#type: Normal,
            category: Special,
            power: 50,
            description: "フィールドの力を借りて攻撃。使った時のフィールドの状態によって技のタイプと威力が変わる。".to_string()
        },
        Move {
            name: "わるあがき".to_string(),
            r#type: Normal,
            category: Physical,
            power: 50,
            description: "自分のＰＰがなくなるとあがいて相手を攻撃する。自分も少しダメージを受ける。".to_string()
        },
        Move {
            name: "みねうち".to_string(),
            r#type: Normal,
            category: Physical,
            power: 40,
            description: "相手のＨＰが必ず１だけ残るように手加減して攻撃する。".to_string()
        },
        Move {
            name: "てかげん".to_string(),
            r#type: Normal,
            category: Physical,
            power: 40,
            description: "手加減した攻撃で相手のＨＰを必ず１だけ残す。".to_string()
        },
        Move {
            name: "はたく".to_string(),
            r#type: Normal,
            category: Physical,
            power: 40,
            description: "長いしっぽや手などを使って相手をはたいて攻撃する。".to_string()
        },
        Move {
            name: "ひっかく".to_string(),
            r#type: Normal,
            category: Physical,
            power: 40,
            description: "硬くとがった鋭いツメで相手をひっかいて攻撃する。".to_string()
        },
        Move {
            name: "たいあたり".to_string(),
            r#type: Normal,
            category: Physical,
            power: 40,
            description: "相手にむかって全身でぶつかっていき攻撃する。".to_string()
        },
        Move {
            name: "はたく".to_string(),
            r#type: Normal,
            category: Physical,
            power: 40,
            description: "長いしっぽや手などを使って相手をはたいて攻撃する。".to_string()
        },
        Move {
            name: "はたく".to_string(),
            r#type: Normal,
            category: Physical,
            power: 40,
            description: "長いしっぽや手などを使って相手をはたいて攻撃する。".to_string()
        },
        Move {
            name: "でんこうせっか".to_string(),
            r#type: Normal,
            category: Physical,
            power: 40,
            description: "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。".to_string()
        },
        Move {
            name: "ネコにこばん".to_string(),
            r#type: Normal,
            category: Physical,
            power: 40,
            description: "相手の体に小判を投げつけて攻撃する。戦闘のあとでお金がもらえる。".to_string()
        },
        Move {
            name: "エコーボイス".to_string(),
            r#type: Normal,
            category: Special,
            power: 40,
            description: "響く声で相手を攻撃する。毎ターンだれかが技を使い続けると威力があがる。".to_string()
        },
        Move {
            name: "ねこだまし".to_string(),
            r#type: Normal,
            category: Physical,
            power: 40,
            description: "先制攻撃で相手をひるませる。戦闘にでたらすぐにださないと成功しない。".to_string()
        },
        Move {
            name: "ダブルアタック".to_string(),
            r#type: Normal,
            category: Physical,
            power: 35,
            description: "しっぽなどを使い相手をたたいて攻撃する。２回連続でダメージを与える。".to_string()
        },
        Move {
            name: "フェイント".to_string(),
            r#type: Normal,
            category: Physical,
            power: 30,
            description: "まもるやみきりなどをしている相手に攻撃ができる。守りの効果を解除させる。".to_string()
        },
        Move {
            name: "スイープビンタ".to_string(),
            r#type: Normal,
            category: Physical,
            power: 25,
            description: "硬いしっぽで相手をたたいて攻撃する。２ー５回の間連続でだす。".to_string()
        },
        Move {
            name: "ネズミざん".to_string(),
            r#type: Normal,
            category: Physical,
            power: 20,
            description: "仲間たちがわらわらと集まってコンビネーションで攻撃を与えていく。１ー１０回の間連続であたる。".to_string()
        },
        Move {
            name: "みだれひっかき".to_string(),
            r#type: Normal,
            category: Physical,
            power: 18,
            description: "ツメやカマなどで相手をひっかいて攻撃する。２ー５回の間連続でだす。".to_string()
        },
        Move {
            name: "しめつける".to_string(),
            r#type: Normal,
            category: Physical,
            power: 15,
            description: "長い体やつるなどを使い４ー５ターンの間相手を締めつけて攻撃する。".to_string()
        },
        Move {
            name: "みだれづき".to_string(),
            r#type: Normal,
            category: Physical,
            power: 15,
            description: "つのやくちばしで相手をつついて攻撃する。２ー５回の間連続でだす。".to_string()
        },
        Move {
            name: "まきつく".to_string(),
            r#type: Normal,
            category: Physical,
            power: 15,
            description: "長い体やつるなどを使って４ー５ターンの間相手にまきついて攻撃する。".to_string()
        },
        Move {
            name: "じたばた".to_string(),
            r#type: Normal,
            category: Physical,
            power: 1,
            description: "じたばた暴れて攻撃する。自分のＨＰが少ないほど技の威力はあがる。".to_string()
        },
        Move {
            name: "プレゼント".to_string(),
            r#type: Normal,
            category: Physical,
            power: 1,
            description: "わなをしかけた箱を相手にわたして攻撃する。ＨＰが回復してしまうこともある。".to_string()
        },
        Move {
            name: "いかりのまえば".to_string(),
            r#type: Normal,
            category: Physical,
            power: 1,
            description: "鋭い前歯で激しくかみついて攻撃する。相手のＨＰは半分になる。".to_string()
        },
        Move {
            name: "はきだす".to_string(),
            r#type: Normal,
            category: Special,
            power: 1,
            description: "蓄えた力を相手にぶつけて攻撃する。蓄えているほど威力があがる。".to_string()
        },
        Move {
            name: "ハサミギロチン".to_string(),
            r#type: Normal,
            category: Physical,
            power: 1,
            description: "大きなハサミで相手を切り裂いて攻撃する。当たれば一撃でひんしにする。".to_string()
        },
        Move {
            name: "つのドリル".to_string(),
            r#type: Normal,
            category: Physical,
            power: 1,
            description: "回転するつのを相手に突き刺して攻撃する。当たれば一撃でひんしにする。".to_string()
        },
        Move {
            name: "がむしゃら".to_string(),
            r#type: Normal,
            category: Physical,
            power: 1,
            description: "相手のＨＰが自分のＨＰと同じくらいになるようにダメージを与える。".to_string()
        },
        Move {
            name: "なきごえ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "かわいいなきごえを聞かせて気をひき油断をさせて相手の攻撃をさげる。".to_string()
        },
        Move {
            name: "いやなおと".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "おもわず耳をふさぎたくなるいやなおとをだして相手の防御をがくっとさげる。".to_string()
        },
        Move {
            name: "まるくなる".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "体をまるめてちぢこまり自分の防御をあげる。".to_string()
        },
        Move {
            name: "はねる".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "攻撃もせずにピョンピョンと跳ねるだけでなにもおこらない……。".to_string()
        },
        Move {
            name: "バトンタッチ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "控えのポケモンと入れ替わる。能力変化は替わったポケモンがそのまま受けつぐ。".to_string()
        },
        Move {
            name: "とおぼえ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "大声でほえて気合を高め自分と味方の攻撃をあげる。".to_string()
        },
        Move {
            name: "おいわい".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "ポケモンがとってもハッピーなあなたのことをお祝いしてくれる。".to_string()
        },
        Move {
            name: "てをつなぐ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "味方のポケモン同士が手をつなぐ。とっても幸せな気持ちになれる。".to_string()
        },
        Move {
            name: "しっぽをふる".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "しっぽを左右にかわいくふって油断を誘う。相手の防御をさげる。".to_string()
        },
        Move {
            name: "にらみつける".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "鋭い目つきでおびえさせて相手の防御をさげる。".to_string()
        },
        Move {
            name: "かたくなる".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "全身に力をこめて体を硬くして自分の防御をあげる。".to_string()
        },
        Move {
            name: "きあいだめ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "深く息を吸い気合をこめる。自分の攻撃が急所に当たりやすくなる。".to_string()
        },
        Move {
            name: "へびにらみ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "おなかの模様でおびえさせて相手をまひの状態にする。".to_string()
        },
        Move {
            name: "つぼをつく".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "つぼおしで体を活性化させる。能力のどれか１つをぐーんとあげる。".to_string()
        },
        Move {
            name: "ふるいたてる".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "自分を奮いたてて攻撃と特攻をあげる。".to_string()
        },
        Move {
            name: "おたけび".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "おたけびをあげて相手を威嚇し相手の攻撃と特攻をさげる。".to_string()
        },
        Move {
            name: "ハッピータイム".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "ハッピータイムの技を使うと戦闘のあとでもらえるお金が倍になる。".to_string()
        },
        Move {
            name: "しんぴのまもり".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "５ターンの間不思議な力に守られて状態異常にならなくなる。".to_string()
        },
        Move {
            name: "つるぎのまい".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "戦いの舞を激しくおどって気合を高める。自分の攻撃をぐーんとあげる。".to_string()
        },
        Move {
            name: "ふきとばし".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "相手を吹きとばして控えのポケモンをひきずりだす。野生の場合は戦闘が終わる。".to_string()
        },
        Move {
            name: "ほえる".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "相手を逃がして控えのポケモンをひきずりだす。野生の場合は戦闘が終わる。".to_string()
        },
        Move {
            name: "ちょうおんぱ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "特殊な音波を体から発して相手を混乱させる。".to_string()
        },
        Move {
            name: "かなしばり".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "相手の動きをとめて直前にだしていた技を４ターンの間使えなくする。".to_string()
        },
        Move {
            name: "せいちょう".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "体を一気に大きく生長させて攻撃と特攻をあげる。".to_string()
        },
        Move {
            name: "えんまく".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "煙や墨などを吹きかけて相手の命中率をさげる。".to_string()
        },
        Move {
            name: "いたみわけ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "自分のＨＰと相手のＨＰをあわせてそれを自分と相手でなかよくわける。".to_string()
        },
        Move {
            name: "あまいかおり".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "香りで相手の回避率をがくっとさげる。".to_string()
        },
        Move {
            name: "たくわえる".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "力を蓄えて自分の防御と特防をあげる。最大３回まで蓄えられる。".to_string()
        },
        Move {
            name: "このゆびとまれ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "自分に注目させて相手からの攻撃をすべて自分にむけさせる。".to_string()
        },
        Move {
            name: "てだすけ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "仲間を助ける。てだすけされたポケモンの技の威力はいつもより大きくなる。".to_string()
        },
        Move {
            name: "フラフラダンス".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "フラフラとダンスをおどって自分の周りにいるものを混乱状態にさせる。".to_string()
        },
        Move {
            name: "くすぐる".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "体をくすぐり笑わせることで相手の攻撃と防御をさげる。".to_string()
        },
        Move {
            name: "まねっこ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "直前にでた技をまねして同じ技をだす。技がでていないと失敗する。".to_string()
        },
        Move {
            name: "なかよくする".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "相手となかよくなって戦う気力を失わせ相手の攻撃をさげる。".to_string()
        },
        Move {
            name: "ないしょばなし".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "ないしょばなしをすることで相手の集中力を失わせ相手の特攻をさげる。".to_string()
        },
        Move {
            name: "なみだめ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "なみだめになって相手の戦力を喪失させる。相手の攻撃と特攻がさがる。".to_string()
        },
        Move {
            name: "うたう".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "心地好いきれいな歌声を聞かせて相手を眠り状態にする。".to_string()
        },
        Move {
            name: "かげぶんしん".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "素早い動きで分身をつくり相手をまどわせて回避率をあげる。".to_string()
        },
        Move {
            name: "いばる".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "相手を怒らせて混乱させる。怒りで相手の攻撃はぐーんとあがってしまう。".to_string()
        },
        Move {
            name: "メロメロ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "♂なら♀を♀なら♂を誘惑してメロメロにする。相手は技がだしにくくなる。".to_string()
        },
        Move {
            name: "シンプルビーム".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "なぞの念波を相手に送る。念波を受けとった相手は特性がたんじゅんになる。".to_string()
        },
        Move {
            name: "なかまづくり".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "不思議なリズムでおどる。動きをまねさせて自分と相手の特性を同じにする。".to_string()
        },
        Move {
            name: "おさきにどうぞ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "相手の行動をサポートして自分の行動のあとに続けて動けるようにする。".to_string()
        },
        Move {
            name: "からをやぶる".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "殻をやぶって自分の防御特防をさげるが攻撃特攻素早さをぐーんとあげる。".to_string()
        },
        Move {
            name: "ミラータイプ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "相手のタイプを反射して自分も同じタイプになる。".to_string()
        },
        Move {
            name: "ものまね".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "相手が最後に使った技を戦闘のあいだ自分の技にすることができる。".to_string()
        },
        Move {
            name: "ちいさくなる".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "体をちぢめて小さくみせて自分の回避率をぐーんとあげる。".to_string()
        },
        Move {
            name: "ゆびをふる".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "指をふり自分の脳を刺激してたくさんの技のなかからどれか１つをくりだす。".to_string()
        },
        Move {
            name: "へんしん".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "相手のポケモンに変身することで相手とまったく同じ技が使える。".to_string()
        },
        Move {
            name: "みがわり".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "自分のＨＰを少し削って分身をだす。分身は自分の身代わりになる。".to_string()
        },
        Move {
            name: "まもる".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "相手の攻撃をまったく受けない。連続でだすと失敗しやすい。".to_string()
        },
        Move {
            name: "こわいかお".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "恐ろしい顔でにらみおびえさせて相手の素早さをがくっとさげる。".to_string()
        },
        Move {
            name: "はらだいこ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "自分のＨＰを最大ＨＰの半分減らして自分の攻撃を最大にあげる。".to_string()
        },
        Move {
            name: "こらえる".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "攻撃を受けてもＨＰを必ず１だけ残せる。連続でだすと失敗しやすい。".to_string()
        },
        Move {
            name: "ねごと".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "自分がおぼえている技のうちどれか１つをくりだす。自分が寝ているときだけ使える。".to_string()
        },
        Move {
            name: "じこあんじ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "自分に暗示をかけることで能力変化の状態を相手と同じにする。".to_string()
        },
        Move {
            name: "のみこむ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "蓄えた力をのみこんで自分のＨＰを回復する。蓄えているほど回復する。".to_string()
        },
        Move {
            name: "ねがいごと".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "次のターンに自分もしくは入れ替わったポケモンのＨＰを最大ＨＰの半分回復する。".to_string()
        },
        Move {
            name: "リサイクル".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "戦闘中に使ってなくなった自分の持ち物を再生させて使えるようにする。".to_string()
        },
        Move {
            name: "あくび".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "大きなあくびで眠気を誘う。次のターンに相手を眠り状態にする。".to_string()
        },
        Move {
            name: "ほおばる".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "持っているきのみを食べて防御をぐーんとあげる。".to_string()
        },
        Move {
            name: "おちゃかい".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "おちゃかいをひらいて場にいるポケモンがそれぞれ持っているきのみを食べる。".to_string()
        },
        Move {
            name: "コートチェンジ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "不思議な力でお互いの場の効果を入れ替える。".to_string()
        },
        Move {
            name: "パワーシフト".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "自分の攻撃と防御を入れ替える。".to_string()
        },
        Move {
            name: "うつしえ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "相手の本質をとらえてうつしだし自分と味方を相手と同じ特性に変化させる。".to_string()
        },
        Move {
            name: "みをけずる".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "自分のＨＰをけずって自分の攻撃と特攻と素早さをぐーんとあげる。".to_string()
        },
        Move {
            name: "しっぽきり".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "自分のＨＰを削って分身をだしたあともどってきて控えのポケモンと入れ替わる。".to_string()
        },
        Move {
            name: "おかたづけ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "まきびしステルスロックねばねばネットどくびしみがわりをすべてかたづける。自分の攻撃と素早さがあがる。".to_string()
        },
        Move {
            name: "じこさいせい".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "細胞を再生させて自分の最大ＨＰの半分のＨＰを回復する。".to_string()
        },
        Move {
            name: "タマゴうみ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "最大ＨＰの半分自分のＨＰを回復する。".to_string()
        },
        Move {
            name: "ほろびのうた".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "歌を聴いたポケモンは３ターンたつとひんしになる。交代すると効果はなくなる。".to_string()
        },
        Move {
            name: "ロックオン".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "照準をしっかりあわせて次の攻撃が必ず相手に当たるようにする。".to_string()
        },
        Move {
            name: "ミルクのみ".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "最大ＨＰの半分自分のＨＰを回復する。".to_string()
        },
        Move {
            name: "くろいまなざし".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "吸いこまれるような黒いまなざしでじっとみつめて相手を戦闘から逃げられなくする。".to_string()
        },
        Move {
            name: "いやしのすず".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "心地好い鈴の音色を聞かせて味方全員の状態異常を回復する。".to_string()
        },
        Move {
            name: "アンコール".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "相手にアンコールした技を３回続けて出させる。".to_string()
        },
        Move {
            name: "あさのひざし".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "自分のＨＰを回復する。天気によって回復の量が変化する。".to_string()
        },
        Move {
            name: "なまける".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "怠けてやすむ。自分のＨＰを最大ＨＰの半分回復する。".to_string()
        },
        Move {
            name: "とおせんぼう".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "両手をひろげてたちはだかり相手の逃げ道をふさいで逃げられなくする。".to_string()
        },
        Move {
            name: "さいきのいのり".to_string(),
            r#type: Normal,
            category: Status,
            power: 0,
            description: "慈愛の心でいのることにより控えにいるひんしのポケモンをＨＰを半分の状態で復活させる。".to_string()
        },
        Move {
            name: "きあいパンチ".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 150,
            description: "精神を高めてパンチをくりだす。技をだすまでに攻撃を受けると失敗する。".to_string()
        },
        Move {
            name: "とびひざげり".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 130,
            description: "ジャンプからのひざげりで相手を攻撃する。はずすと自分がダメージを受ける。".to_string()
        },
        Move {
            name: "かかとおとし".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 120,
            description: "蹴りあげたかかとを落として攻撃する。相手を混乱させることがある。はずすと自分がダメージを受ける。".to_string()
        },
        Move {
            name: "ばかぢから".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 120,
            description: "すごい力を発揮して相手を攻撃する。自分の攻撃と防御がさがる。".to_string()
        },
        Move {
            name: "インファイト".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 120,
            description: "守りを捨てて相手のふところに突撃する。自分の防御と特防がさがる。".to_string()
        },
        Move {
            name: "きあいだま".to_string(),
            r#type: Fighting,
            category: Special,
            power: 120,
            description: "気合を高めてありったけの力を放出する。相手の特防をさげることがある。".to_string()
        },
        Move {
            name: "アームハンマー".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 100,
            description: "強くて重いこぶしをふるってダメージを与える。自分の素早さがさがる。".to_string()
        },
        Move {
            name: "フライングプレス".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 100,
            description: "空中から相手にダイブする。この技はかくとうタイプと同時にひこうタイプでもある。".to_string()
        },
        Move {
            name: "ばくれつパンチ".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 100,
            description: "こん身の力でパンチをくりだして攻撃する。相手を必ず混乱させる。".to_string()
        },
        Move {
            name: "クロスチョップ".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 100,
            description: "両手チョップを相手にたたきつけて攻撃する。急所に当たりやすい。".to_string()
        },
        Move {
            name: "アクセルブレイク".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 100,
            description: "変形しながら荒々しく落下しいにしえの大爆発を引き起こす。弱点をつくとさらに威力が増す。".to_string()
        },
        Move {
            name: "せいなるつるぎ".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 90,
            description: "つるぎで切りつけ攻撃する。相手の能力変化に関係なくダメージを与える。".to_string()
        },
        Move {
            name: "らいめいげり".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 90,
            description: "雷のような動きで相手を翻弄しながらキックする。相手の防御をさげる。".to_string()
        },
        Move {
            name: "３ぼんのや".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 90,
            description: "足技のあと３本の矢を同時に放つ。相手の防御をさげたりひるませることがある。急所に当たりやすい。".to_string()
        },
        Move {
            name: "はどうだん".to_string(),
            r#type: Fighting,
            category: Special,
            power: 80,
            description: "体の奥から波導の力を相手にうち放つ。攻撃は必ず命中する。".to_string()
        },
        Move {
            name: "ボディプレス".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 80,
            description: "体をぶつけて攻撃。防御が高いほど与えるダメージが増える。".to_string()
        },
        Move {
            name: "かわらわり".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 75,
            description: "手刀を勢いよく振りおろして相手を攻撃する。ひかりのかべやリフレクターなども破壊できる。".to_string()
        },
        Move {
            name: "ドレインパンチ".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 75,
            description: "こぶしから相手の力を吸い取る。与えたダメージの半分のＨＰを回復できる。".to_string()
        },
        Move {
            name: "ローキック".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 65,
            description: "素早い動きで相手の足をねらって攻撃する。相手の素早さをさげる。".to_string()
        },
        Move {
            name: "はっけい".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 60,
            description: "相手の体に衝撃波を当てて攻撃する。まひ状態にすることがある。".to_string()
        },
        Move {
            name: "ともえなげ".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 60,
            description: "相手を投げとばして控えのポケモンをひきずりだす。野生の場合は戦闘が終わる。".to_string()
        },
        Move {
            name: "マッハパンチ".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 40,
            description: "目にも留まらぬものすごい速さでパンチをくりだす。必ず先制攻撃できる。".to_string()
        },
        Move {
            name: "しんくうは".to_string(),
            r#type: Fighting,
            category: Special,
            power: 40,
            description: "こぶしをふって真空の波をまきおこす。必ず先制攻撃できる。".to_string()
        },
        Move {
            name: "いわくだき".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 40,
            description: "パンチで攻撃する。相手の防御をさげることがある。".to_string()
        },
        Move {
            name: "にどげり".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 30,
            description: "２本の足で相手をけとばして攻撃する。２回連続でダメージを与える。".to_string()
        },
        Move {
            name: "つっぱり".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 15,
            description: "ひらいた両手で相手をつっぱって攻撃する。２ー５回の間連続でだす。".to_string()
        },
        Move {
            name: "けたぐり".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 1,
            description: "足を強くけり相手を転ばせて攻撃する。相手が重いほど威力があがる。".to_string()
        },
        Move {
            name: "カウンター".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 1,
            description: "相手から受けた物理攻撃のダメージを２倍にして同じ相手に返す。".to_string()
        },
        Move {
            name: "ちきゅうなげ".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 1,
            description: "引力を使い投げとばす。自分のレベルと同じダメージを相手に与える。".to_string()
        },
        Move {
            name: "きしかいせい".to_string(),
            r#type: Fighting,
            category: Physical,
            power: 1,
            description: "力をふりしぼり攻撃する。自分のＨＰが少ないほど技の威力はあがる。".to_string()
        },
        Move {
            name: "いのちがけ".to_string(),
            r#type: Fighting,
            category: Special,
            power: 1,
            description: "命懸けで相手を攻撃する。自分はひんしになるが相手にＨＰ分のダメージを与える。".to_string()
        },
        Move {
            name: "ビルドアップ".to_string(),
            r#type: Fighting,
            category: Status,
            power: 0,
            description: "体に力をこめて筋肉をぶあつくすることで自分の攻撃と防御をあげる。".to_string()
        },
        Move {
            name: "ファストガード".to_string(),
            r#type: Fighting,
            category: Status,
            power: 0,
            description: "自分と味方を相手の先制攻撃から守る。".to_string()
        },
        Move {
            name: "コーチング".to_string(),
            r#type: Fighting,
            category: Status,
            power: 0,
            description: "的確な指導をおこなうことで味方全員の攻撃と防御を上げる。".to_string()
        },
        Move {
            name: "しょうりのまい".to_string(),
            r#type: Fighting,
            category: Status,
            power: 0,
            description: "勝利を呼びこむ舞を激しく踊って自分の攻撃と防御と素早さをあげる。".to_string()
        },
        Move {
            name: "みきり".to_string(),
            r#type: Fighting,
            category: Status,
            power: 0,
            description: "相手の攻撃をまったく受けない。連続でだすと失敗しやすい。".to_string()
        },
        Move {
            name: "はいすいのじん".to_string(),
            r#type: Fighting,
            category: Status,
            power: 0,
            description: "自分のすべての能力が上がるが交代したり逃げることができなくなる。".to_string()
        },
        Move {
            name: "ゴッドバード".to_string(),
            r#type: Flying,
            category: Physical,
            power: 140,
            description: "２ターン目に相手を攻撃する。たまにひるませる。急所にも当たりやすい。".to_string()
        },
        Move {
            name: "ブレイブバード".to_string(),
            r#type: Flying,
            category: Physical,
            power: 120,
            description: "はねをおりたたみ低空飛行で突撃する。自分もかなりダメージを受ける。".to_string()
        },
        Move {
            name: "ガリョウテンセイ".to_string(),
            r#type: Flying,
            category: Physical,
            power: 120,
            description: "大空から急速落下して相手を攻撃する。自分の防御と特防がさがる。".to_string()
        },
        Move {
            name: "ぼうふう".to_string(),
            r#type: Flying,
            category: Special,
            power: 110,
            description: "強烈な風で相手を包みこんで攻撃する。相手を混乱させることがある。".to_string()
        },
        Move {
            name: "こがらしあらし".to_string(),
            r#type: Flying,
            category: Special,
            power: 100,
            description: "身も心も震える冷たく激しい風で攻撃する。相手の素早さをさげることがある。".to_string()
        },
        Move {
            name: "そらをとぶ".to_string(),
            r#type: Flying,
            category: Physical,
            power: 90,
            description: "１ターン目で空へ飛び２ターン目に相手を攻撃する。".to_string()
        },
        Move {
            name: "とびはねる".to_string(),
            r#type: Flying,
            category: Physical,
            power: 85,
            description: "空高く飛び跳ねて２ターン目に相手を攻撃する。まひ状態にすることがある。".to_string()
        },
        Move {
            name: "ドリルくちばし".to_string(),
            r#type: Flying,
            category: Physical,
            power: 80,
            description: "回転しながらとがったくちばしを相手に突き刺して攻撃する。".to_string()
        },
        Move {
            name: "エアスラッシュ".to_string(),
            r#type: Flying,
            category: Special,
            power: 75,
            description: "空をも切り裂く空気の刃で攻撃する。相手をひるませることがある。".to_string()
        },
        Move {
            name: "つばさでうつ".to_string(),
            r#type: Flying,
            category: Physical,
            power: 60,
            description: "大きくひろげたりっぱな翼を相手にぶつけて攻撃する。".to_string()
        },
        Move {
            name: "エアカッター".to_string(),
            r#type: Flying,
            category: Special,
            power: 60,
            description: "鋭い風で相手を切りつけて攻撃する。急所に当たりやすい。".to_string()
        },
        Move {
            name: "つばめがえし".to_string(),
            r#type: Flying,
            category: Physical,
            power: 60,
            description: "素早い動きで相手をほんろうして切りつける。攻撃は必ず命中する。".to_string()
        },
        Move {
            name: "ついばむ".to_string(),
            r#type: Flying,
            category: Physical,
            power: 60,
            description: "くちばしで攻撃。相手がきのみを持っているとき食べてきのみの効果を受けられる。".to_string()
        },
        Move {
            name: "アクロバット".to_string(),
            r#type: Flying,
            category: Physical,
            power: 55,
            description: "軽やかに相手を攻撃する。自分が道具を持っていないとき大きなダメージを与える。".to_string()
        },
        Move {
            name: "かぜおこし".to_string(),
            r#type: Flying,
            category: Special,
            power: 40,
            description: "翼でおこした激しい風を相手にぶつけて攻撃する。".to_string()
        },
        Move {
            name: "ダブルウイング".to_string(),
            r#type: Flying,
            category: Physical,
            power: 40,
            description: "翼を相手にぶつけて攻撃する。２回連続でダメージを与える。".to_string()
        },
        Move {
            name: "つつく".to_string(),
            r#type: Flying,
            category: Physical,
            power: 35,
            description: "鋭くとがったくちばしやつので相手を突いて攻撃する。".to_string()
        },
        Move {
            name: "フェザーダンス".to_string(),
            r#type: Flying,
            category: Status,
            power: 0,
            description: "羽毛をふりまいて相手の体にからませる。相手の攻撃をがくっとさげる。".to_string()
        },
        Move {
            name: "おいかぜ".to_string(),
            r#type: Flying,
            category: Status,
            power: 0,
            description: "激しく吹きあれる風の渦をつくり４ターンの間味方全員の素早さをあげる。".to_string()
        },
        Move {
            name: "きりばらい".to_string(),
            r#type: Flying,
            category: Status,
            power: 0,
            description: "強い風で相手のリフレクターやひかりのかべなどをはらいのける。回避率もさげる。".to_string()
        },
        Move {
            name: "はねやすめ".to_string(),
            r#type: Flying,
            category: Status,
            power: 0,
            description: "地面に降りて体をやすめる。最大ＨＰの半分のＨＰを回復する。".to_string()
        },
        Move {
            name: "ゲップ".to_string(),
            r#type: Poison,
            category: Special,
            power: 120,
            description: "相手に向かってゲップを浴びせてダメージを与える。きのみを食べないとだせない。".to_string()
        },
        Move {
            name: "ダストシュート".to_string(),
            r#type: Poison,
            category: Physical,
            power: 120,
            description: "汚いゴミを相手にぶつけて攻撃する。毒状態にすることがある。".to_string()
        },
        Move {
            name: "ヘドロウェーブ".to_string(),
            r#type: Poison,
            category: Special,
            power: 95,
            description: "ヘドロの波で自分の周りにいるものを攻撃する。毒状態にすることがある。".to_string()
        },
        Move {
            name: "ヘドロばくだん".to_string(),
            r#type: Poison,
            category: Special,
            power: 90,
            description: "汚いヘドロを相手に投げつけて攻撃する。毒状態にすることがある。".to_string()
        },
        Move {
            name: "シェルアームズ".to_string(),
            r#type: Poison,
            category: Special,
            power: 90,
            description: "物理か特殊かより多くダメージを与えられる能力で攻撃する。毒状態にすることがある。".to_string()
        },
        Move {
            name: "どくづき".to_string(),
            r#type: Poison,
            category: Physical,
            power: 80,
            description: "毒にそまった触手や腕で相手を突き刺す。毒状態にすることがある。".to_string()
        },
        Move {
            name: "フェイタルクロー".to_string(),
            r#type: Poison,
            category: Physical,
            power: 80,
            description: "破滅的なツメで急所を狙って攻撃。相手をどくまひねむりのいずれかの状態にすることもある。".to_string()
        },
        Move {
            name: "クロスポイズン".to_string(),
            r#type: Poison,
            category: Physical,
            power: 70,
            description: "毒の刃で相手を切り裂く。毒状態にすることがあり急所にも当たりやすい。".to_string()
        },
        Move {
            name: "ヘドロこうげき".to_string(),
            r#type: Poison,
            category: Special,
            power: 65,
            description: "汚いヘドロを相手に投げつけて攻撃する。毒状態にすることがある。".to_string()
        },
        Move {
            name: "ベノムショック".to_string(),
            r#type: Poison,
            category: Special,
            power: 65,
            description: "特殊な毒液を浴びせかける。毒状態の相手には威力が２倍になる。".to_string()
        },
        Move {
            name: "どくばりセンボン".to_string(),
            r#type: Poison,
            category: Physical,
            power: 60,
            description: "無数の毒針で相手を毒状態にすることもある。相手が毒状態だと威力は２倍になる。".to_string()
        },
        Move {
            name: "ポイズンテール".to_string(),
            r#type: Poison,
            category: Physical,
            power: 50,
            description: "しっぽでたたく。毒状態にすることがあり急所にも当たりやすい。".to_string()
        },
        Move {
            name: "どくどくのキバ".to_string(),
            r#type: Poison,
            category: Physical,
            power: 50,
            description: "毒のあるキバで相手にかみついて攻撃する。猛毒をおわせることがある。".to_string()
        },
        Move {
            name: "クリアスモッグ".to_string(),
            r#type: Poison,
            category: Special,
            power: 50,
            description: "特殊な泥の塊を相手に投げつけて攻撃する。能力変化をもとにもどす。".to_string()
        },
        Move {
            name: "ようかいえき".to_string(),
            r#type: Poison,
            category: Special,
            power: 40,
            description: "強い酸を相手にかけて攻撃する。相手の特防をさげることがある。".to_string()
        },
        Move {
            name: "アシッドボム".to_string(),
            r#type: Poison,
            category: Special,
            power: 40,
            description: "相手をとかす液体を吐きだして攻撃する。相手の特防をがくっとさげる。".to_string()
        },
        Move {
            name: "スモッグ".to_string(),
            r#type: Poison,
            category: Special,
            power: 30,
            description: "汚れたガスを相手に吹きつけて攻撃する。毒状態にすることがある。".to_string()
        },
        Move {
            name: "キラースピン".to_string(),
            r#type: Poison,
            category: Physical,
            power: 30,
            description: "回転して相手を攻撃する。しめつけるまきつくやどりぎのタネなど吹きとばす。相手を毒状態にする。".to_string()
        },
        Move {
            name: "どくばり".to_string(),
            r#type: Poison,
            category: Physical,
            power: 15,
            description: "毒のあるハリを相手に突き刺して攻撃する。毒状態にすることがある。".to_string()
        },
        Move {
            name: "どくガス".to_string(),
            r#type: Poison,
            category: Status,
            power: 0,
            description: "毒ガスを相手の顔に吹きかけて毒の状態にする。".to_string()
        },
        Move {
            name: "ふしょくガス".to_string(),
            r#type: Poison,
            category: Status,
            power: 0,
            description: "強い酸性のガスで周りにいるものを包みこみ持っている道具を溶かしてしまう。".to_string()
        },
        Move {
            name: "どくのこな".to_string(),
            r#type: Poison,
            category: Status,
            power: 0,
            description: "毒のある粉をたくさんふりまいて相手を毒状態にする。".to_string()
        },
        Move {
            name: "とける".to_string(),
            r#type: Poison,
            category: Status,
            power: 0,
            description: "細胞の変化で液状になり自分の防御をぐーんとあげる。".to_string()
        },
        Move {
            name: "どくびし".to_string(),
            r#type: Poison,
            category: Status,
            power: 0,
            description: "相手の足下にどくびしをしかける。交代ででてきた相手のポケモンに毒をおわせる。".to_string()
        },
        Move {
            name: "とぐろをまく".to_string(),
            r#type: Poison,
            category: Status,
            power: 0,
            description: "とぐろをまいて集中する。自分の攻撃と防御と命中率をあげる。".to_string()
        },
        Move {
            name: "どくどく".to_string(),
            r#type: Poison,
            category: Status,
            power: 0,
            description: "相手を猛毒の状態にする。ターンがすすむほど毒のダメージが増えていく。".to_string()
        },
        Move {
            name: "いえき".to_string(),
            r#type: Poison,
            category: Status,
            power: 0,
            description: "胃液を相手の体に吐きつける。ついた胃液は相手の特性の効果を消す。".to_string()
        },
        Move {
            name: "トーチカ".to_string(),
            r#type: Poison,
            category: Status,
            power: 0,
            description: "相手の攻撃を防ぐと同時に触れた相手に毒を与えてしまう。".to_string()
        },
        Move {
            name: "だんがいのつるぎ".to_string(),
            r#type: Ground,
            category: Physical,
            power: 120,
            description: "大地の力を刃に変えて相手を攻撃する。".to_string()
        },
        Move {
            name: "ぶちかまし".to_string(),
            r#type: Ground,
            category: Physical,
            power: 120,
            description: "全身全霊のたいあたりをくらわせる。自分の防御と特防がさがる。".to_string()
        },
        Move {
            name: "じしん".to_string(),
            r#type: Ground,
            category: Physical,
            power: 100,
            description: "地震の衝撃で自分の周りにいるものを攻撃する。".to_string()
        },
        Move {
            name: "ねっさのあらし".to_string(),
            r#type: Ground,
            category: Special,
            power: 100,
            description: "熱く焼けた砂と強烈な風で包みこんで攻撃する。相手をやけど状態にすることがある。".to_string()
        },
        Move {
            name: "１０まんばりき".to_string(),
            r#type: Ground,
            category: Physical,
            power: 95,
            description: "全身を使って相手に猛アタックする。".to_string()
        },
        Move {
            name: "だいちのちから".to_string(),
            r#type: Ground,
            category: Special,
            power: 90,
            description: "相手の足下へ大地の力を放出する。相手の特防をさげることがある。".to_string()
        },
        Move {
            name: "あなをほる".to_string(),
            r#type: Ground,
            category: Physical,
            power: 80,
            description: "１ターン目に潜り２ターン目で相手を攻撃する。".to_string()
        },
        Move {
            name: "ドリルライナー".to_string(),
            r#type: Ground,
            category: Physical,
            power: 80,
            description: "ドリルのように体を回転しながら相手に体当たりする。急所に当たりやすい。".to_string()
        },
        Move {
            name: "じだんだ".to_string(),
            r#type: Ground,
            category: Physical,
            power: 75,
            description: "悔しさをバネにして攻撃する。前のターンに技を外していると威力が倍になる。".to_string()
        },
        Move {
            name: "ねっさのだいち".to_string(),
            r#type: Ground,
            category: Special,
            power: 70,
            description: "熱く焼けた砂を相手にぶつけて攻撃する。やけど状態にすることがある。".to_string()
        },
        Move {
            name: "じならし".to_string(),
            r#type: Ground,
            category: Physical,
            power: 60,
            description: "地面を踏みならして自分の周りにいるものを攻撃する。相手の素早さをさげる。".to_string()
        },
        Move {
            name: "マッドショット".to_string(),
            r#type: Ground,
            category: Special,
            power: 55,
            description: "泥の塊を相手に投げつけて攻撃する。同時に相手の素早さをさげる。".to_string()
        },
        Move {
            name: "すなじごく".to_string(),
            r#type: Ground,
            category: Physical,
            power: 35,
            description: "激しく吹きあれる砂あらしの中に４ー５ターンの間相手を閉じこめて攻撃する。".to_string()
        },
        Move {
            name: "ボーンラッシュ".to_string(),
            r#type: Ground,
            category: Physical,
            power: 25,
            description: "硬いホネで相手をなぐりつけて攻撃する。２ー５回の間連続でだす。".to_string()
        },
        Move {
            name: "どろかけ".to_string(),
            r#type: Ground,
            category: Special,
            power: 20,
            description: "相手の顔などに泥を投げつけて攻撃する。命中率をさげる。".to_string()
        },
        Move {
            name: "じわれ".to_string(),
            r#type: Ground,
            category: Physical,
            power: 1,
            description: "地割れの裂け目に相手を落として攻撃する。当たれば一撃でひんしにする。".to_string()
        },
        Move {
            name: "まきびし".to_string(),
            r#type: Ground,
            category: Status,
            power: 0,
            description: "相手の足下にまきびしをしかける。交代ででてきた相手のポケモンにダメージを与える。".to_string()
        },
        Move {
            name: "すなかけ".to_string(),
            r#type: Ground,
            category: Status,
            power: 0,
            description: "相手の顔に砂をかけて命中率をさげる。".to_string()
        },
        Move {
            name: "すなあつめ".to_string(),
            r#type: Ground,
            category: Status,
            power: 0,
            description: "最大ＨＰの半分自分のＨＰを回復する。すなあらしの時は多く回復。".to_string()
        },
        Move {
            name: "もろはのずつき".to_string(),
            r#type: Rock,
            category: Physical,
            power: 150,
            description: "命を懸けてこん身の力で相手にずつきをする。自分もものすごいダメージを受ける。".to_string()
        },
        Move {
            name: "メテオビーム".to_string(),
            r#type: Rock,
            category: Special,
            power: 120,
            description: "１ターン目に宇宙の力を集めることで特攻があがり２ターン目に相手を攻撃する。".to_string()
        },
        Move {
            name: "ストーンエッジ".to_string(),
            r#type: Rock,
            category: Physical,
            power: 100,
            description: "とがった岩を相手に突き刺して攻撃する。急所に当たりやすい。".to_string()
        },
        Move {
            name: "ダイヤストーム".to_string(),
            r#type: Rock,
            category: Physical,
            power: 100,
            description: "ダイヤの嵐を巻き起こしダメージを与える。自分の防御をぐーんとあげることがある。".to_string()
        },
        Move {
            name: "パワージェム".to_string(),
            r#type: Rock,
            category: Special,
            power: 80,
            description: "宝石のようにきらめく光を発射して相手を攻撃する。".to_string()
        },
        Move {
            name: "いわなだれ".to_string(),
            r#type: Rock,
            category: Physical,
            power: 75,
            description: "大きな岩を激しくぶつけて攻撃する。相手をひるませることがある。".to_string()
        },
        Move {
            name: "がんせきアックス".to_string(),
            r#type: Rock,
            category: Physical,
            power: 65,
            description: "岩の斧で急所を狙って攻撃してばらまかれた岩の破片が相手の周りに浮かぶ。".to_string()
        },
        Move {
            name: "がんせきふうじ".to_string(),
            r#type: Rock,
            category: Physical,
            power: 60,
            description: "岩石を投げつけて攻撃する。相手の動きを封じることで素早さをさげる。".to_string()
        },
        Move {
            name: "げんしのちから".to_string(),
            r#type: Rock,
            category: Special,
            power: 60,
            description: "原始の力で攻撃する。自分のすべての能力があがることがある。".to_string()
        },
        Move {
            name: "いわおとし".to_string(),
            r#type: Rock,
            category: Physical,
            power: 50,
            description: "小さな岩を持ちあげて相手に投げつけて攻撃する。".to_string()
        },
        Move {
            name: "うちおとす".to_string(),
            r#type: Rock,
            category: Physical,
            power: 50,
            description: "石や弾を投げて飛んでいる相手を攻撃する。相手はうち落とされて地面に落ちる。".to_string()
        },
        Move {
            name: "アクセルロック".to_string(),
            r#type: Rock,
            category: Physical,
            power: 40,
            description: "素早いスピードで相手にぶつかって攻撃する。必ず先制攻撃できる。".to_string()
        },
        Move {
            name: "しおづけ".to_string(),
            r#type: Rock,
            category: Physical,
            power: 40,
            description: "相手をしおづけ状態にして毎ターンダメージを与える。はがねみずタイプはより苦しむ。".to_string()
        },
        Move {
            name: "ころがる".to_string(),
            r#type: Rock,
            category: Physical,
            power: 30,
            description: "５ターンの間転がり続けて攻撃する。技が当たるたびに威力があがる。".to_string()
        },
        Move {
            name: "ロックブラスト".to_string(),
            r#type: Rock,
            category: Physical,
            power: 25,
            description: "硬い岩石を相手に発射して攻撃する。２ー５回の間連続でだす。".to_string()
        },
        Move {
            name: "ロックカット".to_string(),
            r#type: Rock,
            category: Status,
            power: 0,
            description: "自分の体を磨いて空気の抵抗を少なくする。素早さをぐーんとあげることができる。".to_string()
        },
        Move {
            name: "ステルスロック".to_string(),
            r#type: Rock,
            category: Status,
            power: 0,
            description: "相手の周りに無数の岩を浮かべて交代ででてきた相手のポケモンにダメージを与える。".to_string()
        },
        Move {
            name: "タールショット".to_string(),
            r#type: Rock,
            category: Status,
            power: 0,
            description: "ねばねばのタールを浴びせて相手の素早さを下げる。相手はほのおが弱点になる。".to_string()
        },
        Move {
            name: "すなあらし".to_string(),
            r#type: Rock,
            category: Status,
            power: 0,
            description: "５ターンの間砂あらしでいわじめんはがねタイプ以外にダメージ。いわタイプの特防があがる。".to_string()
        },
        Move {
            name: "ワイドガード".to_string(),
            r#type: Rock,
            category: Status,
            power: 0,
            description: "味方全員に当たる攻撃を１ターンの間防ぐ。".to_string()
        },
        Move {
            name: "メガホーン".to_string(),
            r#type: Bug,
            category: Physical,
            power: 120,
            description: "硬くてりっぱなつのでおもいっきり相手を突き刺して攻撃する。".to_string()
        },
        Move {
            name: "こうげきしれい".to_string(),
            r#type: Bug,
            category: Physical,
            power: 90,
            description: "しもべを呼びだして相手にむかって攻撃させる。急所に当たりやすい。".to_string()
        },
        Move {
            name: "かふんだんご".to_string(),
            r#type: Bug,
            category: Special,
            power: 90,
            description: "敵には爆発するだんごを使って攻撃。味方には回復するだんごを与える。".to_string()
        },
        Move {
            name: "むしのさざめき".to_string(),
            r#type: Bug,
            category: Special,
            power: 90,
            description: "振動で音波をおこして攻撃する。相手の特防をさげることがある。".to_string()
        },
        Move {
            name: "であいがしら".to_string(),
            r#type: Bug,
            category: Physical,
            power: 90,
            description: "威力が高い技だが戦闘に出たらすぐに出さないと成功しない。".to_string()
        },
        Move {
            name: "シザークロス".to_string(),
            r#type: Bug,
            category: Physical,
            power: 80,
            description: "カマやツメをハサミのように交差させながら相手を切り裂く。".to_string()
        },
        Move {
            name: "とびかかる".to_string(),
            r#type: Bug,
            category: Physical,
            power: 80,
            description: "全力で相手に飛びかかって攻撃。相手の攻撃をさげる。".to_string()
        },
        Move {
            name: "きゅうけつ".to_string(),
            r#type: Bug,
            category: Physical,
            power: 80,
            description: "血を吸い取って相手を攻撃する。与えたダメージの半分のＨＰを回復できる。".to_string()
        },
        Move {
            name: "とんぼがえり".to_string(),
            r#type: Bug,
            category: Physical,
            power: 70,
            description: "攻撃したあとものすごいスピードでもどってきて控えのポケモンと入れ替わる。".to_string()
        },
        Move {
            name: "はいよるいちげき".to_string(),
            r#type: Bug,
            category: Physical,
            power: 70,
            description: "背後からはいより攻撃する。相手の特攻をさげる。".to_string()
        },
        Move {
            name: "むしくい".to_string(),
            r#type: Bug,
            category: Physical,
            power: 60,
            description: "かみついて攻撃する。相手がきのみを持っているとき食べてきのみの効果を受けられる。".to_string()
        },
        Move {
            name: "とどめばり".to_string(),
            r#type: Bug,
            category: Physical,
            power: 50,
            description: "この技を使って相手を倒すと攻撃がぐぐーんとあがる。".to_string()
        },
        Move {
            name: "むしのていこう".to_string(),
            r#type: Bug,
            category: Special,
            power: 50,
            description: "抵抗して相手を攻撃する。相手の特攻をさげる。".to_string()
        },
        Move {
            name: "とびつく".to_string(),
            r#type: Bug,
            category: Physical,
            power: 50,
            description: "相手に飛びついて攻撃する。相手の素早さをさげる。".to_string()
        },
        Move {
            name: "れんぞくぎり".to_string(),
            r#type: Bug,
            category: Physical,
            power: 40,
            description: "カマやツメなどで相手を切りつけて攻撃する。連続で当てると威力があがる。".to_string()
        },
        Move {
            name: "ミサイルばり".to_string(),
            r#type: Bug,
            category: Physical,
            power: 25,
            description: "鋭いハリを相手に発射して攻撃する。２ー５回の間連続でだす。".to_string()
        },
        Move {
            name: "まとわりつく".to_string(),
            r#type: Bug,
            category: Special,
            power: 20,
            description: "４ー５ターンの間相手にまとわりついて攻撃する。そのあいだ相手は逃げられない。".to_string()
        },
        Move {
            name: "いとをはく".to_string(),
            r#type: Bug,
            category: Status,
            power: 0,
            description: "口から吹きだした糸をまきつけて相手の素早さをがくっとさげる。".to_string()
        },
        Move {
            name: "いかりのこな".to_string(),
            r#type: Bug,
            category: Status,
            power: 0,
            description: "イライラさせる粉を自分にふりかけて注意をひく。相手の攻撃をすべて自分にむける。".to_string()
        },
        Move {
            name: "ちょうのまい".to_string(),
            r#type: Bug,
            category: Status,
            power: 0,
            description: "神秘的で美しい舞を軽やかにおどる。自分の特攻と特防と素早さをあげる。".to_string()
        },
        Move {
            name: "ねばねばネット".to_string(),
            r#type: Bug,
            category: Status,
            power: 0,
            description: "相手の周りにねばねばしたネットをはりめぐらせ交代ででてきた相手の素早さをさげる。".to_string()
        },
        Move {
            name: "ぼうぎょしれい".to_string(),
            r#type: Bug,
            category: Status,
            power: 0,
            description: "しもべを呼びだして自分の体におおいつかせる。防御と特防をあげることができる。".to_string()
        },
        Move {
            name: "スレッドトラップ".to_string(),
            r#type: Bug,
            category: Status,
            power: 0,
            description: "糸の罠をはりめぐらせる。相手の攻撃を防ぐと同時に触れた相手の素早さをさげる。".to_string()
        },
        Move {
            name: "シャドーダイブ".to_string(),
            r#type: Ghost,
            category: Physical,
            power: 120,
            description: "１ターン目で姿を消して２ターン目に相手を攻撃する。守っていても攻撃は当たる。".to_string()
        },
        Move {
            name: "アストラルビット".to_string(),
            r#type: Ghost,
            category: Special,
            power: 120,
            description: "たくさんの小さな霊体を相手にぶつけて攻撃する。".to_string()
        },
        Move {
            name: "ポルターガイスト".to_string(),
            r#type: Ghost,
            category: Physical,
            power: 110,
            description: "相手の持ち物をあやつって攻撃。相手が道具を持っていない場合は失敗する。".to_string()
        },
        Move {
            name: "ゴーストダイブ".to_string(),
            r#type: Ghost,
            category: Physical,
            power: 90,
            description: "１ターンめでどこかに消えて２ターンめに相手を攻撃する。守りを無視して攻撃できる。".to_string()
        },
        Move {
            name: "シャドーボール".to_string(),
            r#type: Ghost,
            category: Special,
            power: 80,
            description: "黒い影の塊を投げつけて攻撃する。相手の特防をさげることがある。".to_string()
        },
        Move {
            name: "かげぬい".to_string(),
            r#type: Ghost,
            category: Physical,
            power: 80,
            description: "攻撃と同時に相手の影を縫い付けて逃げられなくする。".to_string()
        },
        Move {
            name: "うらみつらみ".to_string(),
            r#type: Ghost,
            category: Special,
            power: 75,
            description: "背筋が凍るような怨念で攻撃して相手の攻撃をさげる。".to_string()
        },
        Move {
            name: "シャドークロー".to_string(),
            r#type: Ghost,
            category: Physical,
            power: 70,
            description: "影からつくった鋭いツメで相手を切り裂く。急所に当たりやすい。".to_string()
        },
        Move {
            name: "たたりめ".to_string(),
            r#type: Ghost,
            category: Special,
            power: 65,
            description: "たたみかけるように攻撃する。状態異常の相手に大きなダメージを与える。".to_string()
        },
        Move {
            name: "シャドーパンチ".to_string(),
            r#type: Ghost,
            category: Physical,
            power: 60,
            description: "影にまぎれてパンチをくりだす。攻撃は必ず命中する。".to_string()
        },
        Move {
            name: "ひゃっきやこう".to_string(),
            r#type: Ghost,
            category: Special,
            power: 60,
            description: "無数の火の玉で攻撃してやけど状態にすることがある。相手が状態異常だと威力は２倍。".to_string()
        },
        Move {
            name: "おはかまいり".to_string(),
            r#type: Ghost,
            category: Physical,
            power: 50,
            description: "仲間の無念を晴らすため攻撃する。倒された味方のポケモンが多いほど技の威力が増える。".to_string()
        },
        Move {
            name: "ふんどのこぶし".to_string(),
            r#type: Ghost,
            category: Physical,
            power: 50,
            description: "怒りをエネルギーに変えて攻撃。受けた攻撃の回数が多いほど技の威力があがる。".to_string()
        },
        Move {
            name: "かげうち".to_string(),
            r#type: Ghost,
            category: Physical,
            power: 40,
            description: "影をのばして相手の背後から攻撃する。必ず先制攻撃できる。".to_string()
        },
        Move {
            name: "したでなめる".to_string(),
            r#type: Ghost,
            category: Physical,
            power: 30,
            description: "長い舌で相手をなめまわして攻撃する。まひ状態にすることがある。".to_string()
        },
        Move {
            name: "おどろかす".to_string(),
            r#type: Ghost,
            category: Physical,
            power: 30,
            description: "大きな声などで不意に驚かして攻撃する。相手をひるませることがある。".to_string()
        },
        Move {
            name: "ナイトヘッド".to_string(),
            r#type: Ghost,
            category: Special,
            power: 1,
            description: "恐ろしい幻をみせて自分のレベルと同じだけのダメージを相手に与える。".to_string()
        },
        Move {
            name: "あやしいひかり".to_string(),
            r#type: Ghost,
            category: Status,
            power: 0,
            description: "怪しい光を相手にみせてまどわせる。相手を混乱させる。".to_string()
        },
        Move {
            name: "のろい".to_string(),
            r#type: Ghost,
            category: Status,
            power: 0,
            description: "使うポケモンがゴーストタイプとそれ以外とでは効果が変わる。".to_string()
        },
        Move {
            name: "うらみ".to_string(),
            r#type: Ghost,
            category: Status,
            power: 0,
            description: "相手が最後に使った技に恨みを抱いてその技のＰＰを４だけ減らす。".to_string()
        },
        Move {
            name: "みちづれ".to_string(),
            r#type: Ghost,
            category: Status,
            power: 0,
            description: "技のあと相手の攻撃でひんしになると攻撃相手もひんしにする。連続して出すと失敗する。".to_string()
        },
        Move {
            name: "デカハンマー".to_string(),
            r#type: Steel,
            category: Physical,
            power: 160,
            description: "大きなハンマーを体ごとぶんまわして攻撃する。この技は２回連続でだせない。".to_string()
        },
        Move {
            name: "てっていこうせん".to_string(),
            r#type: Steel,
            category: Special,
            power: 140,
            description: "全身から集めたはがねをビームとして激しく撃ちだす。自分もダメージを受けてしまう。".to_string()
        },
        Move {
            name: "アイアンローラー".to_string(),
            r#type: Steel,
            category: Physical,
            power: 130,
            description: "フィールドを破壊しながら攻撃。なんらかのフィールド状態に変わっていないと技は失敗する。".to_string()
        },
        Move {
            name: "ゴールドラッシュ".to_string(),
            r#type: Steel,
            category: Special,
            power: 120,
            description: "大量のコインをぶちまけて攻撃。自分の特攻がさがる。戦闘のあとでお金ももらえる。".to_string()
        },
        Move {
            name: "アイアンテール".to_string(),
            r#type: Steel,
            category: Physical,
            power: 100,
            description: "硬いしっぽで相手をたたきつけて攻撃する。相手の防御をさげることがある。".to_string()
        },
        Move {
            name: "きょじゅうざん".to_string(),
            r#type: Steel,
            category: Physical,
            power: 100,
            description: "全身で強大な剣を振りかざし勢いよく切りかかって攻撃する。".to_string()
        },
        Move {
            name: "きょじゅうだん".to_string(),
            r#type: Steel,
            category: Physical,
            power: 100,
            description: "全身を強固な盾へと変化させ勢いよくぶつかって攻撃する。".to_string()
        },
        Move {
            name: "ホイールスピン".to_string(),
            r#type: Steel,
            category: Physical,
            power: 100,
            description: "足に負荷をかけることにより激しく回転してダメージを与える。自分の素早さががくっとさがる。".to_string()
        },
        Move {
            name: "コメットパンチ".to_string(),
            r#type: Steel,
            category: Physical,
            power: 90,
            description: "すい星のごとくパンチをくりだして相手を攻撃する。自分の攻撃があがることがある。".to_string()
        },
        Move {
            name: "アイアンヘッド".to_string(),
            r#type: Steel,
            category: Physical,
            power: 80,
            description: "鋼のような硬い頭で攻撃する。相手をひるませることがある。".to_string()
        },
        Move {
            name: "ラスターカノン".to_string(),
            r#type: Steel,
            category: Special,
            power: 80,
            description: "体の光を一点に集めて力を放つ。相手の特防をさげることがある。".to_string()
        },
        Move {
            name: "はがねのつばさ".to_string(),
            r#type: Steel,
            category: Physical,
            power: 70,
            description: "硬い翼を相手にたたきつけて攻撃する。自分の防御があがることがある。".to_string()
        },
        Move {
            name: "スマートホーン".to_string(),
            r#type: Steel,
            category: Physical,
            power: 70,
            description: "とがったつので相手を突き刺して攻撃する。攻撃は必ず命中する。".to_string()
        },
        Move {
            name: "メタルクロー".to_string(),
            r#type: Steel,
            category: Physical,
            power: 50,
            description: "鋼鉄のツメで相手を切り裂いて攻撃する。自分の攻撃があがることがある。".to_string()
        },
        Move {
            name: "バレットパンチ".to_string(),
            r#type: Steel,
            category: Physical,
            power: 40,
            description: "弾丸のような速くて硬いパンチを相手にくりだす。必ず先制攻撃できる。".to_string()
        },
        Move {
            name: "メタルバースト".to_string(),
            r#type: Steel,
            category: Physical,
            power: 1,
            description: "技をだす前に最後に受けた技のダメージを大きくしてだした相手に返す。".to_string()
        },
        Move {
            name: "ヘビーボンバー".to_string(),
            r#type: Steel,
            category: Physical,
            power: 1,
            description: "重たい体で相手にぶつかって攻撃する。自分が相手より重いほど威力があがる。".to_string()
        },
        Move {
            name: "ジャイロボール".to_string(),
            r#type: Steel,
            category: Physical,
            power: 1,
            description: "体を高速に回転させて体当たりする。相手より素早さが低いほど強い。".to_string()
        },
        Move {
            name: "きんぞくおん".to_string(),
            r#type: Steel,
            category: Status,
            power: 0,
            description: "金属をこすってでるようないやな音を聞かせる。相手の特防をがくっとさげる。".to_string()
        },
        Move {
            name: "てっぺき".to_string(),
            r#type: Steel,
            category: Status,
            power: 0,
            description: "皮膚を鉄のように硬くすることで自分の防御をぐーんとあげる。".to_string()
        },
        Move {
            name: "ギアチェンジ".to_string(),
            r#type: Steel,
            category: Status,
            power: 0,
            description: "歯車を回して自分の攻撃をあげるだけでなく素早さもぐーんとあげる。".to_string()
        },
        Move {
            name: "たてこもる".to_string(),
            r#type: Steel,
            category: Status,
            power: 0,
            description: "皮膚を鉄の盾のように硬くすることで自分の防御をぐーんとあげる。".to_string()
        },
        Move {
            name: "Ｖジェネレート".to_string(),
            r#type: Fire,
            category: Physical,
            power: 180,
            description: "灼熱の炎を額から発生させて捨て身の体当たり。防御特防素早さがさがる。".to_string()
        },
        Move {
            name: "ふんか".to_string(),
            r#type: Fire,
            category: Special,
            power: 150,
            description: "怒りを爆発させて相手を攻撃する。自分のＨＰが少ないほど技の威力はさがる。".to_string()
        },
        Move {
            name: "ブラストバーン".to_string(),
            r#type: Fire,
            category: Special,
            power: 150,
            description: "爆発の炎で相手を焼きつくして攻撃する。次のターンは動けなくなる。".to_string()
        },
        Move {
            name: "オーバーヒート".to_string(),
            r#type: Fire,
            category: Special,
            power: 130,
            description: "フルパワーで相手を攻撃する。使うと反動で自分の特攻ががくっとさがる。".to_string()
        },
        Move {
            name: "もえつきる".to_string(),
            r#type: Fire,
            category: Special,
            power: 130,
            description: "全身のほのおをすべて燃やして大ダメージを与える。自分のほのおタイプがなくなる。".to_string()
        },
        Move {
            name: "フレアドライブ".to_string(),
            r#type: Fire,
            category: Physical,
            power: 120,
            description: "炎をまとって突進する。自分もかなりダメージを受ける。やけど状態にすることがある。".to_string()
        },
        Move {
            name: "だいふんげき".to_string(),
            r#type: Fire,
            category: Physical,
            power: 120,
            description: "２ー３ターンの間炎を放ちながら暴れまわる。暴れたあとは混乱する。".to_string()
        },
        Move {
            name: "かえんボール".to_string(),
            r#type: Fire,
            category: Physical,
            power: 120,
            description: "小石を燃やした炎のボールで相手を攻撃する。やけど状態にすることがある。".to_string()
        },
        Move {
            name: "アーマーキャノン".to_string(),
            r#type: Fire,
            category: Special,
            power: 120,
            description: "みずからのヨロイを燃えたぎる弾として撃ち出して攻撃する。自分の防御と特防がさがる。".to_string()
        },
        Move {
            name: "だいもんじ".to_string(),
            r#type: Fire,
            category: Special,
            power: 110,
            description: "大の字の炎で相手を焼きつくす。やけど状態にすることがある。".to_string()
        },
        Move {
            name: "マグマストーム".to_string(),
            r#type: Fire,
            category: Special,
            power: 100,
            description: "激しく燃えたぎる炎のなかに４ー５ターンの間相手を閉じこめて攻撃する。".to_string()
        },
        Move {
            name: "れんごく".to_string(),
            r#type: Fire,
            category: Special,
            power: 100,
            description: "激しい炎で相手を包みこみ攻撃する。やけど状態にする。".to_string()
        },
        Move {
            name: "ねっぷう".to_string(),
            r#type: Fire,
            category: Special,
            power: 95,
            description: "熱い息を相手に吹きつけて攻撃する。やけど状態にすることがある。".to_string()
        },
        Move {
            name: "かえんほうしゃ".to_string(),
            r#type: Fire,
            category: Special,
            power: 90,
            description: "激しい炎を相手に発射して攻撃する。やけど状態にすることがある。".to_string()
        },
        Move {
            name: "むねんのつるぎ".to_string(),
            r#type: Fire,
            category: Physical,
            power: 90,
            description: "この世への未練を剣先にこめて切りつける。与えたダメージの半分のＨＰを回復できる。".to_string()
        },
        Move {
            name: "ブレイズキック".to_string(),
            r#type: Fire,
            category: Physical,
            power: 85,
            description: "攻撃した相手をやけど状態にすることがある。急所にも当たりやすい。".to_string()
        },
        Move {
            name: "ふんえん".to_string(),
            r#type: Fire,
            category: Special,
            power: 80,
            description: "真っ赤な炎で自分の周りにいるものを攻撃する。やけど状態にすることがある。".to_string()
        },
        Move {
            name: "ほのおのムチ".to_string(),
            r#type: Fire,
            category: Physical,
            power: 80,
            description: "焼けたムチで相手を打ちつける。攻撃を受けた相手は防御がさがる。".to_string()
        },
        Move {
            name: "ほのおのちかい".to_string(),
            r#type: Fire,
            category: Special,
            power: 80,
            description: "炎の柱で攻撃する。くさと組みあわせると威力があがって周りが火の海になる。".to_string()
        },
        Move {
            name: "ほのおのまい".to_string(),
            r#type: Fire,
            category: Special,
            power: 80,
            description: "炎をまといはばたいて相手を攻撃する。自分の特攻があがることがある。".to_string()
        },
        Move {
            name: "フレアソング".to_string(),
            r#type: Fire,
            category: Special,
            power: 80,
            description: "燃えたぎる火炎を歌うように吹きつけて相手を焦がす。自分の特攻をあげる。".to_string()
        },
        Move {
            name: "ほのおのパンチ".to_string(),
            r#type: Fire,
            category: Physical,
            power: 75,
            description: "炎をこめたパンチで相手を攻撃する。やけど状態にすることがある。".to_string()
        },
        Move {
            name: "マジカルフレイム".to_string(),
            r#type: Fire,
            category: Special,
            power: 75,
            description: "口から吐きだす特別熱い炎で攻撃する。相手の特攻をさげる。".to_string()
        },
        Move {
            name: "しっとのほのお".to_string(),
            r#type: Fire,
            category: Special,
            power: 70,
            description: "しっとのエネルギーで相手を攻撃。そのターン能力があがったポケモンをやけどの状態にする。".to_string()
        },
        Move {
            name: "ほのおのキバ".to_string(),
            r#type: Fire,
            category: Physical,
            power: 65,
            description: "炎をまとったキバでかみつく。相手をひるませたりやけど状態にすることがある。".to_string()
        },
        Move {
            name: "かえんぐるま".to_string(),
            r#type: Fire,
            category: Physical,
            power: 60,
            description: "炎をまとい相手に突進して攻撃する。やけど状態にすることがある。".to_string()
        },
        Move {
            name: "やきつくす".to_string(),
            r#type: Fire,
            category: Special,
            power: 60,
            description: "炎で相手を攻撃する。相手がきのみなどを持っているとき燃やして使えなくする。".to_string()
        },
        Move {
            name: "ニトロチャージ".to_string(),
            r#type: Fire,
            category: Physical,
            power: 50,
            description: "炎をまとい相手を攻撃する。力をためて自分の素早さをあげる。".to_string()
        },
        Move {
            name: "ひのこ".to_string(),
            r#type: Fire,
            category: Special,
            power: 40,
            description: "小さな炎を相手に発射して攻撃する。やけど状態にすることがある。".to_string()
        },
        Move {
            name: "ほのおのうず".to_string(),
            r#type: Fire,
            category: Special,
            power: 35,
            description: "激しく渦をまく炎の中に４ー５ターンの間相手を閉じこめて攻撃する。".to_string()
        },
        Move {
            name: "ヒートスタンプ".to_string(),
            r#type: Fire,
            category: Physical,
            power: 1,
            description: "燃える体で相手にぶつかって攻撃する。自分が相手より重いほど威力があがる。".to_string()
        },
        Move {
            name: "おにび".to_string(),
            r#type: Fire,
            category: Status,
            power: 0,
            description: "不気味で怪しい炎を放って相手をやけどの状態にする。".to_string()
        },
        Move {
            name: "にほんばれ".to_string(),
            r#type: Fire,
            category: Status,
            power: 0,
            description: "５ターンの間日差しを強くしてほのおタイプの威力をあげる。みずタイプの威力はさがる。".to_string()
        },
        Move {
            name: "ハイドロカノン".to_string(),
            r#type: Water,
            category: Special,
            power: 150,
            description: "水の大砲を相手に発射して攻撃する。次のターンは動けなくなる。".to_string()
        },
        Move {
            name: "しおふき".to_string(),
            r#type: Water,
            category: Special,
            power: 150,
            description: "潮を吹きつけて攻撃する。自分のＨＰが少ないほど技の威力はさがる。".to_string()
        },
        Move {
            name: "ウェーブタックル".to_string(),
            r#type: Water,
            category: Physical,
            power: 120,
            description: "水をまといつつ全身で相手にぶつかるが自分もかなりのダメージを受ける。".to_string()
        },
        Move {
            name: "こんげんのはどう".to_string(),
            r#type: Water,
            category: Special,
            power: 110,
            description: "青白く輝く無数の光線で相手を攻撃する。".to_string()
        },
        Move {
            name: "ハイドロポンプ".to_string(),
            r#type: Water,
            category: Special,
            power: 110,
            description: "大量の水を激しい勢いで相手に発射して攻撃する。".to_string()
        },
        Move {
            name: "スチームバースト".to_string(),
            r#type: Water,
            category: Special,
            power: 110,
            description: "ものすごく熱い蒸気を相手に浴びせる。相手はやけどすることがある。".to_string()
        },
        Move {
            name: "クラブハンマー".to_string(),
            r#type: Water,
            category: Physical,
            power: 100,
            description: "大きなハサミを相手にたたきつけて攻撃する。急所に当たりやすい。".to_string()
        },
        Move {
            name: "なみのり".to_string(),
            r#type: Water,
            category: Special,
            power: 90,
            description: "大きな波で自分の周りにいるものを攻撃する。".to_string()
        },
        Move {
            name: "だくりゅう".to_string(),
            r#type: Water,
            category: Special,
            power: 90,
            description: "濁った水を相手に発射して攻撃する。命中率をさげることがある。".to_string()
        },
        Move {
            name: "アクアテール".to_string(),
            r#type: Water,
            category: Physical,
            power: 90,
            description: "激しくあれくるう荒波のように大きなしっぽをふって相手を攻撃する。".to_string()
        },
        Move {
            name: "アクアブレイク".to_string(),
            r#type: Water,
            category: Physical,
            power: 85,
            description: "水の力で相手にぶつかって攻撃する。相手の防御をさげることがある。".to_string()
        },
        Move {
            name: "たきのぼり".to_string(),
            r#type: Water,
            category: Physical,
            power: 80,
            description: "すごい勢いで相手につっこむ。相手をひるませることがある。".to_string()
        },
        Move {
            name: "ねっとう".to_string(),
            r#type: Water,
            category: Special,
            power: 80,
            description: "熱く煮えたぎる水を相手に発射して攻撃する。やけど状態にすることがある。".to_string()
        },
        Move {
            name: "ねらいうち".to_string(),
            r#type: Water,
            category: Special,
            power: 80,
            description: "相手の技を引き受ける特性や技の影響を無視して選んだ相手を攻撃できる。".to_string()
        },
        Move {
            name: "ダイビング".to_string(),
            r#type: Water,
            category: Physical,
            power: 80,
            description: "１ターン目で潜り２ターン目に浮きあがって攻撃する。".to_string()
        },
        Move {
            name: "みずのちかい".to_string(),
            r#type: Water,
            category: Special,
            power: 80,
            description: "水の柱で攻撃する。ほのおと組みあわせると威力があがって空ににじがかかる。".to_string()
        },
        Move {
            name: "アクアステップ".to_string(),
            r#type: Water,
            category: Physical,
            power: 80,
            description: "水もしたたるかろやかな足どりで相手を翻弄しダメージを与える。自分の素早さをあげる。".to_string()
        },
        Move {
            name: "シェルブレード".to_string(),
            r#type: Water,
            category: Physical,
            power: 75,
            description: "鋭い貝殻で切りつけて攻撃する。相手の防御をさげることがある。".to_string()
        },
        Move {
            name: "アクアカッター".to_string(),
            r#type: Water,
            category: Physical,
            power: 70,
            description: "加圧された水を刃のように噴射して相手を切り裂く。急所に当たりやすい。".to_string()
        },
        Move {
            name: "バブルこうせん".to_string(),
            r#type: Water,
            category: Special,
            power: 65,
            description: "泡を勢いよく相手に発射して攻撃する。素早さをさげることがある。".to_string()
        },
        Move {
            name: "しおみず".to_string(),
            r#type: Water,
            category: Special,
            power: 65,
            description: "相手がＨＰの半分くらいきずをおっていると技の威力が２倍になる。".to_string()
        },
        Move {
            name: "みずのはどう".to_string(),
            r#type: Water,
            category: Special,
            power: 60,
            description: "水の振動を相手に与えて攻撃する。相手を混乱させることがある。".to_string()
        },
        Move {
            name: "クイックターン".to_string(),
            r#type: Water,
            category: Physical,
            power: 60,
            description: "攻撃したあとものすごいスピードで戻ってきて控えのポケモンと入れ替わる。".to_string()
        },
        Move {
            name: "ジェットパンチ".to_string(),
            r#type: Water,
            category: Physical,
            power: 60,
            description: "激流をこぶしにまとって目にも留まらぬパンチをくりだす。必ず先制攻撃できる。".to_string()
        },
        Move {
            name: "ひやみず".to_string(),
            r#type: Water,
            category: Special,
            power: 50,
            description: "相手の元気を失わせるくらい冷たい水を浴びせて攻撃する。相手の攻撃をさげる。".to_string()
        },
        Move {
            name: "みずでっぽう".to_string(),
            r#type: Water,
            category: Special,
            power: 40,
            description: "水を勢いよく相手に発射して攻撃する。".to_string()
        },
        Move {
            name: "アクアジェット".to_string(),
            r#type: Water,
            category: Physical,
            power: 40,
            description: "目にも留まらぬものすごい速さで相手につっこむ。必ず先制攻撃できる。".to_string()
        },
        Move {
            name: "うずしお".to_string(),
            r#type: Water,
            category: Special,
            power: 35,
            description: "激しく渦をまく水の中に４ー５ターンの間相手を閉じこめて攻撃する。".to_string()
        },
        Move {
            name: "トリプルダイブ".to_string(),
            r#type: Water,
            category: Physical,
            power: 30,
            description: "息のあった飛びこみをすることで相手に水しぶきをあてる。３回連続でダメージを与える。".to_string()
        },
        Move {
            name: "すいりゅうれんだ".to_string(),
            r#type: Water,
            category: Physical,
            power: 25,
            description: "みずの型を極めし流れるような３回の連撃。必ず急所に当たる。".to_string()
        },
        Move {
            name: "みずしゅりけん".to_string(),
            r#type: Water,
            category: Special,
            power: 15,
            description: "粘液でできた手裏剣を２ー５回の間連続でだす。必ず先制攻撃できる。".to_string()
        },
        Move {
            name: "からにこもる".to_string(),
            r#type: Water,
            category: Status,
            power: 0,
            description: "殻に潜りこんで身を守り自分の防御をあげる。".to_string()
        },
        Move {
            name: "アクアリング".to_string(),
            r#type: Water,
            category: Status,
            power: 0,
            description: "自分の体の周りを水でつくったベールでおおう。毎ターンＨＰを回復する。".to_string()
        },
        Move {
            name: "みずびたし".to_string(),
            r#type: Water,
            category: Status,
            power: 0,
            description: "たくさんの水を浴びせかけて相手をみずタイプにする。".to_string()
        },
        Move {
            name: "いのちのしずく".to_string(),
            r#type: Water,
            category: Status,
            power: 0,
            description: "不思議な水をふりまいて自分と場にいる味方のＨＰを回復する。".to_string()
        },
        Move {
            name: "あまごい".to_string(),
            r#type: Water,
            category: Status,
            power: 0,
            description: "５ターンの間雨を降らせてみずタイプの威力をあげる。ほのおタイプの威力はさがる。".to_string()
        },
        Move {
            name: "ハードプラント".to_string(),
            r#type: Grass,
            category: Special,
            power: 150,
            description: "大きな樹木で相手をたたきつけて攻撃する。次のターンは動けなくなる。".to_string()
        },
        Move {
            name: "クロロブラスト".to_string(),
            r#type: Grass,
            category: Special,
            power: 150,
            description: "自身の葉緑素を集約し放出して攻撃する。自分もダメージを受けてしまう。".to_string()
        },
        Move {
            name: "リーフストーム".to_string(),
            r#type: Grass,
            category: Special,
            power: 130,
            description: "とがったはっぱで相手にあらしをおこす。使うと反動で自分の特攻ががくっとさがる。".to_string()
        },
        Move {
            name: "ソーラーブレード".to_string(),
            r#type: Grass,
            category: Physical,
            power: 125,
            description: "１ターン目に光をいっぱいに集め２ターン目にその力を剣に込めて攻撃する。".to_string()
        },
        Move {
            name: "ウッドハンマー".to_string(),
            r#type: Grass,
            category: Physical,
            power: 120,
            description: "硬い胴体を相手にたたきつけて攻撃する。自分もかなりダメージを受ける。".to_string()
        },
        Move {
            name: "ソーラービーム".to_string(),
            r#type: Grass,
            category: Special,
            power: 120,
            description: "１ターン目に光をいっぱいに集め２ターン目に光の束を発射して攻撃する。".to_string()
        },
        Move {
            name: "はなびらのまい".to_string(),
            r#type: Grass,
            category: Special,
            power: 120,
            description: "２ー３ターンの間花をまきちらして相手を攻撃する。まきちらしたあとは混乱する。".to_string()
        },
        Move {
            name: "パワーウィップ".to_string(),
            r#type: Grass,
            category: Physical,
            power: 120,
            description: "ツタや触手を激しくふるって相手をたたきつけ攻撃する。".to_string()
        },
        Move {
            name: "リーフブレード".to_string(),
            r#type: Grass,
            category: Physical,
            power: 90,
            description: "はっぱを剣のようにあやつり相手を切りつけて攻撃する。急所に当たりやすい。".to_string()
        },
        Move {
            name: "はなふぶき".to_string(),
            r#type: Grass,
            category: Physical,
            power: 90,
            description: "激しい花吹雪を起こし周りにいるものに攻撃してダメージを与える。".to_string()
        },
        Move {
            name: "エナジーボール".to_string(),
            r#type: Grass,
            category: Special,
            power: 90,
            description: "自然から集めた命の力を発射する。相手の特防をさげることがある。".to_string()
        },
        Move {
            name: "タネばくだん".to_string(),
            r#type: Grass,
            category: Physical,
            power: 80,
            description: "硬い殻をもつ大きなタネを上からたたきつけて相手を攻撃する。".to_string()
        },
        Move {
            name: "くさのちかい".to_string(),
            r#type: Grass,
            category: Special,
            power: 80,
            description: "草の柱で攻撃する。みずと組みあわせると威力があがってあたりが湿原になる。".to_string()
        },
        Move {
            name: "ドラムアタック".to_string(),
            r#type: Grass,
            category: Physical,
            power: 80,
            description: "ドラムの根っこをドラミングでコントロールしてこうげきすることで相手の素早さを下げる。".to_string()
        },
        Move {
            name: "りんごさん".to_string(),
            r#type: Grass,
            category: Special,
            power: 80,
            description: "すっぱいりんごからつくりだした酸性の液体で攻撃。相手の特防を下げる。".to_string()
        },
        Move {
            name: "Ｇのちから".to_string(),
            r#type: Grass,
            category: Physical,
            power: 80,
            description: "高いところからりんごを落としてダメージを与える。相手の防御を下げる。".to_string()
        },
        Move {
            name: "ギガドレイン".to_string(),
            r#type: Grass,
            category: Special,
            power: 75,
            description: "養分を吸い取り攻撃する。与えたダメージの半分のＨＰを回復できる。".to_string()
        },
        Move {
            name: "ウッドホーン".to_string(),
            r#type: Grass,
            category: Physical,
            power: 75,
            description: "つのを突き刺して相手の養分を吸い取る。与えたダメージの半分のＨＰを回復できる。".to_string()
        },
        Move {
            name: "トロピカルキック".to_string(),
            r#type: Grass,
            category: Physical,
            power: 70,
            description: "南国由来の熱いキックを相手に浴びせる。相手の攻撃をさげる。".to_string()
        },
        Move {
            name: "トリックフラワー".to_string(),
            r#type: Grass,
            category: Physical,
            power: 70,
            description: "細工がある花たばを相手に投げて攻撃する。必ず命中して急所にも当たる。".to_string()
        },
        Move {
            name: "マジカルリーフ".to_string(),
            r#type: Grass,
            category: Special,
            power: 60,
            description: "相手を追跡する不思議なはっぱをまきちらす。攻撃は必ず命中する。".to_string()
        },
        Move {
            name: "グラススライダー".to_string(),
            r#type: Grass,
            category: Physical,
            power: 60,
            description: "地面を滑るように相手を攻撃。グラスフィールドの時必ず先制攻撃できる。".to_string()
        },
        Move {
            name: "はっぱカッター".to_string(),
            r#type: Grass,
            category: Physical,
            power: 55,
            description: "はっぱをとばして相手を切りつけて攻撃する。急所に当たりやすい。".to_string()
        },
        Move {
            name: "くさわけ".to_string(),
            r#type: Grass,
            category: Physical,
            power: 50,
            description: "草むらから飛びだすように攻撃する。軽快な足どりによって自分の素早さをあげる。".to_string()
        },
        Move {
            name: "つるのムチ".to_string(),
            r#type: Grass,
            category: Physical,
            power: 45,
            description: "ムチのようにしなる細長いつるで相手をたたきつけて攻撃する。".to_string()
        },
        Move {
            name: "このは".to_string(),
            r#type: Grass,
            category: Physical,
            power: 40,
            description: "はっぱを相手に当てて攻撃する。".to_string()
        },
        Move {
            name: "えだづき".to_string(),
            r#type: Grass,
            category: Physical,
            power: 40,
            description: "するどくとがった枝で相手を突いて攻撃する。".to_string()
        },
        Move {
            name: "メガドレイン".to_string(),
            r#type: Grass,
            category: Special,
            power: 40,
            description: "養分を吸い取り攻撃する。相手に与えたダメージの半分のＨＰを回復できる。".to_string()
        },
        Move {
            name: "タネマシンガン".to_string(),
            r#type: Grass,
            category: Physical,
            power: 25,
            description: "タネを勢いよく相手に発射して攻撃する。２ー５回の間連続でだす。".to_string()
        },
        Move {
            name: "すいとる".to_string(),
            r#type: Grass,
            category: Special,
            power: 20,
            description: "養分を吸い取り攻撃する。相手に与えたダメージの半分のＨＰを回復できる。".to_string()
        },
        Move {
            name: "くさむすび".to_string(),
            r#type: Grass,
            category: Special,
            power: 1,
            description: "草をからませて相手を転ばせる。相手が重いほど威力があがる。".to_string()
        },
        Move {
            name: "わたほうし".to_string(),
            r#type: Grass,
            category: Status,
            power: 0,
            description: "綿のようなフワフワの胞子をまとわりつかせて相手の素早さをがくっとさげる。".to_string()
        },
        Move {
            name: "しびれごな".to_string(),
            r#type: Grass,
            category: Status,
            power: 0,
            description: "しびれる粉をたくさんふりまいて相手をまひ状態にする。".to_string()
        },
        Move {
            name: "ねをはる".to_string(),
            r#type: Grass,
            category: Status,
            power: 0,
            description: "大地に根を張り毎ターン自分のＨＰを回復する。根を張っているので入れ替えられない。".to_string()
        },
        Move {
            name: "ねむりごな".to_string(),
            r#type: Grass,
            category: Status,
            power: 0,
            description: "眠くなる粉をたくさんふりまいて相手を眠り状態にする。".to_string()
        },
        Move {
            name: "キノコのほうし".to_string(),
            r#type: Grass,
            category: Status,
            power: 0,
            description: "催眠効果のある胞子をパラパラとふりまき相手を眠り状態にする。".to_string()
        },
        Move {
            name: "ハバネロエキス".to_string(),
            r#type: Grass,
            category: Status,
            power: 0,
            description: "とんでもなく辛いエキスを出す。相手の攻撃がぐーんとあがり防御ががくっとさがる。".to_string()
        },
        Move {
            name: "やどりぎのタネ".to_string(),
            r#type: Grass,
            category: Status,
            power: 0,
            description: "植えつけた相手のＨＰを毎ターン少しだけ吸い取り自分のＨＰを回復する。".to_string()
        },
        Move {
            name: "なやみのタネ".to_string(),
            r#type: Grass,
            category: Status,
            power: 0,
            description: "心をなやませるタネを植えつける。相手を眠れなくして特性をふみんにする。".to_string()
        },
        Move {
            name: "コットンガード".to_string(),
            r#type: Grass,
            category: Status,
            power: 0,
            description: "フワフワの綿毛で自分の体を包みこんで守る。防御をぐぐーんとあげる。".to_string()
        },
        Move {
            name: "グラスフィールド".to_string(),
            r#type: Grass,
            category: Status,
            power: 0,
            description: "５ターンの間グラスフィールドにする。地面にいると毎ターン回復する。くさタイプの威力があがる。".to_string()
        },
        Move {
            name: "ニードルガード".to_string(),
            r#type: Grass,
            category: Status,
            power: 0,
            description: "相手の攻撃を防ぐと同時に触れた相手の体力を削ってしまう。".to_string()
        },
        Move {
            name: "ちからをすいとる".to_string(),
            r#type: Grass,
            category: Status,
            power: 0,
            description: "相手の攻撃力と同じだけ自分のＨＰを回復する。そして相手の攻撃をさげる。".to_string()
        },
        Move {
            name: "ジャングルヒール".to_string(),
            r#type: Grass,
            category: Status,
            power: 0,
            description: "ジャングルと一体化して自分と場にいる味方のＨＰと状態を回復する。".to_string()
        },
        Move {
            name: "こうごうせい".to_string(),
            r#type: Grass,
            category: Status,
            power: 0,
            description: "自分のＨＰを回復する。天気によって回復の量が変化する。".to_string()
        },
        Move {
            name: "ボルテッカー".to_string(),
            r#type: Electric,
            category: Physical,
            power: 120,
            description: "電気をまとって突進する。自分もかなりダメージを受ける。まひ状態にすることがある。".to_string()
        },
        Move {
            name: "でんじほう".to_string(),
            r#type: Electric,
            category: Special,
            power: 120,
            description: "大砲のような電気を発射して攻撃する。相手をまひの状態にする。".to_string()
        },
        Move {
            name: "でんこうそうげき".to_string(),
            r#type: Electric,
            category: Physical,
            power: 120,
            description: "全身のでんきをすべて放って大ダメージを与える。自分のでんきタイプがなくなる。".to_string()
        },
        Move {
            name: "かみなり".to_string(),
            r#type: Electric,
            category: Special,
            power: 110,
            description: "激しい雷を相手に落として攻撃する。まひ状態にすることがある。".to_string()
        },
        Move {
            name: "かみなりあらし".to_string(),
            r#type: Electric,
            category: Special,
            power: 100,
            description: "嵐を起こし雷雲を呼びよせ雷と風で激しく攻撃をする。相手をまひ状態にすることもある。".to_string()
        },
        Move {
            name: "イナズマドライブ".to_string(),
            r#type: Electric,
            category: Special,
            power: 100,
            description: "変形しながら超高速で走行し未知なる電撃が相手をつらぬく。弱点をつくとさらに威力が増す。".to_string()
        },
        Move {
            name: "１０まんボルト".to_string(),
            r#type: Electric,
            category: Special,
            power: 90,
            description: "強い電撃を相手に浴びせて攻撃する。まひ状態にすることがある。".to_string()
        },
        Move {
            name: "ワイルドボルト".to_string(),
            r#type: Electric,
            category: Physical,
            power: 90,
            description: "電気をまとって相手にぶつかって攻撃する。自分も少しダメージを受ける。".to_string()
        },
        Move {
            name: "ほうでん".to_string(),
            r#type: Electric,
            category: Special,
            power: 80,
            description: "まばゆい電撃で自分の周りにいるものを攻撃する。まひ状態にすることがある。".to_string()
        },
        Move {
            name: "サンダープリズン".to_string(),
            r#type: Electric,
            category: Special,
            power: 80,
            description: "ほとばしる電気のおりの中に４ー５ターンの間相手を閉じこめて攻撃する。".to_string()
        },
        Move {
            name: "びりびりちくちく".to_string(),
            r#type: Electric,
            category: Physical,
            power: 80,
            description: "相手にぶつかって強力な電気を浴びせびりびりちくちくさせる。相手をひるませることがある。".to_string()
        },
        Move {
            name: "オーバードライブ".to_string(),
            r#type: Electric,
            category: Special,
            power: 80,
            description: "ギターやベースをかきならして激しく響く大きな振動を相手に与えて攻撃する。".to_string()
        },
        Move {
            name: "かみなりパンチ".to_string(),
            r#type: Electric,
            category: Physical,
            power: 75,
            description: "電撃をこめたパンチで相手を攻撃する。まひ状態にすることがある。".to_string()
        },
        Move {
            name: "ボルトチェンジ".to_string(),
            r#type: Electric,
            category: Special,
            power: 70,
            description: "攻撃したあとものすごいスピードでもどってきて控えポケモンと入れ替わる。".to_string()
        },
        Move {
            name: "ライジングボルト".to_string(),
            r#type: Electric,
            category: Special,
            power: 70,
            description: "地面から立ちのぼる電撃で攻撃。相手がエレキフィールドにいる時技の威力が２倍になる。".to_string()
        },
        Move {
            name: "スパーク".to_string(),
            r#type: Electric,
            category: Physical,
            power: 65,
            description: "電気をまとい相手に突進して攻撃する。まひ状態にすることがある。".to_string()
        },
        Move {
            name: "パラボラチャージ".to_string(),
            r#type: Electric,
            category: Special,
            power: 65,
            description: "周りにいるポケモン全員にダメージ。与えたダメージの半分を自分が回復する。".to_string()
        },
        Move {
            name: "かみなりのキバ".to_string(),
            r#type: Electric,
            category: Physical,
            power: 65,
            description: "電気をためたキバでかみつく。相手をひるませたりまひ状態にすることがある。".to_string()
        },
        Move {
            name: "でんげきは".to_string(),
            r#type: Electric,
            category: Special,
            power: 60,
            description: "電撃を素早く相手に浴びせる。攻撃は必ず命中する。".to_string()
        },
        Move {
            name: "エレキネット".to_string(),
            r#type: Electric,
            category: Special,
            power: 55,
            description: "電気のネットで相手を捕まえて攻撃する。相手の素早さをさげる。".to_string()
        },
        Move {
            name: "チャージビーム".to_string(),
            r#type: Electric,
            category: Special,
            power: 50,
            description: "電撃の束を相手に発射する。電気をためて自分の特攻をあげることがある。".to_string()
        },
        Move {
            name: "でんきショック".to_string(),
            r#type: Electric,
            category: Special,
            power: 40,
            description: "電気の刺激を相手に浴びせて攻撃する。まひ状態にすることがある。".to_string()
        },
        Move {
            name: "ほっぺすりすり".to_string(),
            r#type: Electric,
            category: Physical,
            power: 20,
            description: "電気を帯びたほっぺをすりつけて攻撃。相手をまひ状態にする。".to_string()
        },
        Move {
            name: "エレキボール".to_string(),
            r#type: Electric,
            category: Special,
            power: 1,
            description: "電気の塊を相手にぶつける。相手より素早さが速いほど威力があがる。".to_string()
        },
        Move {
            name: "でんじは".to_string(),
            r#type: Electric,
            category: Status,
            power: 0,
            description: "弱い電撃を浴びせることで相手をまひ状態にする。".to_string()
        },
        Move {
            name: "じゅうでん".to_string(),
            r#type: Electric,
            category: Status,
            power: 0,
            description: "じゅうでん状態になり次にだすでんきタイプの技の威力をあげる。自分の特防もあがる。".to_string()
        },
        Move {
            name: "じばそうさ".to_string(),
            r#type: Electric,
            category: Status,
            power: 0,
            description: "磁場を操作することによって特性プラスとマイナスの防御特防があがる。".to_string()
        },
        Move {
            name: "かいでんぱ".to_string(),
            r#type: Electric,
            category: Status,
            power: 0,
            description: "体からかいでんぱを放ち相手に浴びせることによって特攻をがくっとさげる。".to_string()
        },
        Move {
            name: "でんじふゆう".to_string(),
            r#type: Electric,
            category: Status,
            power: 0,
            description: "電気でつくった磁力の力で宙に浮かぶ。５ターンの間浮遊できる。".to_string()
        },
        Move {
            name: "エレキフィールド".to_string(),
            r#type: Electric,
            category: Status,
            power: 0,
            description: "５ターンの間エレキフィールドにする。地面にいるポケモンは眠らない。でんきタイプの威力があがる。".to_string()
        },
        Move {
            name: "みらいよち".to_string(),
            r#type: Psychic,
            category: Special,
            power: 120,
            description: "技を使った２ターン後に相手に念力の塊を送って攻撃する。".to_string()
        },
        Move {
            name: "ゆめくい".to_string(),
            r#type: Psychic,
            category: Special,
            power: 100,
            description: "寝ている相手の夢を食べて攻撃する。ダメージの半分のＨＰを回復する。".to_string()
        },
        Move {
            name: "サイコブレイク".to_string(),
            r#type: Psychic,
            category: Special,
            power: 100,
            description: "不思議な念波を実体化して相手を攻撃する。物理的なダメージを与える。".to_string()
        },
        Move {
            name: "サイコキネシス".to_string(),
            r#type: Psychic,
            category: Special,
            power: 90,
            description: "強い念力を相手に送って攻撃する。相手の特防をさげることがある。".to_string()
        },
        Move {
            name: "いてつくしせん".to_string(),
            r#type: Psychic,
            category: Special,
            power: 90,
            description: "両目からサイコパワーを撃ちだして攻撃する。こおり状態にすることがある。".to_string()
        },
        Move {
            name: "サイコファング".to_string(),
            r#type: Psychic,
            category: Physical,
            power: 85,
            description: "サイコパワーでかみついて相手を攻撃する。ひかりのかべやリフレクターなども破壊できる。".to_string()
        },
        Move {
            name: "じんつうりき".to_string(),
            r#type: Psychic,
            category: Special,
            power: 80,
            description: "みえない不思議な力を送って攻撃する。相手をひるませることがある。".to_string()
        },
        Move {
            name: "しねんのずつき".to_string(),
            r#type: Psychic,
            category: Physical,
            power: 80,
            description: "思念の力を額に集めて攻撃する。相手をひるませることがある。".to_string()
        },
        Move {
            name: "サイコショック".to_string(),
            r#type: Psychic,
            category: Special,
            power: 80,
            description: "不思議な念波を実体化して相手を攻撃する。物理的なダメージを与える。".to_string()
        },
        Move {
            name: "ワイドフォース".to_string(),
            r#type: Psychic,
            category: Special,
            power: 80,
            description: "サイコパワーで相手を攻撃する。サイコフィールドの時威力があがりすべての相手にダメージを与える。".to_string()
        },
        Move {
            name: "オーラウイング".to_string(),
            r#type: Psychic,
            category: Special,
            power: 80,
            description: "オーラで強化した翼で切り裂く。急所に当たりやすい。自分の素早さをあげる。".to_string()
        },
        Move {
            name: "ルミナコリジョン".to_string(),
            r#type: Psychic,
            category: Special,
            power: 80,
            description: "精神にも作用する奇妙な光を放って攻撃する。相手の特防をがくっとさげる。".to_string()
        },
        Move {
            name: "いじげんホール".to_string(),
            r#type: Psychic,
            category: Special,
            power: 80,
            description: "異次元ホールで突然相手の真横に現れ攻撃する。まもるやみきりなども無視できる。".to_string()
        },
        Move {
            name: "ぶきみなじゅもん".to_string(),
            r#type: Psychic,
            category: Special,
            power: 80,
            description: "強力なサイコパワーで攻撃。相手が最後に使った技のＰＰを３だけ減らす。".to_string()
        },
        Move {
            name: "サイコカッター".to_string(),
            r#type: Psychic,
            category: Physical,
            power: 70,
            description: "実体化させた心の刃で相手を切り裂く。急所に当たりやすい。".to_string()
        },
        Move {
            name: "バリアーラッシュ".to_string(),
            r#type: Psychic,
            category: Physical,
            power: 70,
            description: "思念のエネルギーをまといながら相手にぶつかっていく。自分の防御をあげる。".to_string()
        },
        Move {
            name: "しんぴのちから".to_string(),
            r#type: Psychic,
            category: Special,
            power: 70,
            description: "不思議な力を放出して攻撃する。自分の特攻があがる。".to_string()
        },
        Move {
            name: "サイケこうせん".to_string(),
            r#type: Psychic,
            category: Special,
            power: 65,
            description: "不思議な光線を相手に発射して攻撃する。混乱させることがある。".to_string()
        },
        Move {
            name: "ねんりき".to_string(),
            r#type: Psychic,
            category: Special,
            power: 50,
            description: "弱い念力を相手に送って攻撃する。相手を混乱させることがある。".to_string()
        },
        Move {
            name: "ツインビーム".to_string(),
            r#type: Psychic,
            category: Special,
            power: 40,
            description: "両目から不可思議な光線を発射して攻撃する。２回連続でダメージを与える。".to_string()
        },
        Move {
            name: "アシストパワー".to_string(),
            r#type: Psychic,
            category: Special,
            power: 20,
            description: "蓄積されたパワーで相手を攻撃する。自分の能力があがっているほど威力があがる。".to_string()
        },
        Move {
            name: "ミラーコート".to_string(),
            r#type: Psychic,
            category: Special,
            power: 1,
            description: "相手から受けた特殊攻撃のダメージを２倍にしてその相手に返す。".to_string()
        },
        Move {
            name: "こうそくいどう".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "力をぬいて体を軽くして高速で動く。自分の素早さをぐーんとあげる。".to_string()
        },
        Move {
            name: "ひかりのかべ".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "５ターンの間不思議なかべで相手から受ける特殊攻撃のダメージを弱める。".to_string()
        },
        Move {
            name: "さいみんじゅつ".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "眠気を誘う暗示をかけて相手を眠り状態にする。".to_string()
        },
        Move {
            name: "テレポート".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "ひかえのポケモンがいるときに使うと入れ替わる。野生のポケモンは逃げてしまう。".to_string()
        },
        Move {
            name: "リフレクター".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "５ターンの間不思議なかべで相手から受ける物理攻撃のダメージを弱める。".to_string()
        },
        Move {
            name: "ドわすれ".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "頭をからにして一瞬なにかを忘れることで自分の特防をぐーんとあげる。".to_string()
        },
        Move {
            name: "コスモパワー".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "宇宙から神秘の力をとりこむことで自分の防御と特防をあげる。".to_string()
        },
        Move {
            name: "めいそう".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "静かに精神を統一し心を鎮めることで自分の特攻と特防をあげる。".to_string()
        },
        Move {
            name: "まほうのこな".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "まほうのこなを浴びせて相手をエスパータイプに変化させる。".to_string()
        },
        Move {
            name: "サイドチェンジ".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "不思議な力でテレポートして自分と味方の居場所を入れ替える。連続でだすと失敗しやすい。".to_string()
        },
        Move {
            name: "さいはい".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "相手が出した技を指示してもう一度出させることができる。".to_string()
        },
        Move {
            name: "トリック".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "相手のすきをついて自分と相手の持ち物を交換する。".to_string()
        },
        Move {
            name: "なりきり".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "相手になりきって自分も相手と同じ特性に変化する。".to_string()
        },
        Move {
            name: "スキルスワップ".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "超能力で自分の特性と相手の特性を入れ替える。".to_string()
        },
        Move {
            name: "ふういん".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "相手が自分と同じ技をおぼえていたら相手だけその技を使えなくする。".to_string()
        },
        Move {
            name: "いやしのねがい".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "自分はひんしになるが控えからでてくるポケモンの状態異常とＨＰを回復する。".to_string()
        },
        Move {
            name: "パワートリック".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "超能力で自分の攻撃と防御の力を交換する。".to_string()
        },
        Move {
            name: "パワースワップ".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "超能力で自分と相手の攻撃と特攻の能力変化を入れ替える。".to_string()
        },
        Move {
            name: "ガードスワップ".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "超能力で自分と相手の防御と特防の能力変化を入れ替える。".to_string()
        },
        Move {
            name: "ハートスワップ".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "この技は使えません思い出すことができなくなりますが技を忘れることをおすすめします".to_string()
        },
        Move {
            name: "みかづきのまい".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "自分はひんしになるが控えからでてくるポケモンのすべての状態を回復する。".to_string()
        },
        Move {
            name: "ガードシェア".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "超能力で自分と相手の防御と特防をたして半分にわける。".to_string()
        },
        Move {
            name: "パワーシェア".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "超能力で自分と相手の攻撃と特攻をたして半分にわける。".to_string()
        },
        Move {
            name: "ワンダールーム".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "まか不思議な空間をつくる。５ターンのあいだすべてのポケモンの防御と特防が入れ替わる。".to_string()
        },
        Move {
            name: "マジックルーム".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "まか不思議な空間をつくる。５ターンの間すべてのポケモンの道具の効果がなくなる。".to_string()
        },
        Move {
            name: "いやしのはどう".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "いやしのはどうをとばして最大ＨＰの半分相手のＨＰを回復する。".to_string()
        },
        Move {
            name: "サイコフィールド".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "５ターンの間地面にいると先制技を受けない。エスパータイプの威力があがる。".to_string()
        },
        Move {
            name: "スピードスワップ".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "相手の素早さと自分の素早さを入れ替えてしまう。".to_string()
        },
        Move {
            name: "ねむる".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "２ターンの間眠り続ける。自分のＨＰと状態異常をすべて回復する。".to_string()
        },
        Move {
            name: "じゅうりょく".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "５ターンの間ふゆうやひこうタイプにじめんタイプの技が当たるようになる。空中に飛ぶ技も使えない。".to_string()
        },
        Move {
            name: "トリックルーム".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "まか不思議な空間をつくる。５ターンの間遅いポケモンから行動できる。".to_string()
        },
        Move {
            name: "みかづきのいのり".to_string(),
            r#type: Psychic,
            category: Status,
            power: 0,
            description: "みかづきにいのりをささげて自分と場にいる味方のＨＰと状態を回復する。".to_string()
        },
        Move {
            name: "ブリザードランス".to_string(),
            r#type: Ice,
            category: Physical,
            power: 120,
            description: "吹雪をまとった氷の槍を相手に投げつけて攻撃する。".to_string()
        },
        Move {
            name: "ふぶき".to_string(),
            r#type: Ice,
            category: Special,
            power: 110,
            description: "激しい吹雪を相手に吹きつけて攻撃する。こおり状態にすることがある。".to_string()
        },
        Move {
            name: "アイスハンマー".to_string(),
            r#type: Ice,
            category: Physical,
            power: 100,
            description: "強くて重いこぶしをふるってダメージを与える。自分の素早さがさがる。".to_string()
        },
        Move {
            name: "ひょうざんおろし".to_string(),
            r#type: Ice,
            category: Physical,
            power: 100,
            description: "氷山のような大きな氷塊をぶつけて攻撃する。相手をひるませることがある。".to_string()
        },
        Move {
            name: "れいとうビーム".to_string(),
            r#type: Ice,
            category: Special,
            power: 90,
            description: "凍えるビームを相手に発射して攻撃する。こおり状態にすることがある。".to_string()
        },
        Move {
            name: "つららおとし".to_string(),
            r#type: Ice,
            category: Physical,
            power: 85,
            description: "大きな氷柱を激しくぶつけて攻撃する。相手をひるませることがある。".to_string()
        },
        Move {
            name: "アイススピナー".to_string(),
            r#type: Ice,
            category: Physical,
            power: 80,
            description: "足に薄い氷をまといクルクルと回りながらぶつかる。回転の動きによってフィールドを壊す。".to_string()
        },
        Move {
            name: "れいとうパンチ".to_string(),
            r#type: Ice,
            category: Physical,
            power: 75,
            description: "冷気をこめたパンチで相手を攻撃する。こおり状態にすることがある。".to_string()
        },
        Move {
            name: "フリーズドライ".to_string(),
            r#type: Ice,
            category: Special,
            power: 70,
            description: "相手を急激に冷やしてこおり状態にすることがある。みずタイプにも効果バツグンになる。".to_string()
        },
        Move {
            name: "オーロラビーム".to_string(),
            r#type: Ice,
            category: Special,
            power: 65,
            description: "にじいろのビームを相手に発射して攻撃する。攻撃をさげることがある。".to_string()
        },
        Move {
            name: "こおりのキバ".to_string(),
            r#type: Ice,
            category: Physical,
            power: 65,
            description: "冷気をひめたキバでかみつく。相手をひるませたりこおり状態にすることがある。".to_string()
        },
        Move {
            name: "ゆきなだれ".to_string(),
            r#type: Ice,
            category: Physical,
            power: 60,
            description: "相手から技を受けているとその相手に対して技の威力が２倍になる。".to_string()
        },
        Move {
            name: "こおりのいぶき".to_string(),
            r#type: Ice,
            category: Special,
            power: 60,
            description: "冷たい息を相手に吹きつけて攻撃する。必ず急所に当たる。".to_string()
        },
        Move {
            name: "こごえるかぜ".to_string(),
            r#type: Ice,
            category: Special,
            power: 55,
            description: "凍てつく冷気を相手に吹きつけて攻撃する。相手の素早さをさげる。".to_string()
        },
        Move {
            name: "こおりのつぶて".to_string(),
            r#type: Ice,
            category: Physical,
            power: 40,
            description: "氷の塊を一瞬でつくり相手に素早く放つ。必ず先制攻撃できる。".to_string()
        },
        Move {
            name: "こなゆき".to_string(),
            r#type: Ice,
            category: Special,
            power: 40,
            description: "冷たい粉雪を相手に吹きつけて攻撃する。こおり状態にすることがある。".to_string()
        },
        Move {
            name: "つららばり".to_string(),
            r#type: Ice,
            category: Physical,
            power: 25,
            description: "鋭い氷柱を相手に発射して攻撃する。２ー５回の間連続でだす。".to_string()
        },
        Move {
            name: "トリプルアクセル".to_string(),
            r#type: Ice,
            category: Physical,
            power: 20,
            description: "３回連続でキックをくりだして攻撃する。技が当たるたびに威力はあがる。".to_string()
        },
        Move {
            name: "ぜったいれいど".to_string(),
            r#type: Ice,
            category: Special,
            power: 1,
            description: "相手を一撃で瀕死にする。こおりタイプ以外のポケモンが使うと当たりにくい。".to_string()
        },
        Move {
            name: "しろいきり".to_string(),
            r#type: Ice,
            category: Status,
            power: 0,
            description: "白い霧で体をおおう。５ターンの間相手に能力をさげられなくなる。".to_string()
        },
        Move {
            name: "くろいきり".to_string(),
            r#type: Ice,
            category: Status,
            power: 0,
            description: "黒い霧をだして戦闘にでているポケモン全員の能力変化をもとにもどす。".to_string()
        },
        Move {
            name: "オーロラベール".to_string(),
            r#type: Ice,
            category: Status,
            power: 0,
            description: "５ターンの間物理と特殊のダメージを弱める。ゆきの時しか出すことができない。".to_string()
        },
        Move {
            name: "さむいギャグ".to_string(),
            r#type: Ice,
            category: Status,
            power: 0,
            description: "場を凍らせるギャグを言い残し控えのポケモンと入れ替わる。５ターンの間ゆきを降らす。".to_string()
        },
        Move {
            name: "ゆきげしき".to_string(),
            r#type: Ice,
            category: Status,
            power: 0,
            description: "５ターンの間ゆきを降らせる。こおりタイプの防御があがる。".to_string()
        },
        Move {
            name: "ときのほうこう".to_string(),
            r#type: Dragon,
            category: Special,
            power: 150,
            description: "時間がゆがむほどの力をうちだして相手を攻撃する。次のターンは動けなくなる。".to_string()
        },
        Move {
            name: "ドラゴンエナジー".to_string(),
            r#type: Dragon,
            category: Special,
            power: 150,
            description: "生命力をパワーに変え相手を攻撃する。自分のＨＰが少ないほど技の威力はさがる。".to_string()
        },
        Move {
            name: "りゅうせいぐん".to_string(),
            r#type: Dragon,
            category: Special,
            power: 130,
            description: "天空から隕石を相手に落とす。使うと反動で自分の特攻ががくっとさがる。".to_string()
        },
        Move {
            name: "げきりん".to_string(),
            r#type: Dragon,
            category: Physical,
            power: 120,
            description: "２ー３ターンの間暴れまくって攻撃する。暴れたあとは混乱する。".to_string()
        },
        Move {
            name: "きょけんとつげき".to_string(),
            r#type: Dragon,
            category: Physical,
            power: 120,
            description: "体を投げだす無謀な突撃。次のターン相手からの攻撃は必ず命中しダメージが２倍になってしまう。".to_string()
        },
        Move {
            name: "ドラゴンダイブ".to_string(),
            r#type: Dragon,
            category: Physical,
            power: 100,
            description: "すさまじい殺気で威圧しながら体当たりする。相手をひるませることがある。".to_string()
        },
        Move {
            name: "あくうせつだん".to_string(),
            r#type: Dragon,
            category: Special,
            power: 100,
            description: "周りの空間ごと相手を引き裂きダメージを与える。急所に当たりやすい。".to_string()
        },
        Move {
            name: "ダイマックスほう".to_string(),
            r#type: Dragon,
            category: Special,
            power: 100,
            description: "体内で凝縮したエネルギーをコアから放って攻撃する。".to_string()
        },
        Move {
            name: "りゅうのはどう".to_string(),
            r#type: Dragon,
            category: Special,
            power: 85,
            description: "大きな口から衝撃波をまきおこして相手を攻撃する。".to_string()
        },
        Move {
            name: "ドラゴンクロー".to_string(),
            r#type: Dragon,
            category: Physical,
            power: 80,
            description: "鋭くとがった巨大なツメで相手を切り裂いて攻撃する。".to_string()
        },
        Move {
            name: "いっちょうあがり".to_string(),
            r#type: Dragon,
            category: Physical,
            power: 80,
            description: "いなせな身のこなしで攻撃。口の中にシャリタツがいるとそのすがたによって能力があがる。".to_string()
        },
        Move {
            name: "りゅうのいぶき".to_string(),
            r#type: Dragon,
            category: Special,
            power: 60,
            description: "ものすごい息を相手に吹きつけて攻撃する。まひ状態にすることがある。".to_string()
        },
        Move {
            name: "ワイドブレイカー".to_string(),
            r#type: Dragon,
            category: Physical,
            power: 60,
            description: "きょうじんなしっぽを激しくふりはらって相手を攻撃する。相手の攻撃を下げる。".to_string()
        },
        Move {
            name: "ドラゴンテール".to_string(),
            r#type: Dragon,
            category: Physical,
            power: 60,
            description: "相手をはじきとばして控えのポケモンをひきずりだす。野生の場合は戦闘が終わる。".to_string()
        },
        Move {
            name: "ドラゴンアロー".to_string(),
            r#type: Dragon,
            category: Physical,
            power: 50,
            description: "ドラメシヤで２回攻撃。相手が２匹いるときはそれぞれに１回ずつ攻撃する。".to_string()
        },
        Move {
            name: "たつまき".to_string(),
            r#type: Dragon,
            category: Special,
            power: 40,
            description: "竜巻をおこして相手をまきこみ攻撃する。相手をひるませることがある。".to_string()
        },
        Move {
            name: "スケイルショット".to_string(),
            r#type: Dragon,
            category: Physical,
            power: 25,
            description: "ウロコを撃ちだして攻撃する。２ー５回の間連続でだす。素早さがあがるが防御がさがる。".to_string()
        },
        Move {
            name: "りゅうのまい".to_string(),
            r#type: Dragon,
            category: Status,
            power: 0,
            description: "神秘的で力強い舞を激しくおどる。自分の攻撃と素早さをあげる。".to_string()
        },
        Move {
            name: "いじげんラッシュ".to_string(),
            r#type: Dark,
            category: Physical,
            power: 100,
            description: "たくさんの腕でまもるやみきりなどを無視した連続攻撃。自分の防御がさがる。".to_string()
        },
        Move {
            name: "イカサマ".to_string(),
            r#type: Dark,
            category: Physical,
            power: 95,
            description: "相手の力を利用する。戦っている相手の攻撃が高いほどダメージがあがる。".to_string()
        },
        Move {
            name: "もえあがるいかり".to_string(),
            r#type: Dark,
            category: Special,
            power: 90,
            description: "怒りを炎のようなオーラに変えて攻撃する。相手をひるませることがある。".to_string()
        },
        Move {
            name: "ナイトバースト".to_string(),
            r#type: Dark,
            category: Special,
            power: 85,
            description: "暗黒の衝撃波をとばして相手を攻撃する。命中率をさげることがある。".to_string()
        },
        Move {
            name: "ＤＤラリアット".to_string(),
            r#type: Dark,
            category: Physical,
            power: 85,
            description: "両腕を回し相手に当てる。相手の能力変化に関係なくダメージを与える。".to_string()
        },
        Move {
            name: "ドゲザン".to_string(),
            r#type: Dark,
            category: Physical,
            power: 85,
            description: "土下座して相手を油断させておいて切りかかる。攻撃は必ず命中する。".to_string()
        },
        Move {
            name: "かみくだく".to_string(),
            r#type: Dark,
            category: Physical,
            power: 80,
            description: "鋭い歯で相手をかみくだいて攻撃する。相手の防御をさげることがある。".to_string()
        },
        Move {
            name: "あくのはどう".to_string(),
            r#type: Dark,
            category: Special,
            power: 80,
            description: "体から悪意にみちた恐ろしいオーラを発する。相手をひるませることがある。".to_string()
        },
        Move {
            name: "じごくづき".to_string(),
            r#type: Dark,
            category: Physical,
            power: 80,
            description: "この技を受けた相手は地獄の苦しみから２ターンの間音の技を出すことができなくなる。".to_string()
        },
        Move {
            name: "くらいつく".to_string(),
            r#type: Dark,
            category: Physical,
            power: 80,
            description: "お互いひんしになるまで交代ができなくなる。どちらかのポケモンがいなくなると効果は消える。".to_string()
        },
        Move {
            name: "どげざつき".to_string(),
            r#type: Dark,
            category: Physical,
            power: 80,
            description: "頭を下げるふりをしながら振りみだした髪の毛を突き刺す。攻撃は必ず命中する。".to_string()
        },
        Move {
            name: "うっぷんばらし".to_string(),
            r#type: Dark,
            category: Physical,
            power: 75,
            description: "相手へのいらだちをぶつけて攻撃。そのターンに能力をさげられていると技の威力が２倍になる。".to_string()
        },
        Move {
            name: "あんこくきょうだ".to_string(),
            r#type: Dark,
            category: Physical,
            power: 75,
            description: "あくの型を極めし強烈な一撃。必ず急所に当たる。".to_string()
        },
        Move {
            name: "つじぎり".to_string(),
            r#type: Dark,
            category: Physical,
            power: 70,
            description: "一瞬のすきをついて相手を切りはらう。急所に当たりやすい。".to_string()
        },
        Move {
            name: "ふいうち".to_string(),
            r#type: Dark,
            category: Physical,
            power: 70,
            description: "相手より先に攻撃できる。相手がだす技が攻撃技でないと失敗する。".to_string()
        },
        Move {
            name: "はたきおとす".to_string(),
            r#type: Dark,
            category: Physical,
            power: 65,
            description: "相手の持ち物をはたき落として戦闘が終わるまで使えなくする。物を持つ相手にはダメージが増す。".to_string()
        },
        Move {
            name: "ひけん・ちえなみ".to_string(),
            r#type: Dark,
            category: Physical,
            power: 65,
            description: "貝殻の剣によって急所を狙って攻撃する。ばらまかれた貝殻の破片は相手の足下にまきびしとなって散らばる。".to_string()
        },
        Move {
            name: "かみつく".to_string(),
            r#type: Dark,
            category: Physical,
            power: 60,
            description: "鋭くとがった歯でかみついて攻撃する。相手をひるませることがある。".to_string()
        },
        Move {
            name: "どろぼう".to_string(),
            r#type: Dark,
            category: Physical,
            power: 60,
            description: "攻撃と同時に道具を盗む。自分が道具を持っている場合は盗めない。".to_string()
        },
        Move {
            name: "ぶんまわす".to_string(),
            r#type: Dark,
            category: Physical,
            power: 60,
            description: "自分の体をぶんまわして相手にダメージを与える。".to_string()
        },
        Move {
            name: "ダメおし".to_string(),
            r#type: Dark,
            category: Physical,
            power: 60,
            description: "そのターンに相手がすでにダメージを受けていたら技の威力は２倍になる。".to_string()
        },
        Move {
            name: "バークアウト".to_string(),
            r#type: Dark,
            category: Special,
            power: 55,
            description: "まくしたてるように怒鳴りつけて相手の特攻をさげる。".to_string()
        },
        Move {
            name: "しっぺがえし".to_string(),
            r#type: Dark,
            category: Physical,
            power: 50,
            description: "ためこんで攻撃する。相手よりあとに攻撃できると技の威力は２倍になる。".to_string()
        },
        Move {
            name: "つけあがる".to_string(),
            r#type: Dark,
            category: Physical,
            power: 20,
            description: "自分の強さを鼻高々に攻撃する。自分の能力があがっているほど威力があがる。".to_string()
        },
        Move {
            name: "ふくろだたき".to_string(),
            r#type: Dark,
            category: Physical,
            power: 1,
            description: "味方全員で攻撃する。仲間のポケモンが多いほど技の攻撃回数が増える。".to_string()
        },
        Move {
            name: "なげつける".to_string(),
            r#type: Dark,
            category: Physical,
            power: 1,
            description: "持たせた道具を素早く投げつけて攻撃する。道具で威力と効果が変わる。".to_string()
        },
        Move {
            name: "カタストロフィ".to_string(),
            r#type: Dark,
            category: Special,
            power: 1,
            description: "破滅的な災厄を巻き起こし相手のＨＰを半分にする。".to_string()
        },
        Move {
            name: "ほうふく".to_string(),
            r#type: Dark,
            category: Physical,
            power: 1,
            description: "技をだす前に最後に受けた技のダメージを大きくしてだした相手にやり返す。".to_string()
        },
        Move {
            name: "ちょうはつ".to_string(),
            r#type: Dark,
            category: Status,
            power: 0,
            description: "相手を怒らせる。３ターンの間相手はダメージを与える技しかだせなくなる。".to_string()
        },
        Move {
            name: "うそなき".to_string(),
            r#type: Dark,
            category: Status,
            power: 0,
            description: "ないたふりをして涙を流す。こまらせることで相手の特防をがくっとさげる。".to_string()
        },
        Move {
            name: "わるだくみ".to_string(),
            r#type: Dark,
            category: Status,
            power: 0,
            description: "悪いことを考えて頭を活性化させる。自分の特攻をぐーんとあげる。".to_string()
        },
        Move {
            name: "すてゼリフ".to_string(),
            r#type: Dark,
            category: Status,
            power: 0,
            description: "すてゼリフで相手をいかくし攻撃と特攻をさげたのち控えのポケモンと入れ替わる。".to_string()
        },
        Move {
            name: "いちゃもん".to_string(),
            r#type: Dark,
            category: Status,
            power: 0,
            description: "相手にいちゃもんをつけて同じ技を２回連続でだせなくする。".to_string()
        },
        Move {
            name: "おだてる".to_string(),
            r#type: Dark,
            category: Status,
            power: 0,
            description: "相手をおだてて混乱させる。同時に相手の特攻もあげてしまう。".to_string()
        },
        Move {
            name: "つめとぎ".to_string(),
            r#type: Dark,
            category: Status,
            power: 0,
            description: "ツメを磨いて鋭くする。自分の攻撃と命中率をあげる。".to_string()
        },
        Move {
            name: "さきおくり".to_string(),
            r#type: Dark,
            category: Status,
            power: 0,
            description: "相手をおさえつけて行動の順番を最後にする。".to_string()
        },
        Move {
            name: "おきみやげ".to_string(),
            r#type: Dark,
            category: Status,
            power: 0,
            description: "自分はひんしになるがそのかわりに相手の攻撃と特攻をがくっとさげる。".to_string()
        },
        Move {
            name: "すりかえ".to_string(),
            r#type: Dark,
            category: Status,
            power: 0,
            description: "目にもとまらぬ速さで自分と相手の持ち物を交換する。".to_string()
        },
        Move {
            name: "フルールカノン".to_string(),
            r#type: Fairy,
            category: Special,
            power: 130,
            description: "強力なビームを放ったあと自分の特攻ががくっとさがる。".to_string()
        },
        Move {
            name: "ミストバースト".to_string(),
            r#type: Fairy,
            category: Special,
            power: 100,
            description: "自分の周りにいるすべてを攻撃するが使うと瀕死になる。ミストフィールドで威力があがる。".to_string()
        },
        Move {
            name: "はるのあらし".to_string(),
            r#type: Fairy,
            category: Special,
            power: 100,
            description: "愛憎入りまじった強烈な風で相手を包みこんで攻撃する。相手の攻撃をさげることがある。".to_string()
        },
        Move {
            name: "ムーンフォース".to_string(),
            r#type: Fairy,
            category: Special,
            power: 95,
            description: "月のパワーをかりて相手を攻撃する。相手の特攻をさげることがある。".to_string()
        },
        Move {
            name: "じゃれつく".to_string(),
            r#type: Fairy,
            category: Physical,
            power: 90,
            description: "相手にじゃれついて攻撃する。相手の攻撃をさげることがある。".to_string()
        },
        Move {
            name: "マジカルシャイン".to_string(),
            r#type: Fairy,
            category: Special,
            power: 80,
            description: "強力な光を放ち相手にダメージを与える。".to_string()
        },
        Move {
            name: "ソウルクラッシュ".to_string(),
            r#type: Fairy,
            category: Physical,
            power: 75,
            description: "食らうとくじけるほどの勢いで攻撃。相手の特攻を下げる。".to_string()
        },
        Move {
            name: "ドレインキッス".to_string(),
            r#type: Fairy,
            category: Special,
            power: 50,
            description: "キッスによって相手からＨＰを吸い取る。与えたダメージの半分以上ＨＰを回復する。".to_string()
        },
        Move {
            name: "ようせいのかぜ".to_string(),
            r#type: Fairy,
            category: Special,
            power: 40,
            description: "ようせいのかぜを起こし相手に吹きつけて攻撃する。".to_string()
        },
        Move {
            name: "チャームボイス".to_string(),
            r#type: Fairy,
            category: Special,
            power: 40,
            description: "魅惑の鳴き声をだして相手に精神的なダメージを与える。攻撃は必ず命中する。".to_string()
        },
        Move {
            name: "つぶらなひとみ".to_string(),
            r#type: Fairy,
            category: Status,
            power: 0,
            description: "つぶらなひとみで相手をみつめて攻撃をさげる。必ず先制攻撃できる。".to_string()
        },
        Move {
            name: "あまえる".to_string(),
            r#type: Fairy,
            category: Status,
            power: 0,
            description: "かわいくみつめて油断を誘い相手の攻撃をがくっとさげる。".to_string()
        },
        Move {
            name: "アロマミスト".to_string(),
            r#type: Fairy,
            category: Status,
            power: 0,
            description: "不思議なアロマの香りによって味方の特防をあげる。".to_string()
        },
        Move {
            name: "てんしのキッス".to_string(),
            r#type: Fairy,
            category: Status,
            power: 0,
            description: "天使のようにかわいくキスして相手を混乱させる。".to_string()
        },
        Move {
            name: "ミストフィールド".to_string(),
            r#type: Fairy,
            category: Status,
            power: 0,
            description: "５ターンの間地面にいると状態異常にならずドラゴン技のダメージも半分になる。".to_string()
        },
        Move {
            name: "フェアリーロック".to_string(),
            r#type: Fairy,
            category: Status,
            power: 0,
            description: "ロックをかけることによって次のターンすべてのポケモンを逃げられなくする。".to_string()
        },
        Move {
            name: "つきのひかり".to_string(),
            r#type: Fairy,
            category: Status,
            power: 0,
            description: "自分のＨＰを回復する。天気によって回復の量が変化する。".to_string()
        }
    ]
}