# Chiaki KeyKey Lexicon

[English](README.en.md)

Chiaki KeyKey Lexicon 是 Chiaki KeyKey 的詞庫資料 repository。

主 app repository 應該專注在 macOS 輸入法 runtime、資料庫讀取、builder script、安裝工具，以及一份小型 bundled fallback database。這個 repository 則負責持續演進的詞庫資料、來源 manifest、授權紀錄、normalized intermediate data、release database artifacts、checksums 與 changelog。

## 分工

`Chiaki-KeyKey` 負責：

1. macOS IMK runtime。
2. 輸入引擎整合。
3. 資料庫 schema 與 reader。
4. 可消費此 repo normalized data 的 builder script。
5. bundled fallback `KeyKeySource.db`。

`Chiaki-KeyKey-Lexicon` 負責：

1. source manifests。
2. source license 與 attribution records。
3. normalized lexicon data。
4. release-ready `KeyKeySource` database artifacts。
5. checksums 或 signatures。
6. lexicon release changelog。

## 目前狀態

這個 repository 已有可運作的 seed release pipeline。push 到 `main` 會透過 GitHub Actions 建置並發布版本化詞庫 release。

目前 pipeline 會從已審查來源資料、專案維護修正、生成 metadata、source inventories 與 checksum manifests 建出完整的 `KeyKeySource.db`。本機 release artifacts 會輸出到 `dist/<version>/`，CI 則會上傳到 GitHub Releases。

建議先看：

- [Docs/ReleaseFlow.zh-TW.md](Docs/ReleaseFlow.zh-TW.md)
- [Docs/SourceReview.md](Docs/SourceReview.md)

下載 pinned external source files：

```sh
cargo run --release -- fetch-modern-sources
```

建立本機 release package：

```sh
cargo run --release -- prepare-release
```

## 架構

這個 repository 以可重現的資料 pipeline 為核心：

1. `sources/<source-id>/` 放每個已審查 input source、本地 README，以及 `source-inventory.sha256` provenance file。
2. `LICENSES/` 記錄每個可公開 release source 所需的 license text 或 license notes。
3. `src/` 是 Rust release toolchain，負責驗證 inputs、將資料層匯入 KeyKey database shape、寫出 normalized TSV、更新 release metadata、產生 manifests。
4. `normalized/smart-mandarin.tsv` 是 Smart Mandarin language-model rows 的 generated normalized interchange view。
5. `manifests/lexicon-manifest.json` 是 app 端消費的 generated update contract。
6. `dist/<version>/` 是本機 release artifacts staging 目錄，不 commit。

資料層大致分成四類：

1. **Runtime compatibility data**：app 既有 database reader 與 input modules 需要的 KeyKey-origin data。
2. **Lexicon sources**：現代繁中 / 注音詞彙，以及補充字詞 coverage。
3. **Project-owned corrections**：小型 overlay，用來修已知輸入缺漏、指定讀音、調整候選排序。
4. **Policy layers**：小型已審查規則，讓預設繁中 release 符合 app 的語言與地區期待。

各來源的授權、redistribution decision 與風險紀錄放在 [Docs/SourceReview.md](Docs/SourceReview.md)。日常 release 操作放在 [Docs/ReleaseFlow.zh-TW.md](Docs/ReleaseFlow.zh-TW.md)。

## Repository 目錄

```text
Docs/
  ReleaseFlow.zh-TW.md
  SourceReview.md
LICENSES/
  README.md
src/
  main.rs
manifests/
  lexicon-manifest.example.json
normalized/
  .gitkeep
schemas/
  lexicon-manifest.schema.json
sources/
  .gitkeep
```

建置完成的 release artifacts 不會 commit 進 git。請用 `dist/` 之類的本機 staging 目錄，再由 GitHub Releases 發布 artifacts。

## Release 內容

GitHub Release 應發布：

```text
KeyKeySource-YYYY.MM.N.db
KeyKeySource-YYYY.MM.N.json
lexicon-manifest.json
SHA256SUMS
```

主 app 應下載並驗證 `lexicon-manifest.json`，再把相容的 `KeyKeySource` database 安裝到：

```text
~/Library/Application Support/Chiaki KeyKey/Lexicons/
```

runtime 載入資料庫時，若 active external database 不存在、無效或不相容，應 fallback 到 bundled database。

## 授權政策

Rust release tooling 與 repository scripts 使用 MIT License；見 [LICENSE-CODE](LICENSE-CODE)。

詞庫資料沒有單一 repository-wide license。

每個 source 都必須在公開 release 前宣告自己的 license。未知授權資料只能做本機實驗，不可包含在 public release artifacts。
