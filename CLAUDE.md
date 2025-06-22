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


# Cargoコマンドやアプリケーションコマンドを確認するときに参照
@./ai-docs/rules/02_common-commands.md

# 作業を開始するときに必ず参照すること
@./ai-docs/rules/03_working-rules.md

# Rust言語固有の設定や慣習を確認するときに参照
@./ai-docs/rules/05_language-specific.md

# Rustファイルの編集をするときに参照すること
@./ai-docs/rules/06_rust-rule.md

# 実装設計と仕様を確認するときに参照
@./ai-docs/designs/design.md

# タスクの詳細と進捗を確認するときに参照
@./ai-docs/designs/tasks.md