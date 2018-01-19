# ShinEn コマンド

Aizunaの基本的な[Aizuna コマンド][Aizuna コマンド]の他に、
ShinEnを管理するためのコマンドが定義されています。

ShinEnは[Aizuna コマンド][Aizuna コマンド]の接頭辞(prefix: デフォルトでは",")に続いて、
ShinEnコマンドの接頭辞(prefix: デフォルトでは",")から始まる文字列をコマンドとして認識します。

接頭辞を変更するには[Aizuna 設定][Aizuna 設定]及び
[ShinEn 設定][ShinEn 設定]を参照してください。

## コマンド一覧

----
### ,,session / ,,s
現在のセッションの深淵セッションの情報を表示します。

```
,,session
,, s
```

* 場札の枚数
    * 山札の枚数
    * 捨て札の枚数
* プレイヤーの情報

----
### ,,reload
カードデータの再読み込みを行います。

**Aizunaの管理者権限が必要です**。

```
,,reload
```

Aizuna内の全てのセッションに影響します。

----
### ,,deck
現在のセッションの深淵セッションの場札の情報を表示します。

```
,,deck
```

* 場札の情報
    * 山札の枚数
    * 捨て札の内容

----
### ,,shufle
山札をシャッフルします。

```
,,shuffle
```

----
### ,,tsukimachi
月待ちの処理を行います。

```
,,tsukimachi
```

----
### ,,card
カード情報を表示します

```
,,card {CARD_TEXT}
```

##### {CARD_TEXT}
カード名示す文字列を指定します。
**空白を含むことはできません**。

----
### ,,player / ,,p
プレイヤーを管理します。

```
,,player {SUB_COMMAND} {ARGS..}
,,p {SUB_COMMAND} {ARGS..}

,,player
,,p
```

