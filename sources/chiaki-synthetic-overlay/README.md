# Chiaki.C Synthetic Taiwan Internet Usage Overlay

This source contains reviewed unigram additions with explicit qstrings and
synthetic bigram language-model rows maintained by Chiaki.C.

The raw synthetic corpus is not redistributed here; only reviewed lexicon rows
are kept for release builds:

```text
qstring<TAB>phrase<TAB>weight<TAB>tags
qstring<TAB>previous<TAB>current<TAB>probability
```

The release builder imports `unigrams.tsv` before variant demotion policy and
imports `bigrams.tsv` before reviewed web bigrams. Sentence-boundary bigram rows
may leave either `previous` or `current` empty.

## Bigram calibration

The raw synthetic bigram scores are conditional log-probabilities, but the final
ChiaKey unigram table uses its own lifted weight scale. During release import,
the builder re-anchors this source's bigrams to the current unigram floor:

```text
stored = min(unigram(current) + boost + (raw - raw_max_of_source), -0.05)
```

This keeps the source's internal ordering while letting strong disambiguation
edges beat the unigram path. Weaker pairs stay below the unigram floor and remain
inert. The default `boost` is `1.5` and can be overridden with
`SYNTHETIC_BIGRAM_BOOST`; setting it to `0` leaves the raw values unchanged.

## Files

- `unigrams.tsv`: reviewed unigram additions with explicit qstrings.
- `bigrams.tsv`: synthetic bigram probabilities, including sentence-boundary
  rows.

## License

This source is licensed under CC BY-NC 4.0 by Chiaki.C. Non-commercial and
open-source projects may use this source with attribution to Chiaki.C.
Commercial use requires separate permission from Chiaki.C.

See [LICENSES/chiaki-synthetic-overlay-NC.txt](../../LICENSES/chiaki-synthetic-overlay-NC.txt).

## 中文補充（資料層）

- 資料層分類：專案詞庫。
- 選用理由：補充 Chiaki.C 維護的 synthetic 台灣網路語用資料，覆蓋一般公開詞庫較弱的使用場景。
- 在 release 的角色：匯入 unigram rows 與 runtime bigram probabilities，保留其內部排序訊號。
