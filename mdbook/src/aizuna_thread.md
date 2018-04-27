# Aizuna スレッドとコルーチン
AizunaはデフォルトでOSのスレッドで動作します。

スレッドの代わりに、オプションで[libfringe][libfringe]によるコルーチンを使用して動作させることも可能です。

**コルーチンによる動作は実験的機能であり、スレッドによる動作が推奨されています。**

#### コルーチンによる動作
コルーチンで動作する場合、Aizunaの応答性が高くなることが期待できます。
その分、CPUリソース及び電力消費等が高くなることが予想されます。

手元の観測ではコルーチンによる動作によって、CPUコア1つを占有して消費し続けている様子が見られました。

#### 動作環境
コルーチン動作は下記のlibfringeの動作環境に依存します。**Windowsはサポートされていません**。

* bare metal
* Linux (any libc)
* FreeBSD
* DragonFly BSD
* macOS

#### 設定方法
##### コンパイル
デフォルトではコルーチンは有効化されていません。
コルーチンを有効化してAizunaをコンパイルする必要があります。

`--features=coroutine-fringe` を指定してください。

###### インストールする場合
```shell
cargo install --features=coroutine-fringe
```

###### インストール無しで実行する場合
```shell
cargo run --release --features=coroutine-fringe
```

##### 設定ファイル
Aizunaがlibfringeを利用してコルーチンで動作するように、設定ファイルを書き換えてください。

```
# aizuna v0.1.0
serdever    = 0
greeting    = "aizuna v0.1.0"
driver      = "Fringe"
fringe_stack_size   = 1048576
path_db     = "./db"
prefix      = ","
```

`driver = "Thread"` を `driver = "Fringe"` に書き換えます。

[Aizuna 設定 driver =][Aizuna 設定 設定ファイル driver =]と
[Aizuna 設定 fringe_stack_size =][Aizuna 設定 設定ファイル fringe_stack_size =]
を参照してください。

#### 実行
```shell
aizuna
```

##### Thread
```
Console::new: *** Caution! This is a DEBUG console. ***
Console: Spawn
```

##### Fringe
```
Console::new: *** Caution! This is a DEBUG console. ***
Console: Gen
```

`Spawn` の代わりに `Gen` が表示されれば `"Fringe"` による起動に成功しています。


{{#include link.md}}
