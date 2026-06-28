# KeyKey BPMF Punctuation Table

Source id: `keykey-punctuations-cin`

This source vendors the original KeyKey BPMF punctuation CIN table:

```text
sources/keykey-punctuations-cin/vendor/bpmf-punctuations.cin
```

The upstream source path is:

```text
YahooKeyKey-Source-1.1.2528/DataTables/bpmf-punctuations.cin
```

The release builder imports only rows inside `%chardef` whose keys start with `_punctuation_` or `_ctrl_`. These rows are required by Smart Mandarin runtime punctuation lookup, for example Shift+, resolves `_punctuation_<` and expects `，`.

The source inventory is stored at:

```text
sources/keykey-punctuations-cin/source-inventory.sha256
```

## 中文補充（資料層）

- 資料層分類：相容性基底詞庫。
- 選用理由：Smart Mandarin runtime 會查 `_punctuation_*`、`_ctrl_*`，缺少這批資料會導致標點查表異常。
- 在 release 的角色：提供 BPMF 標點與控制符號相容資料，確保既有按鍵行為不變。
