# Rustで音楽情報ファイル管理を1から設計し直す場合の指針

現状の課題（責務の分散・命名・型の複雑化・テスト困難など）を踏まえ、**シンプル・拡張性・テスト容易性**を重視した設計例を提案します。

---

## 1. ドメインモデルの明確化

- **ファイル単位**と**動画単位**を明確に分離
- 「年/月/ファイルパス/動画リスト」を1つのエントリ型に集約
- ルート（全体）・ファイル（単位）・動画（要素）の3層構造

---

## 2. 型設計例

```rust
/// 1ファイル分の動画情報
#[derive(Debug, Clone)]
pub struct VideoFile {
    pub year: usize,
    pub month: usize,
    pub path: crate::util::FilePath,
    pub videos: Vec<crate::model::VerifiedVideo>,
}

/// ルート全体（複数ファイル）
#[derive(Debug, Clone)]
pub struct VideoLibrary {
    pub root_dir: crate::util::DirPath,
    pub files: Vec<VideoFile>,
}
```

---

## 3. API設計例

- **VideoLibrary**
  - `fn load(dir: &DirPath) -> Result<Self, Error>`
  - `fn save(&self) -> Result<(), Error>`
  - `fn add_video(&mut self, video: VerifiedVideo) -> Result<(), Error>`
  - `fn find_video(&self, id: &VideoId) -> Option<&VerifiedVideo>`
  - `fn validate(&self) -> Result<(), ValidationError>`

- **VideoFile**
  - `fn add_video(&mut self, video: VerifiedVideo) -> Result<(), Error>`
  - `fn validate(&self) -> Result<(), ValidationError>`

---

## 4. バリデーション・エラー設計

- 動画IDの一意性は**VideoLibrary**で保証
- 年/月の整合性は**VideoFile**で保証
- エラー型は`thiserror`などで明確化

---

## 5. シリアライズ/デシリアライズ

- ファイル単位で`serde`を使い、`VideoFile`をJSON等で保存
- ルートはディレクトリ＋ファイル名規則で管理

---

## 6. テスト容易性

- ファイルI/Oを抽象化（トレイト化）し、テスト時はインメモリ実装で検証可能に
- バリデーションや追加処理は純粋関数としてテストしやすく

---

## 7. 命名・責務の明確化

- `VideoLibrary` … 全体管理
- `VideoFile` … 1ファイル管理
- `VerifiedVideo` … 1動画
- `Error`/`ValidationError` … エラー種別

---

## 8. サンプル構造

```rust
pub struct VideoLibrary {
    pub root_dir: crate::util::DirPath,
    pub files: Vec<VideoFile>,
}

impl VideoLibrary {
    pub fn load(dir: &crate::util::DirPath) -> Result<Self, Error> { ... }
    pub fn save(&self) -> Result<(), Error> { ... }
    pub fn add_video(&mut self, video: crate::model::VerifiedVideo) -> Result<(), Error> { ... }
    pub fn validate(&self) -> Result<(), ValidationError> { ... }
}

pub struct VideoFile {
    pub year: usize,
    pub month: usize,
    pub path: crate::util::FilePath,
    pub videos: Vec<crate::model::VerifiedVideo>,
}

impl VideoFile {
    pub fn add_video(&mut self, video: crate::model::VerifiedVideo) -> Result<(), Error> { ... }
    pub fn validate(&self) -> Result<(), ValidationError> { ... }
}
```

---

## 9. 設計のポイント

- **型・責務・命名をシンプルに**
- **ファイルI/Oとロジックを分離**
- **バリデーションや追加処理は純粋関数でテスト容易に**
- **エラー型は明確に分離**
- **拡張性（将来の属性追加など）を意識**

---

## まとめ

- **VideoLibrary**（全体）→**VideoFile**（ファイル単位）→**VerifiedVideo**（動画単位）の3層
- **ファイルI/O・バリデーション・追加処理・検索**などを責務ごとに分離
- **命名・型設計をシンプルにし、テスト容易性・
