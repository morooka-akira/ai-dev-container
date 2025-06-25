# gitws - Git Worktree Manager 設計

## 概要

git worktreeを活用して、複数の作業環境を簡単に管理するシンプルで強力なCLIツール。

## 主要機能

### 1. gitws init コマンド
設定ファイルのテンプレートを生成する。

```bash
# 基本的な使い方
gitws init

# カスタムパスに生成
gitws init --output custom.yml
gitws init -o my-config.yml
```

### 2. gitws start コマンド
作業用のworktreeを作成し、環境をセットアップする。

```bash
# 基本的な使い方
gitws start <task-name>

# オプション付き
gitws start feature-x --config .gitws.yml
```

**動作:**
- git worktreeで新しいブランチとディレクトリを作成
- 設定ファイルに基づいてファイルをコピー
- 事前実行コマンドを実行
- 作成したディレクトリパスを表示

### 3. gitws list コマンド
インタラクティブなTUIでワークスペースを管理する。

```bash
# インタラクティブなTUI画面を起動
gitws list

# パス一覧のみ出力（シェルスクリプト用）
gitws list --path-only
gitws list -p
```

**TUI操作:**
- ↑/↓ または j/k: ワークスペース選択
- Enter: 選択したワークスペースに移動
- Space: 現在のワークスペースの選択状態をトグル（マルチ選択）
- a: 全ワークスペースの選択/選択解除をトグル
- d: 選択したワークスペース（単体または複数）を削除
- i: 詳細情報を表示
- q/Esc: 終了

## 設定ファイル形式

```yaml
# .gitws.yml
workspace:
  # worktreeのベースディレクトリ
  base_dir: "../workspaces"
  
  # ブランチ名のプレフィックス
  branch_prefix: "work/"
  
  # コピーするファイル一覧
  copy_files:
    - .env
    - .env.local
    - config/secrets.json
    
  # 事前実行コマンド
  pre_commands:
    - "npm install"
    - "cargo build"
```

## 実装アーキテクチャ

### ディレクトリ構造
```
gitws/
├── src/
│   ├── main.rs           # CLIエントリポイント
│   ├── cli.rs            # コマンドライン引数処理
│   ├── workspace.rs      # worktree操作
│   ├── config.rs         # 設定ファイル読み込み・テンプレート生成
│   ├── error.rs          # エラーハンドリング
│   ├── utils.rs          # ユーティリティ関数
│   └── tui/
│       ├── mod.rs        # TUIモジュール
│       ├── app.rs        # アプリケーション状態管理
│       ├── ui.rs         # UI描画ロジック
│       └── events.rs     # キーイベント処理
├── Cargo.toml
└── .gitws.yml         # デフォルト設定ファイル
```

### 主要な依存クレート
- `clap`: CLIパーサー
- `serde` + `serde_yaml`: 設定ファイル読み込み
- `git2`: Git操作
- `ratatui`: TUIフレームワーク
- `crossterm`: ターミナル操作
- `tokio`: 非同期実行時

## 詳細仕様

### gitws start の処理フロー

1. **引数とオプションの解析**
   - タスク名（必須）
   - 設定ファイルパス（オプション、デフォルト: ./.gitws.yml）

2. **worktree作成**
   ```rust
   // 擬似コード
   let workspace_name = format!("{}-{}", timestamp(), task_name);
   let branch_name = format!("{}{}-{}", config.branch_prefix, timestamp(), task_name);
   let workspace_path = format!("{}/{}", config.base_dir, workspace_name);
   
   git_worktree_add(&workspace_path, &branch_name);
   ```

3. **ファイルコピー**
   - 設定ファイルに指定されたファイルを新しいworktreeにコピー
   - 存在しないファイルはスキップ（警告表示）

4. **事前コマンド実行**
   - 新しいworktreeディレクトリで指定コマンドを実行
   - エラーが発生しても処理は継続（警告表示）

5. **結果表示**
   ```
   ✅ Workspace created: ../workspaces/20250625-143022-feature-x
   📁 Branch: work/20250625-143022-feature-x
   
   To enter the workspace:
     cd ../workspaces/20250625-143022-feature-x
   ```

### gitws list TUIの詳細設計