サブコマンドを指定しなかった場合、現在のセッション内のプレイヤー一覧を表示します。
[,,player list](#player-list)を参照してください。

##### {SUB_COMMAND}

###### [default](#player-default)
現在のプレイヤーを変更します。
###### [list](#player-list)
現在のセッション内のプレイヤー一覧を表示します。
###### [new](#player-new)
プレイヤーの新規作成。
###### [delete](#player-delete)
現在のプレイヤーの削除。
###### [name](#player-name)
現在のプレイヤーの名前変更。
###### [type](#player-type)
現在のプレイヤーのタイプ変更。
###### [assign](#player-assign)
指定プレイヤーの操作ユーザ変更。

----
#### ,,player default
現在のプレイヤーを変更します。

```
,,player default {PLAYER_UUID}
,,p default {PLAYER_UUID}
```

##### {PLAYER_TYPE}
現在のセッション内で自分が所持しているプレイヤーのUuid。

----
#### ,,player list
現在のセッション内のプレイヤー一覧を表示します。

```
,,player list
,,p list
```

```
ShinEn.player.list
 * o 4 name00 (xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx)
     6 name01 (xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx)
   o 2 name02 (xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx)
     2 name03 (xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx)
```

###### *
自分の現在のプレイヤーに表示されます。
###### o
自分が所持しているプレイヤーに表示されます。
###### 数字
プレイヤーの手札の枚数。
###### 文字列
プレイヤーの名前。
###### xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
プレイヤーのUuid。

----
#### ,,player new

プレイヤーの新規作成。

**新規作成したプレイヤーは自動的に現在のプレイヤーとなります。**

```
,,player new {PLAYER_TYPE}
,,p new {PLAYER_TYPE}
```

##### {PLAYER_TYPE}
Playerの種別を下記の文字列で指定します。

- Player
    - player
    - Player
- GameMaster
    - gm
    - GM
    - gamemaster
    - GameMaster
    - master
    - Master

*今のところ、 {PLAYER_TYPE} は表示以外の意味を持ちません。
Player / GameMaster の違いで機能が制限される等の相違はありません。*

----
#### ,,player delete
現在のプレイヤーの削除。

```
,,player delete
,,p delete
```

現在のプレイヤーのデータは削除されますが、
**現在のプレイヤーは自動的に変更されません**。
[,,player default](#player-default) によって明示的に変更する必要があります。

----
#### ,,player name
現在のプレイヤーの名前変更。

```
,,player name {NAME}
,,p name {NAME}
```

##### {NAME}
名前を示す文字列です。
**空白を含むことはできません**。

----
#### ,,player type
現在のプレイヤーのタイプ変更。

```
,,player type {PLAYER_TYPE}
,,p type {PLAYER_TYPE}
```

##### {PLAYER_TYPE}

[,,player new](#player-new)を参照してください。

----
#### ,,player assign
指定プレイヤーのユーザー変更。

**セッションの所有者権限が必要です**。

```
,,player assign {PLAYER_UUID} {USER_UUID}
,,p assign {PLAYER_UUID} {USER_UUID}
```

##### {PLAYER_UUID}
現在のセッションのプレイヤーのUuid。

##### {USER_UUID}
現在のセッションのメンバーユーザーのUuid。

----
### ,,hand / ,,h
現在のプレイヤーの手札を表示します。

```
,,hand
,h
```

```
3 cards
========
   0. 黒の古鏡 / 基本 / 色： 黒 / 星座： 古鏡 / 数値: 2 / 叙述:  / 語り部: ["英知", "追想"] 汝のなすことはすべて汝に返る。それを忘れるな。 / アクション: ["攻撃", "魔法"] / ダメージ: 演出 相手の反撃が命中。攻撃側に効果値2のダメージ!(精神力-1)。 / 運命: 37
========
   1. 青の野槌 / 基本 / 色： 青 / 星座： 野槌 / 数値: 5 / 叙述:  / 語り部: ["英知", "主張"] 同じことが良いこととは限らない。自由こそよし。 / アクション: ["回避", "移動"] / ダメージ: 胴体 わき腹に当たり、激痛が顔を歪ませる。生命力-6、手札-1。 / 運命: 19
========
   2. 赤の黒剣 / 基本 / 色： 赤 / 星座： 黒剣 / 数値: 4 / 叙述:  / 語り部: ["前兆", "出自"] 波乱の予感がする。新たな何かが生まれ、世界に飛び立っていく。 / アクション: ["攻撃", "防御"] / ダメージ: 頭部 頭に当たる。頭蓋骨に損傷。生命力-10。 / 運命: 53
```

一部のコマンドでは行頭の数字でカードを指定することができます。

----
### ,,pick
場札(山札と捨て札)からカードを取得し、現在のプレイヤーの手札に加えます。

```
,,pick {CARD_TEXT}
```

##### {CARD_TEXT}
取得するカードの名前を示す文字列です。
**空白を含むことはできません**。

----
### ,,draw / ,,d
山札からカードを引き現在のプレイヤーの手札に加えます。

```
,,draw {NUMBER}
,,d {NUMBER}
```

##### {NUMBER}
手札に加えるカードの枚数を指定する数字です。

省略した場合、 1 が指定されたものとして動作します。

----
### ,,top
山札のカードの一番上を公開し捨て札に置きます。

```
,,top {NUMBER}
```
##### {NUMBER}
山札から公開するカードの枚数を指定する数字です。

省略した場合、 1 が指定されたものとして動作します。

----
### ,,totop
現在のプレイヤーの手札のカードを公開し山札の一番上に置きます。

```
,,totop {HAND_NUMBER}
```

##### {HAND_NUMBER}
手札のカードを指定する数字です。

[,,hand](#hand--h)を参照してください。

----
### ,,use / ,u
現在のプレイヤーの手札のカードを公開し捨て札に置きます。

```
,,use {HAND_NUMBER}
,,u {HAND_NUMBER}
```

##### {HAND_NUMBER}
手札のカードを指定する数字です。

[,,hand](#hand--h)を参照してください。

----
### ,,give
現在のプレイヤーの手札のカードを現在のセッションのプレイヤーに与えます。

```
,,give {HAND_NUMBER} {PLAYER_UUID}
```

##### {HAND_NUMBER}
手札のカードを指定する数字です。

[,,hand](#hand--h)を参照してください。

##### {PLAYER_UUID}
カードを与える現在のセッションのプレイヤーを指定するUuidです。

[,,player list](#player-list)を参照してください。

[Aizuna 設定]:./aizuna_config.html
[Aizuna コマンド]:./aizuna_command.html
[ShinEn 設定]:./shinen_config.html
