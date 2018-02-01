# Aizuna 設定
Aizunaの動作は設定ファイルによって制御されます。

## Aizuna 起動オプション
`aizuna --help`で起動オプションを確認できます。

```shell
$ aizuna --help

aizuna v0.1.0

Usage:
    aizuna [Options]

Options:
    -v, --version   print version
    -h, --help      print this help menu
    -R, --root PATH set Aizuna root path. default '${HOME}/.config/aizuna'
```

## Aizuna ルートディレクトリ
設定ファイルが保存されるディレクトリを差します。

デフォルトでは `{HOME}/.confog/aizuna/` です。

ルートディレクトリは、Aizuna起動オプション `--root / -R` で 変更できます。

```
aizuna --root ./
```

Aizunaの実行プロセスはルートディレクトリに対してファイルの作成権限を必要とします。

## 設定ファイル
ルートディレクトリに存在する`config.toml`という名前の、toml形式のテキストファイルです。

設定ファイルが存在しない場合、自動的に作成されます。


```toml
# -*- mode:toml; coding:utf-8-unix; -*-
# /////////////////////////////////////////////////////////////////////////////
# =============================================================================
# aizuna v0.1.0
serdever    = 0
greeting    = "aizuna v0.1.0"
driver      = "Thread"
fringe_stack_size   = 1048576
path_db     = "./db"
prefix      = ","
# /////////////////////////////////////////////////////////////////////////////
# =============================================================================
[connectors.console]
serdever    = 0
enable      = true
connector   = "console"
# =============================================================================
[connectors.discord-00]
serdever    = 0
enable      = false
connector   = "discord"
[connectors.discord-00.config]
serdever    = 0
token       = "DISCORD_BOT_TOKEN"
# /////////////////////////////////////////////////////////////////////////////
# =============================================================================
[rules.shinen]
serdever    = 0
enable      = false
prefix      = ","
[rules.shinen.config]
serdever    = 0
root        = "SHINEN_ROOT_PATH"
# /////////////////////////////////////////////////////////////////////////////
# =============================================================================
[admin]
console     = [".*"]
discord-00  = ["DISCORD_USER_ID_00", "DISCORD_USER_ID_01"]
```

### serdever =

設定ファイルには`serdever = number`という項目がいくつか存在します。

serdeverは"serialize desserialize version"の略で、
設定ファイルの構成を管理するバージョンを表します。

**`serdever`は自動的に決定されるので、Aizunaから指示がないかぎり書き換えてはいけません**。

### セクション
#### aizuna

----
##### greeting =
[,greeting][greeting]コマンドによって、Aizunaが表示する文章です。
自由に書き換えて構いません。

----
##### driver =
Aizunaの動作モードを下記の文字列で指定します。デフォルトは`"Thread"`です。

- "Thread": OSスレッドで動作します。
- "Fringe": libfringeを使用してコルーチンで動作します。

詳しくは[Aizuna スレッドとコルーチン][Aizuna スレッドとコルーチン]を参照してください。

----
##### fringe_stack_size =
`"Fringe"`モードで動作する場合のコルーチンのスタックサイズをバイト数で指定します。
デフォルトは`1048576`(= 1024 * 1024 bytes = 1MiB)です。

詳しくは[Aizuna スレッドとコルーチン][Aizuna スレッドとコルーチン]を参照してください。

----
##### path_db =
Aizunaの情報を保持するデータベースへのパスです。デフォルトは`"./db"`です。

相対パスを指定した場合、ルートディレクトリからのパスとして認識されます。
絶対パスを指定することも可能です。

データベースはAizunaによって管理されています。**データベースの内容を書き換えてはいけません**。

----
##### prefix =
[Aizuna コマンド][Aizuna コマンド]を識別するための接頭辞です。デフォルトは`","`です。

```toml
prefix = "aizuna-"
```

このように設定すれば

```
aizuna-help
aizuna-2d6
```

このようにコマンドを使用することが出来ます。

#### [connectors]

外部のチャットサービスと接続するためのコネクタの設定を行います。

コネクタには識別名を決定する必要があります。
自動生成された設定ファイルには`console`と`discord00`が設定されています。

```toml
[connectors.console]
serdever    = 0
enable      = true
connector   = "console"
```

```toml
[connectors.discord-00]
serdever    = 0
enable      = false
connector   = "discord"
[connectors.discord-00.config]
serdever    = 0
token       = "DISCORD_BOT_TOKEN"
```

`[connectors.{識別名}]`のように記述することでコネクタの設定を開始します。

識別名はユーザの判別に使用され、同一のコネクタ識別名を持つユーザ同士のみがセッションを共有できます。
**セッションを作成したコネクタの識別名を書き換えてしまうと、
セッションに接続できなくなってしまう場合があります**。
よってコネクタ識別名の変更は推奨されません。

----
##### enable =

コネクタを使用するかどうかのbool値です。`true`, `false`で指定します。

`false`を指定した場合、そのコネクタは使用されません。

----
##### connector =

コネクタの種別を文字列で指定します。

- `"console"`: デバックコンソールを使用します。
- `"discord"`: Discordを使用します。

> デフォルトでは[connectors.console]が有効、[connectors.discode00]が無効と設定されています。
>
> 実運用環境ではデバックコンソールを無効にし、その他のコネクタを有効化することを推奨します。
>
> Aizunaは全てのコネクタが終了するまで、プロセスを終了しません。[,quit][quit]を参照してください。

----
##### [connectors.{識別名}.config]

コネクタによっては追加の設定が必要となります。

###### "discord"

[Discord 設定][Discord 設定]を参照してください。

#### [rules]

Aizunaで使用するゲームのルールを設定します。

ルールは識別名で区別されます。
自動生成された設定ファイルには`shinen`が設定されています。

```toml
[rules.shinen]
serdever    = 0
enable      = false
prefix      = ","
[rules.shinen.config]
serdever    = 0
root        = "SHINEN_ROOT_PATH"
```

`[rules.{識別名}]`のように記述することでルールの設定を開始します。

----
##### enable =

ルールを使用するかどうかのbool値です。`true`, `false`で指定します。

`false`を指定した場合、そのルールは使用されません。

----
##### prefix =

コマンドでルールを識別する接頭辞を示す文字列です。

----
##### [rules.{識別名}.config]

ルールによっては追加の設定が必要となります。

###### "shinen"

[ShinEn 設定][ShinEn 設定]を参照してください。

#### [admin]

コネクタ毎にAizunaの管理者として扱われるユーザを設定します。

```
{connetor識別名} = ["{Rust正規表現}", "{Rust正規表現}"]
```

デフォルトではconsole接続が管理者として扱われます。

Discord接続はデフォルトでは無効化されています。
有効化する場合、管理者として扱うDiscordID(数字列)を指定してください

```toml
[admin]
console     = [".*"]
discord-00  = ["DISCORD_USER_ID_00", "DISCORD_USER_ID_01"]
```

管理者を設定していないコネクタは、[,quit][quit]を使用できません。
その場合、Aizunaプロセスを強制終了させなくてはいけないことに注意してください。
タイミングによってはデータベースに書き込みが完了していないデータが消失してしまう可能性があります。
**管理者権限を与えるユーザを必ず設定してください**。

[Aizuna コマンド]:./aizuna_command.html
[quit]:./aizuna_command.html#quit--q
[greeting]:./aizuna_command.html#greeting
[Aizuna スレッドとコルーチン]:./aizuna_thread.html
[Discord 設定]:./discord.html#設定
[ShinEn 設定]:./shinen_config.html