#### 画面レイアウト
```
┌─ gitws Git Worktree Manager ───────────────────────┐
│ Workspace Management                                │
├─────────────────────────────────────────────────────┤
│ ↑/↓ Select  Space: Multi-select  a: All  d: Delete │
│ i: Details  q: Quit                                 │
├─────────────────────────────────────────────────────┤
│ → [*] work/20250625-143022-feature-x              │
│   └─ ../workspaces/20250625-143022-feature-x       │
│                                                     │
│   [ ] work/20250625-140512-bugfix-y                │
│   └─ ../workspaces/20250625-140512-bugfix-y        │
│                                                     │
│   [*] work/20250625-120345-refactor-z              │
│   └─ ../workspaces/20250625-120345-refactor-z      │
│                                                     │
├─────────────────────────────────────────────────────┤
│ Selected: 2/3 workspaces                            │
└─────────────────────────────────────────────────────┘
```

#### キーバインディング
- **↑/↓ または j/k**: ワークスペース選択移動
- **Enter**: 選択ワークスペースに移動（シェルスクリプト出力）
- **Space**: 現在のワークスペースの選択状態をトグル（マルチ選択）
- **d**: 選択したワークスペース（単体または複数）の削除確認ダイアログ
- **a**: 全てのワークスペースを選択/選択解除
- **i**: 詳細情報モーダル表示
- **r**: ワークスペース一覧リフレッシュ
- **q または Esc**: TUI終了

#### 詳細情報モーダル
```
┌─ Workspace Details ─────────────────┐
│ Branch: work/feature-x              │
│ Path: ../workspaces/20250619-...    │
│ Created: 2025-06-19 14:30:22        │
│ Last Modified: 2025-06-19 16:45:12  │
│ Status: Clean                       │
│ Files: 42 tracked, 3 untracked     │
│ Size: 1.2MB                         │
│                                     │
│ Recent Commits:                     │
│ - Fix input validation (2h ago)     │
│ - Add error handling (5h ago)       │
│                                     │
│ Press any key to close              │
└─────────────────────────────────────┘
```

#### 削除確認ダイアログ

**単体削除の場合：**
```
┌─ Delete Workspace ──────────┐
│ Are you sure you want to    │
│ delete this workspace?      │
│                             │
│ work/feature-x              │
│ ../workspaces/20250619-...  │
│                             │
│ [Y]es  [N]o                 │
└─────────────────────────────┘
```

**バルク削除の場合：**
```
┌─ Delete Multiple Workspaces ──────┐
│ Are you sure you want to delete   │
│ these 2 workspaces?               │
│                                   │
│ • work/feature-x                  │
│ • work/refactor-z                 │
│                                   │
│ This action cannot be undone.     │
│                                   │
│ [Y]es  [N]o                       │
└───────────────────────────────────┘
```

#### ディレクトリ移動の実装
TUIツール終了時に、選択されたパスを標準出力に出力し、シェル関数で`cd`を実行：

```bash
# ~/.bashrcまたは~/.zshrcに追加
awl() {
    local target_path
    target_path=$(gitws list)
    if [ -n "$target_path" ]; then
        cd "$target_path"
    fi
}

# パス一覧のみ出力（スクリプト用）
awl-list() {
    gitws list --path-only
}
```

## エラーハンドリング

- Git worktree作成エラー: 明確なエラーメッセージを表示
- ファイルコピーエラー: 警告を表示して継続
- 事前コマンド実行エラー: 警告を表示して継続
- 設定ファイル不在: デフォルト設定で動作
- 非Gitリポジトリでの実行: 適切なエラーメッセージを表示（init コマンドは除く）

GitwsError型で構造化されたエラーハンドリングを提供：
- IoError: ファイル操作エラー
- GitError: Git操作エラー
- ConfigError: 設定ファイルエラー
- ValidationError: 入力値検証エラー

## 実装済み機能

### 完了済み
- ✅ 基本CLI構造（clap derive API）
- ✅ YAML設定ファイル読み込み（serde + serde_yaml）
- ✅ Git worktree作成機能（git2）
- ✅ TUI基本機能（ratatui + crossterm）
- ✅ ワークスペース一覧表示
- ✅ TUI マルチ選択機能（Space、'a'キー）
- ✅ TUI 削除機能（単体・バルク削除）
- ✅ ワークスペース詳細情報表示
- ✅ gitws init コマンド（設定テンプレート生成）
- ✅ エラーハンドリング強化
- ✅ 国際化対応（TUIメッセージ）

### 今後の拡張案

1. **gitws clean** コマンド
   - 不要になったworktreeを削除

2. **gitws sync** コマンド
   - メインブランチの変更を取り込む

3. **テンプレート機能**
   - タスクタイプ別の設定テンプレート

4. **履歴機能**
   - 過去に作成したworkspaceの履歴管理

5. **設定ファイル管理**
   - プロジェクト別設定ファイルの管理