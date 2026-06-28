# KeyKey Boneyard Bootstrap Source

這個來源是 ChiaKey Lexicon 的基礎 bootstrap 層。

為了讓 CI 與 release build 不依賴本機的 `../KeyKey-Boneyard` checkout，repo 內保留一份已知可用的 cooked database：

```text
sources/keykey-boneyard-bootstrap/vendor/KeyKeySource.db
```

對應的資料庫 SHA-256 記錄在：

```text
sources/keykey-boneyard-bootstrap/vendor/KeyKeySource.db.sha256
```

`source-inventory.sha256` 則保留當初產生這份 cooked database 時使用的 KeyKey Boneyard upstream 檔案清單與 SHA-256。也就是說：

1. `vendor/KeyKeySource.db` 是 release builder 的實際 bootstrap 輸入。
2. `source-inventory.sha256` 是這份 bootstrap DB 的來源 provenance。
3. `KeyKey-Boneyard` 的完整歷史原始資料不複製進這個 repo。

從 repository root 執行 release build：

```sh
cargo run --release -- prepare-release
```

預設會使用 vendored bootstrap DB。若要在本機測試其他 bootstrap DB，可以用 `BONEYARD_DB` override：

```sh
BONEYARD_DB=/path/to/KeyKeySource.db cargo run --release -- prepare-release
```

建置會寫出：

- `normalized/smart-mandarin.tsv`
- `manifests/lexicon-manifest.json`
- `dist/<version>/ChiaKeySource-<version>.db`
- `dist/<version>/ChiaKeySource-<version>.json`
- `dist/<version>/lexicon-manifest.json`
- `dist/<version>/SHA256SUMS`

## 中文補充（資料層）

- 資料層分類：相容性基底詞庫。
- 選用理由：ChiaKey 的 runtime 與資料庫 reader 建立在 KeyKey 既有資料形狀上，bootstrap DB 可保留既有 schema、metadata 與基本注音資料。
- 在 release 的角色：作為 release DB 的初始基底，後續資料層都疊加在這份資料上。
