# Public Domain Extended BPMF Character Table

Source id: `bpmf-ext-cin`

This source vendors the public-domain `bpmf-ext.cin` character table from the KeyKey Boneyard tree:

```text
sources/bpmf-ext-cin/vendor/bpmf-ext.cin
```

The file header says it was revised from `opendesktop.org.tw`'s `phone.cin` to include CNS11643 and Unicode-compatible characters, with license marked as Public Domain.

The release builder uses this source only as a low-priority single-character reading supplement:

1. It imports CJK BMP characters only.
2. It excludes non-BMP and private-use characters.
3. It only adds missing exact `(reading, character)` pairs.
4. It does not override libchewing character frequencies.

This source fills character-level gaps such as the native/Yahoo `ㄨㄛˇ` candidate set:

```text
我 婐 捰 倭 䂺 婑 䰀 㦱
```

The source inventory is stored at:

```text
sources/bpmf-ext-cin/source-inventory.sha256
```

## 中文補充（資料層）

- 資料層分類：外部詞庫。
- 選用理由：libchewing 與 bootstrap 仍可能缺少部分單字候選，這份 public-domain CIN 能補齊 coverage。
- 在 release 的角色：只補缺失的單字 `(reading, character)` pair，不覆蓋既有資料權重。
