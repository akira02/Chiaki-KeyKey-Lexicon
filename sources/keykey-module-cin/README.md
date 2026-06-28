# KeyKey module CIN tables

Source id: `keykey-module-cin`

This source vendors original Yahoo KeyKey / OpenVanilla CIN tables used by
runtime modules outside the Smart Mandarin language model:

- `vendor/cj-ext.cin` -> `Generic-cj-cin`
- `vendor/simplex-ext.cin` -> `Generic-simplex-cin`
- `vendor/cj-punctuations-halfwidth.cin` -> `Punctuations-cj-halfwidth-cin`
- `vendor/cj-punctuations-mixedwidth.cin` -> `Punctuations-cj-mixedwidth-cin`
- `vendor/bopomofo-correction.cin` -> `BopomofoCorrection-bopomofo-correction-cin`

Upstream paths are under:

```text
YahooKeyKey-Source-1.1.2528/DataTables/
```

The release builder imports these as module SQLite key/value tables. They
are not merged into `unigrams`, so they do not affect Smart Mandarin candidate
ranking.

Verify vendored files with:

```sh
cd sources/keykey-module-cin
shasum -a 256 -c source-inventory.sha256
```

## 中文補充（資料層）

- 資料層分類：相容性基底詞庫。
- 選用理由：KeyKey runtime 不只讀 Smart Mandarin 詞庫，也會使用其他 module tables；缺少會造成相容性破洞。
- 在 release 的角色：匯入倉頡、簡體、標點與注音校正等 module CIN tables，但不參與 Smart Mandarin 候選排序。
