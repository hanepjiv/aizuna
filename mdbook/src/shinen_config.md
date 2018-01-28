# ShinEn 設定

## 設定ファイル
[Aizuna 設定 ルートディレクトリ][Aizuna 設定 ルートディレクトリ]に存在する`config.toml`に設定を記述する必要があります。

```toml
[rules.shinen]
serdever    = 0
enable      = false
prefix      = ","
[rules.shinen.config]
serdever    = 0
root        = "SHINEN_ROOT_PATH"
```

##### [rules.{識別名}.config]

----
###### root =
ShinEnのファイルを保存するディレクトリへのパスです。

相対パスを指定した場合、Aizunaルートディレクトリからのパスとして認識されます。
絶対パスを指定することも可能です。

## カード
『深淵』はカードを使用しますので、そのデータが必要です。

### カードデータファイル
カードの情報は[ShinEnルートディレクトリ](#root-)に保存されたtoml形式のテキストファイルから読み込まれます。

```shell
{SHINEN_ROOT_PATH}
└── cards
    ├── basic.toml
    └── crimson.toml
```

##### {SHINEN_ROOT_PATH}/cards/basic.toml
基本のカード情報です。

##### {SHINEN_ROOT_PATH}/cards/crimson.toml
『血のごとく赤き 〜夢魔の占い札〜』のカード情報です

### tomlフォーマット

#### 例
```toml
["白の黒剣"]
serdever = 0
name = "白の黒剣"
card_set = "基本"
color = "白"
constellation = "黒剣"
value = 1
story = ["前兆", "出自"]
story_desc = "血が一滴、したたった。無明の闇から、今、何かが誕生し、世界に波紋をもたらしていく。"
action = ["攻撃", "防御"]
damage = "衝撃"
damage_desc = "頭がくらくらする。精神力-4。"
destiny = 1
```

```toml
["狂気"]
serdever = 0
name = "狂気"
card_set = "血のごとく赤き 〜夢魔の占い札〜"
value = 0
desc = "このカードには意味がない。そう意味はない。たぶん……"
story_desc = "いつか お前は私を求めるだろう"
destiny = 95
```

##### ["カード名"]
カード情報の記述を開始します。
##### serdever =
構造のバージョン番号です。現在は0が要求されます。
##### name =
カード名を示す文字列です。
##### card_set =
セットを示す文字列です。
##### color =
色を示す文字列です。
##### constellation =
星座を示す文字列です。
##### value =
カラーナンバーを示す数値です。
##### desc =
カードの文章を示す文字列です。
##### story =
語り部の種別を示す文字列の配列です。
##### story_desc =
語り部の文章を示す文字列です。
##### action =
行動の種別を示す文字列の配列です。
##### damage =
ダメージの種別を示す文字列です。
##### damage_desc =
ダメージの文章を示す文字列です。
##### destiny =
運命番号を示す数値です。


[Aizuna 設定 ルートディレクトリ]:./aizuna_config.html#aizuna-ルートディレクトリ
