# AI Workspace Manager

[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Git worktreeを活用して複数の開発ワークスペースを管理するシンプルで強力なCLIツール。自動セットアップと直感的なTUIで、独立した作業環境を数秒で作成できます。

## ✨ 特徴

- **🚀 高速ワークスペース作成**: 専用ブランチ付きのGit worktreeを1コマンドで作成
- **📁 ファイル管理**: 設定ファイルを新しいワークスペースに自動コピー
- **⚡ 事前コマンド実行**: セットアップコマンド（npm install、cargo buildなど）を自動実行
- **🎯 インタラクティブTUI**: 直感的なターミナルインターフェースでワークスペースの閲覧・操作・管理
- **⚙️ 設定可能**: YAML設定ファイルで動作をカスタマイズ
- **🔗 シェル統合**: ターミナルからワークスペースへシームレスに移動

## 🛠️ インストール

### ソースコードから

```bash
git clone https://github.com/morooka-akira/gwork.git
cd gwork
cargo build --release
```

### Cargoを使用

```bash
cargo install --path .
```

## 🚀 クイックスタート

### 1. 最初のワークスペースを作成

```bash
ai-workspace start feature-authentication
```

これにより以下が作成されます：
- `../workspaces/20250621-143022-feature-authentication` に新しいGit worktree
- 新しいブランチ `work/feature-authentication`
- 設定されたファイルを新しいワークスペースにコピー
- 事前設定されたセットアップコマンドを実行

### 2. ワークスペースの一覧表示と管理

```bash
ai-workspace list
```

インタラクティブなTUIが開き、以下の操作が可能です：
- ↑/↓ または j/k でワークスペースをナビゲート
- Enter を押してワークスペースに切り替え
- 'd' を押してワークスペースを削除
- 'i' を押して詳細情報を表示
- 'q' を押して終了

## ⚙️ 設定

プロジェクトルートに `workspace.yml` ファイルを作成：

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

#### `start <タスク名>`
指定されたタスクの新しいワークスペースを作成します。

```bash
ai-workspace start feature-user-auth
ai-workspace start bugfix-login --config custom.yml
```

オプション:
- `--config <ファイル>`: カスタム設定ファイルを使用（デフォルト: `workspace.yml`）

#### `list`
ワークスペース管理のためのインタラクティブTUIを開きます。

```bash
ai-workspace list
ai-workspace list --config custom.yml
```

### TUI操作

| キー | アクション |
|-----|--------|
| ↑/↓ または j/k | ワークスペースをナビゲート |
| Enter | 選択したワークスペースを開く |
| d | ワークスペースを削除（確認あり） |
| i | ワークスペースの詳細を表示 |
| r | ワークスペース一覧を更新 |
| q/Esc | 終了 |

### シェル統合

シームレスなナビゲーションのために、`.bashrc` または `.zshrc` にこの関数を追加：

```bash
awl() {
    local target_path
    target_path=$(ai-workspace list --print-path-only)
    if [ -n "$target_path" ]; then
        cd "$target_path"
    fi
}
```

## 🏗️ 開発

### 前提条件

- Rust 2024 edition以降
- Git（worktree操作のため）

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
├── utils.rs         # ユーティリティ関数
└── tui/             # ターミナルUIコンポーネント（予定）
    ├── mod.rs
    ├── app.rs       # アプリケーション状態
    ├── ui.rs        # UI描画
    └── events.rs    # イベント処理
```

## 🎯 ロードマップ

- [x] 基本CLIフレームワーク
- [x] Git worktree統合
- [x] 設定ファイルサポート
- [x] ファイルコピー機能
- [x] インタラクティブTUI（基本）
- [ ] 事前コマンド実行
- [ ] 高度なTUI機能（削除、詳細）
- [ ] シェル統合ヘルパー
- [ ] ワークスペーステンプレート
- [ ] 同期コマンド

## 🤝 コントリビューション

コントリビューションを歓迎します！プルリクエストをお気軽に送信してください。大きな変更については、まず何を変更したいかを議論するためにissueを開いてください。

1. リポジトリをフォーク
2. 機能ブランチを作成（`git checkout -b feature/amazing-feature`）
3. 変更をコミット（`git commit -m 'Add some amazing feature'`）
4. ブランチにプッシュ（`git push origin feature/amazing-feature`）
5. プルリクエストを開く

## 📄 ライセンス

このプロジェクトはMITライセンスの下でライセンスされています。詳細については[LICENSE](LICENSE)ファイルを参照してください。

## 🙏 謝辞

- Git worktree機能にインスパイアされました
- CLI解析には[clap](https://github.com/clap-rs/clap)を使用
- TUIは[ratatui](https://github.com/ratatui-org/ratatui)で実装
- Git操作には[git2](https://github.com/rust-lang/git2-rs)を使用

---

**[日本語のドキュメント](README.ja.md) | [English Documentation](README.md)**