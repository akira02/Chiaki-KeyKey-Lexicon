# Mozc Emoticon Data

Source id: `mozc-emoticon-data`

This source vendors the Mozc emoticon data used to populate ChiaKey's
preloaded `顏文字` canned-message category.

Vendored files:

- `sources/mozc-emoticon-data/raw/categorized.tsv`
- `sources/mozc-emoticon-data/raw/emoticon.tsv`

Upstream:

- Repository: <https://github.com/google/mozc>
- Commit: `28da5a39f9a7fd70251c85d269f4a8b47aa31cf8`
- Source directory: `src/data/emoticon/`

License:

- BSD-3-Clause, stored at `LICENSES/mozc-BSD-3-Clause.txt`

The release builder reads `categorized.tsv` first, then appends additional
unique emoticons from `emoticon.tsv`. Only the emoticon value column is shipped
in `prepopulated_service_data/canned_messages`; Japanese reading keys and
descriptions are retained only as source context and are not displayed in the
symbol table.

Verify vendored files with:

```sh
cd sources/mozc-emoticon-data
shasum -a 256 -c source-inventory.sha256
```

## 中文補充（資料層）

- 資料層分類：相容性基底詞庫。
- 選用理由：Mozc 提供乾淨、可再散布的 IME 顏文字資料，可替換舊資料中帶註解與雜訊的內容。
- 在 release 的角色：提供 `顏文字` 預載分類來源；最終只輸出顏文字本體，不輸出日文讀音與說明欄位。
