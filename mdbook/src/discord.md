# Discord

## 設定ファイル

ルートフォルダに存在する`config.toml`に設定を記述する必要があります。


詳説は[Aizuna 設定 connectors][Aizuna 設定 connectors]を参照してください。

```
[connectors.discord-00]
serdever    = 0
enable      = false
connector   = "discord"
[connectors.discord-00.config]
serdever    = 0
token       = "DISCORD_BOT_TOKEN"
```

#### [connector.{識別名}.config]

----
##### token =
DiscordのBOT Tokenを文字列で指定します。

#### [admin]

**[Aizuna 設定 admin][Aizuna 設定 admin]を参考に管理者権限を与えるユーザを必ず設定してください**。


{{#include link.md}}
