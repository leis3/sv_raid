use crate::Ability;

/// とくせいの一覧を返す
pub(crate) fn abl() -> Vec<Ability> {
    vec![
        Ability {
            name: "とれないにおい".to_string(),
            description: "相手に触られるととれないにおいが相手にうつってしまう。".to_string()
        },
        Ability {
            name: "こぼれダネ".to_string(),
            description: "攻撃を受けるとグラスフィールドにする。".to_string()
        },
        Ability {
            name: "ねつこうかん".to_string(),
            description: "ほのおタイプの技を受けると攻撃が上がる。やけど状態にならない。".to_string()
        },
        Ability {
            name: "いかりのこうら".to_string(),
            description: "相手の攻撃でＨＰが半分になると怒りで防御と特防が下がるが攻撃特攻素早さが上がる。".to_string()
        },
        Ability {
            name: "きよめのしお".to_string(),
            description: "清らかな塩で状態異常にならない。ゴーストタイプの技のダメージを半減させる。".to_string()
        },
        Ability {
            name: "こんがりボディ".to_string(),
            description: "ほのおタイプの技を受けるとダメージを受けずに防御がぐーんと上がる。".to_string()
        },
        Ability {
            name: "かぜのり".to_string(),
            description: "おいかぜが吹いたり風技を受けるとダメージを受けずに攻撃が上がる。".to_string()
        },
        Ability {
            name: "ばんけん".to_string(),
            description: "いかくされると攻撃が上がる。ポケモンを入れ替えさせる技や道具が効かない。".to_string()
        },
        Ability {
            name: "いわはこび".to_string(),
            description: "いわタイプの技の威力が上がる。".to_string()
        },
        Ability {
            name: "ふうりょくでんき".to_string(),
            description: "風技を受けるとじゅうでん状態になる。".to_string()
        },
        Ability {
            name: "マイティチェンジ".to_string(),
            description: "手持ちにひっこむとマイティフォルムに変化する。".to_string()
        },
        Ability {
            name: "しれいとう".to_string(),
            description: "登場したとき味方にヘイラッシャがいると口の中に入ってそこから指令をだす。".to_string()
        },
        Ability {
            name: "でんきにかえる".to_string(),
            description: "ダメージを受けるとじゅうでん状態になる。".to_string()
        },
        Ability {
            name: "こだいかっせい".to_string(),
            description: "ブーストエナジーを持たせるか天気が晴れのときいちばん高い能力が上がる。".to_string()
        },
        Ability {
            name: "クォークチャージ".to_string(),
            description: "ブーストエナジーを持たせるかエレキフィールドのときいちばん高い能力が上がる。".to_string()
        },
        Ability {
            name: "おうごんのからだ".to_string(),
            description: "酸化せず丈夫な黄金の体は相手からの変化技を受けない。".to_string()
        },
        Ability {
            name: "わざわいのうつわ".to_string(),
            description: "災厄を呼ぶ器の力で自分以外の特攻が弱くなる。".to_string()
        },
        Ability {
            name: "わざわいのつるぎ".to_string(),
            description: "災厄を呼ぶ剣の力で自分以外の防御が弱くなる。".to_string()
        },
        Ability {
            name: "わざわいのおふだ".to_string(),
            description: "災厄を呼ぶ木札の力で自分以外の攻撃が弱くなる。".to_string()
        },
        Ability {
            name: "わざわいのたま".to_string(),
            description: "災厄を呼ぶ勾玉の力で自分以外の特防が弱くなる。".to_string()
        },
        Ability {
            name: "ひひいろのこどう".to_string(),
            description: "登場したとき天気を晴れにする。日差しが強いと古代の鼓動により攻撃が高まる。".to_string()
        },
        Ability {
            name: "ハドロンエンジン".to_string(),
            description: "登場したときエレキフィールドをはる。エレキフィールドだと未来の機関により特攻が高まる。".to_string()
        },
        Ability {
            name: "びんじょう".to_string(),
            description: "相手の能力が上がったとき自分も便乗して同じように能力を上げる。".to_string()
        },
        Ability {
            name: "はんすう".to_string(),
            description: "きのみを食べると次のターンの終わりに胃から出してもう１回だけ食べる。".to_string()
        },
        Ability {
            name: "きれあじ".to_string(),
            description: "相手を切る技の威力が上がる。".to_string()
        },
        Ability {
            name: "そうだいしょう".to_string(),
            description: "登場したとき今まで倒された味方の数が多いほど少しずつ攻撃と特攻が上がる。".to_string()
        },
        Ability {
            name: "きょうえん".to_string(),
            description: "登場したときに味方の能力変化をコピーする。".to_string()
        },
        Ability {
            name: "どくげしょう".to_string(),
            description: "物理技でダメージを受けると相手の足下にどくびしがちらばる。".to_string()
        },
        Ability {
            name: "テイルアーマー".to_string(),
            description: "頭を包む謎のしっぽがこちらにむかって先制技を出せないようにする。".to_string()
        },
        Ability {
            name: "どしょく".to_string(),
            description: "じめんタイプの技を受けるとダメージを受けずに回復する。".to_string()
        },
        Ability {
            name: "きんしのちから".to_string(),
            description: "変化技を出すとき必ず行動が遅くなるが相手の特性にジャマされない。".to_string()
        },
        Ability {
            name: "あくしゅう".to_string(),
            description: "臭いにおいを放つことによって攻撃したときに相手をひるませることがある。".to_string()
        },
        Ability {
            name: "あめふらし".to_string(),
            description: "登場したときに天気を雨にする。".to_string()
        },
        Ability {
            name: "かそく".to_string(),
            description: "毎ターン素早さが上がる。".to_string()
        },
        Ability {
            name: "カブトアーマー".to_string(),
            description: "硬い甲羅に守られて相手の攻撃が急所に当たらない。".to_string()
        },
        Ability {
            name: "がんじょう".to_string(),
            description: "ＨＰが満タンのとき技を受けても一撃で倒されることがない。一撃必殺技も効かない。".to_string()
        },
        Ability {
            name: "しめりけ".to_string(),
            description: "あたりを湿らせることによってじばくなどの爆発する技をだれも使えなくなる。".to_string()
        },
        Ability {
            name: "じゅうなん".to_string(),
            description: "柔軟な体によってまひ状態にならない。".to_string()
        },
        Ability {
            name: "すながくれ".to_string(),
            description: "砂あらしのとき回避率が上がる。".to_string()
        },
        Ability {
            name: "せいでんき".to_string(),
            description: "静電気を体にまとい触った相手をまひさせることがある。".to_string()
        },
        Ability {
            name: "ちくでん".to_string(),
            description: "でんきタイプの技を受けるとダメージを受けずに回復する。".to_string()
        },
        Ability {
            name: "ちょすい".to_string(),
            description: "みずタイプの技を受けるとダメージを受けずに回復する。".to_string()
        },
        Ability {
            name: "どんかん".to_string(),
            description: "鈍感なのでメロメロやちょうはつ状態にならない。いかくにも動じない。".to_string()
        },
        Ability {
            name: "ノーてんき".to_string(),
            description: "あらゆる天気の影響がなくなってしまう。".to_string()
        },
        Ability {
            name: "ふくがん".to_string(),
            description: "複眼を持っているため技の命中率が上がる。".to_string()
        },
        Ability {
            name: "ふみん".to_string(),
            description: "眠れない体質なのでねむり状態にならない。".to_string()
        },
        Ability {
            name: "へんしょく".to_string(),
            description: "相手から受けた技のタイプに自分のタイプが変化する。".to_string()
        },
        Ability {
            name: "めんえき".to_string(),
            description: "体内に免疫を持っているためどく状態にならない。".to_string()
        },
        Ability {
            name: "もらいび".to_string(),
            description: "ほのおタイプの技を受けると炎をもらい自分が出すほのおタイプの技が強くなる。".to_string()
        },
        Ability {
            name: "りんぷん".to_string(),
            description: "りんぷんに守られて技の追加効果を受けなくなる。".to_string()
        },
        Ability {
            name: "マイペース".to_string(),
            description: "マイペースなのでこんらん状態にならない。いかくにも動じない。".to_string()
        },
        Ability {
            name: "きゅうばん".to_string(),
            description: "吸盤で地面に張り付きポケモンを入れ替えさせる技や道具が効かなくなる。".to_string()
        },
        Ability {
            name: "いかく".to_string(),
            description: "登場したとき威嚇して相手を萎縮させ相手の攻撃を下げてしまう。".to_string()
        },
        Ability {
            name: "かげふみ".to_string(),
            description: "相手の影を踏み逃げたり交代できなくする。".to_string()
        },
        Ability {
            name: "さめはだ".to_string(),
            description: "攻撃を受けたとき自分に触れた相手をざらざらの肌でキズつける。".to_string()
        },
        Ability {
            name: "ふしぎなまもり".to_string(),
            description: "効果バツグンの技しか当たらない不思議な力。".to_string()
        },
        Ability {
            name: "ふゆう".to_string(),
            description: "地面から浮くことによってじめんタイプの技を受けない。".to_string()
        },
        Ability {
            name: "ほうし".to_string(),
            description: "攻撃で自分に触れた相手をどくやまひやねむり状態にすることがある。".to_string()
        },
        Ability {
            name: "シンクロ".to_string(),
            description: "自分がなってしまったどくやまひややけどを相手にうつす。".to_string()
        },
        Ability {
            name: "クリアボディ".to_string(),
            description: "相手の技や特性で能力を下げられない。".to_string()
        },
        Ability {
            name: "しぜんかいふく".to_string(),
            description: "手持ちにひっこむと状態異常が治る。".to_string()
        },
        Ability {
            name: "ひらいしん".to_string(),
            description: "でんきタイプの技を自分に寄せつけダメージを受けずに特攻が上がる。".to_string()
        },
        Ability {
            name: "てんのめぐみ".to_string(),
            description: "天の恵みのおかげで技の追加効果がでやすい。".to_string()
        },
        Ability {
            name: "すいすい".to_string(),
            description: "天気が雨のとき素早さが上がる。".to_string()
        },
        Ability {
            name: "ようりょくそ".to_string(),
            description: "天気が晴れのとき素早さが上がる。".to_string()
        },
        Ability {
            name: "はっこう".to_string(),
            description: "あたりを明るくすることで野生のポケモンに遭遇しやすくなる。".to_string()
        },
        Ability {
            name: "トレース".to_string(),
            description: "登場したとき相手の特性をトレースして同じ特性になる。".to_string()
        },
        Ability {
            name: "ちからもち".to_string(),
            description: "物理攻撃の威力が２倍になる。".to_string()
        },
        Ability {
            name: "どくのトゲ".to_string(),
            description: "自分に触った相手をどく状態にすることがある。".to_string()
        },
        Ability {
            name: "せいしんりょく".to_string(),
            description: "鍛えられた精神によって相手の攻撃にひるまない。いかくにも動じない。".to_string()
        },
        Ability {
            name: "マグマのよろい".to_string(),
            description: "熱いマグマを身にまといこおり状態にならない。".to_string()
        },
        Ability {
            name: "みずのベール".to_string(),
            description: "水のベールを身にまといやけど状態にならない。".to_string()
        },
        Ability {
            name: "じりょく".to_string(),
            description: "はがねタイプのポケモンを磁力で引きつけて逃げられなくする。".to_string()
        },
        Ability {
            name: "ぼうおん".to_string(),
            description: "音を遮断することによって音の技を受けない。".to_string()
        },
        Ability {
            name: "あめうけざら".to_string(),
            description: "天気が雨のとき少しずつＨＰを回復する。".to_string()
        },
        Ability {
            name: "すなおこし".to_string(),
            description: "登場したとき天気を砂あらしにする。".to_string()
        },
        Ability {
            name: "プレッシャー".to_string(),
            description: "プレッシャーをあたえて相手の使う技のＰＰを多く減らす。".to_string()
        },
        Ability {
            name: "あついしぼう".to_string(),
            description: "厚い脂肪で守られているのでほのおタイプとこおりタイプの技のダメージを半減させる。".to_string()
        },
        Ability {
            name: "はやおき".to_string(),
            description: "ねむり状態になっても２倍の早さで目覚めることができる。".to_string()
        },
        Ability {
            name: "ほのおのからだ".to_string(),
            description: "自分に触った相手をやけど状態にすることがある。".to_string()
        },
        Ability {
            name: "にげあし".to_string(),
            description: "野生のポケモンから必ず逃げられる。".to_string()
        },
        Ability {
            name: "するどいめ".to_string(),
            description: "鋭い目のおかげで命中率を下げられない。".to_string()
        },
        Ability {
            name: "かいりきバサミ".to_string(),
            description: "力自慢のハサミを持っているので相手に攻撃を下げられない。".to_string()
        },
        Ability {
            name: "ものひろい".to_string(),
            description: "相手の使った道具を拾ってくることがある。冒険中も拾ってくる。".to_string()
        },
        Ability {
            name: "なまけ".to_string(),
            description: "技を出すと次のターンは休んでしまう。".to_string()
        },
        Ability {
            name: "はりきり".to_string(),
            description: "自分の攻撃が高くなるが命中率が下がる。".to_string()
        },
        Ability {
            name: "メロメロボディ".to_string(),
            description: "自分に触った相手をメロメロにすることがある。".to_string()
        },
        Ability {
            name: "プラス".to_string(),
            description: "プラスかマイナスの特性を持つポケモンが仲間にいると自分の特攻が上がる。".to_string()
        },
        Ability {
            name: "マイナス".to_string(),
            description: "プラスかマイナスの特性を持つポケモンが仲間にいると自分の特攻が上がる。".to_string()
        },
        Ability {
            name: "てんきや".to_string(),
            description: "天気の影響を受けてみずタイプほのおタイプこおりタイプのどれかに変化する。".to_string()
        },
        Ability {
            name: "ねんちゃく".to_string(),
            description: "粘着質の体に道具がくっついているため相手に道具を奪われない。".to_string()
        },
        Ability {
            name: "だっぴ".to_string(),
            description: "体の皮を脱ぎ捨てることで状態異常を治すことがある。".to_string()
        },
        Ability {
            name: "こんじょう".to_string(),
            description: "状態異常になると根性をだして攻撃が上がる。".to_string()
        },
        Ability {
            name: "ふしぎなうろこ".to_string(),
            description: "状態異常になると不思議なウロコが反応して防御が上がる。".to_string()
        },
        Ability {
            name: "ヘドロえき".to_string(),
            description: "ヘドロ液を吸い取った相手は強烈な悪臭でダメージを受けてＨＰを減らす。".to_string()
        },
        Ability {
            name: "しんりょく".to_string(),
            description: "ＨＰが減ったときくさタイプの技の威力が上がる。".to_string()
        },
        Ability {
            name: "もうか".to_string(),
            description: "ＨＰが減ったときほのおタイプの技の威力が上がる。".to_string()
        },
        Ability {
            name: "げきりゅう".to_string(),
            description: "ＨＰが減ったときみずタイプの技の威力が上がる。".to_string()
        },
        Ability {
            name: "むしのしらせ".to_string(),
            description: "ＨＰが減ったときむしタイプの技の威力が上がる。".to_string()
        },
        Ability {
            name: "いしあたま".to_string(),
            description: "反動を受ける技を出してもＨＰが減らない。".to_string()
        },
        Ability {
            name: "ひでり".to_string(),
            description: "登場したときに天気を晴れにする。".to_string()
        },
        Ability {
            name: "ありじごく".to_string(),
            description: "戦闘で相手を逃げられなくする。".to_string()
        },
        Ability {
            name: "やるき".to_string(),
            description: "やる気をだすことによってねむり状態にならない。".to_string()
        },
        Ability {
            name: "しろいけむり".to_string(),
            description: "白い煙に守られて相手に能力を下げられない。".to_string()
        },
        Ability {
            name: "ヨガパワー".to_string(),
            description: "ヨガの力で物理攻撃の威力が２倍になる。".to_string()
        },
        Ability {
            name: "シェルアーマー".to_string(),
            description: "硬い殻に守られ相手の攻撃が急所に当たらない。".to_string()
        },
        Ability {
            name: "エアロック".to_string(),
            description: "あらゆる天気の影響が消えてしまう。".to_string()
        },
        Ability {
            name: "ちどりあし".to_string(),
            description: "こんらん状態のときは回避率がアップする。".to_string()
        },
        Ability {
            name: "でんきエンジン".to_string(),
            description: "でんきタイプの技を受けるとダメージを受けずに素早さが上がる。".to_string()
        },
        Ability {
            name: "とうそうしん".to_string(),
            description: "性別が同じだと闘争心を燃やして強くなる。性別が違うと弱くなる。".to_string()
        },
        Ability {
            name: "ふくつのこころ".to_string(),
            description: "ひるむたびに不屈の心を燃やして素早さが上がる。".to_string()
        },
        Ability {
            name: "ゆきがくれ".to_string(),
            description: "天気がゆきのとき回避率が上がる。".to_string()
        },
        Ability {
            name: "くいしんぼう".to_string(),
            description: "ＨＰが少なくなったら食べるきのみをＨＰ半分の時に食べてしまう。".to_string()
        },
        Ability {
            name: "いかりのつぼ".to_string(),
            description: "急所に攻撃が当たると怒りくるって攻撃力が最大になる。".to_string()
        },
        Ability {
            name: "かるわざ".to_string(),
            description: "持っていた道具がなくなると素早さが上がる。".to_string()
        },
        Ability {
            name: "たいねつ".to_string(),
            description: "耐熱の体によってほのおタイプの技のダメージを半減させる。".to_string()
        },
        Ability {
            name: "たんじゅん".to_string(),
            description: "能力変化がいつもの２倍になる。".to_string()
        },
        Ability {
            name: "かんそうはだ".to_string(),
            description: "天気が雨の時やみずタイプの技でＨＰが回復しはれの時やほのおタイプの技で減ってしまう。".to_string()
        },
        Ability {
            name: "ダウンロード".to_string(),
            description: "相手の防御と特防をくらべて低いほうの能力にあわせて自分の攻撃か特攻を上げる。".to_string()
        },
        Ability {
            name: "てつのこぶし".to_string(),
            description: "パンチを使う技の威力が上がる。".to_string()
        },
        Ability {
            name: "ポイズンヒール".to_string(),
            description: "どく状態になるとＨＰが減らずに増えていく。".to_string()
        },
        Ability {
            name: "てきおうりょく".to_string(),
            description: "自分とおなじタイプの技の威力が上がる。".to_string()
        },
        Ability {
            name: "スキルリンク".to_string(),
            description: "連続技を使うといつも最高回数出すことができる。".to_string()
        },
        Ability {
            name: "うるおいボディ".to_string(),
            description: "天気が雨のとき状態異常が治る。".to_string()
        },
        Ability {
            name: "サンパワー".to_string(),
            description: "天気が晴れると特攻が上がるが毎ターンＨＰが減る。".to_string()
        },
        Ability {
            name: "はやあし".to_string(),
            description: "状態異常になると素早さが上がる。".to_string()
        },
        Ability {
            name: "ノーマルスキン".to_string(),
            description: "どんなタイプの技でもすべてノーマルタイプになる。威力が少し上がる。".to_string()
        },
        Ability {
            name: "スナイパー".to_string(),
            description: "攻撃を急所に当てると威力がさらに上がる。".to_string()
        },
        Ability {
            name: "マジックガード".to_string(),
            description: "攻撃以外ではダメージを受けない。".to_string()
        },
        Ability {
            name: "ノーガード".to_string(),
            description: "ノーガード戦法によってお互いの出す技がかならず当たるようになる。".to_string()
        },
        Ability {
            name: "あとだし".to_string(),
            description: "技を出す順番がかならず最後になる。".to_string()
        },
        Ability {
            name: "テクニシャン".to_string(),
            description: "威力が低い技の威力を高くして攻撃できる。".to_string()
        },
        Ability {
            name: "リーフガード".to_string(),
            description: "天気が晴れのときは状態異常にならない。".to_string()
        },
        Ability {
            name: "ぶきよう".to_string(),
            description: "持っている道具を使うことができない。".to_string()
        },
        Ability {
            name: "かたやぶり".to_string(),
            description: "相手の特性にジャマされることなく相手に技を出すことができる。".to_string()
        },
        Ability {
            name: "きょううん".to_string(),
            description: "強運を持っているため相手の急所に攻撃が当たりやすい。".to_string()
        },
        Ability {
            name: "ゆうばく".to_string(),
            description: "ひんしになったとき触った相手にダメージをあたえる。".to_string()
        },
        Ability {
            name: "きけんよち".to_string(),
            description: "相手の持つ危険な技を察知することができる。".to_string()
        },
        Ability {
            name: "よちむ".to_string(),
            description: "登場したとき相手の持つ技をひとつだけ読み取る。".to_string()
        },
        Ability {
            name: "てんねん".to_string(),
            description: "相手の能力の変化を無視して攻撃ができる。".to_string()
        },
        Ability {
            name: "いろめがね".to_string(),
            description: "効果がいまひとつの技を通常の威力で出すことができる。".to_string()
        },
        Ability {
            name: "フィルター".to_string(),
            description: "効果バツグンになってしまう攻撃の威力を弱めることができる。".to_string()
        },
        Ability {
            name: "スロースタート".to_string(),
            description: "５ターンのあいだ攻撃と素早さが半分になる。".to_string()
        },
        Ability {
            name: "きもったま".to_string(),
            description: "ノーマルタイプとかくとうタイプの技をゴーストタイプに当てることができる。いかくにも動じない。".to_string()
        },
        Ability {
            name: "よびみず".to_string(),
            description: "みずタイプの技を自分によせつけダメージは受けずに特攻が上がる。".to_string()
        },
        Ability {
            name: "アイスボディ".to_string(),
            description: "天気がゆきのときＨＰを少しずつ回復する。".to_string()
        },
        Ability {
            name: "ハードロック".to_string(),
            description: "効果バツグンになってしまう攻撃の威力を弱めることができる。".to_string()
        },
        Ability {
            name: "ゆきふらし".to_string(),
            description: "登場したときに天気をゆきにする。".to_string()
        },
        Ability {
            name: "みつあつめ".to_string(),
            description: "戦闘が終わったときあまいミツを拾うことがある。".to_string()
        },
        Ability {
            name: "おみとおし".to_string(),
            description: "登場したとき相手の持ち物を見通すことができる。".to_string()
        },
        Ability {
            name: "すてみ".to_string(),
            description: "反動でダメージを受ける技の威力が上がる。".to_string()
        },
        Ability {
            name: "マルチタイプ".to_string(),
            description: "持っているプレートによって自分のタイプが変わる。".to_string()
        },
        Ability {
            name: "フラワーギフト".to_string(),
            description: "天気が晴れのとき自分と味方の攻撃と特防の能力が上がる。".to_string()
        },
        Ability {
            name: "ナイトメア".to_string(),
            description: "ねむり状態の相手にダメージをあたえる。".to_string()
        },
        Ability {
            name: "わるいてぐせ".to_string(),
            description: "触られた相手の道具を盗んでしまう。".to_string()
        },
        Ability {
            name: "ちからずく".to_string(),
            description: "技の追加効果はなくなるがそのぶん高い威力で技を出すことができる。".to_string()
        },
        Ability {
            name: "あまのじゃく".to_string(),
            description: "能力の変化が逆転して上がるときに下がり下がるときに上がる。".to_string()
        },
        Ability {
            name: "きんちょうかん".to_string(),
            description: "相手を緊張させてきのみを食べられなくさせる。".to_string()
        },
        Ability {
            name: "まけんき".to_string(),
            description: "相手に能力を下げられると攻撃がぐーんと上がる。".to_string()
        },
        Ability {
            name: "よわき".to_string(),
            description: "ＨＰが半分になると弱気になって攻撃と特攻が半減する。".to_string()
        },
        Ability {
            name: "のろわれボディ".to_string(),
            description: "攻撃を受けると相手の技をかなしばり状態にすることがある。".to_string()
        },
        Ability {
            name: "いやしのこころ".to_string(),
            description: "状態異常の味方をたまに治してあげる。".to_string()
        },
        Ability {
            name: "フレンドガード".to_string(),
            description: "味方のダメージを減らすことができる。".to_string()
        },
        Ability {
            name: "くだけるよろい".to_string(),
            description: "物理技でダメージを受けると防御が下がり素早さがぐーんと上がる。".to_string()
        },
        Ability {
            name: "ヘヴィメタル".to_string(),
            description: "自分の重さが２倍になる。".to_string()
        },
        Ability {
            name: "ライトメタル".to_string(),
            description: "自分の重さが半分になる。".to_string()
        },
        Ability {
            name: "マルチスケイル".to_string(),
            description: "ＨＰが満タンのときに受けるダメージが少なくなる。".to_string()
        },
        Ability {
            name: "どくぼうそう".to_string(),
            description: "どく状態になったとき物理技の威力が上がる。".to_string()
        },
        Ability {
            name: "ねつぼうそう".to_string(),
            description: "やけど状態になったとき特殊技の威力が上がる。".to_string()
        },
        Ability {
            name: "しゅうかく".to_string(),
            description: "使ったきのみを何回も作りだす。".to_string()
        },
        Ability {
            name: "テレパシー".to_string(),
            description: "味方の攻撃を読み取って技を回避する。".to_string()
        },
        Ability {
            name: "ムラっけ".to_string(),
            description: "毎ターン能力のどれかがぐーんと上がってどれかが下がる。".to_string()
        },
        Ability {
            name: "ぼうじん".to_string(),
            description: "すなあらしのダメージを受けない。粉や胞子の影響も受けない。".to_string()
        },
        Ability {
            name: "どくしゅ".to_string(),
            description: "触るだけで相手をどく状態にすることがある。".to_string()
        },
        Ability {
            name: "さいせいりょく".to_string(),
            description: "手持ちに引っ込むとＨＰが少し回復する。".to_string()
        },
        Ability {
            name: "はとむね".to_string(),
            description: "防御を下げる効果を受けない。".to_string()
        },
        Ability {
            name: "すなかき".to_string(),
            description: "天気がすなあらしのとき素早さが上がる。".to_string()
        },
        Ability {
            name: "ミラクルスキン".to_string(),
            description: "変化技を受けにくい体になっている。".to_string()
        },
        Ability {
            name: "アナライズ".to_string(),
            description: "いちばん最後に技を出すと技の威力が上がる。".to_string()
        },
        Ability {
            name: "イリュージョン".to_string(),
            description: "手持ちのいちばんうしろにいるポケモンになりきって登場して相手を化かす。".to_string()
        },
        Ability {
            name: "かわりもの".to_string(),
            description: "目の前のポケモンに変身してしまう。".to_string()
        },
        Ability {
            name: "すりぬけ".to_string(),
            description: "相手の壁や身代わりをすりぬけて攻撃できる。".to_string()
        },
        Ability {
            name: "ミイラ".to_string(),
            description: "相手に触られると相手をミイラにしてしまう。".to_string()
        },
        Ability {
            name: "じしんかじょう".to_string(),
            description: "相手を倒すと自信がついて攻撃が上がる。".to_string()
        },
        Ability {
            name: "せいぎのこころ".to_string(),
            description: "あくタイプの攻撃を受けると正義感で攻撃が上がる。".to_string()
        },
        Ability {
            name: "びびり".to_string(),
            description: "あくゴーストむしタイプの攻撃を受けたりいかくをされるとびびって素早さが上がる。".to_string()
        },
        Ability {
            name: "マジックミラー".to_string(),
            description: "相手にだされた変化技を受けずにそのまま返すことができる。".to_string()
        },
        Ability {
            name: "そうしょく".to_string(),
            description: "くさタイプの技を受けるとダメージを受けずに攻撃が上がる。".to_string()
        },
        Ability {
            name: "いたずらごころ".to_string(),
            description: "変化技を先制で出すことができる。".to_string()
        },
        Ability {
            name: "すなのちから".to_string(),
            description: "天気がすなあらしのときいわタイプとじめんタイプとはがねタイプの威力が上がる。".to_string()
        },
        Ability {
            name: "てつのトゲ".to_string(),
            description: "自分に触った相手に鉄のトゲでダメージをあたえる。".to_string()
        },
        Ability {
            name: "ダルマモード".to_string(),
            description: "ＨＰが半分以下になると姿が変化する。".to_string()
        },
        Ability {
            name: "しょうりのほし".to_string(),
            description: "自分や味方の命中率が上がる。".to_string()
        },
        Ability {
            name: "ターボブレイズ".to_string(),
            description: "相手の特性にジャマされず相手に技を出すことができる。".to_string()
        },
        Ability {
            name: "テラボルテージ".to_string(),
            description: "相手の特性にジャマされず相手に技を出すことができる。".to_string()
        },
        Ability {
            name: "アロマベール".to_string(),
            description: "自分と味方へのメンタル攻撃を防ぐことができる。".to_string()
        },
        Ability {
            name: "フラワーベール".to_string(),
            description: "味方の草ポケモンは能力が下がらず状態異常にもならない。".to_string()
        },
        Ability {
            name: "ほおぶくろ".to_string(),
            description: "どんなきのみでも食べるとＨＰも回復する。".to_string()
        },
        Ability {
            name: "へんげんじざい".to_string(),
            description: "戦闘に出るたび１回だけ自分が出す技と同じタイプに変化する。".to_string()
        },
        Ability {
            name: "ファーコート".to_string(),
            description: "相手から受ける物理技のダメージが半分になる。".to_string()
        },
        Ability {
            name: "マジシャン".to_string(),
            description: "技を当てた相手の道具を奪ってしまう。".to_string()
        },
        Ability {
            name: "ぼうだん".to_string(),
            description: "相手の弾や爆弾などの技を防ぐことができる。".to_string()
        },
        Ability {
            name: "かちき".to_string(),
            description: "相手に能力を下げられると特攻がぐーんと上がる。".to_string()
        },
        Ability {
            name: "がんじょうあご".to_string(),
            description: "あごが頑丈で噛む技の威力が高くなる。".to_string()
        },
        Ability {
            name: "フリーズスキン".to_string(),
            description: "ノーマルタイプの技がこおりタイプになる。威力が少し上がる。".to_string()
        },
        Ability {
            name: "スイートベール".to_string(),
            description: "自分と味方のポケモンは眠らなくなる。".to_string()
        },
        Ability {
            name: "バトルスイッチ".to_string(),
            description: "攻撃技を出すとブレードフォルムに技キングシールドを出すとシールドフォルムに変化する。".to_string()
        },
        Ability {
            name: "はやてのつばさ".to_string(),
            description: "ＨＰが満タンだとひこうタイプの技を先制で出すことができる。".to_string()
        },
        Ability {
            name: "メガランチャー".to_string(),
            description: "波動の技の威力が高くなる。".to_string()
        },
        Ability {
            name: "くさのけがわ".to_string(),
            description: "グラスフィールドのとき防御が上がる。".to_string()
        },
        Ability {
            name: "きょうせい".to_string(),
            description: "味方が道具を使うと自分の持っている道具を味方に渡す。".to_string()
        },
        Ability {
            name: "かたいツメ".to_string(),
            description: "相手に接触する技の威力が高くなる。".to_string()
        },
        Ability {
            name: "フェアリースキン".to_string(),
            description: "ノーマルタイプの技がフェアリータイプになる。威力が少し上がる。".to_string()
        },
        Ability {
            name: "ぬめぬめ".to_string(),
            description: "攻撃で自分に触れた相手の素早さを下げる。".to_string()
        },
        Ability {
            name: "スカイスキン".to_string(),
            description: "ノーマルタイプの技がひこうタイプになる。威力が少し上がる。".to_string()
        },
        Ability {
            name: "おやこあい".to_string(),
            description: "親子２匹で２回攻撃することができる。".to_string()
        },
        Ability {
            name: "ダークオーラ".to_string(),
            description: "全員のあくタイプの技が強くなる。".to_string()
        },
        Ability {
            name: "フェアリーオーラ".to_string(),
            description: "全員のフェアリータイプの技が強くなる。".to_string()
        },
        Ability {
            name: "オーラブレイク".to_string(),
            description: "オーラの効果を逆転させて威力を下げる。".to_string()
        },
        Ability {
            name: "はじまりのうみ".to_string(),
            description: "ほのおタイプの攻撃を受けない天気にする。".to_string()
        },
        Ability {
            name: "おわりのだいち".to_string(),
            description: "みずタイプの攻撃を受けない天気にする。".to_string()
        },
        Ability {
            name: "デルタストリーム".to_string(),
            description: "ひこうタイプの弱点がなくなる天気にする。".to_string()
        },
        Ability {
            name: "じきゅうりょく".to_string(),
            description: "攻撃を受けると防御が上がる。".to_string()
        },
        Ability {
            name: "にげごし".to_string(),
            description: "ＨＰが半分になるとあわてて逃げ出して手持ちに引っ込んでしまう。".to_string()
        },
        Ability {
            name: "ききかいひ".to_string(),
            description: "ＨＰが半分になると危険を回避するため手持ちに引っ込んでしまう。".to_string()
        },
        Ability {
            name: "みずがため".to_string(),
            description: "みずタイプの技を受けると防御がぐーんと上がる。".to_string()
        },
        Ability {
            name: "ひとでなし".to_string(),
            description: "どく状態の相手を攻撃するとかならず急所に当たる。".to_string()
        },
        Ability {
            name: "リミットシールド".to_string(),
            description: "ＨＰが半分になると殻が壊れて攻撃的になる。".to_string()
        },
        Ability {
            name: "はりこみ".to_string(),
            description: "交代で出てきた相手に２倍のダメージで攻撃できる。".to_string()
        },
        Ability {
            name: "すいほう".to_string(),
            description: "自分に対するほのおタイプの技の威力を下げる。やけどしない。".to_string()
        },
        Ability {
            name: "はがねつかい".to_string(),
            description: "はがねタイプの技の威力が上がる。".to_string()
        },
        Ability {
            name: "ぎゃくじょう".to_string(),
            description: "相手の攻撃でＨＰが半分になると特攻が上がる。".to_string()
        },
        Ability {
            name: "ゆきかき".to_string(),
            description: "天気がゆきのとき素早さが上がる。".to_string()
        },
        Ability {
            name: "えんかく".to_string(),
            description: "すべての技を相手に接触しないで出すことができる。".to_string()
        },
        Ability {
            name: "うるおいボイス".to_string(),
            description: "すべての音技がみずタイプになる。".to_string()
        },
        Ability {
            name: "ヒーリングシフト".to_string(),
            description: "回復技を先制で出すことができる。".to_string()
        },
        Ability {
            name: "エレキスキン".to_string(),
            description: "ノーマルタイプの技がでんきタイプになる。威力が少し上がる。".to_string()
        },
        Ability {
            name: "サーフテール".to_string(),
            description: "エレキフィールドのとき素早さが２倍になる。".to_string()
        },
        Ability {
            name: "ぎょぐん".to_string(),
            description: "ＨＰが多いときは群れて強くなる。ＨＰの残りが少なくなると群れは散り散りになってしまう。".to_string()
        },
        Ability {
            name: "ばけのかわ".to_string(),
            description: "体を被う化けの皮で１回攻撃を防ぐことができる。".to_string()
        },
        Ability {
            name: "きずなへんげ".to_string(),
            description: "相手を倒すとトレーナーとのキズナが深まり自分の攻撃特攻素早さが上がる。".to_string()
        },
        Ability {
            name: "スワームチェンジ".to_string(),
            description: "ＨＰが半分になるとセルたちが応援に駆けつけパーフェクトフォルムに姿を変える。".to_string()
        },
        Ability {
            name: "ふしょく".to_string(),
            description: "はがねタイプやどくタイプもどく状態にすることができる。".to_string()
        },
        Ability {
            name: "ぜったいねむり".to_string(),
            description: "つねに夢うつつの状態で絶対に目覚めない。眠ったまま攻撃ができる。".to_string()
        },
        Ability {
            name: "じょおうのいげん".to_string(),
            description: "相手に威圧感をあたえこちらにむかって先制技を出せないようにする。".to_string()
        },
        Ability {
            name: "とびだすなかみ".to_string(),
            description: "相手に倒されたときＨＰの残りのぶんだけ相手にダメージをあたえる。".to_string()
        },
        Ability {
            name: "おどりこ".to_string(),
            description: "だれかが踊り技を使うと自分もそれに続いて踊り技を出すことができる。".to_string()
        },
        Ability {
            name: "バッテリー".to_string(),
            description: "味方の特殊技の威力を上げる。".to_string()
        },
        Ability {
            name: "もふもふ".to_string(),
            description: "相手から受けた接触する技のダメージを半減するがほのおタイプの技のダメージは２倍になる。".to_string()
        },
        Ability {
            name: "ビビッドボディ".to_string(),
            description: "相手をびっくりさせてこちらにむかって先制技を出せないようにする。".to_string()
        },
        Ability {
            name: "ソウルハート".to_string(),
            description: "ポケモンがひんしになるたびに特攻が上がる。".to_string()
        },
        Ability {
            name: "カーリーヘアー".to_string(),
            description: "攻撃で自分に触れた相手の素早さを下げる。".to_string()
        },
        Ability {
            name: "レシーバー".to_string(),
            description: "倒された味方の特性を受け継いで同じ特性になる。".to_string()
        },
        Ability {
            name: "かがくのちから".to_string(),
            description: "倒された味方の特性を受け継ぎ同じ特性に変わる。".to_string()
        },
        Ability {
            name: "ビーストブースト".to_string(),
            description: "相手を倒したとき自分のいちばん高い能力が上がる。".to_string()
        },
        Ability {
            name: "ＡＲシステム".to_string(),
            description: "持っているメモリで自分のタイプが変わる。".to_string()
        },
        Ability {
            name: "エレキメイカー".to_string(),
            description: "登場したときにエレキフィールドをはりめぐらせる。".to_string()
        },
        Ability {
            name: "サイコメイカー".to_string(),
            description: "登場したときにサイコフィールドをはりめぐらせる。".to_string()
        },
        Ability {
            name: "ミストメイカー".to_string(),
            description: "登場したときにミストフィールドをはりめぐらせる。".to_string()
        },
        Ability {
            name: "グラスメイカー".to_string(),
            description: "登場したときにグラスフィールドをはりめぐらせる。".to_string()
        },
        Ability {
            name: "メタルプロテクト".to_string(),
            description: "相手の技や特性で能力を下げられない。".to_string()
        },
        Ability {
            name: "ファントムガード".to_string(),
            description: "ＨＰが満タンのときに受けるダメージが少なくなる。".to_string()
        },
        Ability {
            name: "プリズムアーマー".to_string(),
            description: "効果バツグンになってしまう攻撃の威力を弱めることができる。".to_string()
        },
        Ability {
            name: "ブレインフォース".to_string(),
            description: "効果バツグンの攻撃で威力がさらに上がる。".to_string()
        },
        Ability {
            name: "ふとうのけん".to_string(),
            description: "最初に登場したとき攻撃が上がる。".to_string()
        },
        Ability {
            name: "ふくつのたて".to_string(),
            description: "最初に登場したとき防御が上がる。".to_string()
        },
        Ability {
            name: "リベロ".to_string(),
            description: "戦闘に出るたび１回だけ自分が出す技と同じタイプに変化する。".to_string()
        },
        Ability {
            name: "たまひろい".to_string(),
            description: "道具を持っていない場合１回目に投げて失敗したモンスターボールを拾ってくる。".to_string()
        },
        Ability {
            name: "わたげ".to_string(),
            description: "攻撃を受けるとわたげをばらまいて自分以外のポケモンすべての素早さを下げる。".to_string()
        },
        Ability {
            name: "スクリューおびれ".to_string(),
            description: "相手の技を引き受ける特性や技の影響を無視できる。".to_string()
        },
        Ability {
            name: "ミラーアーマー".to_string(),
            description: "自分が受けた能力ダウンの効果だけを跳ね返す。".to_string()
        },
        Ability {
            name: "うのミサイル".to_string(),
            description: "なみのりかダイビングをすると獲物をくわえてくる。ダメージを受けると獲物を吐きだして攻撃。".to_string()
        },
        Ability {
            name: "すじがねいり".to_string(),
            description: "相手の技を引き受ける特性や技の影響を無視できる。".to_string()
        },
        Ability {
            name: "じょうききかん".to_string(),
            description: "みずタイプほのおタイプの技を受けると素早さがぐぐーんと上がる。".to_string()
        },
        Ability {
            name: "パンクロック".to_string(),
            description: "音技の威力が上がる。受けた音技のダメージは半分になる。".to_string()
        },
        Ability {
            name: "すなはき".to_string(),
            description: "攻撃を受けると砂あらしを起こす。".to_string()
        },
        Ability {
            name: "こおりのりんぷん".to_string(),
            description: "こおりのりんぷんに守られて特殊攻撃で受けるダメージが半減する。".to_string()
        },
        Ability {
            name: "じゅくせい".to_string(),
            description: "熟成させることできのみの効果が倍になる。".to_string()
        },
        Ability {
            name: "アイスフェイス".to_string(),
            description: "物理攻撃は頭の氷がみがわりになるが姿も変わる。氷はゆきが降ると元に戻る。".to_string()
        },
        Ability {
            name: "パワースポット".to_string(),
            description: "隣にいるだけで技の威力が上がる。".to_string()
        },
        Ability {
            name: "ぎたい".to_string(),
            description: "フィールドの状態にあわせてポケモンのタイプが変わる。".to_string()
        },
        Ability {
            name: "バリアフリー".to_string(),
            description: "登場したときに敵と味方のひかりのかべリフレクターオーロラベールの効果が消える。".to_string()
        },
        Ability {
            name: "はがねのせいしん".to_string(),
            description: "味方のはがねタイプの攻撃の威力が上がる。".to_string()
        },
        Ability {
            name: "ほろびのボディ".to_string(),
            description: "接触する技を受けるとお互い３ターンたつとひんしになる。交代すると効果はなくなる。".to_string()
        },
        Ability {
            name: "さまようたましい".to_string(),
            description: "接触する技で攻撃してきたポケモンと特性を入れ替える。".to_string()
        },
        Ability {
            name: "ごりむちゅう".to_string(),
            description: "攻撃は上がるが最初に選んだ技しか出せなくなる。".to_string()
        },
        Ability {
            name: "かがくへんかガス".to_string(),
            description: "かがくへんかガスのポケモンが場にいるとすべてのポケモンの特性の効果が消えたり発動しなくなる。".to_string()
        },
        Ability {
            name: "パステルベール".to_string(),
            description: "自分も味方もどくの状態異常を受けなくなる。".to_string()
        },
        Ability {
            name: "はらぺこスイッチ".to_string(),
            description: "ターンの終わりにまんぷくもようはらぺこもようまんぷくもよう……と交互に姿を変える。".to_string()
        },
        Ability {
            name: "クイックドロウ".to_string(),
            description: "相手より先に行動できることがある。".to_string()
        },
        Ability {
            name: "ふかしのこぶし".to_string(),
            description: "相手に接触する技なら守りの効果を無視して攻撃することができる。".to_string()
        },
        Ability {
            name: "きみょうなくすり".to_string(),
            description: "登場したときに貝がらから薬を振りまいて味方の能力変化を元に戻す。".to_string()
        },
        Ability {
            name: "トランジスタ".to_string(),
            description: "でんきタイプの技の威力が上がる。".to_string()
        },
        Ability {
            name: "りゅうのあぎと".to_string(),
            description: "ドラゴンタイプの技の威力が上がる。".to_string()
        },
        Ability {
            name: "しろのいななき".to_string(),
            description: "相手を倒すと冷たい声でいなないて攻撃が上がる。".to_string()
        },
        Ability {
            name: "くろのいななき".to_string(),
            description: "相手を倒すと恐ろしい声でいなないて特攻が上がる。".to_string()
        },
        Ability {
            name: "じんばいったい".to_string(),
            description: "バドレックスのきんちょうかんとブリザポスのしろのいななきの二つの特性をあわせ持つ。".to_string()
        },
        Ability {
            name: "じんばいったい".to_string(),
            description: "バドレックスのきんちょうかんとレイスポスのくろのいななきの二つの特性をあわせ持つ。".to_string()
        }
      ]
}