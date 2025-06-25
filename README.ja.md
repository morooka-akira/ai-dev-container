[English](README.md) | 日本語

# gitws - Git Worktree Manager

[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Git worktree を活用して複数の開発ワークスペースを管理するシンプルで強力な CLI ツール。自動セットアップと直感的な TUI で、独立した作業環境を数秒で作成できます。

## ✨ 特徴

- **🚀 高速ワークスペース作成**: 専用ブランチ付きの Git worktree を 1 コマンドで作成
- **📁 ファイル管理**: 設定ファイルを新しいワークスペースに自動コピー
- **⚡ 事前コマンド実行**: セットアップコマンド（npm install、cargo build など）を自動実行
- **🎯 インタラクティブ TUI**: 直感的なターミナルインターフェースでワークスペースの閲覧・操作・管理
- **⚙️ 設定可能**: YAML 設定ファイルで動作をカスタマイズ
- **🔗 シェル統合**: ターミナルからワークスペースへシームレスに移動

## 🛠️ インストール

### ソースコードから

```bash
git clone https://github.com/morooka-akira/gitws.git
cd gitws
cargo build --release
```

### Cargo を使用

```bash
cargo install --path .
```

## 🚀 クイックスタート

### 1. 設定ファイルの初期化（初回のみ）

```bash
gitws init
```

これにより `workspace.yml` が作成されます。必要に応じて編集してください。

### 2. 最初のワークスペースを作成

```bash
gitws start feature-authentication
```

これにより以下が作成されます：

- `../workspaces/20250625-HHMMSS-feature-authentication` に新しい Git worktree
- 新しいブランチ `work/20250625-HHMMSS-feature-authentication`
- 設定されたファイルを新しいワークスペースにコピー
- 事前設定されたセットアップコマンドを実行

### 3. ワークスペースの一覧表示と管理

**重要**: ワークスペースに移動するには、シェル関数の設定が必要です。

#### シェル関数の設定（初回のみ）

`.bashrc` または `.zshrc` に以下を追加：

```bash
# TUIでワークスペースを選択して移動
awl() {
    local target_path
    target_path=$(gitws list)
    if [ -n "$target_path" ]; then
        cd "$target_path"
    fi
}
```

設定後、シェルを再起動するか以下を実行：

```bash
source ~/.bashrc  # または source ~/.zshrc
```

#### 使用方法

```bash
awl  # TUIでワークスペースを選択して移動
```

**TUI 操作**：

- ↑/↓ または j/k でワークスペースをナビゲート
- **Enter を押してワークスペースに移動**
- Space でマルチ選択のトグル
- 'a' で全選択/全選択解除
- 'd' で選択したワークスペースを削除（確認あり）
- 'i' でワークスペースの詳細情報を表示
- 'q' を押して終了

**直接実行では移動しません**：

```bash
# ❌ これではディレクトリ移動しません
gitws list
```

## ⚙️ 設定

### 設定ファイルの生成

```bash
gitws init
# または カスタムパスに生成
gitws init --output my-config.yml
```

### 設定ファイルの例

プロジェクトルートの `workspace.yml`：

```yaml
workspace:
  # ワークスペースのベースディレクトリ
  base_dir: "../workspaces"

  # ブランチ名のプレフィックス
  branch_prefix: "work/"

  # 新しいワークスペースにコピーするファイル
  copy_files:
    - .env
    - .env.local
    - config/secrets.json

  # ワークスペース作成後に実行するコマンド
  pre_commands:
    - "npm install"
    - "cargo build"
```

## 📖 使用方法

### コマンド

#### `init`

設定ファイルテンプレートを生成します。

```bash
gitws init
gitws init --output custom.yml
gitws init -o my-config.yml
```

オプション:

- `--output <ファイル>` または `-o <ファイル>`: 出力ファイルパスを指定（デフォルト: `workspace.yml`）

#### `start <タスク名>`

指定されたタスクの新しいワークスペースを作成します。

```bash
gitws start feature-user-auth
gitws start bugfix-login --config custom.yml
```

オプション:

- `--config <ファイル>` または `-c <ファイル>`: カスタム設定ファイルを使用（デフォルト: `workspace.yml`）

#### `list`

ワークスペース管理のためのインタラクティブ TUI を開きます。

```bash
gitws list
gitws list --config custom.yml
gitws list --path-only  # パス一覧を出力（シェルスクリプト用）
```

オプション:

- `--config <ファイル>` または `-c <ファイル>`: カスタム設定ファイルを使用（デフォルト: `workspace.yml`）
- `--path-only` または `-p`: ワークスペースのパス一覧のみを出力

### TUI 操作

| キー           | アクション                               |
| -------------- | ---------------------------------------- |
| ↑/↓ または j/k | ワークスペースをナビゲート               |
| Enter          | 選択したワークスペースを開く             |
| Space          | 現在のワークスペースの選択状態をトグル   |
| a              | 全ワークスペースの選択/選択解除をトグル  |
| d              | 選択したワークスペースを削除（確認あり） |
| i              | ワークスペースの詳細を表示               |
| q/Esc          | 終了                                     |

### シェル統合

シームレスなナビゲーションのために、`.bashrc` または `.zshrc` にこの関数を追加：

```bash
# TUIでワークスペースを選択して移動
awl() {
    local target_path
    target_path=$(gitws list)
    if [ -n "$target_path" ]; then
        cd "$target_path"
    fi
}

# 全ワークスペースのパス一覧を表示
awl-list() {
    gitws list --path-only
}
```

## 🏗️ 開発

### 前提条件

- Rust 2024 edition 以降
- Git（worktree 操作のため）

### ビルド

```bash
# デバッグビルド
cargo build

# リリースビルド
cargo build --release

# テスト実行
cargo test

# コードスタイルチェック
cargo fmt
cargo clippy --all-targets -- -D warnings
```

### プロジェクト構造

```
src/
├── main.rs          # CLIエントリポイント
├── cli.rs           # コマンドライン引数解析
├── workspace.rs     # Git worktree操作
├── config.rs        # 設定ファイル処理
├── error.rs         # エラーハンドリング
├── utils.rs         # ユーティリティ関数
└── tui/             # ターミナルUIコンポーネント
    ├── mod.rs
    ├── app.rs       # アプリケーション状態
    ├── ui.rs        # UI描画
    └── events.rs    # イベント処理
```

## 🤝 コントリビューション

コントリビューションを歓迎します！プルリクエストをお気軽に送信してください。大きな変更については、まず何を変更したいかを議論するために issue を開いてください。

1. リポジトリをフォーク
2. 機能ブランチを作成（`git checkout -b feature/amazing-feature`）
3. 変更をコミット（`git commit -m 'Add some amazing feature'`）
4. ブランチにプッシュ（`git push origin feature/amazing-feature`）
5. プルリクエストを開く

## 📄 ライセンス

このプロジェクトは MIT ライセンスの下でライセンスされています。詳細については[LICENSE](LICENSE)ファイルを参照してください。

## 🙏 謝辞

- Git worktree 機能にインスパイアされました
- CLI 解析には[clap](https://github.com/clap-rs/clap)を使用
- TUI は[ratatui](https://github.com/ratatui-org/ratatui)で実装
- Git 操作には[git2](https://github.com/rust-lang/git2-rs)を使用
