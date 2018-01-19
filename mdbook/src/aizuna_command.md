# Aizuna コマンド

Aizunaの機能はチャットによる入力をコマンドとして解釈することで動作します。

Aizunaは接頭辞(prefix: デフォルトでは",")から始まる文字列をコマンドとして認識します。

接頭辞を変更するには[Aizuna 設定 prefix][Aizuna 設定 prefix]を参照してください。


----
#### ,help

ヘルプメッセージを表示します。

```
,help
```

```
Aizuna v0.1.0:
 ,help              Print this message.
 ,user / ,u         Print User info.
 ,session / ,s      Session controll.
 ,[0-9]*d[0-9]*     Dice roll. etc. ,2d6 / ,3d / ,d10
 ,quit / ,Q         Aizuna logout server. Need administrator's authority.
```

----
#### ,quit / ,Q

Aizunaの接続を終了します。

**Aizunaの管理者権限が必要です**。

```
,quit
,Q
```

**コマンドを送ったコネクタの接続のみを終了します**。
デバッグコンソールとDiscordを同時に起動しているような場合、
その両方が`,quit`コマンドで終了されるまでAizunaは終了されません。

----
#### ,greeting

設定ファイルに記述された任意の文章を表示します。

```
,greeting
```

```
Console: Whisper: aizuna v0.1.0
```

文章を変更するには[Aizuna 設定 greeting][Aizuna 設定 greeting]を参照してください。

----
#### ,d

サイコロを振り結果を得ます。

```
,d
Console: Send: 1d6 = [1] = 1.
```

```
,2d6
Console: Send: 2d6 = [5, 4] = 9.
```

正規表現で `,([0-9]*)d([0-9]*)` のパターンを受け付け、
maches[1] をサイコロの数 n 、 maches[2] をサイコロの種類 m として認識します。

n が省略された場合、n = 1、m が省略された場合、m = 6 になります。

n の最大値は 99 に制限されます。

----
#### ,user ,u

ユーザ情報を表示します。

```
,user
,u
```

```
Console: Send: user: Uuid: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
```

----
#### ,session / ,s

Aizuna セッションを管理します。

```
,session
,s
```

{SUB_COMMAND}を省略した場合、`,session list`が指定されたように動作します。

```
,session {SUB_COMMAND} {ARGS..}
,s {SUB_COMMAND} {ARGS..}
```

##### {SUB_COMMAND}

###### [info](#session-info)
現在のセッションの情報を表示します。
###### [list](#session-list)
セッションの一覧を表示します。
###### [all](#session-all)
閉じたセッションを含むセッションの一覧を表示します。
###### [default](#session-default)
現在のセッションを設定します。
###### [new](#session-new)
セッションを新規作成します。
###### [close](#session-close)
現在のセッションを閉じます。
###### [reopen](#session-reopen)
閉じられた現在のセッションを再開します。
###### [delete](#session-delete)
閉じられた現在のセッションを消去します。
###### [title](#session-title)
現在のセッションのタイトルを変更します。
###### [owner](#session-owner)
現在のセッションのメンバーにセッションの所有者権限を与えます。
###### [waiver](#session-waiver)
現在のセッションの所有者権限を放棄します。
###### [invite](#session-invite)
現在のセッションにメンバーを追加します。
###### [kick](#session-kick)
現在のセッションからメンバーを除外します。
###### [request](#session-request)
セッションの所持者に参加要求を送ります。
###### [bye](#session-bye)
現在のセッションから脱退します。

----
#### ,session info
現在のセッションの情報を表示します。

```
,sesssion info
,s info
```

----
#### ,session list
自分が参加しているセッションの一覧を表示します。

```
,sesssion list
,s list
```

表示の見方については[,session all](#session-all)を参照してください。

----
#### ,session all
閉じたセッションを含む自分が参加しているセッションの一覧を表示します。

```
,sesssion all
,s all
```

```
 * o   title0  shinen  2018-01-01 12:00 +09:00  xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
   o x title1  shinen  2018-01-01 12:00 +09:00  xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
   o   title2  shinen  2018-01-01 12:00 +09:00  xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
```

###### *
自分の現在のセッションに表示されます。
###### o
自分が所持しているセッションに表示されます。
###### x
自分が所持している閉じたセッションに表示されます。
###### 文字列
セッションのタイトル。
###### UTC時刻
セッションの作成日時。
###### xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
セッションのUuid。

----
#### ,session default
現在のセッションを設定します。

```
,sesssion default {SESSION_UUID}
,s default {SESSION_UUID}
```

##### {SESSION_UUID}
現在のセッションに設定するセッションのUuidです。

----
#### ,session new
セッションを新規作成します。

```
,sesssion new {RULE_TEXT}
,s new
```

##### {RULE_TEXT}
セッションに使用するゲームのルールを指定する文字列です。

- shinen

----
#### ,session close
現在のセッションを閉じます。

**セッションの所有者権限が必要です**。

```
,sesssion close
,s close
```

----
#### ,session reopen
閉じられた現在のセッションを再開します。

**セッションの所有者権限が必要です**。

```
,sesssion reopen
,s reopen
```

----
#### ,session delete
閉じられた現在のセッションを消去します。

**セッションの所有者権限が必要です**。

```
,sesssion delete
,s delete
```

----
#### ,session title
現在のセッションのタイトルを変更します。

**セッションの所有者権限が必要です**。

閉じたセッションのタイトルは変更できません。
[,reopen](#session-reopen) で再開する必要があります。

```
,sesssion title {TITLE_TEXT}
,s title {TITLE_TEXT}
```

##### {TITLE_TEXT}
現在のセッションのタイトルを指定する文字列です。
**空白を含むことはできません**。

----
#### ,session owner
現在のセッションのメンバーにセッションの所有者権限を与えます。

```
,sesssion owner {USER_UUID}
,s owner {USER_UUID}
```

##### {USER_UUID}
セッションの所有者権限を与える現在のセッションのユーザを指定するUuidです。

----
#### ,session waiver
現在のセッションの所有者権限を放棄します。

```
,sesssion waiver
,s waiver
```

----
#### ,session invite
現在のセッションにメンバーを追加します。

```
,sesssion invite {USER_UUID}
,s invite
```

##### {USER_UUID}
現在のセッションに追加するユーザを指定するUuidです。

----
#### ,session kick
現在のセッションからメンバーを除外します。

```
,sesssion kick {USER_UUID}
,s kick {USER_UUID}
```
##### {USER_UUID}
現在のセッションから除外するユーザを指定するUuidです。

----
#### ,session request
セッションの所持者に参加要求を送ります。

```
,sesssion request {SESSION_UUID}
,s request {SESSION_UUID}
```

##### {SESSION_UUID}
参加要求を送るセッションを指定するUuidです。

----
#### ,session bye
現在のセッションから脱退します。

```
,sesssion bye
,s bye
```

セッションの所有者が再び[,session invite](#session-invite)を行なえば
セッションに再度参加できます。

[Aizuna 設定]:./aizuna_config.html
[Aizuna 設定 greeting]:./aizuna_config.html#greeting-
[Aizuna 設定 prefix]:./aizuna_config.html#prefix-
