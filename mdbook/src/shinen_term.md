# ShinEn 用語

Aizuna ShinEnで使用される用語を解説します。

その他の用語については、[Aizuna 用語][Aizuna 用語]を参照してください。

#### ShinEnセッション
ShinEnセッションとは、下記の情報を保持する構造を差します。

* Uuid
* 場札
    * 山札
    * 捨て札
* プレイヤー(複数)
* 参加ユーザの現在のプレイヤー

項目の一部は[ShinEn コマンド][ShinEn コマンド]によって、変更できます。

ShinEn セッションは[Aizuna セッション][Aizuna セッション]の構造の一部として扱われます。
よって所有者やメンバーやタイトル等を
[Aizuna セッション][Aizuna セッション]の機能によって変更できます。


#### プレイヤー
プレイヤーとは下記の情報を保持する構造を差します。

* Uuid
* 所有ユーザUuid
* プレイヤー名
* 手札

項目の一部は[ShinEn コマンド][ShinEn コマンド]によって、変更できます。

[ShinEn セッション][ShinEn セッション]は複数のプレイヤーを保持します。

[Aizuna ユーザ][Aizuna ユーザ]はセッション内で複数のプレイヤーを所持することができます。

##### ユーザの現在のプレイヤー
ユーザの現在のプレイヤーとは、一部のコマンドにおいて、暗黙の内に操作対象となるプレイヤーを差します。

[,,player default][player-default] によって変更できます。

ユーザの現在のプレイヤーは [,,player list][player-list] で __*__ で表示されます。

##### ユーザが所持しているプレイヤー
ユーザが所持しているプレイヤーとは現在のセッション内でユーザが変更の権限を所有しているプレイヤーを差します。

ユーザが所持しているプレイヤーは [,,player list][player-list] で __o__ で表示されます。

[Aizuna 用語]:./aizuna_term.html
[Aizuna セッション]:./aizuna_term.html#セッション
[Aizuna ユーザ]:./aizuna_term.html#ユーザ
[Aizuna コマンド]:./aizuna_command.html
[ShinEn セッション]:./shinen_term.html#shinenセッション
[ShinEn コマンド]:./shinen_command.html
[player-default]:./shinen_command.html#player-default
[player-list]:./shinen_command.html#player-list
[player-default]:./shinen_command.html#player-default
