# 📈 nine-stock

一個 CLI 工具，用嚟 call [nine-poe](https://github.com/nine-ai-2046-1/nine-poe) 做股票分析，save 結果落 disk，仲可以透過 [opencb](https://github.com/nine-ai-2046-1/opencb) 通知你 🔔

---

## 🚀 安裝

```bash
cargo install --path .
```

---

## 📋 參數一覽

| 參數         | 類型   | 必填 | 說明                                          |
| ------------ | ------ | ---- | --------------------------------------------- |
| `--session`  | String | ✅   | Session ID，用嚟 track 分析                   |
| `--code`     | String | ✅   | 股票代號（例如 `AAPL`、`TSLA`）                |
| `--ktype`    | String | ✅   | 分析類型（會做埋 model suffix）                |
| `--data`     | String | ✅   | 你想問嘅嘢 / prompt                           |
| `--debug`    | Flag   | ❌   | 執行之前 print 實際會跑嘅 CLI command          |

---

## 💡 用法示範

### 基本分析

```bash
nine-stock \
  --session "20260608-analysis" \
  --code "AAPL" \
  --ktype "short" \
  --data "分析AAPL短期走勢"
```

### 開 debug mode

```bash
nine-stock \
  --session "20260608-analysis" \
  --code "TSLA" \
  --ktype "mid" \
  --data "TSLA 中期前景點睇？" \
  --debug
```

### 同一個 session 入面做多次分析

```bash
# 第一次 call — 會新建 file
nine-stock --session "morning" --code "AAPL" --ktype "short" --data "開盤分析"

# 第二次 call — 會 append 落同一個 file
nine-stock --session "morning" --code "AAPL" --ktype "short" --data "更新走勢分析"
```

---

## 📂 檔案結構

分析結果會 save 到：

```
~/.opens/nine-stock/analysis/{code}/{session}/{ktype}.txt
```

例如：

```
~/.opens/nine-stock/analysis/AAPL/20260608-analysis/short.txt
```

每一筆分析會用 `\n#LINEBREAK#\n` 分隔，方便你之後 parse 📖

---

## ⚙️ 設定

Config file 喺：`~/.config/nine-stock/config.toml`

第一次 run 會自動帮你建，用預設值 👇

```toml
opencb = true
opencb-channel-id = "1513197866598928494"
```

| Key                  | 預設值                        | 說明                                      |
| -------------------- | ---------------------------- | ----------------------------------------- |
| `opencb`             | `true`                       | 開關 opencb 通知                          |
| `opencb-channel-id`  | `"1513197866598928494"`      | opencb send 嘅 channel ID                 |

Set `opencb = false` 就唔會 call opencb，靜靜鸡咁 🤫

---

## 🔄 執行流程

```
nine-stock --session X --code Y --ktype Z --data D
       │
       ▼
┌──────────────────┐
│  Call nine-poe   │
│  (subprocess)    │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐     ┌─────────────────┐
│  Save 落 file    │────▶│  Print 出 console│
│  ~/.opens/...    │     └────────┬────────┘
└──────────────────┘              │
                                  ▼
                        ┌──────────────────┐
                        │  睇 config       │
                        │  opencb 開咗未？  │
                        └────────┬─────────┘
                                 │ 開咗
                                 ▼
                        ┌──────────────────┐
                        │  Call opencb     │
                        │  send (subprocess)│
                        └──────────────────┘
```

---

## 🛠️ 必須要裝嘅 CLI

| CLI        | 說明                  | Link                                                    |
| ---------- | --------------------- | ------------------------------------------------------- |
| `nine-poe` | AI 股票分析           | [nine-ai-2046-1/nine-poe](https://github.com/nine-ai-2046-1/nine-poe) |
| `opencb`   | 通知 / 收發訊息       | [nine-ai-2046-1/opencb](https://github.com/nine-ai-2046-1/opencb)     |

### 可選 CLI

| CLI         | 說明                    | Link                                                      |
| ----------- | ---------------------- | --------------------------------------------------------- |
| `nine-futu` | 股票報價數據源          | [nine-ai-2046-1/nine-futu](https://github.com/nine-ai-2046-1/nine-futu) |

---

## 📝 小提醒

- `nine-poe` 嘅 error 會行 **stderr** — `nine-stock` 兩邊（stdout 同 stderr）都會 capture
- 如果 response 係空嘅，CLI 會靜靜鸡 exit，唔會 write file，唔會 call opencb
- File write 係 **append-only** — 多次 call 會累積喺同一個 file 入面
- `--debug` flag 會將 command print 去 **stderr**，唔会影响 stdout 嘅 output
