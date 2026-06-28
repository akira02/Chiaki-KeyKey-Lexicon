# libchewing-data

Source id: `libchewing-data`

This source imports the New Chewing built-in dictionary data as the main modern Traditional Chinese / Zhuyin lexicon layer.

Pinned upstream files:

- `dict/chewing/tsi.csv` from `chewing/libchewing-data` tag `v2026.3.22`
- `dict/chewing/word.csv` from `chewing/libchewing-data` tag `v2026.3.22`
- `dict/chewing/alt.csv` from `chewing/libchewing-data` tag `v2026.3.22`

Local raw files are vendored under `sources/libchewing-data/raw/` so normal
release builds do not need network access. Maintainers refresh pinned snapshots
with:

```sh
cargo run --release -- fetch-modern-sources
```

The raw files are tracked in git. Their checksums are recorded in
`source-inventory.sha256`.

License: LGPL-2.1-or-later, based on the source file `dc:license` headers and libchewing project license.

## 中文補充（資料層）

- 資料層分類：外部詞庫。
- 選用理由：libchewing-data 為持續維護的繁中注音資料，含明確讀音，可靠度高於僅靠舊 bootstrap 推導。
- 在 release 的角色：作為主要現代詞庫層；`tsi.csv`、`alt.csv` 提供詞與替代讀音，`word.csv` 補單字讀音與排序訊號。
