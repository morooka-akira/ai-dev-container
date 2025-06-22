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