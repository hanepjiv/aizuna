# Aizuna イントロダクション

## 実行ファイル の取得

### ソースからコンパイルする

#### [Rust][Rust] nightly 環境の準備
[Rust][Rust]を参照して開発環境(`rustup`, `cargo`)を準備してください。

nightly toolchaine をインストールします。

```shell
rustup install nightly
```

##### GitHub からインストール
GitHubの[repository][repository]から直接インストールします。

```shell
cargo install --git https://github.com/hanepjiv/aizuna.git
```

##### ソースコードの入手
GitHubの[repository][repository]からソースコードを入手します。

```shell
git clone https://github.com/hanepjiv/aizuna.git
cd asizuna
```

###### インストール
nightlyでコンパイルすることを指示し、`cargo`コマンドでインストールします。

```shell
rustup override set nightly
cargo install
aizuna
```

###### インストールをしないで実行
`cargo run`コマンドでインストールせずに実行することもできます。

```shell
rustup override set nightly
cargo run --release
```


## 初回起動

#### 設定ファイルの新規作成
初回起動で設定ファイルが無い場合、設定ファイルを新規作成するかを問われます。

```
create config file? "${HOME}/.config/aizuna/config.toml" [Y/n]:
```

> --root オプションを付与して実行すると設定ファイルを参照するディレクトリを指定できます。
> デフォルトでは ${HOME}/.confog/aizuna/ を参照します。
> ```shell
> aizuna --root ./
> ```
>
> `cargo run`で起動する場合は次のように指定します。
> ```shell
> cargo run --release -- --root ./
> ```

設定ファイルを新規作成すると、デバッグコンソールのみに接続した状態で起動します。

```
Console::new: *** Caution! This is a DEBUG console. ***
Console: Spawn
```

詳しくは、[Aizuna 設定][Aizuna 設定]を参照してください。

## 基本コマンド
Aizunaは接頭辞(prefix: デフォルトでは",")から始まる文字列をコマンドとして認識します。

#### ,help
先ずは ",help" コマンドを入力してください。

```
,help
Console: Whisper:
Aizuna v0.1.0:
 ,help      Print this message.
 ,user/ ,u  Print User info.
 ,session / ,s  Session controll.
 ,[0-9]*d[0-9]* Dice roll. etc. ,2d6 / ,3d / ,d10
 ,quit / ,Q Aizuna logout server. Need administrator's authority.
 ,database  Dump database. Need administrator's authority.
```

入力と出力にプロンプト(" >" のような入力を促す表示)による区別がないことに注意してください。

#### ,d
",d" コマンドでサイコロを振ることが出来ます。


```
,d
Console: Send: 1d6 = [4] = 4.
```

```
,2d6
Console: Send: 2d6 = [3, 4] = 7.
```

#### ,quit
",quit" コマンド で Aizunaを終了出来ます。

```
,quit
Console: Exit
```

その他のコマンドの詳説は[Aizuna コマンド][Aizuna コマンド]を参照してください。

[repository]:https://github.com/hanepjiv/aizuna.git
[Rust]:https://www.rust-lang.org
[Aizuna 用語]:./aizuna_term.html
[Aizuna セッション]:./aizuna_term.html#セッション
[Aizuna ユーザ]:./aizuna_term.html#ユーザ
[Aizuna 設定]:./aizuna_config.html
[Aizuna 設定 - コネクタ]:./aizuna_config.html#コネクタ
[Aizuna コマンド]:./aizuna_command.html
