# 📈 nine-stock

A CLI tool that calls [nine-poe](https://github.com/nine-ai-2046-1/nine-poe) for stock analysis, saves results to disk, and optionally notifies via [opencb](https://github.com/nine-ai-2046-1/opencb).

---

## 🚀 Installation

```bash
cargo install --path .
```

---

## 📋 Arguments

| Argument     | Type   | Required | Description                                      |
| ------------ | ------ | -------- | ------------------------------------------------ |
| `--session`  | String | ✅       | Session identifier for tracking                  |
| `--code`     | String | ✅       | Stock code (e.g. `AAPL`, `TSLA`)                 |
| `--ktype`    | String | ✅       | Analysis type (used as model suffix)             |
| `--data`     | String | ✅       | Prompt / question to send to nine-poe            |
| `--debug`    | Flag   | ❌       | Print the exact CLI commands before execution    |

---

## 💡 Usage Examples

### Basic analysis

```bash
nine-stock \
  --session "20260608-analysis" \
  --code "AAPL" \
  --ktype "short" \
  --data "分析AAPL短期走勢"
```

### With debug mode

```bash
nine-stock \
  --session "20260608-analysis" \
  --code "TSLA" \
  --ktype "mid" \
  --data "What is the mid-term outlook for TSLA?" \
  --debug
```

### Multiple analyses in one session

```bash
# First call — creates file
nine-stock --session "morning" --code "AAPL" --ktype "short" --data "開盤分析"

# Second call — appends to same file
nine-stock --session "morning" --code "AAPL" --ktype "short" --data "更新走勢分析"
```

---

## 📂 Output Structure

Results are saved to:

```
~/.opens/nine-stock/analysis/{code}/{session}/{ktype}.txt
```

Example:

```
~/.opens/nine-stock/analysis/AAPL/20260608-analysis/short.txt
```

Each entry is separated by `\n#LINEBREAK#\n` for easy parsing.

---

## ⚙️ Configuration

Config file: `~/.config/nine-stock/config.toml`

Created automatically on first run with default values.

```toml
opencb = true
opencb-channel-id = "1513197866598928494"
```

| Key                  | Default                        | Description                              |
| -------------------- | ------------------------------ | ---------------------------------------- |
| `opencb`             | `true`                         | Enable/disable opencb notification       |
| `opencb-channel-id`  | `"1513197866598928494"`       | Channel ID for opencb send               |

Set `opencb = false` to disable the second CLI call silently.

---

## 🔄 Execution Flow

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
│  Save to file    │────▶│  Print to stdout │
│  ~/.opens/...    │     └────────┬────────┘
└──────────────────┘              │
                                  ▼
                        ┌──────────────────┐
                        │  Check config    │
                        │  opencb enabled? │
                        └────────┬─────────┘
                                 │ yes
                                 ▼
                        ┌──────────────────┐
                        │  Call opencb     │
                        │  send (subprocess)│
                        └──────────────────┘
```

---

## 🛠️ Required CLIs

| CLI        | Description                  | Link                                                    |
| ---------- | ---------------------------- | ------------------------------------------------------- |
| `nine-poe` | AI-powered stock analysis    | [nine-ai-2046-1/nine-poe](https://github.com/nine-ai-2046-1/nine-poe) |
| `opencb`   | Notification / messaging     | [nine-ai-2046-1/opencb](https://github.com/nine-ai-2046-1/opencb)     |

### Optional CLI

| CLI         | Description                    | Link                                                      |
| ----------- | ------------------------------ | --------------------------------------------------------- |
| `nine-futu` | Stock price data source        | [nine-ai-2046-1/nine-futu](https://github.com/nine-ai-2046-1/nine-futu) |

---

## 📝 Notes

- `nine-poe` returns errors on **stderr** — `nine-stock` captures both stdout and stderr
- If the response is empty, the CLI exits silently (no file write, no opencb call)
- File writes are **append-only** — multiple calls accumulate in the same file
- The `--debug` flag prints commands to **stderr** before execution
