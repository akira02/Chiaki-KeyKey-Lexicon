# Chiaki Web Overlay

This source contains reviewed unigram and bigram values derived outside the
repository. Source text is not redistributed here; only final lexicon rows are
kept for review and release builds:

```text
qstring<TAB>phrase<TAB>weight<TAB>tags
qstring<TAB>previous<TAB>current<TAB>probability
```

The release builder imports `explicit.tsv` after the project modern overlay and
before variant demotion policy. It imports `bigrams.tsv` into the runtime
`bigrams` table after unigram policy layers have been applied.

## Files

- `explicit.tsv`: reviewed explicit-qstring unigram additions.
- `bigrams.tsv`: reviewed bigram additions.

## License

The reviewed overlay rows are maintained by the ChiaKey Lexicon maintainers and
released as CC0-1.0.

## 中文補充（資料層）

- 資料層分類：專案詞庫。
- 選用理由：經人工審核的網路語料可補足現代用語，但應維持窄範圍、可追蹤、可回退。
- 在 release 的角色：匯入 reviewed explicit unigram 與 runtime bigram rows，不保存原始語料。
