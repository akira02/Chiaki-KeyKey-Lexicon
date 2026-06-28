# ChiaKey Supplemental Symbol List

This source contains project-owned supplemental symbols for the Smart Mandarin
punctuation list.

Yahoo KeyKey's original `bpmf-punctuations.cin` remains the compatibility base.
This overlay only appends missing symbols to `_punctuation_list`; it does not
change direct punctuation key mappings such as `_punctuation_<`.

`symbols.tsv` format:

```text
symbol<TAB>tags
```

The release builder writes every accepted symbol as:

```text
_punctuation_list<TAB>symbol
```

Existing Yahoo punctuation-list symbols are skipped so their ordering is not
changed.

License: CC0-1.0.

## 中文補充（資料層）

- 資料層分類：專案詞庫。
- 選用理由：Yahoo 原始符號列表偏舊，缺少現代常用的貨幣、數學、圈號數字、勾叉與音樂符號。
- 在 release 的角色：只補 `_punctuation_list` 缺漏符號，且不改動既有直接按鍵標點映射。
