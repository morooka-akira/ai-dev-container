# プロジェクト概要

## 基本情報

プロジェクト名: `ai-dev-container` (AI Workspace Manager)
言語: Rust 2024 edition
タイプ: バイナリアプリケーション

## プロジェクトの目的

Git worktreeを活用して複数の開発ワークスペースを管理するシンプルで強力なCLIツール。
自動セットアップと直感的なTUIで、独立した作業環境を数秒で作成できます。

## 主要機能

- **高速ワークスペース作成**: 専用ブランチ付きのGit worktreeを1コマンドで作成
- **ファイル管理**: 設定ファイルを新しいワークスペースに自動コピー
- **事前コマンド実行**: セットアップコマンド（npm install、cargo buildなど）を自動実行
- **インタラクティブTUI**: 直感的なターミナルインターフェースでワークスペースの閲覧・操作・管理
- **設定可能**: YAML設定ファイルで動作をカスタマイズ
- **シェル統合**: ターミナルからワークスペースへシームレスに移動

## プロジェクト構造

```
src/
├── main.rs          # CLIエントリポイント
├── cli.rs           # コマンドライン引数処理
├── workspace.rs     # worktree操作
├── config.rs        # 設定ファイル読み込み
├── utils.rs         # ユーティリティ関数
└── tui/             # ターミナルUIコンポーネント（予定）
    ├── mod.rs
    ├── app.rs       # アプリケーション状態管理
    ├── ui.rs        # UI描画ロジック
    └── events.rs    # キーイベント処理
```

## 現在の実装状況

- [x] 基本CLIフレームワーク (Task 1-1, 1-2)
- [x] 設定ファイル機能 (Task 2-1, 2-2)
- [ ] Git Worktree機能
- [ ] TUI機能
- [ ] ファイル操作機能

# 共通コマンド

## Cargo コマンド

```bash
# プロジェクトをビルド
cargo build

# アプリケーションを実行
cargo run

# リリースモードでビルド・実行
cargo run --release

# テストを実行
cargo test

# 特定のテストを実行
cargo test test_name

# ビルドせずにコードをチェック
cargo check

# コードをフォーマット
cargo fmt

# リンターを実行
cargo clippy --all-targets -- -D warnings
```

## アプリケーションコマンド

```bash
# 新しいワークスペースを作成
cargo run -- start <task-name>

# カスタム設定ファイルを使用
cargo run -- start <task-name> --config custom.yml

# ワークスペース一覧をTUIで表示
cargo run -- list

# ヘルプを表示
cargo run -- --help
```

## Git コマンド

```bash
# ワークスペースの手動削除
git worktree remove <path>

# ワークスペース一覧
git worktree list
```

## 開発フロー必須コマンド

実装完了後は必ず以下を実行:

```bash
# 1. フォーマット
cargo fmt

# 2. 警告チェック
cargo clippy --all-targets -- -D warnings

# 3. テスト実行
cargo test

# 4. PRの作成
gh pr create
```

# 現在の実装状況

## 完了したタスク

### Phase 1: 基本CLI構造の構築
- [x] Task 1-1: 基本CLIフレームワークの実装
- [x] Task 1-2: プロジェクト構造の整備

### Phase 2: 設定ファイル機能の実装
- [x] Task 2-1: 設定ファイル構造体の定義
- [x] Task 2-2: 設定ファイル読み込み機能

## 現在の機能

### 1. CLI基本機能
- clapベースのCLI引数解析
- start/listコマンドの基本構造
- ヘルプメッセージ表示

### 2. 設定ファイル機能
- YAML形式の設定ファイル読み込み
- --configオプションでのファイル指定
- エラー時のデフォルト設定フォールバック
- 設定構造体の定義（WorkspaceConfig/WorkspaceSettings）

### 3. プロジェクト構造
- モジュール分割（cli, config, workspace, utils）
- 基本的なエラーハンドリング
- タイムスタンプ生成ユーティリティ

## 次の実装予定

### Phase 3: Git Worktree機能の実装
- [ ] Task 3-1: Git操作の基本実装
- [ ] Task 3-2: workspace startコマンドの実装

## 依存関係

```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
```

## コード品質

- cargo fmt: ✅ 通過
- cargo clippy: ✅ 警告なし
- cargo test: ✅ 通過（0テスト）

## 最新PR

- [PR #6](https://github.com/morooka-akira/gwork/pull/6): 設定ファイル読み込み機能の実装

# 言語固有の設定

## 回答言語

**回答は日本語で答えてください**

## Rust Edition

プロジェクトはRust 2024 editionを使用しています。

## 標準的なRust/Cargoの慣習

プロジェクトは標準的なRust/Cargoの慣習に従います：

- `Cargo.toml`: プロジェクトマニフェストと依存関係
- `src/main.rs`: アプリケーションエントリポイント
- `src/lib.rs`: ライブラリのエントリポイント（該当する場合）
- モジュール構造は機能別に分割

## コーディング規約

### 命名規則
- 関数名: snake_case
- 構造体名: PascalCase  
- 定数: SCREAMING_SNAKE_CASE
- モジュール名: snake_case

### エラーハンドリング
- `Result<T, E>`型を適切に使用
- `?`演算子でエラー伝播
- 具体的なエラーメッセージを提供

### 非同期処理
- 必要に応じて`async/await`を使用
- `tokio`ランタイムを使用

### テスト
- 単体テストは各モジュール内の`#[cfg(test)]`モジュール
- 統合テストは`tests/`ディレクトリ
- `cargo test`で全テスト実行


# 作業を開始するときに参照すること
@./ai-docs/contexts/03_working-rules.md

# Rustファイルの編集をするときに参照すること
@./ai-docs/contexts/06_rust-rule.md