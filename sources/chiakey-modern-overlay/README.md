# ChiaKey Modern Overlay

This source contains small project-owned overrides used to fix obvious misses in
the seed lexicon before larger public frequency corpora are integrated.

`phrases.tsv` format:

```text
phrase<TAB>weight<TAB>tags
```

The release script infers each phrase reading from single-character readings in the bootstrap KeyKey database, then inserts or replaces the unigram with the supplied weight.

`explicit.tsv` format:

```text
qstring<TAB>phrase<TAB>weight<TAB>tags
```

Use `explicit.tsv` when the fix depends on a specific reading, tone, or KeyKey
internal qstring. These rows replace only the exact qstring/phrase pair.

License: CC0-1.0.

## 中文補充（資料層）

- 資料層分類：專案詞庫。
- 選用理由：真實打字會出現少量需要立即修正的缺漏或排序問題，這些不應等待大型外部來源更新。
- 在 release 的角色：提供專案自有詞、明確 qstring 修正與精準排序調整。
