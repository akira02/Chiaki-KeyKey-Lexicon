# Rime essay

Source id: `rime-essay`

This source imports Rime's shared vocabulary and language model as a low-priority supplemental phrase layer. Because `essay.txt` does not include Zhuyin readings, the release builder only imports entries whose readings can be inferred from the current database's single-character readings.

Pinned upstream file:

- `essay.txt` from `rime/rime-essay` commit `48c7538f0b760fcc8c9d6bf08711f82cfbd2e9ed`

Local raw files are vendored under `sources/rime-essay/raw/` so normal release
builds do not need network access. Maintainers refresh pinned snapshots with:

```sh
cargo run --release -- fetch-modern-sources
```

The raw file is tracked in git. Its checksum is recorded in
`source-inventory.sha256`.

License: LGPL-3.0.

## 中文補充（資料層）

- 資料層分類：外部詞庫。
- 選用理由：Rime essay 提供廣泛詞彙與語言模型分數，但缺少注音讀音，適合當補充與排序證據層。
- 在 release 的角色：對既有弱詞做有限度 rerank，並僅在可安全推得讀音時匯入補充詞。
