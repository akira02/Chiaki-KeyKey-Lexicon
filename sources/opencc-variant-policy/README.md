# OpenCC-derived Traditional Chinese variant policy

Source id: `opencc-variant-policy`

This source contains small, reviewed Traditional Chinese variant preferences
derived from OpenCC's simplified/traditional conversion policy. It is not a
frequency dictionary. The release builder uses it only to demote Simplified or
non-Taiwan-preferred variants when those forms otherwise tie with Traditional
Chinese candidates.

The table format is:

```text
phrase<TAB>max_weight<TAB>tags
```

`max_weight` is an upper bound. Existing candidates above that value are lowered
to the bound; candidates already below it are left unchanged.

Upstream reference:

- <https://github.com/BYVoid/OpenCC>

Verify vendored files with:

```sh
cd sources/opencc-variant-policy
shasum -a 256 -c source-inventory.sha256
```

## 中文補充（資料層）

- 資料層分類：校正層。
- 選用理由：預設繁中輸入不應因 tie-break 讓簡體或非台灣慣用 variant 排在前面。
- 在 release 的角色：以 policy cap 降低指定 variant 的最大權重，讓候選排序更符合 zh-TW 預期。
