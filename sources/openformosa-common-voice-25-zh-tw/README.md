# OpenFormosa Common Voice 25 zh-TW Bigram Overlay

This source contains selected runtime bigram rows derived from OpenFormosa's
Common Voice 25 zh-TW validated sentences dataset.

Only reviewed, release-ready bigram rows are redistributed here:

```text
qstring<TAB>previous<TAB>current<TAB>probability
```

The candidate rows were generated with the repository's `build-bigram-stats`
workflow against the current normalized ChiaKey lexicon.

## Bigram calibration

These bigram rows are imported as conditional log-probabilities, but the final
ChiaKey unigram table uses its own lifted weight scale. During release import,
the builder re-anchors this source's bigrams to the current unigram floor:

```text
stored = min(unigram(current) + boost + (raw - raw_max_of_source), -0.05)
```

This keeps the source's internal ordering while letting strong disambiguation
edges beat the unigram path. Weaker pairs stay below the unigram floor and remain
inert. The default `boost` is `1.5` and can be overridden with
`COMMONVOICE_BIGRAM_BOOST`; setting it to `0` leaves the raw values unchanged.

## Files

- `bigrams.tsv`: selected Common Voice-derived runtime bigram rows.

## Source

- Dataset: <https://huggingface.co/datasets/OpenFormosa/common_voice_25_zh-TW>
- Source file: `validated_sentences.tsv`
- License: CC0-1.0
- Provider: OpenFormosa / Mozilla Common Voice

## 中文補充（資料層）

- 資料層分類：專案詞庫。
- 選用理由：Common Voice zh-TW 提供公開且可再散布的句級語料，適合挑選成 runtime bigram 補充。
- 在 release 的角色：匯入已審查 bigram rows，強化詞間連接排序，不保存原始語音句庫。
